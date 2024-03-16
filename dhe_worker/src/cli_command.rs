use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
    thread::sleep,
    time::Duration,
};

use dhe_sdk::keyboard::Key;
use serde::Deserialize;
use thiserror::Error;
use tracing::error;

use crate::translate::TranslateParam;

#[derive(Error, Debug)]
pub enum CliCommandError {
    #[error("cli commands configuration file not specified")]
    CommandsFileNotFound,
    #[error("failed to read cli commands configuration file at {0} path")]
    ReadCommands(PathBuf),
    #[error("wrong cli commands configuration file format at {0} path")]
    WrongCommandsFormat(PathBuf),
    #[error("failed to execute/validate command {0} with args {1:?}; error: {2}")]
    ExecuteCommand(String, Vec<String>, String),
}

#[derive(Deserialize)]
pub struct CliCommandsConfig {
    commands: Vec<CliCommand>,
}

impl CliCommandsConfig {
    pub fn parse<P: AsRef<Path>>(commands_file_path: Option<P>) -> Result<Self, CliCommandError> {
        use CliCommandError::*;

        let path = commands_file_path
            .map(|p| p.as_ref().to_path_buf())
            .or_else(Self::default_cli_command_file_name)
            .ok_or(CommandsFileNotFound)?;

        let commands_data = fs::read_to_string(&path).map_err(|_| ReadCommands(path.clone()))?;
        toml::from_str(&commands_data).map_err(|_| WrongCommandsFormat(path))
    }

    pub fn execute_bash_init_commands(&self) -> Result<(), CliCommandError> {
        for command in self.by_handler(CliCommandHandler::BashStarter) {
            sleep(Duration::from_millis(300));
            let CliCommand { name, args, .. } = command;

            Command::new(name).args(args).spawn().map_err(|err| {
                error!("cli command {name} {args:?} error: {err}");
                CliCommandError::ExecuteCommand(name.clone(), args.clone(), err.to_string())
            })?;
        }

        Ok(())
    }

    pub fn translate_params(&self) -> Result<Vec<TranslateParam<&str>>, CliCommandError> {
        self.by_handler(CliCommandHandler::Translator)
            .map(|c| {
                let param = TranslateParam {
                    name: c.name.as_str(),
                    keys: c
                        .args
                        .iter()
                        .map(|a| {
                            Key::from_str(a).map_err(|err| {
                                CliCommandError::ExecuteCommand(
                                    c.name.clone(),
                                    c.args.clone(),
                                    err.to_string(),
                                )
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                };
                param.validation().map_err(|err| {
                    CliCommandError::ExecuteCommand(c.name.clone(), c.args.clone(), err.to_string())
                })?;
                Ok(param)
            })
            .collect()
    }

    fn by_handler(&self, handler: CliCommandHandler) -> impl Iterator<Item = &CliCommand> {
        self.commands.iter().filter(move |&c| c.handler == handler)
    }

    fn default_cli_command_file_name() -> Option<PathBuf> {
        match homedir::get_my_home() {
            Ok(Some(home)) => Some(home.join("dhe_commands.toml")),
            _ => None,
        }
    }
}

#[derive(Deserialize)]
pub struct CliCommand {
    name: String,
    handler: CliCommandHandler,
    #[serde(default)]
    args: Vec<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CliCommandHandler {
    BashStarter,
    Translator,
}
