use sea_orm_migration::prelude::*;

use crate::m20220101_000001_users_roles::Users;
use crate::m20240503_120246_goods_category_parfumery::CategoryParfumery;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        
        manager
            .create_table(
                Table::create()
                    .table(WishList::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WishList::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(WishList::UserId)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(WishList::ItemId)
                            .integer()
                            .not_null()   
                    )
                        .to_owned()
            )
                .await?;

            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk-wish-list-user-id")
                        .from(WishList::Table, WishList::UserId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned()
                )
                    .await?;
            
            manager
                    .create_foreign_key(
                        ForeignKey::create()
                            .name("fk-wish-list-item-id")
                            .from(WishList::Table, WishList::UserId)
                            .to(CategoryParfumery::Table, CategoryParfumery::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .to_owned()
                    )
                        .await

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
        .drop_foreign_key(
            ForeignKey::drop()
                .table(WishList::Table)
                .name("fk-wish-list-user-id")
                .name("fk-wish-list-item-id")
                .to_owned()
        )
            .await?;

        manager
            .drop_table(Table::drop().table(WishList::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum WishList {
    Table,
    Id,
    UserId,
    ItemId,
}
