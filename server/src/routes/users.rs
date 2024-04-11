use axum::{Json, extract::State};
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde::{Deserialize, Serialize};

use crate::{
    database::users::{self, Model, Entity as UserTable},
    database::sessions::{self, Entity as Sessions},
    utils::{app_error::AppError, jwt::{self, create_token}}
};

use axum::http::StatusCode;
use shared::models::user::User;

pub async fn login(
    State(database): State<DatabaseConnection>,
    Json(request_user): Json<User>
) -> Result<Json<String>, AppError> {

    // Check if database has the email
    let db_user = UserTable::find()
        .filter(users::Column::Email.eq(&request_user.email))
        .one(&database)
        .await
        .map_err(|_| AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR, 
            "Internal Server Error"))?;

    return if let Some(db_user) = db_user {
        // if password is wrong -> error
        if !verify_password(request_user.password, &db_user.password)? {            
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error"));
        }
        
        // if user is real and password is correct
        let jwt = create_token()?;

        // Rewrite expired session or crete a new one
        let db_session = Sessions::find()
            .filter(sessions::Column::UserId.eq(db_user.id))
            .filter(sessions::Column::UniqueId.eq(&request_user.unique_id))
            .one(&database)
            .await
            .map_err(|_| AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error"))?;

        if let Some(db_session) = db_session {
            let mut session_update = db_session.into_active_model();
            
            session_update.token = Set(Some(jwt.clone()));

            let _ = session_update.save(&database)
                .await
                .map_err(|_| AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error"
                ))?;
        } else {
            let new_session = sessions::ActiveModel {
                user_id: Set(Some(db_user.id)),
                token: Set(Some(jwt.clone())),
                unique_id: Set(request_user.unique_id),
                ..Default::default()
            };

            // Try to save
            let _ = new_session.save(&database)
                .await
                .map_err(|_| AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error"
                ))?;
        }

        Ok(Json::from(jwt))
    } else {
        Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
        "Internal Server Error"))
    }
}

pub async fn register(
    State(database): State<DatabaseConnection>,
    Json(req_user): Json<User>
) -> Result<Json<String>, AppError> {
    
    // Check if the database has the email
    let db_user = UserTable::find()
        .filter(users::Column::Email.eq(&req_user.email))
        .one(&database)
        .await
        .map_err(|_| AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error"))?;
    
    if db_user.is_some() {
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
             "Internal Server Error"));
    } else {
        // if user is real and password is correct
        let jwt = create_token()?;

        // Create the user
        let new_user = users::ActiveModel {
            email: Set(req_user.email),
            password: Set(hash_password(req_user.password)?),
            ..Default::default()
        };

        let new_user_result = new_user.save(&database).await
            .map_err(|_| AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error"))?;

        // Create the session
        let new_session = sessions::ActiveModel {
            token: Set(Some(jwt.clone())),
            unique_id: Set(req_user.unique_id),
            user_id: Set(Some(new_user_result.id.unwrap())),
            ..Default::default()
        };

        new_session.save(&database).await
            .map_err(|_| AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error"))?;

        Ok(Json(jwt))
    }

}



#[derive(Deserialize)]
pub struct RequestUser {
    token: String,
    unique_id: String,
}

pub async fn logout(
    State(database): State<DatabaseConnection>,
    Json(requested_user): Json<RequestUser>
) -> Result<(), StatusCode> {
    
    if let Ok(session) = Sessions::find()
        .filter(sessions::Column::Token.eq(&requested_user.token))
        .filter(sessions::Column::UniqueId.eq(&requested_user.unique_id))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR) {
            if let Some(s) = session {
                let result = s.into_active_model().delete(&database).await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

                if result.is_ok() { return Ok(()) }
                else { Err(StatusCode::INTERNAL_SERVER_ERROR) }

            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        } else {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
}

fn hash_password(password: String) -> Result<String, AppError> {
    // 31 is too long, 8 is not enough
    bcrypt::hash(password, 14)
        .map_err(|_| AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error"))
}

fn verify_password(password: String, hash: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, hash)
        .map_err(|_| AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error"))
}