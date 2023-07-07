use sea_orm::Iterable;
use sea_orm_migration::prelude::*;

use super::{Dish, DishProduct, DishesScheme, Menu, MenuData, PeriodType, Product};

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
                    .table(Product::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Product::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Product::Name)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Product::Measure).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(DishProduct::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DishProduct::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(DishProduct::DishId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DishProduct::ProductId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(DishProduct::Amount).double().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_col(DishProduct::DishId)
                            .to(Dish::Table, Dish::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_col(DishProduct::ProductId)
                            .to(Product::Table, Product::Id),
                    )
                    .index(
                        Index::create()
                            .col(DishProduct::DishId)
                            .col(DishProduct::ProductId)
                            .unique(),
                    )
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
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MenuData::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MenuData::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MenuData::MenuId).big_unsigned().not_null())
                    .col(ColumnDef::new(MenuData::DishId).big_unsigned().not_null())
                    .col(
                        ColumnDef::new(MenuData::Period)
                            .enumeration(PeriodType::Table, PeriodType::iter().skip(1))
                            .not_null(),
                    )
                    .col(ColumnDef::new(MenuData::Order).tiny_unsigned().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_col(MenuData::DishId)
                            .to(Dish::Table, Dish::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_col(MenuData::MenuId)
                            .to(Menu::Table, Menu::Id),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Menu::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Menu::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Menu::DateTime).date_time().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DishProduct::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Dish::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(DishesScheme::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MenuData::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Menu::Table).to_owned())
            .await?;
        Ok(())
    }
}
