use std::process::Command;

use dhe_sdk::setup_logs;
use tracing::Level;

fn main() {
    setup_logs(Level::INFO);

    Command::new("gnome-terminal")
        .args(["--tab", "--working-directory=./dev"])
        .spawn()
        .expect("failed to execute process");
}
