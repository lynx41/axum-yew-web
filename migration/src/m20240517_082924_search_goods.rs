use sea_orm_migration::prelude::*;

use crate::m20240503_120246_goods_category_parfumery::CategoryParfumery;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(SearchGoods::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SearchGoods::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SearchGoods::Name).string().not_null())
                    .col(ColumnDef::new(SearchGoods::GoodsId).integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-searchgoods-goods-id")
                    .from(SearchGoods::Table, SearchGoods::GoodsId)
                    .to(CategoryParfumery::Table, CategoryParfumery::Id)
                    .to_owned()
            )
                .await?;

        let insert_goods_names = Query::insert()
            .into_table(SearchGoods::Table)
            .columns([SearchGoods::GoodsId, SearchGoods::Name])
            .values_panic([1.into(), "Туалетна вода чоловіча Carolina Herrera 212 Sexy Men 30 мл (8411061906965)".to_lowercase().into()])
            .values_panic([2.into(), "Набір для чоловіків Carolina Herrera Bad Boy Туалетна вода 100 мл + мініатюрка 10 мл (8411061064665)".to_lowercase().into()])
            .values_panic([3.into(), "Туалетна вода для чоловіків Carolina Herrera Bad Boy Dazzling Garden 100 мл (8411061056738)".to_lowercase().into()])
            .values_panic([4.into(), "Набір для чоловіків Carolina Herrera Bad Boy Туалетна вода 100 мл + мініатюрка 10 мл + дезодорант 100 мл (8411061064627)".to_lowercase().into()])
            .values_panic([5.into(), "Набір для чоловіків Carolina Herrera Bad Boy Extreme Парфумована вода 100 мл + гель для душу 100 мл (8411061057391)".to_lowercase().into()])
            .values_panic([6.into(), "Набір для чоловіків Carolina Herrera 212 Men Heroes Туалетна вода 90 мл + мініатюрка 10 мл + гель для душу 100 мл (8411061065198)".to_lowercase().into()])
            .values_panic([7.into(), "Тестер Туалетна вода для жінок Carolina Herrera For Women 100 мл (8411061061145)".to_lowercase().into()])
            .values_panic([8.into(), "Туалетна вода унісекс Carolina Herrera Blond Jasmine 100 мл (8411061869161)".to_lowercase().into()])
            .values_panic([9.into(), "Тестер Парфумована вода для жінок Carolina Herrera Ch 20 мл (8411061934258)".to_lowercase().into()])
            .values_panic([10.into(), "Парфумована вода для чоловіків By Kilian Straight to Heaven White Cristal з клатчем 30 мл (3700550218784)".to_lowercase().into()])

            .values_panic([11.into(), "Характеристики Набір Christmas 2023 Парфумована вода Carolina Herrera Good Girl 80мл + Body Lotion 100мл + Spray Edp 10мл (8411061075234)".to_lowercase().into()])
            .values_panic([12.into(), "Набір Christmas 2023 Carolina Herrera парфумована вода 212 Vip Rose 80 мл + Body Lotion 100 мл + Spray Edp Mini (8411061074930)".to_lowercase().into()])
            .values_panic([13.into(), "Набір Christmas 2023 Carolina Herrera Men туалетна вода 100 мл + After Shave 100 мл + Spray Edt 10 мл (8411061074947)".to_lowercase().into()])
            .values_panic([14.into(), "Набір Carolina Herrera туалетна вода 100мл + Body Lotion 100мл + Spray Edt 10мл (8411061074954)".to_lowercase().into()])
            .values_panic([15.into(), "Набір Christmas 2023 Парфумована вода Carolina Herrera 212 Spray 80мл + Body Lotion 100мл + Roll-On 10мл (8411061074909)".to_lowercase().into()])
            .values_panic([16.into(), "Набір Christmas 2024 туалетна вода Carolina Herrera 212 Spray 100мл + Body Lotion 100мл + EDT Travel (8411061074893)".to_lowercase().into()])
            .values_panic([17.into(), "Набір Christmas 2023 Carolina Herrera 212 Men туалетна вода 100 мл + Shower Gel 100 мл + Spray Edt 10 мл (8411061074916)".to_lowercase().into()])
            .values_panic([18.into(), "Туалетна вода для чоловіків Carolina Herrera 212 VIP Men 100 мл (8411061723760)".to_lowercase().into()])
            .values_panic([19.into(), "Туалетна вода для чоловіків Carolina Herrera 212 Sexy Men Eau De Toilette Spray 30 мл (8411061865583)".to_lowercase().into()])
            .values_panic([20.into(), "Туалетна вода для чоловіків Carolina Herrera 212 Men Heroes 20 мл (8411061972656)".to_lowercase().into()])
            .values_panic([21.into(), "Набір для чоловіків Carolina Herrera 212 Men Heroes Forever Young Туалетна вода 20 мл + Туалетна вода 10 мл (8411061032770)".to_lowercase().into()])
            .values_panic([22.into(), "Набір Carolina Herrera Bad Boy Eau De Toilette 100 мл + Гель для душу 100 мл Christmas Set 2022 (8411061046210)".to_lowercase().into()])
            .values_panic([23.into(), "Тестер Туалетна вода для чоловіків Carolina Herrera Chic For Men 100 мл".to_lowercase().into()])
            .values_panic([24.into(), "Туалетна вода для чоловіків Carolina Herrera 212 Men 100 мл".to_lowercase().into()])
            .values_panic([25.into(), "Тестер Туалетна вода для чоловіків Carolina Herrera Bad Boy 100 мл".to_lowercase().into()])
            .values_panic([26.into(), "Туалетна вода для чоловіків Carolina Herrera CH Men 100 мл".to_lowercase().into()])
            .values_panic([27.into(), "Парфумована вода для жінок Carolina Herrera Good Girl Blush 30 мл".to_lowercase().into()])
            .values_panic([28.into(), "Тестер Туалетна вода для чоловіків Carolina Herrera CH Men 100 мл".to_lowercase().into()])
            .values_panic([29.into(), "Тестер Туалетна вода для чоловіків Carolina Herrera 212 Men Heroes Forever Young 20 мл".to_lowercase().into()])
            .values_panic([30.into(), "Туалетна вода для чоловіків Carolina Herrera 212 Men 30 мл".to_lowercase().into()])
            .values_panic([31.into(), "Туалетна вода для чоловіків Carolina Herrera Herrera For Men 100 мл".to_lowercase().into()])
            .values_panic([32.into(), "Змінний блок парфумованої води для жінок Kilian Love Don't Be Shy Eau Fraiche 100 мл (3700550234005)".to_lowercase().into()])
            .values_panic([33.into(), "Тестер Парфумована вода для жінок Montale Roses Musk 100 мл".to_lowercase().into()])
            .values_panic([34.into(), "Парфумована вода унісекс Montale Intense Cafe Ristretto 100 мл".to_lowercase().into()])
            .values_panic([35.into(), "Парфумована вода унісекс Kilian Angels' Share 100 мл".to_lowercase().into()])
            .values_panic([36.into(), "Парфумована вода для жінок Montale Dark Purple 100 мл".to_lowercase().into()])
            .values_panic([37.into(), "Парфумована вода унісекс Montale Sensual Instinct 100 мл".to_lowercase().into()])
            .values_panic([38.into(), "Тестер Парфумована вода унісекс Montale Oudmazing 100 мл".to_lowercase().into()])
            .values_panic([39.into(), "Тестер Парфумована вода унісекс Montale Wild Pears 100 мл".to_lowercase().into()])
            .values_panic([40.into(), "Парфумована вода унісекс Montale Crystal Flowers 100 мл".to_lowercase().into()])
            .values_panic([41.into(), "Парфумована вода унісекс Montale Musk to Musk 100 мл".to_lowercase().into()])
            .values_panic([42.into(), "Парфумована вода унісекс Montale Aoud Forest 100 мл".to_lowercase().into()])
            .values_panic([43.into(), "Парфумована вода унісекс Montale Intense Cafe 100 мл".to_lowercase().into()])
            .values_panic([44.into(), "Парфумована вода для чоловіків Montale Red Vetiver 100 мл".to_lowercase().into()])
            .values_panic([45.into(), "Подарунковий набір мініатюр для жінок Good Girl Gone Bad парфумована вода 4 х 7,5 мл".to_lowercase().into()])
            .values_panic([46.into(), "Парфумована вода унісекс Montale Fantastic Basilic 100 мл".to_lowercase().into()])
            .values_panic([47.into(), "Парфумована вода для жінок Montale Pure Gold 100 мл".to_lowercase().into()])
            .values_panic([48.into(), "Тестер Парфумована вода унісекс By Kilian Voulez-Vous Coucher Avec Moi 100 мл (refill)".to_lowercase().into()])
            .values_panic([49.into(), "Парфумована вода унісекс Montale Oud Tobacco 100 мл".to_lowercase().into()])
            .to_owned();

        manager.exec_stmt(insert_goods_names)
            .await

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(SearchGoods::Table)
                    .name("fk-searchgoods-goods-id")
                    .to_owned()
            )
                .await?;

        manager
            .drop_table(Table::drop().table(SearchGoods::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SearchGoods {
    Table,
    Id,
    Name,
    GoodsId,
}