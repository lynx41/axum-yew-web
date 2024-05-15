use std::collections::HashSet;

use axum::{extract::State, Json};
use linked_hash_set::LinkedHashSet;
use log::info;
use reqwest::StatusCode;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set, Value};
use serde_json::json;
use shared::models::history::HistoryUpdate;

use crate::database::guest::{Entity as Guests, self};
use crate::database::guest_history::{Entity as GuestHistories, self};

use crate::utils::app_error::AppError;

const ALLOWED_AMOUNT: usize = 6;

pub async fn guest_history(
    State(database): State<DatabaseConnection>,
    Json(guest_history): Json<HistoryUpdate>
) -> Result<(), AppError> {
    // 0. Validate the guest.
    // 1. Find the guest record, if no than create a new one
    // 2. Get the list of its history, and try to add this one (check for the size, no more than 10 records)

    let guest_unique_id = guest_history.unique_id;
    let guest_item_id = guest_history.item_id;

    if let Some(guest) = Guests::find()
        .filter(guest::Column::UniqueId.eq(&guest_unique_id))
        .one(&database)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))? {

            // Find the guest history record
            if let Some(guest_history) = GuestHistories::find()
                .filter(guest_history::Column::GuestId.eq(guest.id))
                .one(&database)
                .await
                .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))? {

                    // Divide into parts
                    let list = guest_history.list.split_terminator(",")
                        .map(|elem| elem.to_owned())
                        .collect::<Vec<String>>();

                    // Remove all dublicates by LinkedHashSet 
                    // (linked orders pieces, instead of random order of just a HashSet)

                    let unique_strings: LinkedHashSet<String> = list.into_iter().collect();
                    let mut list: Vec<String> = unique_strings.into_iter().collect();

                    if list.len() >= ALLOWED_AMOUNT {
                        list.rotate_right(1);
                        list[0] = guest_item_id;
                    } else {
                        list.insert(0, guest_item_id);
                    }

                    let updated_list = list.join(",");

                    let mut active_history: guest_history::ActiveModel = guest_history.into_active_model();
                    active_history.list = Set(updated_list);
                    let _ = active_history.save(&database)
                        .await
                        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?;

                } else {
                    // The guest has no history, create a new one
                    let new_history = guest_history::ActiveModel {
                        guest_id: Set(guest.id),
                        list: Set(guest_item_id),
                        ..Default::default()
                    };

                    let _ = new_history.save(&database)
                        .await
                        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?;
                }

            return Ok(())

        }

    Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))

}