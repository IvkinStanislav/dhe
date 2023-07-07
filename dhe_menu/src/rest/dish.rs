use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, LoaderTrait, ModelTrait, QueryFilter,
    QuerySelect, Set,
};
use serde::{Deserialize, Serialize};

use crate::{
    db::CorruptedDataError,
    entity::{dish, dish_product, product},
    migration,
    rest::{error::HttpError, product::Product, PeriodSet, PeriodType},
    state::AppState,
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Dish {
    pub name: String,
    pub periods: Vec<PeriodType>,
    pub products: Vec<Product>,
    pub amount: u8,
}

impl TryFrom<(dish::Model, Vec<product::Model>)> for Dish {
    type Error = CorruptedDataError;

    fn try_from((dish, products): (dish::Model, Vec<product::Model>)) -> Result<Self, Self::Error> {
        use sea_orm::sea_query::Iden;

        let err_creator = |column| {
            CorruptedDataError::new(
                migration::Dish::Table.to_string(),
                dish.id.to_string(),
                column,
            )
        };
        let products = products.into_iter().map(|p| p.into()).collect();

        Ok(Dish {
            name: dish.name,
            periods: PeriodSet(dish.periods).into(),
            products,
            amount: dish
                .amount
                .try_into()
                .map_err(|_| err_creator("amount_days".to_string()))?,
        })
    }
}

pub async fn get_dishes(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Dish>>, HttpError> {
    let dishes = dish::Entity::find().all(&state.db_conn).await?;
    let products = dishes
        .load_many_to_many(product::Entity, dish_product::Entity, &state.db_conn)
        .await?;
    let dishes: Result<Vec<_>, _> = dishes
        .into_iter()
        .zip(products)
        .map(|d| d.try_into())
        .collect();

    Ok(Json(dishes?))
}

pub async fn get_dish(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Json<Dish>, HttpError> {
    let dish = dish::Entity::find()
        .filter(dish::Column::Name.eq(name))
        .one(&state.db_conn)
        .await?;
    let Some(dish) = dish else {
        return Err(HttpError::NotFound)
    };
    let products = dish
        .find_related(product::Entity)
        .all(&state.db_conn)
        .await?;

    let dish = (dish, products).try_into()?;
    Ok(Json(dish))
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateDish {
    pub name: String,
    pub periods: Vec<PeriodType>,
    pub amount: u8,
}

impl From<CreateDish> for dish::ActiveModel {
    fn from(value: CreateDish) -> Self {
        let period_set = PeriodSet::from(value.periods.as_ref());
        dish::ActiveModel {
            name: Set(value.name),
            periods: Set(period_set.0),
            amount: Set(value.amount as i32),
            ..Default::default()
        }
    }
}

pub async fn add_dish(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateDish>,
) -> Result<(), HttpError> {
    let model: dish::ActiveModel = payload.into();
    dish::Entity::insert(model).exec(&state.db_conn).await?;

    Ok(())
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UpdateDish {
    pub name: Option<String>,
    pub periods: Option<Vec<PeriodType>>,
    pub amount: Option<u8>,
}

pub async fn update_dish(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
    Json(payload): Json<UpdateDish>,
) -> Result<(), HttpError> {
    let dish = dish::Entity::find()
        .filter(product::Column::Name.eq(name))
        .one(&state.db_conn)
        .await?;
    let Some(dish) = dish else {
        return Err(HttpError::NotFound)
    };
    let mut dish: dish::ActiveModel = dish.into();

    if let Some(name) = payload.name {
        dish.name = Set(name);
    }
    if let Some(periods) = payload.periods {
        let period_set = PeriodSet::from(periods.as_ref());
        dish.periods = Set(period_set.0);
    }
    if let Some(amount) = payload.amount {
        dish.amount = Set(amount as i32);
    }

    dish.update(&state.db_conn).await?;

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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AddProductQuery {
    pub amount: f64,
}

pub async fn add_product_to_dish(
    State(state): State<Arc<AppState>>,
    Path(dish_name): Path<String>,
    Path(product_name): Path<String>,
    Query(query): Query<AddProductQuery>,
) -> Result<(), HttpError> {
    let dish_id: Option<u64> = dish::Entity::find()
        .select_only()
        .column(dish::Column::Id)
        .filter(dish::Column::Name.eq(dish_name))
        .into_tuple()
        .one(&state.db_conn)
        .await?;
    let Some(dish_id) = dish_id else {
        return Err(HttpError::NotFound)
    };

    let product_id: Option<u64> = product::Entity::find()
        .select_only()
        .column(product::Column::Id)
        .filter(product::Column::Name.eq(product_name))
        .into_tuple()
        .one(&state.db_conn)
        .await?;
    let Some(product_id) = product_id else {
        return Err(HttpError::NotFound)
    };

    let dish_product = dish_product::ActiveModel {
        dish_id: Set(dish_id as i32),
        product_id: Set(product_id as i32),
        amount: Set(query.amount),
        ..Default::default()
    };
    dish_product::Entity::insert(dish_product)
        .exec(&state.db_conn)
        .await?;

    Ok(())
}

pub async fn delete_product_from_dish(
    State(state): State<Arc<AppState>>,
    Path(dish_name): Path<String>,
    Path(product_name): Path<String>,
) -> Result<(), HttpError> {
    let dish_id: Option<u64> = dish::Entity::find()
        .select_only()
        .column(dish::Column::Id)
        .filter(dish::Column::Name.eq(dish_name))
        .into_tuple()
        .one(&state.db_conn)
        .await?;
    let Some(dish_id) = dish_id else {
        return Err(HttpError::NotFound)
    };

    let product_id: Option<u64> = product::Entity::find()
        .select_only()
        .column(product::Column::Id)
        .filter(product::Column::Name.eq(product_name))
        .into_tuple()
        .one(&state.db_conn)
        .await?;
    let Some(product_id) = product_id else {
        return Err(HttpError::NotFound)
    };

    dish_product::Entity::delete_many()
        .filter(
            Condition::all()
                .add(dish_product::Column::DishId.eq(dish_id))
                .add(dish_product::Column::ProductId.eq(product_id)),
        )
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
        .map(|m| (m, vec![]).try_into())
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
