pub mod keyboard;
pub mod language;
pub mod translate;
pub mod algorithm;

use language::LanguageError;
use thiserror::Error;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::keyboard::KeyboardError;
use crate::translate::TranslateError;

#[derive(Error, Debug)]
pub enum DheError {
    #[error("language error: {0}")]
    Language(LanguageError),
    #[error("translate error: {0}")]
    Translate(TranslateError),
    #[error("keyboard error: {0}")]
    Keyboard(KeyboardError),
}

/// Launching the logging system.
pub fn setup_logs(level: Level) {
    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
