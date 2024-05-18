use axum::{extract::{Path, State}, http::HeaderMap, Json};
use reqwest::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use shared::models::categories::perfume::PerfumeGoods;

use crate::utils::app_error::AppError;

use crate::database::category_parfumery::{Entity as CategoryPerfume, self};
use crate::database::goods_list::{Entity as TitleTable, self};
use crate::database::parfumery_brand::{Entity as PerfumeryBrand, self};
use crate::database::parfumery_seasonality::{Entity as PeftumerySeasonality, self};
use crate::database::parfumery_volume::{Entity as PerfumeryVolume, self};
use crate::database::parfumery_class::{Entity as PerfumeryClass, self};

use crate::database::sessions::{Entity as Sessions, self};
use crate::database::user_portrait::{Entity as UserPortraits, self};
use crate::database::guest_portrait::{Entity as GuestPortraits, self};

use super::perfume::portret_update;


pub async fn perfume_id(
    State(database): State<DatabaseConnection>,
    Path(item_id): Path<String>,
    headers: HeaderMap
) -> Result<Json<PerfumeGoods>, AppError> {

    let id = item_id.parse::<i32>()
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?;

    if let Some(goods) = CategoryPerfume::find()
        .filter(category_parfumery::Column::ProductPageSrc.eq(id))
        .one(&database)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))? {

            let mut user_portrait = None;
            let mut guest_portrait = None;

            // Step 1 - Define if the headers have UniqueID
            if let Some(some_header) = headers.get("UniqueID") {
                let unique_id = some_header.to_str().unwrap().to_owned();

                // Step 2 - Find a session with such unique_id
                if let Some(session) = Sessions::find()
                    .filter(sessions::Column::UniqueId.eq(&unique_id))
                    .one(&database)
                    .await
                    .map_err(|_| AppError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal Server Error"))? {
                            // Step 3 - Define is this an user or a guest
                                // Step 4 - Get the portrait record or create it
                            if let Some(user_id) = session.user_id {
                                if let Some(usr_portrait) = UserPortraits::find()
                                    .filter(user_portrait::Column::UserId.eq(user_id))
                                    .one(&database)
                                    .await
                                    .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR,"Internal Server Error"))? {
                                        user_portrait = Some(usr_portrait.into_active_model());
                                    } else {
                                        user_portrait = Some(user_portrait::ActiveModel {
                                            user_id: Set(user_id),
                                            price_list: Set(None),
                                            volume_list: Set(None),
                                            class_list: Set(None),
                                            seasson_list: Set(None),
                                            brand_list: Set(None),
                                            ..Default::default()
                                        });
                                    }
                            }else if let Some(guest_id) = session.guest_id {
                                if let Some(gst_portrait) = GuestPortraits::find()
                                    .filter(guest_portrait::Column::GuestId.eq(guest_id))
                                    .one(&database)
                                    .await
                                    .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR,"Internal Server Error"))? {
                                        guest_portrait = Some(gst_portrait.into_active_model());
                                    } else {
                                        guest_portrait = Some(guest_portrait::ActiveModel {
                                            guest_id: Set(guest_id),
                                            price_list: Set(None),
                                            volume_list: Set(None),
                                            class_list: Set(None),
                                            seasson_list: Set(None),
                                            brand_list: Set(None),
                                            ..Default::default()
                                        });
                                    }
                            }
                        }
            }
            
            // Step 5 - Update the record
            if user_portrait.is_some() {

                let old_brand = user_portrait.as_ref().unwrap().brand_list.clone().unwrap();
                let updated_brand = portret_update(old_brand, goods.brand_id, true).await;
                user_portrait.as_mut().unwrap().brand_list = Set(Some(updated_brand));
                
                let old_seasson = user_portrait.as_ref().unwrap().seasson_list.clone().unwrap();
                let updated_seasson = portret_update(old_seasson, goods.seasonality_id, true).await;
                user_portrait.as_mut().unwrap().seasson_list = Set(Some(updated_seasson));
                
                let old_class = user_portrait.as_ref().unwrap().class_list.clone().unwrap();
                let updated_class = portret_update(old_class, goods.class_id, true).await;
                user_portrait.as_mut().unwrap().class_list = Set(Some(updated_class));
                
                let old_volume = user_portrait.as_ref().unwrap().volume_list.clone().unwrap();
                let updated_volume = portret_update(old_volume, goods.volume_id, true).await;
                user_portrait.as_mut().unwrap().volume_list = Set(Some(updated_volume));

                let old_price = user_portrait.as_ref().unwrap().price_list.clone().unwrap();
                let updated_price = portret_update(old_price, goods.price, true).await;
                user_portrait.as_mut().unwrap().price_list = Set(Some(updated_price));

                // save
                user_portrait.unwrap()
                    .save(&database)
                    .await
                    .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR,"Internal Server Error"))?;

            } else if guest_portrait.is_some() {

                let old_brand = guest_portrait.as_ref().unwrap().brand_list.clone().unwrap();
                let updated_brand = portret_update(old_brand, goods.brand_id, false).await;
                guest_portrait.as_mut().unwrap().brand_list = Set(Some(updated_brand));
                
                let old_seasson = guest_portrait.as_ref().unwrap().seasson_list.clone().unwrap();
                let updated_seasson = portret_update(old_seasson, goods.seasonality_id, false).await;
                guest_portrait.as_mut().unwrap().seasson_list = Set(Some(updated_seasson));
                
                let old_class = guest_portrait.as_ref().unwrap().class_list.clone().unwrap();
                let updated_class = portret_update(old_class, goods.class_id, false).await;
                guest_portrait.as_mut().unwrap().class_list = Set(Some(updated_class));
                
                let old_volume = guest_portrait.as_ref().unwrap().volume_list.clone().unwrap();
                let updated_volume = portret_update(old_volume, goods.volume_id, false).await;
                guest_portrait.as_mut().unwrap().volume_list = Set(Some(updated_volume));

                let old_price = guest_portrait.as_ref().unwrap().price_list.clone().unwrap();
                let updated_price = portret_update(old_price, goods.price, false).await;
                guest_portrait.as_mut().unwrap().price_list = Set(Some(updated_price));

                // save
                guest_portrait.unwrap()
                .save(&database)
                .await
                .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR,"Internal Server Error"))?;
            }

            // Step 6 - Save the record

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
                    product_page_src: goods.product_page_src,
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