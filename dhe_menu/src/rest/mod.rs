mod dish;
mod dishes_scheme;
mod error;

use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    rest::{
        dish::{add_dish, delete_dish, get_dish, get_dishes},
        dishes_scheme::{get_schemes, overwrite_schemes},
    },
    state::AppState,
};

#[derive(Deserialize, Serialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum PeriodType {
    Breakfast,
    Lunch,
    Dinner,
}

pub fn create_router(state: Arc<AppState>) -> Router {
    let dish_router = Router::new()
        .route("/:name", get(get_dish))
        .route("/list", get(get_dishes))
        .route("/", post(add_dish))
        .route("/:name", delete(delete_dish));
    let dishes_scheme_router = Router::new()
        .route("/", get(get_schemes))
        .route("/", post(overwrite_schemes));

    Router::new()
        .nest("/dish", dish_router)
        .nest("/dishes_scheme", dishes_scheme_router)
        .with_state(state)
}
