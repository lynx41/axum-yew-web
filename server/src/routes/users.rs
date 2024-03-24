use axum::{Json, extract::State};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, IntoActiveModel, ActiveModelTrait, Set};

use crate::{
    database::users::{self, Model, Entity as UserTable},
    database::sessions::{self, Model, Entity as SessionTable},
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
        let mut user = db_user.into_active_model();

        let jwt = create_token()?;


        user.token = Set(Some(jwt.clone()));

        let saved_user = user.save(&database)
            .await
            .map_err(|_| AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error"
            ))?;
        
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