use std::fmt::Display;

use lingua::LanguageDetectorBuilder;
use strum::{EnumIter, IntoEnumIterator};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LanguageError {
    #[error("the specified language is not supported")]
    LanguageNotSupported,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum Language {
    En,
    Ru,
}

impl From<Language> for lingua::Language {
    fn from(value: Language) -> Self {
        use Language::*;
        match value {
            En => lingua::Language::English,
            Ru => lingua::Language::Russian,
        }
    }
}

impl From<lingua::Language> for Language {
    fn from(value: lingua::Language) -> Self {
        use Language::*;
        match value {
            lingua::Language::English => En,
            lingua::Language::Russian => Ru,
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Language::*;
        match self {
            En => write!(f, "en"),
            Ru => write!(f, "ru"),
        }
    }
}

pub struct LanguageDetector {
    detector: lingua::LanguageDetector,
}

impl LanguageDetector {
    pub fn new() -> Self {
        let languages: Vec<lingua::Language> = Language::iter().map(|lang| lang.into()).collect();
        let detector = LanguageDetectorBuilder::from_languages(&languages).build();
        Self { detector }
    }

    pub fn recognize(&self, text: &str) -> Result<Language, LanguageError> {
        self.detector
            .detect_language_of(text)
            .map(|detected_language| detected_language.into())
            .ok_or(LanguageError::LanguageNotSupported)
    }
}
impl Default for LanguageDetector {
    fn default() -> Self {
        Self::new()
    }
}
