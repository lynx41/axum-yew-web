use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

use crate::m20240324_113306_token::Sessions;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    
                    .modify_column(
                        ColumnDef::new(Alias::new("user_id"))
                            .null()
                    )
                        .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)

                    .modify_column(
                        ColumnDef::new(Alias::new("user_id"))
                            .not_null()
                    )
                        .to_owned()
            )
            .await
    }
}