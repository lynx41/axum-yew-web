use sea_orm_migration::prelude::*;

use crate::m20220101_000001_users_roles::{Users, Guest};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Sessions::Table)
                    .if_not_exists()

                    .col(
                        ColumnDef::new(Sessions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )

                    .col(
                        ColumnDef::new(Sessions::UserId)
                            .integer()
                    )
                    
                    .col(
                        ColumnDef::new(Sessions::Token)
                            .string()
                    )

                    .col(
                        ColumnDef::new(Sessions::GuestId)
                            .integer()
                    )

                    .col(
                        ColumnDef::new(Sessions::UniqueId)
                            .string()
                            .not_null()
                    )

                        .to_owned()
            )
                .await?;
        
        
        // Defining FOREIGN KEYS
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-sessions-user-id")
                    .from(Sessions::Table, Sessions::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned()
            )
                .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-sessions-guest-id")
                    .from(Sessions::Table, Sessions::GuestId)
                    .to(Guest::Table, Guest::Id)
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
                    .table(Sessions::Table)
                    .name("fk-sessions-user-id")
                    .to_owned()
            )
                .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Sessions::Table)
                    .name("fk-sessions-guest-id")
                    .to_owned()
            )
                .await?;

        manager
            .drop_table(Table::drop().table(Sessions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Sessions {
    Table,
    Id,
    UserId,
    GuestId,
    Token,
    UniqueId
}
