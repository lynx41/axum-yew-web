use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        
        // write here for `up`
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string_len(63)
                            .not_null()
                            .unique_key()
                    )
                    .col(ColumnDef::new(Users::Password)
                        .string_len(89)
                        .not_null()
                    )
                    
                    .col(ColumnDef::new(Users::Name).string_len(31))
                    .col(ColumnDef::new(Users::Surname).string_len(31))
                    .col(ColumnDef::new(Users::MiddleName).string_len(31))
                    .col(ColumnDef::new(Users::CreatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Users::LastLogin).timestamp_with_time_zone())
                    .col(ColumnDef::new(Users::ModifiedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Users::DeletedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Users::IsAdmin).boolean().default(false))
                    .col(ColumnDef::new(Users::Token).string())
                    .col(ColumnDef::new(Users::AvatarUrl).string_len(89))
                    .to_owned(),
            )
            .await?;


        // Seed for the table
        let insert = Query::insert()
            .into_table(Users::Table)
            .columns([Users::Email, Users::Password, Users::IsAdmin])
            .values_panic([
                "hslv@proton.me".into(),
                "$2b$14$OHaCi4Lel1xO1yn0meik/OjMUchHh5wqKi5H.BC9O4DPHl5s7lcH6".into(),
                true.into()
            ])
            .values_panic([
                "aiqo.main@proton.me".into(),
                "$2b$14$I3wYkxG7NBA0je7qQqo65eKd4q3xHF5xAH3p0AQpkhONElxAdCl5q".into(),
                false.into()
            ])
                .to_owned();

        manager.exec_stmt(insert).await

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        // write here for `down`
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Email,
    Name,
    Surname,
    MiddleName,
    Password,
    CreatedAt,
    LastLogin,
    ModifiedAt,
    DeletedAt,
    IsAdmin,
    Token,
    AvatarUrl,
}
