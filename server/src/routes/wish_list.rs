use axum::{extract::State, Json};
use reqwest::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use shared::models::wish_list::WishListRequest;

use crate::utils::app_error::AppError;

use crate::database::sessions::{Entity as Sessions, self};
use crate::database::wish_list::{Entity as WishList, self};

// POST
pub async fn wish_list_check(
    State(database): State<DatabaseConnection>,
    Json(wish_list_request): Json<WishListRequest>
) -> Result<Json<bool>, AppError> {

    // 1. Find a session by the token, and get the user_id from it
    // 2. Check if such user has the item wishlisted

    if let Some(session) = Sessions::find()
        .filter(sessions::Column::Token.eq(&wish_list_request.token))
        .one(&database)
        .await
        .map_err(|_| AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error"))? {
                // If the session is used by user
                if let Some(user_id) = session.user_id {
                    // If user hasn't wishlisted the item
                    if let None = WishList::find()
                        .filter(wish_list::Column::UserId.eq(user_id))
                        .filter(wish_list::Column::ItemId.eq(wish_list_request.item_id))
                        .one(&database)
                        .await
                        .map_err(|_| AppError::new(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Internal Server Error"))? {
                                return Ok(Json(false));
                            } else {
                                return Ok(Json(true));
                            }
                } else {
                    return Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"))
                }
        }
        

    Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"))
}

// POST
pub async fn wish_list_add(
    State(database): State<DatabaseConnection>,
    Json(wish_list_request): Json<WishListRequest>
) -> Result<(), AppError>{

    // 1. Find a session by the token, and get the user_id from it
    // 2. Write a new record in wish_list

    if let Some(session) = Sessions::find()
        .filter(sessions::Column::Token.eq(&wish_list_request.token))
        .one(&database)
        .await
        .map_err(|_|
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error"))? {
                if let Some(user_id) = session.user_id {
                    // check if there's no such record in the db
                    if let None = WishList::find()
                        .filter(wish_list::Column::UserId.eq(user_id))
                        .filter(wish_list::Column::ItemId.eq(wish_list_request.item_id))
                        .one(&database)
                        .await
                        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR,"Internal Server Error"))? {
                            let new_wish_list_record = wish_list::ActiveModel {
                                user_id: Set(user_id),
                                item_id: Set(wish_list_request.item_id),
                                ..Default::default()
                            };
                            let _ = new_wish_list_record.save(&database)
                                .await
                                .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"))?;

                            return Ok(());
                        }
                }
        }

    Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"))
}

// DELETE
pub async fn wish_list_delete() {

}