use axum::{Json, extract::State};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, IntoActiveModel, ActiveModelTrait, Set};

use crate::{
    database::users::{self, Model, Entity as UserTable},
    database::sessions::{self, Entity as SessionTable},
    utils::{app_error::AppError, jwt::{self, create_token}}
};

use axum::http::StatusCode;
use shared::models::user::User;

pub async fn login(
    State(database): State<DatabaseConnection>,
    Json(request): Json<User>
) -> Result<Json<String>, AppError> {

    // Check if database has the email
    let db_user = UserTable::find()
        .filter(users::Column::Email.eq(&request.email))
        .one(&database)
        .await
        .map_err(|_| AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR, 
            "Internal Server Error"))?;

    return if let Some(db_user) = db_user {
        // if password is wrong -> error
        if !verify_password(request.password, &db_user.password)? {
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error"));
        }
        
        // if user is real and password is correct
        let jwt = create_token()?;

        // Rewrite expired session or crete a new one
        let db_session = SessionTable::find()
            .filter(sessions::Column::UserId.eq(db_user.id))
            .filter(sessions::Column::UniqueId.eq(&request.unique_id))
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
                user_id: Set(db_user.id),
                token: Set(Some(jwt.clone())),
                unique_id: Set(request.unique_id),
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

        Ok(Json::from(format!("token: {}", jwt)))
    } else {
        Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
        "Internal Server Error"))
    }
}


fn hash_password(password: String) -> Result<String, AppError> {
    todo!()
}

fn verify_password(password: String, hash: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, hash)
        .map_err(|_| AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error"))
}