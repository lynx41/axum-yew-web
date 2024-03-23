use sea_orm_migration::prelude::*;

use crate::m20220101_000001_users_roles::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(UserLogs::Table)
                    .if_not_exists()

                    .col(
                        ColumnDef::new(UserLogs::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )

                    .col(
                        ColumnDef::new(UserLogs::UserId)
                            .integer()
                            .not_null()
                    )

                    .col(
                        ColumnDef::new(UserLogs::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                    )

                    .col(
                        ColumnDef::new(UserLogs::LastLogin)
                            .timestamp_with_time_zone()
                            .not_null()
                    )

                    .col(
                        ColumnDef::new(UserLogs::ModifiedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                    )

                    .col(
                        ColumnDef::new(UserLogs::DeletedAt)
                            .timestamp_with_time_zone()   
                    )

                    .to_owned()
            ).await?;

            manager
                .create_table(
                    Table::create()
                        .table(UserInfo::Table)
                        .if_not_exists()

                        .col(
                            ColumnDef::new(UserInfo::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key()
                        )

                        .col(
                            ColumnDef::new(UserInfo::UserId)
                                .integer()
                                .not_null()
                        )

                        .col(
                            ColumnDef::new(UserInfo::FirstName)
                                .string_len(31)
                        )

                        .col(
                            ColumnDef::new(UserInfo::MiddleName)
                                .string_len(31)
                        )

                        .col(
                            ColumnDef::new(UserInfo::LastName)
                                .string_len(31)
                        )

                        .to_owned()
                )
                    .await?;


            // Defining FOREIGN_KEYS
            manager.create_foreign_key(
                ForeignKey::create()
                    .name("fk-userlogs-user-id")
                    .from(UserLogs::Table, UserLogs::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned()
            )
                .await?;

            manager.create_foreign_key(
                ForeignKey::create()
                    .name("fk-userinfo-user-id")
                    .from(UserInfo::Table, UserInfo::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned()
            )
                .await

            
            // Seed init data for the tables
                // I don't know how to make values of TimestampWithTimeZone, so no seed
                
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(UserLogs::Table)
                    .name("fk-userlogs-user-id")
                    .to_owned()
            )
                .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(UserInfo::Table)
                    .name("fk-userinfo-user-id")
                    .to_owned()
            )
                .await?;
        
        manager
            .drop_table(Table::drop().table(UserLogs::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserInfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserLogs {
    Table,
    Id,
    UserId,
    CreatedAt,
    LastLogin,
    ModifiedAt,
    DeletedAt
}

#[derive(DeriveIden)]
enum UserInfo {
    Table,
    Id,
    UserId,
    FirstName,
    MiddleName,
    LastName,
}