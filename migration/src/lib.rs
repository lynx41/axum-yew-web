pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users_roles;
mod m20240323_143315_userlogs_n_userinfo;
mod m20240324_113306_token;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users_roles::Migration),
            Box::new(m20240323_143315_userlogs_n_userinfo::Migration),
            Box::new(m20240324_113306_token::Migration),
        ]
    }
}
