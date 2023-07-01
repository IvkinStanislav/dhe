use sea_orm::{Database, DatabaseConnection, DbErr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StateError {
    #[error("failed to connect to database")]
    DatabaseConnect(#[from] DbErr),
}

pub struct StateConfig {
    pub db_conn_str: String,
}

pub struct AppState {
    pub db_conn: DatabaseConnection,
}

impl AppState {
    pub async fn create(config: StateConfig) -> Result<Self, StateError> {
        let StateConfig { db_conn_str } = config;

        let db_conn = Database::connect(db_conn_str).await?;

        Ok(Self { db_conn })
    }
}
