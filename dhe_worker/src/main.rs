mod action_listener;
mod cli_command;

use clap::Parser;
use dhe_sdk::setup_logs;
use tracing::Level;

use crate::{action_listener::start_action_listener_loop, cli_command::CliCommandsConfig};

#[derive(Parser)]
#[clap(version, about, long_about)]
struct Cli {
    #[arg(short, long)]
    commands_file: Option<String>,
    #[arg(short, long, default_value_t = false)]
    init: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logs(Level::INFO);
    let Cli {
        commands_file,
        init,
    } = Cli::parse();

    let commands_config = CliCommandsConfig::parse(commands_file).unwrap();
    if init {
        commands_config.execute_bash_starter_commands().unwrap();
    }

    start_action_listener_loop(commands_config.action_listener_params()?.into_iter()).await?;

    Ok(())
}
