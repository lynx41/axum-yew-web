use sea_orm_migration::prelude::*;
use crate::m20220101_000001_users_roles::{Guest, Users};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(GuestHistory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GuestHistory::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()   
                    )
                    .col(
                        ColumnDef::new(GuestHistory::GuestId)
                            .integer()
                            .not_null()   
                    )
                    .col(
                        ColumnDef::new(GuestHistory::List)
                            .string()
                            .not_null()
                    )
                        .to_owned()
            )
                .await?;
            
        manager
            .create_table(
                Table::create()
                    .table(UserHistory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserHistory::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()   
                    )
                    .col(
                        ColumnDef::new(UserHistory::UserId)
                            .integer()
                            .not_null()   
                    )
                    .col(
                        ColumnDef::new(UserHistory::List)
                            .string()
                            .not_null()
                    )
                        .to_owned()
            )
                .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-guest-history-id")
                    .from(GuestHistory::Table, GuestHistory::GuestId)
                    .to(Guest::Table, Guest::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned()
            )
                .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-user-history-id")
                    .from(UserHistory::Table, UserHistory::UserId)
                    .to(Users::Table, Users::Id)
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
                    .table(GuestHistory::Table)
                    .name("fk-guest-history-id")
                    .to_owned()
            )
                .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(UserHistory::Table)
                    .name("fk-user-history-id")
                    .to_owned()
            )
                .await?;
        
        manager
            .drop_table(Table::drop().table(GuestHistory::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserHistory::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum GuestHistory {
    Table,
    Id,
    GuestId,
    List,
}

#[derive(DeriveIden)]
enum UserHistory {
    Table,
    Id,
    UserId,
    List,
}