use std::{fs, process::Command, thread::sleep, time::Duration};

use clap::Parser;
use dhe_sdk::setup_logs;
use serde::Deserialize;
use tracing::Level;

#[derive(Deserialize)]
struct CommandsConfig {
    // TODO hotkeys
    commands: Vec<ConsoleCommand>,
}

#[derive(Deserialize)]
struct ConsoleCommand {
    name: String,
    #[serde(default)]
    args: Vec<String>,
}

const DEFAULT_COMMAND_FILE_NAME: &str = "dhe_commands.toml";

#[derive(Parser)]
#[clap(version, about, long_about)]
struct Cli {
    #[arg(short, long)]
    commands_file: Option<String>,
}

fn main() {
    setup_logs(Level::INFO);
    let Cli { commands_file } = Cli::parse();

    let commands_file = commands_file.unwrap_or_else(|| {
        let err = "command configuration not specified";
        match homedir::get_my_home() {
            Ok(Some(home)) => home
                .join(DEFAULT_COMMAND_FILE_NAME)
                .to_str()
                .expect(err)
                .to_owned(),
            _ => panic!("{err}"),
        }
    });

    let commands_data = fs::read_to_string(&commands_file)
        .unwrap_or_else(|_| panic!("command file at {commands_file} path not found"));
    let CommandsConfig { commands } = toml::from_str(&commands_data)
        .unwrap_or_else(|_| panic!("wrong file format at {commands_file} path"));

    for command in commands {
        sleep(Duration::from_millis(300));
        let ConsoleCommand { name, args } = command;

        Command::new(&name)
            .args(args)
            .spawn()
            .unwrap_or_else(|_| panic!("failed to execute process {name} with args {name:?}"));
    }
}
