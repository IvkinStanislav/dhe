use std::{fs, process::Command};

use dhe_sdk::setup_logs;
use serde::Deserialize;
use tracing::Level;

const COMMANDS_FILE: &str = "./commands.toml";

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

fn main() {
    setup_logs(Level::INFO);

    let commands_data = fs::read_to_string(COMMANDS_FILE)
        .unwrap_or_else(|_| panic!("command file at {COMMANDS_FILE} path not found"));
    let CommandsConfig { commands } = toml::from_str(&commands_data)
        .unwrap_or_else(|_| panic!("wrong file format at {COMMANDS_FILE} path"));

    for command in commands {
        let ConsoleCommand { name, args } = command;

        Command::new(&name)
            .args(args)
            .spawn()
            .unwrap_or_else(|_| panic!("failed to execute process {name} with args {name:?}"));
    }
}
