use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

use crate::{entity::product, rest::error::HttpError, state::AppState};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Product {
    pub name: String,
    pub measure: String,
}

impl From<product::Model> for Product {
    fn from(model: product::Model) -> Self {
        Product {
            name: model.name,
            measure: model.measure,
        }
    }
}

impl From<Product> for product::ActiveModel {
    fn from(value: Product) -> Self {
        product::ActiveModel {
            name: Set(value.name),
            measure: Set(value.measure),
            ..Default::default()
        }
    }
}

pub async fn get_products(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Product>>, HttpError> {
    let products: Vec<_> = product::Entity::find()
        .all(&state.db_conn)
        .await?
        .into_iter()
        .map(|m| m.into())
        .collect();

    Ok(Json(products))
}

pub async fn get_product(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Json<Product>, HttpError> {
    let product = product::Entity::find()
        .filter(product::Column::Name.eq(name))
        .one(&state.db_conn)
        .await?;
    let Some(product) = product else {
        return Err(HttpError::NotFound)
    };

    let product = product.into();
    Ok(Json(product))
}

pub async fn add_product(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Product>,
) -> Result<(), HttpError> {
    let model: product::ActiveModel = payload.into();
    product::Entity::insert(model).exec(&state.db_conn).await?;

    Ok(())
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub measure: Option<String>,
}

pub async fn update_product(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
    Json(payload): Json<UpdateProduct>,
) -> Result<(), HttpError> {
    let product = product::Entity::find()
        .filter(product::Column::Name.eq(name))
        .one(&state.db_conn)
        .await?;
    let Some(product) = product else {
        return Err(HttpError::NotFound)
    };
    let mut product: product::ActiveModel = product.into();

    if let Some(name) = payload.name {
        product.name = Set(name);
    }
    if let Some(measure) = payload.measure {
        product.measure = Set(measure);
    }

    product.update(&state.db_conn).await?;

    Ok(())
}

pub async fn delete_product(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<(), HttpError> {
    product::Entity::delete_many()
        .filter(product::Column::Name.eq(name))
        .exec(&state.db_conn)
        .await?;

    Ok(())
}
