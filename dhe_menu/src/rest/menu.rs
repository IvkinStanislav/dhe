use std::{cmp::Ordering, sync::Arc};

use axum::{
    extract::{Path, State},
    Json,
};
use rand::{seq::SliceRandom, thread_rng};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

use crate::{
    entity::{dish, dishes_scheme},
    rest::{dish::Dish, dishes_scheme::DishesScheme, error::HttpError, PeriodType},
    state::AppState,
};

#[derive(Deserialize, Serialize, Default)]
pub struct Menu {
    breakfasts: Vec<Dish>,
    lunches: Vec<Dish>,
    dinners: Vec<Dish>,
}

pub async fn get_menu(
    State(state): State<Arc<AppState>>,
    Path(amount): Path<u8>,
) -> Result<Json<Menu>, HttpError> {
    let dishes: Result<Vec<Dish>, _> = dish::Entity::find()
        .all(&state.db_conn)
        .await?
        .into_iter()
        .map(|m| m.try_into())
        .collect();

    use Ordering::*;
    let dishes: Vec<_> = dishes?
        .into_iter()
        .filter(|d| match d.amount.cmp(&amount) {
            Less => amount % d.amount == 0,
            Equal => true,
            Greater => d.amount % amount == 0,
        })
        .map(|mut d| {
            match d.amount.cmp(&amount) {
                Less => d.amount = 1,
                Equal | Greater => d.amount /= amount,
            };
            d
        })
        .collect();

    let mut breakfasts = vec![];
    let mut lunches = vec![];
    let mut dinners = vec![];
    for dish in dishes {
        for period in &dish.periods {
            use PeriodType::*;
            match period {
                Breakfast => breakfasts.push(dish.clone()),
                Lunch => lunches.push(dish.clone()),
                Dinner => dinners.push(dish.clone()),
            }
        }
    }

    let schemes: Result<Vec<DishesScheme>, _> = dishes_scheme::Entity::find()
        .all(&state.db_conn)
        .await?
        .into_iter()
        .map(|m| m.try_into())
        .collect();
    let schemes = schemes?;
    let mut breakfast_schemes = vec![];
    let mut lunch_schemes = vec![];
    let mut dinner_schemes = vec![];
    for scheme in schemes {
        use PeriodType::*;
        match scheme.period {
            Breakfast => breakfast_schemes.push(scheme),
            Lunch => lunch_schemes.push(scheme),
            Dinner => dinner_schemes.push(scheme),
        }
    }

    let menu = Menu {
        breakfasts: choose_random(breakfast_schemes, breakfasts),
        lunches: choose_random(lunch_schemes, lunches),
        dinners: choose_random(dinner_schemes, dinners),
    };
    Ok(Json(menu))
}

fn choose_random(mut schemes: Vec<DishesScheme>, mut dishes: Vec<Dish>) -> Vec<Dish> {
    schemes.shuffle(&mut thread_rng());
    dishes.shuffle(&mut thread_rng());

    for scheme in schemes {
        let mut scheme = scheme.scheme;
        let mut res = vec![];

        for dish in &dishes {
            let index = scheme.iter().position(|x| *x == dish.amount);
            if let Some(index) = index {
                scheme.remove(index);
                res.push(dish.clone());
                if scheme.is_empty() {
                    return res;
                }
            }
        }
    }

    vec![]
}
