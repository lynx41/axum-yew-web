pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users_roles;
mod m20240323_143315_userlogs_n_userinfo;
mod m20240324_113306_token;
mod m20240503_120246_goods_category_parfumery;
mod m20240515_090501_customer_view_history;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users_roles::Migration),
            Box::new(m20240323_143315_userlogs_n_userinfo::Migration),
            Box::new(m20240324_113306_token::Migration),
            Box::new(m20240503_120246_goods_category_parfumery::Migration),
            Box::new(m20240515_090501_customer_view_history::Migration),
        ]
    }
}
