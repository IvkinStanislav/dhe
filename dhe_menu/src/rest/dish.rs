use std::{str::FromStr, sync::Arc};

use axum::{
    extract::{Path, State},
    Json,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

use crate::{
    db::CorruptedDataError,
    entity::dish,
    migration,
    rest::{error::HttpError, PeriodType},
    state::AppState,
};

#[derive(Deserialize, Serialize)]
pub struct Dish {
    pub name: String,
    pub period: PeriodType,
    pub part: u8,
    pub amount_days: u8,
}

impl TryFrom<dish::Model> for Dish {
    type Error = CorruptedDataError;

    fn try_from(model: dish::Model) -> Result<Self, Self::Error> {
        use sea_orm::sea_query::Iden;

        let err_creator = |column| {
            CorruptedDataError::new(
                migration::Dish::Table.to_string(),
                model.id.to_string(),
                column,
            )
        };
        Ok(Dish {
            name: model.name,
            period: PeriodType::from_str(&model.period)
                .map_err(|_| err_creator("period".to_string()))?,
            part: model
                .part
                .try_into()
                .map_err(|_| err_creator("part".to_string()))?,
            amount_days: model
                .amount_days
                .try_into()
                .map_err(|_| err_creator("amount_days".to_string()))?,
        })
    }
}

impl From<Dish> for dish::ActiveModel {
    fn from(value: Dish) -> Self {
        dish::ActiveModel {
            name: Set(value.name),
            period: Set(value.period.to_string()),
            part: Set(value.part as i32),
            amount_days: Set(value.amount_days as i32),
            ..Default::default()
        }
    }
}

pub async fn get_dishes(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Dish>>, HttpError> {
    let dishes: Result<Vec<_>, _> = dish::Entity::find()
        .all(&state.db_conn)
        .await?
        .into_iter()
        .map(|m| m.try_into())
        .collect();

    Ok(Json(dishes?))
}

pub async fn get_dish(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Json<Option<Dish>>, HttpError> {
    let model = dish::Entity::find()
        .filter(dish::Column::Name.eq(name))
        .one(&state.db_conn)
        .await?;

    let dish = model.map(|m| m.try_into()).transpose()?;
    Ok(Json(dish))
}

pub async fn add_dish(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Dish>,
) -> Result<(), HttpError> {
    let model: dish::ActiveModel = payload.into();
    dish::Entity::insert(model).exec(&state.db_conn).await?;

    Ok(())
}

pub async fn delete_dish(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<(), HttpError> {
    dish::Entity::delete_many()
        .filter(dish::Column::Name.eq(name))
        .exec(&state.db_conn)
        .await?;

    Ok(())
}
