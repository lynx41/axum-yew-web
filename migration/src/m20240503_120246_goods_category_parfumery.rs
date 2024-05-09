use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        
        manager
            .create_table(
                Table::create()
                    .table(ParfumeryBrand::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ParfumeryBrand::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()   
                    )
                    .col(
                        ColumnDef::new(ParfumeryBrand::Brand)
                            .string_len(49)
                            .not_null()
                    )
                        .to_owned()
            )
                .await?;
        
        manager
            .create_table(
                Table::create()
                    .table(ParfumeryVolume::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ParfumeryVolume::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()   
                    )
                    .col(
                        ColumnDef::new(ParfumeryVolume::Volume)
                            .small_unsigned()
                            .not_null()
                    )
                        .to_owned()
            )
                .await?;

        manager
            .create_table(
                Table::create()
                    .table(ParfumerySeasonality::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ParfumerySeasonality::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(ParfumerySeasonality::Seasonality)
                            .string_len(21)
                            .not_null()   
                    )
                        .to_owned()
            )
                .await?;

        manager
            .create_table(
                Table::create()
                    .table(ParfumeryClass::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ParfumeryClass::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(ParfumeryClass::Class)
                            .string_len(25)
                            .not_null()
                    )
                        .to_owned()
            )
                .await?;
        
        manager
            .create_table(
                Table::create()
                    .table(GoodsList::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GoodsList::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(GoodsList::Name)
                            .string()
                            .not_null()   
                    )
                        .to_owned()
            )
                .await?;

        manager
            .create_table(
                Table::create()
                    .table(CategoryParfumery::Table)
                    .if_not_exists()
                    // BASE
                    .col(
                        ColumnDef::new(CategoryParfumery::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(CategoryParfumery::TilePictureSrc)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(CategoryParfumery::ProductPageSrc)
                            .integer()
                            .not_null()   
                    )
                    .col(
                        ColumnDef::new(CategoryParfumery::ProductBigDesc)
                            .text()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(CategoryParfumery::OldPrice)
                            .integer()   
                    )
                    .col(
                        ColumnDef::new(CategoryParfumery::Price)
                            .integer()
                            .not_null()   
                    )
                    .col(
                        ColumnDef::new(CategoryParfumery::TitleId)
                            .integer()
                            .not_null()
                    )
                    // SPECS
                    .col(
                        ColumnDef::new(CategoryParfumery::BrandId)
                            .integer()
                            .not_null()   
                    )
                    .col(
                        ColumnDef::new(CategoryParfumery::VolumeId)
                            .integer()
                            .not_null()   
                    )
                    .col(
                        ColumnDef::new(CategoryParfumery::SeasonalityId)
                            .integer()
                            .not_null()   
                    )
                    .col(
                        ColumnDef::new(CategoryParfumery::ClassId)
                            .integer()
                            .not_null()
                    )
                    // META
                    .col(
                        ColumnDef::new(CategoryParfumery::VisitedCount)
                            .integer()
                            .not_null()   
                    )
                    .col(
                        ColumnDef::new(CategoryParfumery::BoughtCount)
                            .integer()
                            .not_null()   
                    )
                    .col(
                        ColumnDef::new(CategoryParfumery::WishListCount)
                            .integer()
                            .not_null()   
                    )
                        .to_owned()
            ).await?;

        // Defining foreign keys
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-parfumery-brand-id")
                    .from(CategoryParfumery::Table, CategoryParfumery::BrandId)
                    .to(ParfumeryBrand::Table, ParfumeryBrand::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned()
            )
                .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-parfumery-volume-id")
                    .from(CategoryParfumery::Table, CategoryParfumery::VolumeId)
                    .to(ParfumeryVolume::Table, ParfumeryVolume::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned()
            )
                .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-parfumery-seasonality-id")
                    .from(CategoryParfumery::Table, CategoryParfumery::SeasonalityId)
                    .to(ParfumerySeasonality::Table, ParfumerySeasonality::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned()
            )
                .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-parfumery-class-id")
                    .from(CategoryParfumery::Table, CategoryParfumery::ClassId)
                    .to(ParfumeryClass::Table, ParfumeryClass::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned()
            )
                .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-parfumery-title-id")
                    .from(CategoryParfumery::Table, CategoryParfumery::TitleId)
                    .to(GoodsList::Table, GoodsList::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned()
            )
                .await?;

        // SEED

        let insert_parfumery_brand = Query::insert()
            .into_table(ParfumeryBrand::Table)
            .columns([ParfumeryBrand::Brand])
            .values_panic(["Kilian".into()])
            .values_panic(["Carolina Herrera".into()])
            .values_panic(["Montale".into()])
            .to_owned();
            
        let insert_parfumery_seasonality = Query::insert()
            .into_table(ParfumerySeasonality::Table)
            .columns([ParfumerySeasonality::Seasonality])
            .values_panic(["Весінній".into()])
            .values_panic(["Зимовий".into()])
            .values_panic(["Літній".into()])
            .values_panic(["Осінній".into()])
            .to_owned();

        let insert_parfumery_volume = Query::insert()
            .into_table(ParfumeryVolume::Table)
            .columns([ParfumeryVolume::Volume])
            .values_panic([(100).into()])
            .values_panic([(20).into()])
            .values_panic([(30).into()])
            .to_owned();

        let insert_parfumery_class = Query::insert()
            .into_table(ParfumeryClass::Table)
            .columns([ParfumeryClass::Class])
            .values_panic(["Мас-маркет".into()])
            .values_panic(["Елітні".into()])
            .values_panic(["Нішова".into()])
            .to_owned();


        let insert_goods_list_part1 = Query::insert()
            .into_table(GoodsList::Table)
            .columns([GoodsList::Name])
            .values_panic(["Туалетна вода чоловіча Carolina Herrera 212 Sexy Men 30 мл (8411061906965)".into()])
            .values_panic(["Набір для чоловіків Carolina Herrera Bad Boy Туалетна вода 100 мл + мініатюрка 10 мл (8411061064665)".into()])
            .values_panic(["Туалетна вода для чоловіків Carolina Herrera Bad Boy Dazzling Garden 100 мл (8411061056738)".into()])
            .values_panic(["Набір для чоловіків Carolina Herrera Bad Boy Туалетна вода 100 мл + мініатюрка 10 мл + дезодорант 100 мл (8411061064627)".into()])
            .values_panic(["Набір для чоловіків Carolina Herrera Bad Boy Extreme Парфумована вода 100 мл + гель для душу 100 мл (8411061057391)".into()])
            .values_panic(["Набір для чоловіків Carolina Herrera 212 Men Heroes Туалетна вода 90 мл + мініатюрка 10 мл + гель для душу 100 мл (8411061065198)".into()])
            .values_panic(["Тестер Туалетна вода для жінок Carolina Herrera For Women 100 мл (8411061061145)".into()])
            .values_panic(["Туалетна вода унісекс Carolina Herrera Blond Jasmine 100 мл (8411061869161)".into()])
            .values_panic(["Тестер Парфумована вода для жінок Carolina Herrera Ch 20 мл (8411061934258)".into()])
            .values_panic(["Парфумована вода для чоловіків By Kilian Straight to Heaven White Cristal з клатчем 30 мл (3700550218784)".into()])
            .to_owned();

        let insert_goods_list_part2 = Query::insert()
            .into_table(GoodsList::Table)
            .columns([GoodsList::Name])
            .values_panic(["Характеристики Набір Christmas 2023 Парфумована вода Carolina Herrera Good Girl 80мл + Body Lotion 100мл + Spray Edp 10мл (8411061075234)".into()])
            .values_panic(["Набір Christmas 2023 Carolina Herrera парфумована вода 212 Vip Rose 80 мл + Body Lotion 100 мл + Spray Edp Mini (8411061074930)".into()])
            .values_panic(["Набір Christmas 2023 Carolina Herrera Men туалетна вода 100 мл + After Shave 100 мл + Spray Edt 10 мл (8411061074947)".into()])
            .values_panic(["Набір Carolina Herrera туалетна вода 100мл + Body Lotion 100мл + Spray Edt 10мл (8411061074954)".into()])
            .values_panic(["Набір Christmas 2023 Парфумована вода Carolina Herrera 212 Spray 80мл + Body Lotion 100мл + Roll-On 10мл (8411061074909)".into()])
            .values_panic(["Набір Christmas 2024 туалетна вода Carolina Herrera 212 Spray 100мл + Body Lotion 100мл + EDT Travel (8411061074893)".into()])
            .values_panic(["Набір Christmas 2023 Carolina Herrera 212 Men туалетна вода 100 мл + Shower Gel 100 мл + Spray Edt 10 мл (8411061074916)".into()])
            .values_panic(["Туалетна вода для чоловіків Carolina Herrera 212 VIP Men 100 мл (8411061723760)".into()])
            .values_panic(["Туалетна вода для чоловіків Carolina Herrera 212 Sexy Men Eau De Toilette Spray 30 мл (8411061865583)".into()])
            .values_panic(["Туалетна вода для чоловіків Carolina Herrera 212 Men Heroes 20 мл (8411061972656)".into()])
            .values_panic(["Набір для чоловіків Carolina Herrera 212 Men Heroes Forever Young Туалетна вода 20 мл + Туалетна вода 10 мл (8411061032770)".into()])
            .values_panic(["Набір Carolina Herrera Bad Boy Eau De Toilette 100 мл + Гель для душу 100 мл Christmas Set 2022 (8411061046210)".into()])
            .values_panic(["Тестер Туалетна вода для чоловіків Carolina Herrera Chic For Men 100 мл".into()])
            .values_panic(["Туалетна вода для чоловіків Carolina Herrera 212 Men 100 мл".into()])
            .values_panic(["Тестер Туалетна вода для чоловіків Carolina Herrera Bad Boy 100 мл".into()])
            .values_panic(["Туалетна вода для чоловіків Carolina Herrera CH Men 100 мл".into()])
            .values_panic(["Парфумована вода для жінок Carolina Herrera Good Girl Blush 30 мл".into()])
            .values_panic(["Тестер Туалетна вода для чоловіків Carolina Herrera CH Men 100 мл".into()])
            .values_panic(["Тестер Туалетна вода для чоловіків Carolina Herrera 212 Men Heroes Forever Young 20 мл".into()])
            .values_panic(["Туалетна вода для чоловіків Carolina Herrera 212 Men 30 мл".into()])
            .values_panic(["Туалетна вода для чоловіків Carolina Herrera Herrera For Men 100 мл".into()])
            .values_panic(["Змінний блок парфумованої води для жінок Kilian Love Don't Be Shy Eau Fraiche 100 мл (3700550234005)".into()])
            .values_panic(["Тестер Парфумована вода для жінок Montale Roses Musk 100 мл".into()])
            .values_panic(["Парфумована вода унісекс Montale Intense Cafe Ristretto 100 мл".into()])
            .values_panic(["Парфумована вода унісекс Kilian Angels' Share 100 мл".into()])
            .values_panic(["Парфумована вода для жінок Montale Dark Purple 100 мл".into()])
            .values_panic(["Парфумована вода унісекс Montale Sensual Instinct 100 мл".into()])
            .values_panic(["Тестер Парфумована вода унісекс Montale Oudmazing 100 мл".into()])
            .values_panic(["Тестер Парфумована вода унісекс Montale Wild Pears 100 мл".into()])
            .values_panic(["Парфумована вода унісекс Montale Crystal Flowers 100 мл".into()])
            .values_panic(["Парфумована вода унісекс Montale Musk to Musk 100 мл".into()])
            .values_panic(["Парфумована вода унісекс Montale Aoud Forest 100 мл".into()])
            .values_panic(["Парфумована вода унісекс Montale Intense Cafe 100 мл".into()])
            .values_panic(["Парфумована вода для чоловіків Montale Red Vetiver 100 мл".into()])
            .values_panic(["Подарунковий набір мініатюр для жінок Good Girl Gone Bad парфумована вода 4 х 7,5 мл".into()])
            .values_panic(["Парфумована вода унісекс Montale Fantastic Basilic 100 мл".into()])
            .values_panic(["Парфумована вода для жінок Montale Pure Gold 100 мл".into()])
            .values_panic(["Тестер Парфумована вода унісекс By Kilian Voulez-Vous Coucher Avec Moi 100 мл (refill)".into()])
            .values_panic(["Парфумована вода унісекс Montale Oud Tobacco 100 мл".into()])
            .to_owned();


        let insert_caregory_parfumery_part1 = Query::insert()
            .into_table(CategoryParfumery::Table)
            .columns(
                [
                    CategoryParfumery::TilePictureSrc, CategoryParfumery::ProductPageSrc, CategoryParfumery::ProductBigDesc,
                    CategoryParfumery::Price,
                    CategoryParfumery::TitleId, CategoryParfumery::BrandId, CategoryParfumery::VolumeId,
                    CategoryParfumery::SeasonalityId, CategoryParfumery::ClassId, CategoryParfumery::VisitedCount,
                    CategoryParfumery::BoughtCount, CategoryParfumery::WishListCount
                ]
            )

            // record 1
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/413654724.jpg".into(),

                // ProductPageSrc (Integer, not null)
                841106965.into(),

                // ProductBigDesc (Text, not null)
                "Верхня нота: зелені ноти, мандарин, бергамот;
                Переклад: квіти, кардамон;
                Базова нота: амбра, ваніль, мускус, сандалове дерево, гуаяк.".into(),

                // Price (Integer, not null)
                1367.into(),

                // TitleId (Integer, not null)
                1.into(),

                // BrandId (Integer, not null)
                2.into(),

                // VolumeId (Integer, not null)
                3.into(),
                
                // SeasonalityId (Integer, not null)
                1.into(),

                // ClassId (Integer, not null)
                1.into(),

                // VisitedCount (Integer, not null)
                0.into(),

                // BoughtCount (Integer, not null)
                0.into(),

                // WishListCount (Integer, not null)
                0.into(),
            ])

            // record 3
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/418811944.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (841106665).into(),

                // ProductBigDesc (Text, not null)
                "Верхня нота: грейпфрут;
                Переклад: чорний перець, герань;
                Базові ноти: шкіра, ветивер".into(),

                // Price (Integer, not null)
                (2799).into(),

                // TitleId (Integer, not null)
                (3).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (3).into(),

                // ClassId (Integer, not null)
                (1).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])
            
            // record 8
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/408932792.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (841056738).into(),

                // ProductBigDesc (Text, not null)
                "Верхня нота: білий перець, чорний перець, бергамот;
                Нота серця: кедр;
                Базові ноти: какао, боби тонка, амбра".into(),

                // Price (Integer, not null)
                (3425).into(),

                // TitleId (Integer, not null)
                (8).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (1).into(),

                // ClassId (Integer, not null)
                (1).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 10
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/408932868.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (841064627).into(),

                // ProductBigDesc (Text, not null)
                "Верхня нота чорний перець, бергамот, білий перець;
                Нота серця: кедр, шавлія;
                Базові ноти: какао, боби тонка, амбра ".into(),
                
                // Price (Integer, not null)
                (3648).into(),

                // TitleId (Integer, not null)
                (10).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (3).into(),

                // ClassId (Integer, not null)
                (1).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 11
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/408932867.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (841057391).into(),

                // ProductBigDesc (Text, not null)
                "Верхня нота: шавлія, імбир;
                Нота серця: ветівер, какао;
                Базові ноти: боби тонка, пачулі ".into(),

                // Price (Integer, not null)
                (3768).into(),

                // TitleId (Integer, not null)
                (11).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (1).into(),

                // ClassId (Integer, not null)
                (1).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 12
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/408932861.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (861065198).into(),

                // ProductBigDesc (Text, not null)
                "Верхня нота: ноти імбиру та еліксиру груші, посилені коноплею;
                Нота серця: олія герані;
                Базові ноти: шкіра, дерево".into(),

                // Price (Integer, not null)
                (3414).into(),

                // TitleId (Integer, not null)
                (12).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (1).into(),

                // ClassId (Integer, not null)
                (1).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 14
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/406918695.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (841161145).into(),

                // ProductBigDesc (Text, not null)
                "".into(),

                // Price (Integer, not null)
                (4199).into(),

                // TitleId (Integer, not null)
                (14).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (3).into(),

                // ClassId (Integer, not null)
                (1).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 16
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/428804448.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (841869161).into(),

                // ProductBigDesc (Text, not null)
                "".into(),

                // Price (Integer, not null)
                (5391).into(),

                // TitleId (Integer, not null)
                (16).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (4).into(),

                // ClassId (Integer, not null)
                (2).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 19
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/422874857.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (861934258).into(),

                // ProductBigDesc (Text, not null)
                "".into(),

                // Price (Integer, not null)
                (1546).into(),

                // TitleId (Integer, not null)
                (19).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (2).into(),
                
                // SeasonalityId (Integer, not null)
                (3).into(),

                // ClassId (Integer, not null)
                (2).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 31
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/407111600.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (350218784).into(),

                // ProductBigDesc (Text, not null)
                "".into(),

                // Price (Integer, not null)
                (9382).into(),

                // TitleId (Integer, not null)
                (31).into(),

                // BrandId (Integer, not null)
                (1).into(),

                // VolumeId (Integer, not null)
                (3).into(),
                
                // SeasonalityId (Integer, not null)
                (1).into(),

                // ClassId (Integer, not null)
                (3).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            .to_owned();


        let insert_caregory_parfumery_part2 = Query::insert()
            .into_table(CategoryParfumery::Table)
            .columns(
                [
                    CategoryParfumery::TilePictureSrc, CategoryParfumery::ProductPageSrc,
                    CategoryParfumery::ProductBigDesc, CategoryParfumery::OldPrice, CategoryParfumery::Price,
                    CategoryParfumery::TitleId, CategoryParfumery::BrandId, CategoryParfumery::VolumeId,
                    CategoryParfumery::SeasonalityId, CategoryParfumery::ClassId, CategoryParfumery::VisitedCount,
                    CategoryParfumery::BoughtCount, CategoryParfumery::WishListCount
                ]
            )

            // record 2
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/381869931.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (841105234).into(),

                // ProductBigDesc (Text, not null)
                "".into(),
                
                // Old Price (Integer, optional)
                (4809).into(),

                // Price (Integer, not null)
                (4323).into(),

                // TitleId (Integer, not null)
                (2).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (4).into(),

                // ClassId (Integer, not null)
                (1).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 4
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/382260937.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (841174930).into(),

                // ProductBigDesc (Text, not null)
                "".into(),
                
                // Old Price (Integer, optional)
                (4333).into(),

                // Price (Integer, not null)
                (2999).into(),

                // TitleId (Integer, not null)
                (4).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (2).into(),

                // ClassId (Integer, not null)
                (1).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 5
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/382203121.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (841074947).into(),

                // ProductBigDesc (Text, not null)
                "".into(),
                
                // Old Price (Integer, optional)
                (3750).into(),

                // Price (Integer, not null)
                (2699).into(),

                // TitleId (Integer, not null)
                (5).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (1).into(),

                // ClassId (Integer, not null)
                (1).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 6
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/381962495.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (841106954).into(),

                // ProductBigDesc (Text, not null)
                "".into(),
                
                // Old Price (Integer, optional)
                (4440).into(),

                // Price (Integer, not null)
                (3072).into(),

                // TitleId (Integer, not null)
                (6).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (4).into(),

                // ClassId (Integer, not null)
                (1).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 7
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/381690844.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (841074909).into(),

                // ProductBigDesc (Text, not null)
                "".into(),
                
                // Old Price (Integer, optional)
                (4243).into(),

                // Price (Integer, not null)
                (2936).into(),

                // TitleId (Integer, not null)
                (7).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (3).into(),

                // ClassId (Integer, not null)
                (1).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 9
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/381681659.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (841074893).into(),

                // ProductBigDesc (Text, not null)
                "".into(),
                
                // Old Price (Integer, optional)
                (4209).into(),

                // Price (Integer, not null)
                (3265).into(),

                // TitleId (Integer, not null)
                (9).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (2).into(),

                // ClassId (Integer, not null)
                (1).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 13
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/382003898.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (841174916).into(),

                // ProductBigDesc (Text, not null)
                "".into(),
                
                // Old Price (Integer, optional)
                (3595).into(),

                // Price (Integer, not null)
                (3158).into(),

                // TitleId (Integer, not null)
                (13).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (2).into(),

                // ClassId (Integer, not null)
                (1).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 15
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/353549500.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (841723760).into(),

                // ProductBigDesc (Text, not null)
                "Горілка, м'ята, лайм, спеції, королівське дерево, амбра, боби.".into(),
                
                // Old Price (Integer, optional)
                (3331).into(),

                // Price (Integer, not null)
                (2946).into(),

                // TitleId (Integer, not null)
                (15).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (1).into(),

                // ClassId (Integer, not null)
                (2).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 17
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/359925807.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (841865583).into(),

                // ProductBigDesc (Text, not null)
                "".into(),
                
                // Old Price (Integer, optional)
                (2914).into(),

                // Price (Integer, not null)
                (2352).into(),

                // TitleId (Integer, not null)
                (17).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (3).into(),
                
                // SeasonalityId (Integer, not null)
                (1).into(),

                // ClassId (Integer, not null)
                (2).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 18
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/356734586.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (841172656).into(),

                // ProductBigDesc (Text, not null)
                "Верхні ноти: бергамот, лаванда, м'ята, кмин
                Переклад: Кедр, Сандал
                Кінцева нота: Амбра, Боби ".into(),
                
                // Old Price (Integer, optional)
                (3138).into(),

                // Price (Integer, not null)
                (2496).into(),

                // TitleId (Integer, not null)
                (18).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (2).into(),
                
                // SeasonalityId (Integer, not null)
                (2).into(),

                // ClassId (Integer, not null)
                (2).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 20
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/384515183.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (861032770).into(),

                // ProductBigDesc (Text, not null)
                "".into(),
                
                // Old Price (Integer, optional)
                (3048).into(),

                // Price (Integer, not null)
                (2109).into(),

                // TitleId (Integer, not null)
                (20).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (3).into(),
                
                // SeasonalityId (Integer, not null)
                (1).into(),

                // ClassId (Integer, not null)
                (2).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 21
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/360760380.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (841046210).into(),

                // ProductBigDesc (Text, not null)
                "Нота серця: кедр, шавлія
                Кінцева нота: боби тонка, какао
                Початкова нота: білий перець, чорний перець".into(),
                
                // Old Price (Integer, optional)
                (3988).into(),

                // Price (Integer, not null)
                (2760).into(),

                // TitleId (Integer, not null)
                (21).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (2).into(),

                // ClassId (Integer, not null)
                (2).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 22
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/377286377.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (402425172).into(),

                // ProductBigDesc (Text, not null)
                "".into(),
                
                // Old Price (Integer, optional)
                (2790).into(),

                // Price (Integer, not null)
                (2034).into(),

                // TitleId (Integer, not null)
                (22).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (4).into(),

                // ClassId (Integer, not null)
                (2).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 23
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/377286087.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (402425160).into(),

                // ProductBigDesc (Text, not null)
                "".into(),
                
                // Old Price (Integer, optional)
                (3962).into(),

                // Price (Integer, not null)
                (3189).into(),

                // TitleId (Integer, not null)
                (23).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (1).into(),

                // ClassId (Integer, not null)
                (2).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 24
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/348989587.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (386034240).into(),

                // ProductBigDesc (Text, not null)
                "".into(),
                
                // Old Price (Integer, optional)
                (3621).into(),

                // Price (Integer, not null)
                (2872).into(),

                // TitleId (Integer, not null)
                (24).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (2).into(),

                // ClassId (Integer, not null)
                (2).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 25
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/421796195.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (425479677).into(),

                // ProductBigDesc (Text, not null)
                "Верхні ноти: Трава, Бергамот та Грейпфрут.
                Середні: Деревні ноти, Мускатний горіх, Фіалка, Шафран і Жасмин.
                Базові ноти: Цукор, Шкіра, Ваніль, Замша, Амбра, Кашемірове дерево, Сандал, Дубовий мох та Ветивер. ".into(),
                
                // Old Price (Integer, optional)
                (4378).into(),

                // Price (Integer, not null)
                (3368).into(),

                // TitleId (Integer, not null)
                (25).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (3).into(),

                // ClassId (Integer, not null)
                (2).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 26
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/399508805.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (414612546).into(),

                // ProductBigDesc (Text, not null)
                "Верхні ноти: Бергамот та Горький мигдаль.
                Середні: Півонія та Іланг-іланг.
                Базові ноти: Ваніль та Кумарін.".into(),
                
                // Old Price (Integer, optional)
                (3707).into(),

                // Price (Integer, not null)
                (2802).into(),

                // TitleId (Integer, not null)
                (26).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (3).into(),
                
                // SeasonalityId (Integer, not null)
                (1).into(),

                // ClassId (Integer, not null)
                (2).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 27
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/421795472.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (425479668).into(),

                // ProductBigDesc (Text, not null)
                "Верхні ноти: Трава, Бергамот та Грейпфрут.
                Середні: Деревні ноти, Мускатний горіх, Фіалка, Шафран і Жасмин.
                Базові ноти: Цукор, Шкіра, Ваніль, Замша, Амбра, Кашемірове дерево, Сандал, Дубовий мох та Ветивер.".into(),
                
                // Old Price (Integer, optional)
                (3734).into(),

                // Price (Integer, not null)
                (2872).into(),

                // TitleId (Integer, not null)
                (27).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (4).into(),

                // ClassId (Integer, not null)
                (2).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 28
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/409870030.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (418447656).into(),

                // ProductBigDesc (Text, not null)
                "Верхні ноти: Груша, Конопля та Імбир.
                Середні: Серінь: Герань і Шавлія.
                Базові ноти: Мускус та Шкіра.".into(),
                
                // Old Price (Integer, optional)
                (3745).into(),

                // Price (Integer, not null)
                (2802).into(),

                // TitleId (Integer, not null)
                (28).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (2).into(),
                
                // SeasonalityId (Integer, not null)
                (2).into(),

                // ClassId (Integer, not null)
                (2).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 29
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/377285410.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (402425418).into(),

                // ProductBigDesc (Text, not null)
                "Верхні ноти: Зелені ноти, Грейпфрут, Спеції, Бергамот, Лаванда та Петітгрейн.
                Середні: Імбір, Фіалка, Гарденія і Шавлія.
                Базові ноти: Мускус, Сандал, Ладан, Гваяк, Ветивер та Лабданум. ".into(),
                
                // Old Price (Integer, optional)
                (1936).into(),

                // Price (Integer, not null)
                (1509).into(),

                // TitleId (Integer, not null)
                (29).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (3).into(),
                
                // SeasonalityId (Integer, not null)
                (3).into(),

                // ClassId (Integer, not null)
                (2).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 30
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/409870004.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (418447611).into(),

                // ProductBigDesc (Text, not null)
                "Верхні ноти: Лимон, Лаванда, Розмарин та Неролі.
                Середні: Гвоздика (пряність), Герань і Конюшина.
                Базові ноти: Тютюн, Сандал та Сіра амбра. ".into(),
                
                // Old Price (Integer, optional)
                (3325).into(),

                // Price (Integer, not null)
                (2274).into(),

                // TitleId (Integer, not null)
                (30).into(),

                // BrandId (Integer, not null)
                (2).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (4).into(),

                // ClassId (Integer, not null)
                (2).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 32
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/408849694.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (417844266).into(),

                // ProductBigDesc (Text, not null)
                "".into(),
                
                // Old Price (Integer, optional)
                (10784).into(),

                // Price (Integer, not null)
                (10108).into(),

                // TitleId (Integer, not null)
                (32).into(),

                // BrandId (Integer, not null)
                (1).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (1).into(),

                // ClassId (Integer, not null)
                (3).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 33
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/377287202.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (402425259).into(),

                // ProductBigDesc (Text, not null)
                "Верхня нота: Жасмин. Переклад: Роза. 
                Базові ноти: Мускус, Амбра.".into(),
                
                // Old Price (Integer, optional)
                (3710).into(),

                // Price (Integer, not null)
                (2949).into(),

                // TitleId (Integer, not null)
                (33).into(),

                // BrandId (Integer, not null)
                (3).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (3).into(),

                // ClassId (Integer, not null)
                (3).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 34
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/377285417.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (402425430).into(),

                // ProductBigDesc (Text, not null)
                "Верхні ноти: Кава та Турецька троянда.
                Середні: Обсмажена кава, Французька троянда та Деревні ноти.
                Базові ноти: Ваніль, Карамель, Білий мускус та Амбра. ".into(),
                
                // Old Price (Integer, optional)
                (4993).into(),

                // Price (Integer, not null)
                (3957).into(),

                // TitleId (Integer, not null)
                (34).into(),

                // BrandId (Integer, not null)
                (3).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (2).into(),

                // ClassId (Integer, not null)
                (3).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 35
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/426317122.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (427863989).into(),

                // ProductBigDesc (Text, not null)
                "Верхня нота: Коньяк.
                Середня: Сериця: Кориця, Боби тонка і Дуб.
                Базові ноти: Ваніль, Праліне та Сандал. ".into(),
                
                // Old Price (Integer, optional)
                (19284).into(),

                // Price (Integer, not null)
                (14834).into(),

                // TitleId (Integer, not null)
                (35).into(),

                // BrandId (Integer, not null)
                (1).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (1).into(),

                // ClassId (Integer, not null)
                (3).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 36
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/373305533.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (400332444).into(),

                // ProductBigDesc (Text, not null)
                "Верхні ноти: Слива та Апельсин.
                Середні: Роза, Червоні ягоди, Пачулі та Герань.
                Базові ноти: Мускус, Тикове дерево та Амбра. ".into(),
                
                // Old Price (Integer, optional)
                (4209).into(),

                // Price (Integer, not null)
                (3316).into(),

                // TitleId (Integer, not null)
                (36).into(),

                // BrandId (Integer, not null)
                (3).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (2).into(),

                // ClassId (Integer, not null)
                (3).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 37
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/426315643.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (427863953).into(),

                // ProductBigDesc (Text, not null)
                "Верхня нота: Обсмажена кава.
                Середні: Серця: Праліне і Роза.
                Базові ноти: Дубовий мох, Амбра та Кедр. ".into(),
                
                // Old Price (Integer, optional)
                (4581).into(),

                // Price (Integer, not null)
                (3426).into(),

                // TitleId (Integer, not null)
                (37).into(),

                // BrandId (Integer, not null)
                (3).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (4).into(),

                // ClassId (Integer, not null)
                (3).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 38
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/426317094.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (427863959).into(),

                // ProductBigDesc (Text, not null)
                "Верхні ноти: Уд, Інжир, Груша, Сицилійський апельсин та Сицилійський бергамот.
                Середні: Лист пачулі, Корінь ірису та Єгипетський жасмин.
                Базові ноти: Виноград, Мадагаскарська ваніль, Шкіра та Білий мускус.".into(),
                
                // Old Price (Integer, optional)
                (3980).into(),

                // Price (Integer, not null)
                (2917).into(),

                // TitleId (Integer, not null)
                (38).into(),

                // BrandId (Integer, not null)
                (3).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (4).into(),

                // ClassId (Integer, not null)
                (3).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 39
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/365483344.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (395965497).into(),

                // ProductBigDesc (Text, not null)
                "Верхні ноти: Груша та Бергамот.
                Середні: Гвоздика (квітка) і Конвалія.
                Базові ноти: Ваніль, Мускус та Сандал.".into(),
                
                // Old Price (Integer, optional)
                (3091).into(),

                // Price (Integer, not null)
                (2503).into(),

                // TitleId (Integer, not null)
                (39).into(),

                // BrandId (Integer, not null)
                (3).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (1).into(),

                // ClassId (Integer, not null)
                (3).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 40
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/365026590.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (395654277).into(),

                // ProductBigDesc (Text, not null)
                "Верхні ноти: Роза, Конвалія.
                Середні: Серце: Мандарин.
                Базові ноти: Мускус, Амбра.".into(),
                
                // Old Price (Integer, optional)
                (4441).into(),

                // Price (Integer, not null)
                (3360).into(),

                // TitleId (Integer, not null)
                (40).into(),

                // BrandId (Integer, not null)
                (3).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (2).into(),

                // ClassId (Integer, not null)
                (3).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 41
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/365483613.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (395965539).into(),

                // ProductBigDesc (Text, not null)
                "Верхні ноти: Амбра, Тикове дерево.
                Середні: Серця: Деревні ноти, Уд.
                Базові ноти: Мускус, Мускатний горіх.".into(),
                
                // Old Price (Integer, optional)
                (3995).into(),

                // Price (Integer, not null)
                (3062).into(),

                // TitleId (Integer, not null)
                (41).into(),

                // BrandId (Integer, not null)
                (3).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (1).into(),

                // ClassId (Integer, not null)
                (3).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 42
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/373305526.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (400332411).into(),

                // ProductBigDesc (Text, not null)
                "Верхні ноти: Імбир, Грейпфрут, Лимон.
                Середні: Морська вода, Розмарин, Троянда.
                Базові ноти: Уд, Мускус, Сіра Амбра".into(),
                
                // Old Price (Integer, optional)
                (4138).into(),

                // Price (Integer, not null)
                (3120).into(),

                // TitleId (Integer, not null)
                (42).into(),

                // BrandId (Integer, not null)
                (3).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (2).into(),

                // ClassId (Integer, not null)
                (3).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 43
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/377287521.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (402425589).into(),

                // ProductBigDesc (Text, not null)
                "Верхня нота: Квіткові ноти.
                Середні: Роза і Кава.
                Базові ноти: Ваніль, Білий мускус та Амбра.".into(),
                
                // Old Price (Integer, optional)
                (4281).into(),

                // Price (Integer, not null)
                (3233).into(),

                // TitleId (Integer, not null)
                (43).into(),

                // BrandId (Integer, not null)
                (3).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (3).into(),

                // ClassId (Integer, not null)
                (3).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 44
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/377287212.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (402425280).into(),

                // ProductBigDesc (Text, not null)
                "Верхні ноти: Ветивер, Грейпфрут.
                Середні: Перець.
                Базові ноти: Кедр, Перуанський бальзам, Пачулі".into(),
                
                // Old Price (Integer, optional)
                (4209).into(),

                // Price (Integer, not null)
                (3235).into(),

                // TitleId (Integer, not null)
                (44).into(),

                // BrandId (Integer, not null)
                (3).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (4).into(),

                // ClassId (Integer, not null)
                (3).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 45
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/349291459.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (386340642).into(),

                // ProductBigDesc (Text, not null)
                "Верхні ноти: Османтус, Жасмін та Травнева троянда;
                Середні ноти: Індійська тубероза та Нарцис;
                Базові ноти: Амбра та Кедр.".into(),
                
                // Old Price (Integer, optional)
                (9606).into(),

                // Price (Integer, not null)
                (6974).into(),

                // TitleId (Integer, not null)
                (45).into(),

                // BrandId (Integer, not null)
                (1).into(),

                // VolumeId (Integer, not null)
                (3).into(),
                
                // SeasonalityId (Integer, not null)
                (4).into(),

                // ClassId (Integer, not null)
                (3).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 46
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/365076907.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (395711229).into(),

                // ProductBigDesc (Text, not null)
                "Верхні ноти: Базилік, Кумкват та Груша.
                Переклад: Серце Франжіпані і Тубероза.
                Базові ноти: Боби тонка та Бензоїн.".into(),
                
                // Old Price (Integer, optional)
                (4637).into(),

                // Price (Integer, not null)
                (3235).into(),

                // TitleId (Integer, not null)
                (46).into(),

                // BrandId (Integer, not null)
                (3).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (1).into(),

                // ClassId (Integer, not null)
                (3).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 47
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/373304661.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (400332453).into(),

                // ProductBigDesc (Text, not null)
                "Верхні ноти: Абрикос та Мандарин.
                Середні: Жасмин, Неролі і Квітка апельсина.
                Базові ноти: Мускус, Пачулі та Ваніль. ".into(),
                
                // Old Price (Integer, optional)
                (4138).into(),

                // Price (Integer, not null)
                (3235).into(),

                // TitleId (Integer, not null)
                (47).into(),

                // BrandId (Integer, not null)
                (3).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (1).into(),

                // ClassId (Integer, not null)
                (3).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 48
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/416136011.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (422063853).into(),

                // ProductBigDesc (Text, not null)
                "Верхні ноти: Тубероза, Іланг – іланг, Гарденія.
                Середні: Болгарська троянда.
                Базові ноти: Ваніль, Сандал, Кедр.".into(),
                
                // Old Price (Integer, optional)
                (9684).into(),

                // Price (Integer, not null)
                (7778).into(),

                // TitleId (Integer, not null)
                (48).into(),

                // BrandId (Integer, not null)
                (1).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (1).into(),

                // ClassId (Integer, not null)
                (3).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            // record 49
            .values_panic([
                // TilePictureSrc (String, not null)
                "https://localhost:5000/public/categories/perfume/373305529.jpg".into(),

                // ProductPageSrc (Integer, not null)
                (4032441).into(),

                // ProductBigDesc (Text, not null)
                "Верхні ноти: Сумах, Цитруси та Кумін.
                Середні: Уд, Квітка апельсина і Ладан.
                Базові ноти: Тютюн, Боби тонка та Уд.".into(),
                
                // Old Price (Integer, optional)
                (4138).into(),

                // Price (Integer, not null)
                (3310).into(),

                // TitleId (Integer, not null)
                (49).into(),

                // BrandId (Integer, not null)
                (3).into(),

                // VolumeId (Integer, not null)
                (1).into(),
                
                // SeasonalityId (Integer, not null)
                (4).into(),

                // ClassId (Integer, not null)
                (3).into(),

                // VisitedCount (Integer, not null)
                (0).into(),

                // BoughtCount (Integer, not null)
                (0).into(),

                // WishListCount (Integer, not null)
                (0).into()
            ])

            .to_owned();



        manager.exec_stmt(insert_parfumery_brand).await?;
        manager.exec_stmt(insert_parfumery_seasonality).await?;
        manager.exec_stmt(insert_parfumery_volume).await?;
        manager.exec_stmt(insert_parfumery_class).await?;
        manager.exec_stmt(insert_goods_list_part1).await?;
        manager.exec_stmt(insert_goods_list_part2).await?;
        manager.exec_stmt(insert_caregory_parfumery_part1).await?;
        manager.exec_stmt(insert_caregory_parfumery_part2).await

        // SEED END
        
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        
        // drop foreign keys first
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(CategoryParfumery::Table)
                    .name("fk-parfumery-brand-id")
                    .to_owned()
            )
                .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(CategoryParfumery::Table)
                    .name("fk-parfumery-volume-id")
                    .to_owned()
            )
                .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(CategoryParfumery::Table)
                    .name("fk-parfumery-seasonality-id")
                    .to_owned()
            )
                .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(CategoryParfumery::Table)
                    .name("fk-parfumery-class-id")
                    .to_owned()
            )
                .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(CategoryParfumery::Table)
                    .name("fk-parfumery-title-id")
                    .to_owned()
            )
                .await?;

        // then drop tables
        manager
            .drop_table(Table::drop().table(CategoryParfumery::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(GoodsList::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ParfumeryClass::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ParfumerySeasonality::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ParfumeryVolume::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ParfumeryBrand::Table).to_owned())
            .await

    }
}

#[derive(DeriveIden)]
enum ParfumeryVolume {
    Table,
    Id,
    Volume,
}

#[derive(DeriveIden)]
enum ParfumeryClass {
    Table,
    Id,
    Class,
}

#[derive(DeriveIden)]
enum ParfumerySeasonality {
    Table,
    Id,
    Seasonality,
}

#[derive(DeriveIden)]
enum ParfumeryBrand {
    Table,
    Id,
    Brand,
}

#[derive(DeriveIden)]
enum CategoryParfumery {
    Table,
    Id,
    // Base fields
    TilePictureSrc,
    ProductPageSrc,
    ProductSmallDesc,
    ProductBigDesc,
    OldPrice,
    Price,
    TitleId,
    // Specs (refs to other tables)
    VolumeId,
    ClassId,
    SeasonalityId,
    BrandId,
    // Meta
    VisitedCount,
    BoughtCount,
    WishListCount,
}

#[derive(DeriveIden)]
enum GoodsList {
    Table,
    Id,
    Name,
}