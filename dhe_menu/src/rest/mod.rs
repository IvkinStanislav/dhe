mod dish;
mod dishes_scheme;
mod error;
mod menu;
mod product;

use std::sync::Arc;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{
    rest::{
        dish::{
            add_dish, add_product_to_dish, delete_dish, delete_product_from_dish, dish_stat,
            get_dish, get_dishes, update_dish,
        },
        dishes_scheme::{add_scheme, delete_scheme, get_schemes},
        menu::get_menu,
        product::{add_product, delete_product, get_product, get_products, update_product},
    },
    state::AppState,
};

#[derive(
    Deserialize, Serialize, strum::Display, strum::EnumString, strum::EnumIter, Clone, Copy, Debug,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PeriodType {
    Breakfast,
    Lunch,
    Dinner,
}

pub struct PeriodSet(pub i32);

impl From<&[PeriodType]> for PeriodSet {
    fn from(value: &[PeriodType]) -> Self {
        let mut set = Self(0);
        for pt in value {
            use PeriodType::*;
            set.0 |= match pt {
                Breakfast => 1,
                Lunch => 2,
                Dinner => 4,
            };
        }
        set
    }
}

impl From<PeriodSet> for Vec<PeriodType> {
    fn from(mut value: PeriodSet) -> Self {
        let mut res = vec![];
        for pt in PeriodType::iter() {
            if value.0 & 1 == 1 {
                res.push(pt);
            }
            value.0 >>= 1;
        }
        res
    }
}

pub fn create_router(state: Arc<AppState>) -> Router {
    let dish_router = Router::new()
        .route("/:name", get(get_dish))
        .route("/list", get(get_dishes))
        .route("/", post(add_dish))
        .route("/:name", patch(update_dish))
        .route("/:name", delete(delete_dish))
        .route("/stat", get(dish_stat))
        .route(
            "/product:dish_name/:product_name",
            post(add_product_to_dish),
        )
        .route(
            "/product:dish_name/:product_name",
            delete(delete_product_from_dish),
        );
    let product_router = Router::new()
        .route("/:name", get(get_product))
        .route("/list", get(get_products))
        .route("/", post(add_product))
        .route("/:name", patch(update_product))
        .route("/:name", delete(delete_product));
    let dishes_scheme_router = Router::new()
        .route("/", get(get_schemes))
        .route("/", post(add_scheme))
        .route("/:id", delete(delete_scheme));

    Router::new()
        .nest("/dish", dish_router)
        .nest("/product", product_router)
        .nest("/dishes_scheme", dishes_scheme_router)
        .route("/menu/:amount", get(get_menu))
        .with_state(state)
}
