mod config;
mod db;
mod entity;
mod migration;
mod rest;
mod state;

use std::{error::Error, sync::Arc};

use axum::Server;
use config::{DB_NAME, SERVER_ADDRESS};
use dhe_sdk::setup_logs;
use migration::Migrator;
use rest::create_router;
use sea_orm_migration::MigratorTrait;
use state::{AppState, StateConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    setup_logs();

    let db_dir = dirs::home_dir()
        .and_then(|p| p.to_str().map(|p| p.to_string()))
        .unwrap_or(String::new());
    let db_conn_str = format!("sqlite:{db_dir}/{DB_NAME}");

    let config = StateConfig { db_conn_str };
    let state = Arc::new(AppState::create(config).await?);

    Migrator::up(&state.db_conn, None).await?;

    let server_address = SERVER_ADDRESS.parse().unwrap();
    Server::bind(&server_address)
        .serve(create_router(state).into_make_service())
        .await?;
    Ok(())
}
