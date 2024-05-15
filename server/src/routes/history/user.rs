use axum::{extract::State, Json};
use linked_hash_set::LinkedHashSet;
use reqwest::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use shared::models::history::HistoryUpdate;

use crate::utils::app_error::AppError;

use crate::database::sessions::{Entity as Sessions, self};
use crate::database::user_history::{Entity as UsersHistory, self};

const ALLOWED_AMOUNT: usize = 6;

pub async fn user_history(
    State(database): State<DatabaseConnection>,
    Json(user_history): Json<HistoryUpdate>
) -> Result<(), AppError> {

    // 0. Find the user
    // 1. Find the history records
    // 2. Update the records

    let user_unique_id = user_history.unique_id;
    let user_item_id = user_history.item_id;

    if let Some(session) = Sessions::find()
        .filter(sessions::Column::UniqueId.eq(&user_unique_id))
        .one(&database)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))? {
            
            // Find the user
            if let Some(user_id) = session.user_id {
                // Try to find the user's history
                if let Some(history_record) = UsersHistory::find()
                    .filter(user_history::Column::UserId.eq(user_id))
                    .one(&database)
                    .await
                    .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))? {
                        // if something was found

                        // Divide into parts
                        let list = history_record.list.split_terminator(",")
                        .map(|elem| elem.to_owned())
                        .collect::<Vec<String>>();

                        // Remove all dublicates by LinkedHashSet 
                        // (linked orders pieces, instead of random order of just a HashSet)

                        let unique_strings: LinkedHashSet<String> = list.into_iter().collect();
                        let mut list: Vec<String> = unique_strings.into_iter().collect();

                        if list.len() >= ALLOWED_AMOUNT {
                            list.rotate_right(1);
                            list[0] = user_item_id;
                        } else {
                            list.insert(0, user_item_id);
                        }

                        let updated_list = list.join(",");

                        let mut active_history: user_history::ActiveModel = history_record.into_active_model();
                        active_history.list = Set(updated_list);
                        let _ = active_history.save(&database)
                            .await
                            .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?;
    
                    } else {
                        // The guest has no history, create a new one
                        let new_history = user_history::ActiveModel {
                            user_id: Set(user_id),
                            list: Set(user_item_id),
                            ..Default::default()
                        };
    
                        let _ = new_history.save(&database)
                            .await
                            .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?;
                    }
                
                return Ok(())
            }
        }

    
    Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))
}