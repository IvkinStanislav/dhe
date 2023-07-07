use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::DbErr;
use thiserror::Error;

use crate::db::CorruptedDataError;

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("database error: {0}")]
    Db(#[from] DbErr),
    #[error("{0}")]
    CorruptedData(#[from] CorruptedDataError),
    #[error("not found")]
    NotFound,
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
