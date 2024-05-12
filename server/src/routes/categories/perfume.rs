use std::io::Bytes;

use axum::{extract::{Query, State}, Json};
use log::info;
use reqwest::StatusCode;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{database::category_parfumery::{self, Entity as Parfumeries}, utils::app_error::AppError};
use crate::database::goods_list::{Entity as GoodsList, self};

use crate::database::parfumery_brand::{Entity as PerfumeBrands, self};
use crate::database::parfumery_seasonality::{Entity as PerfumeSeason, self};
use crate::database::parfumery_volume::{Entity as PerfumeVolume, self};
use crate::database::parfumery_class::{Entity as PerfumeClass, self};

use shared::models::categories::perfume::{PerfumeQuery, PerfumeTile};

// returns goods from the perfume category
pub async fn perfume(
    State(database): State<DatabaseConnection>,
    Query(query_params): Query<PerfumeQuery>
) -> Result<Json<Vec<PerfumeTile>>, AppError> {


    let mut applyed_filters = Condition::all();

    if let Some(brands) = query_params.brand.clone() {
        // split brands to vec and try to find needed ID's
        let picked_brand_lst = brands.split_terminator(",").map(|name| { name.to_owned() }).collect::<Vec<String>>();
        
        for picked_brand in picked_brand_lst.into_iter() {
            if let Some(record) = PerfumeBrands::find()
            .filter(parfumery_brand::Column::Brand.eq(&picked_brand))
            .one(&database)
            .await
            .map_err(|_|
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))? {
                    applyed_filters = applyed_filters.add(category_parfumery::Column::BrandId.eq(record.id));
                }
        }
    }

    if let Some(seasons) = query_params.seasonality.clone() {
        // split brands to vec and try to find needed ID's
        let picked_season_lst = seasons.split_terminator(",").map(|name| { name.to_owned() }).collect::<Vec<String>>();
        
        for picked_season in picked_season_lst.into_iter() {
            if let Some(record) = PerfumeSeason::find()
            .filter(parfumery_seasonality::Column::Seasonality.eq(&picked_season))
            .one(&database)
            .await
            .map_err(|_|
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))? {
                    applyed_filters = applyed_filters.add(category_parfumery::Column::SeasonalityId.eq(record.id));
                }
        }
    }

    if let Some(volume) = query_params.volume.clone() {
        // split brands to vec and try to find needed ID's
        let picked_volume_lst = volume.split_terminator(",").map(|name| { name.to_owned() }).collect::<Vec<String>>();
        
        for picked_volume in picked_volume_lst.into_iter() {
            if let Some(record) = PerfumeVolume::find()
            .filter(parfumery_volume::Column::Volume.eq(picked_volume.parse::<i32>().unwrap()))
            .one(&database)
            .await
            .map_err(|_|
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))? {
                    applyed_filters = applyed_filters.add(category_parfumery::Column::VolumeId.eq(record.id));
                }
        }
    }

    if let Some(class) = query_params.class.clone() {
        // split brands to vec and try to find needed ID's
        let picked_class_lst = class.split_terminator(",").map(|name| { name.to_owned() }).collect::<Vec<String>>();
        
        for picked_class in picked_class_lst.into_iter() {
            if let Some(record) = PerfumeClass::find()
            .filter(parfumery_class::Column::Class.eq(&picked_class))
            .one(&database)
            .await
            .map_err(|_|
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))? {
                    applyed_filters = applyed_filters.add(category_parfumery::Column::ClassId.eq(record.id));
                }
        }
    } 

    // 1. Run the query and get results
    // 2. Write each data to struct
    // 3. Return structured data

    let data_vec = Parfumeries::find()
        .filter(applyed_filters)
        .all(&database)
        .await
        .map_err(|_| AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
        "Internal storage error"))?;
    

    let mut output_vec = Vec::<PerfumeTile>::new();

    for data in data_vec.into_iter() {

        // get the Title as String
        if let Some(title) = GoodsList::find()
            .filter(goods_list::Column::Id.eq(data.title_id))
            .one(&database)
            .await
            .map_err(|_| AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
            "Internal storage error"))? {
                // if the title exist
                output_vec.push({
                    PerfumeTile::from(
                        data.tile_picture_src,
                        data.product_page_src,
                        data.old_price,
                        data.price,
                        title.name
                    )
                });
            } else {
                return Err(AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error"));
            }
    }

    // let serialize = serde_json::to_string(&output_vec)
    //     .map_err(|_| {
    //         AppError::new(
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             "Internal server error")
    //     })?;

    info!("============");
    if let Some(brand) = query_params.brand.clone() {
        info!("Brand: {}", brand);
    } else {
        info!("Brand: None");
    }
    if let Some(seasonality) = query_params.seasonality {
        info!("Seasonality: {}", seasonality);
    } else {
        info!("Seasonality: None");
    }
    if let Some(volume) = query_params.volume {
        info!("Volume: {}", volume);
    } else {
        info!("Volume: None");
    }
    if let Some(class) = query_params.class {
        info!("Class: {}", class);
    } else {
        info!("Class: None");
    }

    if let Some(brands) = query_params.brand.clone() {
        let a = brands.split_terminator(",").map(|name| { name.to_owned() }).collect::<Vec<String>>();

        info!("{:?}", a);
    }


    Ok(Json(output_vec))
}