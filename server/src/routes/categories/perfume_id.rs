use axum::{extract::{Path, State}, Json};
use reqwest::StatusCode;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use shared::models::categories::perfume::PerfumeGoods;

use crate::utils::app_error::AppError;

use crate::database::category_parfumery::{Entity as CategoryPerfume, self};
use crate::database::goods_list::{Entity as TitleTable, self};
use crate::database::parfumery_brand::{Entity as PerfumeryBrand, self};
use crate::database::parfumery_seasonality::{Entity as PeftumerySeasonality, self};
use crate::database::parfumery_volume::{Entity as PerfumeryVolume, self};
use crate::database::parfumery_class::{Entity as PerfumeryClass, self};


pub async fn perfume_id(
    State(database): State<DatabaseConnection>,
    Path(item_id): Path<String>
) -> Result<Json<PerfumeGoods>, AppError> {

    let id = item_id.parse::<i32>()
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?;

    if let Some(goods) = CategoryPerfume::find()
        .filter(category_parfumery::Column::ProductPageSrc.eq(id))
        .one(&database)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))? {

            // One item was found, now found the columns data by id's

            let title = TitleTable::find()
                .filter(goods_list::Column::Id.eq(goods.title_id))
                .one(&database)
                .await
                .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?
                .unwrap();

            let brand_name = PerfumeryBrand::find()
                .filter(parfumery_brand::Column::Id.eq(goods.brand_id))
                .one(&database)
                .await
                .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?
                .unwrap();

            let season = PeftumerySeasonality::find()
                .filter(parfumery_seasonality::Column::Id.eq(goods.seasonality_id))
                .one(&database)
                .await
                .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?
                .unwrap();

            let volume = PerfumeryVolume::find()
                .filter(parfumery_volume::Column::Id.eq(goods.volume_id))
                .one(&database)
                .await
                .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?
                .unwrap();

            let class = PerfumeryClass::find()
                .filter(parfumery_class::Column::Id.eq(goods.class_id))
                .one(&database)
                .await
                .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?
                .unwrap();


            return Ok(Json(
                PerfumeGoods {
                    tile_picture_src: goods.tile_picture_src,
                    product_big_desc: goods.product_big_desc,
                    old_price: goods.old_price,
                    price: goods.price,
                    title: title.name,
                    brand: brand_name.brand,
                    volume: volume.volume.to_string(),
                    seasonality: season.seasonality,
                    class: class.class         
                })
            )
        }

    Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))
}