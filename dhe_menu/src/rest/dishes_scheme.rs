use std::{str::FromStr, sync::Arc};

use axum::{extract::State, Json};
use sea_orm::{EntityTrait, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::{
    db::CorruptedDataError,
    entity::dishes_scheme,
    migration,
    rest::{error::HttpError, PeriodType},
    state::AppState,
};

#[derive(Deserialize, Serialize)]
pub struct DishesScheme {
    pub scheme: Vec<u8>,
    pub period: PeriodType,
}

impl DishesScheme {
    const DISHES_SCHEME_DELIMITER: &str = "+";

    fn scheme_to_string(data: Vec<u8>) -> String {
        data.iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(Self::DISHES_SCHEME_DELIMITER)
    }

    fn string_to_scheme(data: String) -> Result<Vec<u8>, String> {
        let scheme: Result<Vec<u8>, _> = data
            .split(Self::DISHES_SCHEME_DELIMITER)
            .map(|number| number.parse())
            .collect();
        scheme.map_err(|_| data)
    }
}

impl TryFrom<dishes_scheme::Model> for DishesScheme {
    type Error = CorruptedDataError;

    fn try_from(model: dishes_scheme::Model) -> Result<Self, Self::Error> {
        use sea_orm::sea_query::Iden;

        let err_creator = |column| {
            CorruptedDataError::new(
                migration::DishesScheme::Table.to_string(),
                model.id.to_string(),
                column,
            )
        };
        Ok(DishesScheme {
            scheme: DishesScheme::string_to_scheme(model.scheme)
                .map_err(|_| err_creator("scheme".to_string()))?,
            period: PeriodType::from_str(&model.period)
                .map_err(|_| err_creator("period".to_string()))?,
        })
    }
}

impl From<DishesScheme> for dishes_scheme::ActiveModel {
    fn from(value: DishesScheme) -> Self {
        dishes_scheme::ActiveModel {
            scheme: Set(DishesScheme::scheme_to_string(value.scheme)),
            period: Set(value.period.to_string()),
            ..Default::default()
        }
    }
}

pub async fn get_schemes(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<DishesScheme>>, HttpError> {
    let schemes: Result<Vec<_>, _> = dishes_scheme::Entity::find()
        .all(&state.db_conn)
        .await?
        .into_iter()
        .map(|m| m.try_into())
        .collect();

    Ok(Json(schemes?))
}

pub async fn overwrite_schemes(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Vec<DishesScheme>>,
) -> Result<(), HttpError> {
    let schemes: Vec<dishes_scheme::ActiveModel> =
        payload.into_iter().map(|ds| ds.into()).collect();

    let txn = state.db_conn.begin().await?;
    dishes_scheme::Entity::delete_many().exec(&txn).await?;
    dishes_scheme::Entity::insert_many(schemes)
        .exec(&txn)
        .await?;
    txn.commit().await?;

    Ok(())
}
