use axum::http::HeaderMap;
use axum::{extract::State, Json};
use log::info;
use reqwest::StatusCode;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::database::sessions::{Entity as Sessions, self};
use crate::database::guest::{Entity as Guests, self};

use crate::utils::app_error::AppError;
use crate::utils::jwt::create_token;


pub async fn create_unique_session(
    State(database): State<DatabaseConnection>,
) -> Result<Json<String>, AppError> {

    // We don't need to check if the database already contains the unique_id,
    // because table "Sessions" has column unique_id with option `unique key` wich makes the job for us,
    // we just need to try to save our record to the database and check the result. 

    info!("HI");

    // Generate Unique Id
    let unique_session = create_token()?;

    let new_guest = guest::ActiveModel {
        unique_id: Set(unique_session),
        ..Default::default()  
    };

    // Making a new guest
    let saved_guest = new_guest.save(&database)
        .await
        .map_err(|_| AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error"
        ))?;

    // Creating the new session
    let new_session = sessions::ActiveModel {
        guest_id: Set(Some(saved_guest.id.unwrap())),
        unique_id: Set(saved_guest.unique_id.unwrap()),
        ..Default::default()
    };

    let saved_session = new_session.save(&database)
        .await
        .map_err(|_| AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error"
        ))?;

    Ok(Json(saved_session.unique_id.unwrap()))
}


pub async fn validate_unique_session(
    State(database): State<DatabaseConnection>,
    header: HeaderMap
) -> Json<String> {

    let given_session_id = header.get("UniqueID");

    // We need to verify if the guest's unique_id was given by this server
    if given_session_id.is_some()  {
        let session_id_st = given_session_id.unwrap().to_str().unwrap().to_owned();

        if let Ok(session) = Sessions::find()
        .filter(sessions::Column::UniqueId.eq(&session_id_st))
        .one(&database)
        .await {
            if session.is_some() { return Json(session_id_st) }
        }
    };

    match create_unique_session(State(database.clone())).await {
        Ok(new_session) => return new_session,
        Err(_) => { return Json(given_session_id.unwrap().to_str().unwrap().to_owned()) }
    }

}