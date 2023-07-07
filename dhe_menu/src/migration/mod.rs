use sea_orm::EnumIter;
pub use sea_orm_migration::prelude::*;

mod m20230628_144053_create_table;

#[derive(Iden, EnumIter)]
pub enum PeriodType {
    Table,
    Breakfast,
    Lunch,
    Dinner,
}

#[derive(Iden)]
pub enum Dish {
    Table,
    Id,
    Name,
    Periods,
    Amount,
}

#[derive(Iden)]
pub enum Product {
    Table,
    Id,
    Name,
    Measure,
}

#[derive(Iden)]
pub enum DishProduct {
    Table,
    Id,
    DishId,
    ProductId,
    Amount,
}

#[derive(Iden)]
pub enum DishesScheme {
    Table,
    Id,
    Scheme,
    Period,
}

#[derive(Iden)]
pub enum Menu {
    Table,
    Id,
    DateTime,
}

#[derive(Iden)]
pub enum MenuData {
    Table,
    Id,
    MenuId,
    DishId,
    Period,
    Order,
}

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20230628_144053_create_table::Migration)]
    }
}
