use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct CorruptedDataError {
    pub table: String,
    pub id: String,
    pub column: String,
}

impl CorruptedDataError {
    pub fn new(table: String, id: String, column: String) -> Self {
        CorruptedDataError { table, id, column }
    }
}

impl Display for CorruptedDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "corrupted data in table {}, row id {} column {}",
            self.table, self.id, self.column
        )
    }
}

impl Error for CorruptedDataError {}
