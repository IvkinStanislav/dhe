use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Language {
    En,
    Ru,
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
