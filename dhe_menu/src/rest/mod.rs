mod dish;
mod dishes_scheme;
mod error;
mod menu;

use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{
    rest::{
        dish::{add_dish, delete_dish, dish_stat, get_dish, get_dishes},
        dishes_scheme::{get_schemes, overwrite_schemes},
        menu::get_menu,
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

impl From<i32> for PeriodSet {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

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
        .route("/:name", delete(delete_dish))
        .route("/stat", get(dish_stat));
    let dishes_scheme_router = Router::new()
        .route("/", get(get_schemes))
        .route("/", post(overwrite_schemes));

    Router::new()
        .nest("/dish", dish_router)
        .nest("/dishes_scheme", dishes_scheme_router)
        .route("/menu/:amount", get(get_menu))
        .with_state(state)
}
