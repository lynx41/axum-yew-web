use std::io::Bytes;

use axum::{extract::{Query, State}, headers::Header, http::HeaderMap, Json};
use log::info;
use reqwest::StatusCode;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};

use crate::{database::category_parfumery::{self, Entity as Parfumeries}, utils::app_error::AppError};
use crate::database::goods_list::{Entity as GoodsList, self};

use crate::database::parfumery_brand::{Entity as PerfumeBrands, self};
use crate::database::parfumery_seasonality::{Entity as PerfumeSeason, self};
use crate::database::parfumery_volume::{Entity as PerfumeVolume, self};
use crate::database::parfumery_class::{Entity as PerfumeClass, self};

use crate::database::sessions::{Entity as Sessions, self};
use crate::database::user_portrait::{Entity as UserPortraits, self};
use crate::database::guest_portrait::{Entity as GuestPortraits, self};

use shared::models::categories::perfume::{PerfumeQuery, PerfumeTile};

pub async fn portret_update(
    field: Option<String>,
    new_id: i32,
    is_for_user: bool,
) -> String {

    let limiter: usize = if is_for_user { 10 } else { 5 };
    let new_id_st = new_id.to_string();

    if let Some(field_st) = field {

        // Divide into parts
        let mut list = field_st.split_terminator(",")
            .map(|elem| elem.to_owned())
            .collect::<Vec<String>>();

        // Update the list
        if list.len() >= limiter {
            list.rotate_right(1);
            list[0] = new_id_st;
        } else {
            list.push(new_id_st);
        }

        // Collect into String with separator
        list.join(",")
    } else {
        new_id_st
    }
}


// returns goods from the perfume category
pub async fn perfume(
    State(database): State<DatabaseConnection>,
    Query(query_params): Query<PerfumeQuery>,
    headers: HeaderMap,
) -> Result<Json<Vec<PerfumeTile>>, AppError> {

    // Step 1 - Define if the headers have UniqueID 
    let option_unique_id = headers.get("UniqueID");
    // let mut customer_type = AccountType::None;
    let mut guest_portrait = None;
    let mut user_portrait = None;

    if let Some(some_header) = option_unique_id {
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
        // body on condition that header has the unique_id
    }


        // Step 5 - Update the record



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

                    if user_portrait.is_some() {
                        let field = user_portrait.as_ref().unwrap().brand_list.clone().unwrap();
                        let updated_field = portret_update(field, record.id, true).await;
                        user_portrait.as_mut().unwrap().brand_list = Set(Some(updated_field));
                    } else if guest_portrait.is_some() {
                        let field = guest_portrait.as_ref().unwrap().brand_list.clone().unwrap();
                        let updated_field = portret_update(field, record.id, false).await;
                        guest_portrait.as_mut().unwrap().brand_list = Set(Some(updated_field));
                    }
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

                    if user_portrait.is_some() {
                        let field = user_portrait.as_ref().unwrap().seasson_list.clone().unwrap();
                        let updated_field = portret_update(field, record.id, true).await;
                        user_portrait.as_mut().unwrap().seasson_list = Set(Some(updated_field));
                    } else if guest_portrait.is_some() {
                        let field = guest_portrait.as_ref().unwrap().seasson_list.clone().unwrap();
                        let updated_field = portret_update(field, record.id, false).await;
                        guest_portrait.as_mut().unwrap().seasson_list = Set(Some(updated_field));
                    }
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

                    if user_portrait.is_some() {
                        let field = user_portrait.as_ref().unwrap().volume_list.clone().unwrap();
                        let updated_field = portret_update(field, record.id, true).await;
                        user_portrait.as_mut().unwrap().volume_list = Set(Some(updated_field));
                    } else if guest_portrait.is_some() {
                        let field = guest_portrait.as_ref().unwrap().volume_list.clone().unwrap();
                        let updated_field = portret_update(field, record.id, false).await;
                        guest_portrait.as_mut().unwrap().volume_list = Set(Some(updated_field));
                    }
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

                    if user_portrait.is_some() {
                        let field = user_portrait.as_ref().unwrap().class_list.clone().unwrap();
                        let updated_field = portret_update(field, record.id, true).await;
                        user_portrait.as_mut().unwrap().class_list = Set(Some(updated_field));
                    } else if guest_portrait.is_some() {
                        let field = guest_portrait.as_ref().unwrap().class_list.clone().unwrap();
                        let updated_field = portret_update(field, record.id, false).await;
                        guest_portrait.as_mut().unwrap().class_list = Set(Some(updated_field));
                    }
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

    // Step 6 - Save the record of the portrait
    if let Some(u_portrait) = user_portrait {
        u_portrait.save(&database)
            .await
            .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR,"Internal Server Error"))?;
    } else if let Some(g_portrait) = guest_portrait {
        g_portrait.save(&database)
            .await
            .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR,"Internal Server Error"))?;
    }

    Ok(Json(output_vec))
}