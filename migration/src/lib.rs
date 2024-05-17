pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users_roles;
mod m20240323_143315_userlogs_n_userinfo;
mod m20240324_113306_token;
mod m20240503_120246_goods_category_parfumery;
mod m20240515_090501_customer_view_history;
mod m20240515_144211_wish_list_for_users;
mod m20240517_082924_search_goods;
mod m20240517_090204_perfume_portrait;

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
            Box::new(m20240515_144211_wish_list_for_users::Migration),
            Box::new(m20240517_082924_search_goods::Migration),
            Box::new(m20240517_090204_perfume_portrait::Migration),
        ]
    }
}
