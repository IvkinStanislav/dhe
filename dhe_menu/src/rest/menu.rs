use std::sync::Arc;

use axum::{extract::State, Json};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

use crate::{
    entity::{dish, dishes_scheme},
    rest::{dish::Dish, dishes_scheme::DishesScheme, error::HttpError},
    state::AppState,
};

#[derive(Deserialize, Serialize)]
pub struct Menu {
    
}

pub async fn get_menu(State(state): State<Arc<AppState>>) -> Result<Json<Menu>, HttpError> {
    let dishes: Result<Vec<Dish>, _> = dish::Entity::find()
        .all(&state.db_conn)
        .await?
        .into_iter()
        .map(|m| m.try_into())
        .collect();
    let schemes: Result<Vec<DishesScheme>, _> = dishes_scheme::Entity::find()
        .all(&state.db_conn)
        .await?
        .into_iter()
        .map(|m| m.try_into())
        .collect();

    todo!()
    // Ok(Json(menu))
}
