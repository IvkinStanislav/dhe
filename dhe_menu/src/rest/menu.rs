use std::{cmp::Ordering, str::FromStr, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    Json,
};
use rand::{seq::SliceRandom, thread_rng};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, ModelTrait,
    QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait,
};
use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, OffsetDateTime, PrimitiveDateTime};

use crate::{
    db::CorruptedDataError,
    entity::{dish, dish_product, dishes_scheme, menu, menu_data, product},
    migration,
    rest::{dish::Dish, dishes_scheme::DishesScheme, error::HttpError, PeriodType},
    state::AppState,
};

#[derive(Deserialize, Serialize, Default)]
pub struct Menu {
    breakfasts: Vec<Dish>,
    lunches: Vec<Dish>,
    dinners: Vec<Dish>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct MenuQuery {
    force: bool,
}

pub async fn get_menu(
    State(state): State<Arc<AppState>>,
    Path(amount): Path<u8>,
    Query(query): Query<MenuQuery>,
) -> Result<Json<Menu>, HttpError> {
    if query.force {
        let menu = generate_menu(amount, &state.db_conn).await?;
        save_menu(&menu, &state.db_conn).await?;
        return Ok(Json(menu));
    };

    let menu = menu::Entity::find().one(&state.db_conn).await?;
    let Some(menu) = menu else {
        let menu = generate_menu(amount, &state.db_conn).await?;
        save_menu(&menu, &state.db_conn).await?;
        return Ok(Json(menu));
    };

    let menu_week = PrimitiveDateTime::parse(&menu.date_time, &Rfc3339)
        .unwrap()
        .iso_week();
    let now = OffsetDateTime::now_utc();
    let week = PrimitiveDateTime::new(now.date(), now.time()).iso_week();
    if week > menu_week {
        let menu = generate_menu(amount, &state.db_conn).await?;
        save_menu(&menu, &state.db_conn).await?;
        return Ok(Json(menu));
    }

    let menu = read_menu(menu, &state.db_conn).await?;
    Ok(Json(menu))
}

async fn save_menu(menu: &Menu, db_conn: &DatabaseConnection) -> Result<(), HttpError> {
    let Menu {
        breakfasts,
        lunches,
        dinners,
    } = menu;

    let menu_dishes = breakfasts
        .iter()
        .map(|d| (d, PeriodType::Breakfast))
        .chain(lunches.iter().map(|d| (d, PeriodType::Lunch)))
        .chain(dinners.iter().map(|d| (d, PeriodType::Dinner)))
        .enumerate()
        .map(|(order, (dish, period))| (order, period, dish));

    let txn = db_conn.begin().await?;

    menu_data::Entity::delete_many().exec(&txn).await?;
    menu::Entity::delete_many().exec(&txn).await?;

    let now = OffsetDateTime::now_utc();
    let date_time = PrimitiveDateTime::new(now.date(), now.time())
        .format(&Rfc3339)
        .unwrap();
    let menu = menu::ActiveModel {
        date_time: Set(date_time),
        ..Default::default()
    };
    let menu = menu.insert(&txn).await?;

    let mut menu_items = vec![];
    for (order, period, dish) in menu_dishes {
        let dish_id: Option<u64> = dish::Entity::find()
            .select_only()
            .column(dish::Column::Id)
            .filter(dish::Column::Name.eq(&dish.name))
            .into_tuple()
            .one(&txn)
            .await?;

        menu_items.push(menu_data::ActiveModel {
            menu_id: Set(menu.id),
            dish_id: Set(dish_id.unwrap() as i32),
            period: Set(period.to_string()),
            order: Set(order as i32),
            ..Default::default()
        })
    }
    menu_data::Entity::insert_many(menu_items)
        .exec(&txn)
        .await?;

    txn.commit().await?;
    Ok(())
}

async fn read_menu(
    menu_model: menu::Model,
    db_conn: &DatabaseConnection,
) -> Result<Menu, HttpError> {
    let menu_items = menu_model
        .find_related(menu_data::Entity)
        .order_by_asc(menu_data::Column::Order)
        .all(db_conn)
        .await?;
    let dishes: Vec<_> = menu_items
        .load_one(dish::Entity, db_conn)
        .await?
        .into_iter()
        .flatten()
        .collect();
    assert_eq!(menu_items.len(), dishes.len());
    let products = dishes
        .load_many_to_many(product::Entity, dish_product::Entity, db_conn)
        .await?;
    let dishes: Result<Vec<Dish>, _> = dishes
        .into_iter()
        .zip(products)
        .map(|d| d.try_into())
        .collect();

    let mut menu = Menu::default();
    for (menu_item, dish) in menu_items.into_iter().zip(dishes?) {
        use sea_orm::sea_query::Iden;
        let period = PeriodType::from_str(&menu_item.period).map_err(|_| {
            CorruptedDataError::new(
                migration::DishesScheme::Table.to_string(),
                menu_item.id.to_string(),
                "period".to_string(),
            )
        })?;

        use PeriodType::*;
        match period {
            Breakfast => menu.breakfasts.push(dish),
            Lunch => menu.lunches.push(dish),
            Dinner => menu.dinners.push(dish),
        }
    }

    Ok(menu)
}

async fn generate_menu(amount: u8, db_conn: &DatabaseConnection) -> Result<Menu, HttpError> {
    let dishes = dish::Entity::find().all(db_conn).await?;
    let products = dishes
        .load_many_to_many(product::Entity, dish_product::Entity, db_conn)
        .await?;
    let dishes: Result<Vec<Dish>, _> = dishes
        .into_iter()
        .zip(products)
        .map(|d| d.try_into())
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
        .all(db_conn)
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

    Ok(Menu {
        breakfasts: choose_random(breakfast_schemes, breakfasts),
        lunches: choose_random(lunch_schemes, lunches),
        dinners: choose_random(dinner_schemes, dinners),
    })
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
