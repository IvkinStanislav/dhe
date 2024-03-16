mod cli_command;
mod translate;

use clap::Parser;
use dhe_sdk::setup_logs;
use tracing::Level;

use crate::{cli_command::CliCommandsConfig, translate::start_translate_loop};

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
        commands_config.execute_bash_init_commands().unwrap();
    }

    start_translate_loop(commands_config.translate_params()?.into_iter()).await?;

    Ok(())
}
