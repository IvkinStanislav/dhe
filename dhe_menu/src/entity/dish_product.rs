//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "dish_product")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub dish_id: i32,
    pub product_id: i32,
    #[sea_orm(column_type = "Double")]
    pub amount: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::dish::Entity",
        from = "Column::DishId",
        to = "super::dish::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Dish,
    #[sea_orm(
        belongs_to = "super::product::Entity",
        from = "Column::ProductId",
        to = "super::product::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Product,
}

impl Related<super::dish::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Dish.def()
    }
}

impl Related<super::product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}