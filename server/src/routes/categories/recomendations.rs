use std::collections::{HashMap, HashSet};

use axum::{extract::State, http::HeaderMap, Json};
use reqwest::StatusCode;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use shared::models::categories::perfume::PerfumeGoods;

use crate::database::sessions::{Entity as Sessions, self};
use crate::database::user_portrait::{Entity as UserPortraits, self};
use crate::database::guest_portrait::{Entity as GuestPortraits, self};
use crate::database::category_parfumery::{Entity as CategoryPerfumeries, self};

use crate::utils::app_error::AppError;

struct PortraitParams {
    price_list: Option<String>,
    brand_list: Option<String>,
    volume_list: Option<String>,
    seasson_list: Option<String>,
    class_list: Option<String>,
} 

// For example we have: brand, volume, seasson, class, with 0.5 we only use two of them in random
const PERCENTAGE_OF_CATEGORIES: f32 = 0.5;
// For example we have a category with filters: Brand: AC, AD, A, with 0.6 we only use two of most used ones
const PERCENTAGE_OF_FILTERS: f32 = 0.6;

pub async fn create_filter(
    params: PortraitParams
) -> Condition {

    // Codes: Price - 1, Brand - 2, Volume - 3, Seasson - 4, Class - 5

    // Step 1 - Put the categories in a hashset, and get random ones to needed amount
    let choosen_categories = {
        let mut category_pool: HashMap<i32, String> = HashMap::new();

        if let Some(price_st) = params.price_list { category_pool.insert(1, price_st); }
        if let Some(brand_st) = params.brand_list { category_pool.insert(2, brand_st); }
        if let Some(volume_st) = params.volume_list { category_pool.insert(3, volume_st); }
        if let Some(seasson_st) = params.seasson_list { category_pool.insert(4, seasson_st); }
        if let Some(class_st) = params.class_list { category_pool.insert(5, class_st); }

        // get the size of the pool, so you can get the right part of it
        let available_len = category_pool.len();

        let mut target_len = (available_len as f32 * PERCENTAGE_OF_CATEGORIES).round() as usize;
        let mut picked_result = Vec::new();

        for record in category_pool.into_iter() {
            picked_result.push(record);
            target_len -= 1;
            if target_len == 0 { break; }
        };

        picked_result
    };

    // Step 2 - From the choosen categories remove dublicates, then pick the right amount and apply a filter
        
    let mut applyed_filters = Condition::all();

    // Codes: Price - 1, Brand - 2, Volume - 3, Seasson - 4, Class - 5
    for (code_id, filters_st) in choosen_categories.into_iter() {

        // Divide into parts
        let list = filters_st.split_terminator(",")
            .map(|elem| elem.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();

        // Remove dublicates but add counters, and sort then out by biggest counter
        let mut counter = HashMap::new();
        for key in list.into_iter() {
            *counter.entry(key).or_insert(1) += 1;
        }

        // FOR ALL EXCEPT PRICE

        // Sorting the HashMap by value, because the value is counter
        let list = {
            let mut sorted_counts:Vec<_> = counter.into_iter().collect();
            sorted_counts.sort_by(|a, b| b.1.cmp(&a.1));
            sorted_counts
        };

        // get the size of the pool, to get the right amount of them
        let target_len = (list.len() as f32 * PERCENTAGE_OF_FILTERS).round() as usize;

        // Iterate over tags and apply new filter condition
        for (filter, _) in list[..target_len].iter() {
            // match the code_id and apply filter
            match code_id {
                // Category: Price
                    // SKIP
                
                // Category: Brand
                2 => {  },
                // Category: Volume
                3 => {  },
                // Category: Seasson
                4 => {  },
                // Category: Class
                5 => {  },
                _ => {  }
            }
        }

        // FOR PRICE ONLY
    }

    applyed_filters
}

pub async fn perfume_suggestions(
    State(database): State<DatabaseConnection>,
    headers: HeaderMap
) -> Result<Json<Vec<PerfumeGoods>>, AppError> {

    let mut output_vec = Vec::<PerfumeGoods>::new();

    // Step 1 - Define if the headers have UniqueID
    let option_unique_id = headers.get("UniqueID");

    let mut user_portrait = None;
    let mut guest_portrait = None;

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
                    // Step 3 - Define whos session is was "user | guest"
                    if let Some(user_id) = session.user_id {
                        if let Some(usr_portrait) = UserPortraits::find()
                            .filter(user_portrait::Column::UserId.eq(user_id))
                            .one(&database)
                            .await
                            .map_err(|_| AppError::new(
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "Internal Server Error"))? {
                                    // user portrait was found
                                    user_portrait = Some(usr_portrait);
                                }
                    } else if let Some(guest_id) = session.guest_id {
                        if let Some(gst_portrait) = GuestPortraits::find()
                            .filter(guest_portrait::Column::GuestId.eq(guest_id))
                            .one(&database)
                            .await
                            .map_err(|_| AppError::new(
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "Internal Server Error"))? {
                                    // guest portrait was found
                                    guest_portrait = Some(gst_portrait);
                                }
                    }
                }
    }

    // Step 4 - Load the portrait
    let loaded_portrait = if let Some(portrait) = user_portrait {
        PortraitParams {
            price_list: portrait.price_list,
            brand_list: portrait.brand_list,
            volume_list: portrait.volume_list,
            seasson_list: portrait.seasson_list,
            class_list: portrait.class_list,
        }
    } else {
        let portrait = guest_portrait.unwrap();
        PortraitParams {
            price_list: portrait.price_list,
            brand_list: portrait.brand_list,
            volume_list: portrait.volume_list,
            seasson_list: portrait.seasson_list,
            class_list: portrait.class_list,
        }
    };

    // Step 5 - Make a filter from a portrait parts
    let applyed_filter = create_filter(loaded_portrait).await;

    // Step 6 - Get the goods with the filters


    Ok(Json(output_vec))
}