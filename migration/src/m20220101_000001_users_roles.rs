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

                    .col(
                        ColumnDef::new(Users::Password)
                            .string_len(89)
                            .not_null()
                    )

                    .to_owned()
            )
                .await?;
            
        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                
                    .col(
                        ColumnDef::new(Roles::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )

                    .col(
                        ColumnDef::new(Roles::RoleName)
                            .string_len(31)
                            .not_null()
                            .unique_key()
                    )

                    .to_owned()
        )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserRoles::Table)
                    .if_not_exists()

                    .col(
                        ColumnDef::new(UserRoles::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )

                    .col(
                        ColumnDef::new(UserRoles::RoleId)
                            .integer()
                            .not_null()
                    )

                    .col(
                        ColumnDef::new(UserRoles::UserId)
                            .integer()
                            .not_null()
                            .unique_key()
                    )

                    .to_owned()

            )
                .await?;
        
        manager
            .create_table(
                Table::create()
                    .table(Guest::Table)
                    .if_not_exists()

                    .col(
                        ColumnDef::new(Guest::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )

                    .col(
                        ColumnDef::new(Guest::UniqueId)
                            .integer()
                            .not_null()
                            .unique_key()
                    )

                    .to_owned()
            )
                .await?;
        
        // Defining FOREIGN_KEYS
        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk-userroles-user-id")
                .from(UserRoles::Table, UserRoles::UserId)
                .to(Users::Table, Users::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned()
        )
            .await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk-userroles-role-id")
                .from(UserRoles::Table, UserRoles::RoleId)
                .to(Roles::Table, Roles::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned()
        )
            .await?;


        // Seed init data for the tables

        let insert_users = Query::insert()
            .into_table(Users::Table)
            .columns([Users::Email, Users::Password])
            
            .values_panic([
                "hslv@proton.me".into(),
                "$2b$14$OHaCi4Lel1xO1yn0meik/OjMUchHh5wqKi5H.BC9O4DPHl5s7lcH6".into(),
            ])
            
            .values_panic([
                "aiqo.main@proton.me".into(),
                // pass: 1234
                "$2b$14$I3wYkxG7NBA0je7qQqo65eKd4q3xHF5xAH3p0AQpkhONElxAdCl5q".into(),
            ])
                .to_owned();
                
        let insert_roles = Query::insert()
            .into_table(Roles::Table)
            .columns([Roles::RoleName])

            .values_panic([
                "customer".into(),
            ])

            .values_panic([
                "manager".into()
            ])

            .values_panic([
                "admin".into()
            ])

            .values_panic([
                "support".into()
            ])
                .to_owned();

        let insert_users_roles = Query::insert()
            .into_table(UserRoles::Table)
            .columns([UserRoles::UserId, UserRoles::RoleId])

            .values_panic([
                1.into(),
                3.into()
            ])

            .values_panic([
                2.into(),
                1.into()
            ])
                .to_owned();

        manager.exec_stmt(insert_users).await?;
        manager.exec_stmt(insert_roles).await?;
        manager.exec_stmt(insert_users_roles).await

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(UserRoles::Table)
                    .name("fk-userroles-user-id")
                    .name("fk-userroles-role-id")
                    .to_owned()
            )
            .await?;

        // write here for `down`

        manager
            .drop_table(Table::drop().table(UserRoles::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Roles::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await


    }
}


#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Email,
    Password
}

#[derive(DeriveIden)]
enum UserRoles {
    Table,
    Id,
    UserId,
    RoleId
}

#[derive(DeriveIden)]
enum Roles {
    Table,
    Id,
    RoleName
}

#[derive(DeriveIden)]
pub enum Guest {
    Table,
    Id,
    UniqueId
}