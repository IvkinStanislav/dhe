use sea_orm::EnumIter;
pub use sea_orm_migration::prelude::*;

mod m20230628_144053_create_table;

#[derive(Iden)]
pub enum Dish {
    Table,
    Id,
    Name,
    Period,
    Part,
    AmountDays,
}

#[derive(Iden, EnumIter)]
pub enum PeriodType {
    Table,
    Breakfast,
    Lunch,
    Dinner,
}

#[derive(Iden)]
pub enum DishesScheme {
    Table,
    Id,
    Scheme,
    Period,
}

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20230628_144053_create_table::Migration)]
    }
}
