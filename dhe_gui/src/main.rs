mod gui;

use dhe_sdk::setup_logs;
use tracing::Level;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logs(Level::INFO);
    gui::run()?;
    Ok(())
}
