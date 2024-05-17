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
                    .table(GuestPortrait::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GuestPortrait::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(GuestPortrait::GuestId).integer().not_null())

                    .col(ColumnDef::new(GuestPortrait::PriceList).string())
                    .col(ColumnDef::new(GuestPortrait::VolumeList).string())
                    .col(ColumnDef::new(GuestPortrait::ClassList).string())
                    .col(ColumnDef::new(GuestPortrait::SeassonList).string())
                    .col(ColumnDef::new(GuestPortrait::BrandList).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserPortrait::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserPortrait::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserPortrait::UserId).integer().not_null())

                    .col(ColumnDef::new(UserPortrait::PriceList).string())
                    .col(ColumnDef::new(UserPortrait::VolumeList).string())
                    .col(ColumnDef::new(UserPortrait::ClassList).string())
                    .col(ColumnDef::new(UserPortrait::SeassonList).string())
                    .col(ColumnDef::new(UserPortrait::BrandList).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-guest-portrait-id")
                    .from(GuestPortrait::Table, GuestPortrait::GuestId)
                    .to(Guest::Table, Guest::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned()
            )
                .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-user-portrait-id")
                    .from(UserPortrait::Table, UserPortrait::UserId)
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
                    .table(GuestPortrait::Table)
                    .name("fk-guest-portrait-id")
                    .to_owned()
            )
                .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(UserPortrait::Table)
                    .name("fk-user-portrait-id")
                    .to_owned()
            )
                .await?;
        
        manager
            .drop_table(Table::drop().table(GuestPortrait::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserPortrait::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum GuestPortrait {
    Table,
    Id,
    GuestId,
    // Portrait
    PriceList,
    VolumeList,
    ClassList,
    SeassonList,
    BrandList,
}

#[derive(DeriveIden)]
enum UserPortrait {
    Table,
    Id,
    UserId,
    // Portrait
    PriceList,
    VolumeList,
    ClassList,
    SeassonList,
    BrandList,
}