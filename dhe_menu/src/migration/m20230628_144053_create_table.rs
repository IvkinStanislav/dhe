use sea_orm::Iterable;
use sea_orm_migration::prelude::*;

use super::{Dish, DishesScheme, PeriodType};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Dish::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Dish::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Dish::Name).string().unique_key().not_null())
                    .col(ColumnDef::new(Dish::Periods).tiny_unsigned().not_null())
                    .col(ColumnDef::new(Dish::Amount).tiny_unsigned().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(DishesScheme::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DishesScheme::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DishesScheme::Scheme).string().not_null())
                    .col(
                        ColumnDef::new(DishesScheme::Period)
                            .enumeration(PeriodType::Table, PeriodType::iter().skip(1))
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Dish::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(DishesScheme::Table).to_owned())
            .await
    }
}
