use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    thread::sleep,
    time::Duration,
};

use serde::Deserialize;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum CliCommandError {
    #[error("cli commands configuration file not specified")]
    CliCommandsNotFound,
    #[error("failed to read cli commands configuration file at {0} path")]
    ReadCliCommands(PathBuf),
    #[error("wrong cli commands configuration file format at {0} path")]
    WrongFormatCliCommands(PathBuf),
    #[error("failed to execute process {0} with args {1:?}")]
    ExecuteCliCommand(String, Vec<String>),
}

#[derive(Deserialize)]
pub struct CliCommandsConfig {
    commands: Vec<CliCommand>,
}

#[derive(Deserialize)]
struct CliCommand {
    // TODO hotkeys
    name: String,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    init: bool,
}

impl CliCommandsConfig {
    pub fn parse<P: AsRef<Path>>(commands_file_path: Option<P>) -> Result<Self, CliCommandError> {
        use CliCommandError::*;

        let path = commands_file_path
            .map(|p| p.as_ref().to_path_buf())
            .or_else(Self::default_cli_command_file_name)
            .ok_or(CliCommandsNotFound)?;

        let commands_data = fs::read_to_string(&path).map_err(|_| ReadCliCommands(path.clone()))?;
        toml::from_str(&commands_data).map_err(|_| WrongFormatCliCommands(path))
    }

    pub fn execute_init_commands(&self) -> Result<(), CliCommandError> {
        for command in self.commands.iter().filter(|c| c.init) {
            sleep(Duration::from_millis(300));
            let CliCommand { name, args, .. } = command;

            Command::new(name).args(args).spawn().map_err(|err| {
                error!("cli command {name} {args:?} error: {err}");
                CliCommandError::ExecuteCliCommand(name.clone(), args.clone())
            })?;
        }

        Ok(())
    }

    fn default_cli_command_file_name() -> Option<PathBuf> {
        match homedir::get_my_home() {
            Ok(Some(home)) => Some(home.join("dhe_commands.toml")),
            _ => None,
        }
    }
}
