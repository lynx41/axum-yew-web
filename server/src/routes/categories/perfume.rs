use axum::{extract::State, Json};
use reqwest::StatusCode;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{database::category_parfumery::{self, Entity as Parfumeries}, utils::app_error::AppError};
use crate::database::goods_list::{Entity as GoodsList, self};
use shared::models::categories::perfume::PerfumeTile;

// returns goods from the perfume category
pub async fn perfume(
    State(database): State<DatabaseConnection>
) -> Result<Json<String>, AppError> {

    // 0. Build filters
    let applyed_filters = Condition::all();
    
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

    let serialize = serde_json::to_string(&output_vec)
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error")
        })?;

    Ok(Json(serialize))
}