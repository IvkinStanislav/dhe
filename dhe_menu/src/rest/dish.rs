use std::sync::Arc;

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
    rest::{error::HttpError, PeriodSet, PeriodType},
    state::AppState,
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Dish {
    pub name: String,
    pub periods: Vec<PeriodType>,
    pub amount: u8,
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
            periods: PeriodSet(model.periods).into(),
            amount: model
                .amount
                .try_into()
                .map_err(|_| err_creator("amount_days".to_string()))?,
        })
    }
}

impl From<Dish> for dish::ActiveModel {
    fn from(value: Dish) -> Self {
        let period_set = PeriodSet::from(value.periods.as_ref());
        dish::ActiveModel {
            name: Set(value.name),
            periods: Set(period_set.0),
            amount: Set(value.amount as i32),
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

#[derive(Deserialize, Serialize, Default)]
pub struct DishStat {
    pub count: usize,
    pub breakfasts: usize,
    pub lunches: usize,
    pub dinners: usize,
}

pub async fn dish_stat(State(state): State<Arc<AppState>>) -> Result<Json<DishStat>, HttpError> {
    let dishes: Result<Vec<Dish>, _> = dish::Entity::find()
        .all(&state.db_conn)
        .await?
        .into_iter()
        .map(|m| m.try_into())
        .collect();

    let mut dish_stat = DishStat::default();
    let dishes = dishes?;
    dish_stat.count = dishes.len();

    for dish in dishes {
        for period in dish.periods {
            use PeriodType::*;
            match period {
                Breakfast => dish_stat.breakfasts += 1,
                Lunch => dish_stat.lunches += 1,
                Dinner => dish_stat.dinners += 1,
            }
        }
    }

    Ok(Json(dish_stat))
}
