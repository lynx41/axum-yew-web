use axum::{Json, extract::State};
use chrono::{DateTime, Datelike, FixedOffset, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use tower_http::follow_redirect::policy::PolicyExt;
use tracing::field::debug;

use crate::{
    database::users::{self, Model, Entity as UserTable},
    database::sessions::{self, Entity as Sessions},
    database::guest::{self, Entity as Guests},
    database::user_logs,
    database::user_roles,
    utils::{app_error::AppError, jwt::{self, create_token}}
};

use axum::http::StatusCode;
use shared::models::user::User;

pub async fn login(
    State(database): State<DatabaseConnection>,
    Json(request_user): Json<User>
) -> Result<Json<String>, AppError> {

/*
    We have an account, we only need to generate token and update last_login inside the user_logs.
    1. Validate request data
    2. update the user_info table
    3. make a session and submit the token as a response
*/

    // Check if database has the email
    let db_user = UserTable::find()
        .filter(users::Column::Email.eq(&request_user.email))
        .one(&database)
        .await
        .map_err(|_| AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR, 
            "Internal Server Error"))?;
    
    // if we have such email in the database
    return if let Some(db_user) = db_user {
        
        // validate the password
        if !verify_password(request_user.password, &db_user.password)? {            
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error"));
        }
        
        // if user is real and password is correct
        let jwt = create_token()?;

        // Rewrite expired session or crete a new one

        /*
            We are trying to find some existed session with the request unique_id so we can update the session.
            But if that's the first login on the browser we just create a new one, and in either way we
            add to the session the user_id. And we update the last_login in user_logs
        */

        let db_session = Sessions::find()
            .filter(sessions::Column::UniqueId.eq(&request_user.unique_id))
            .one(&database)
            .await
            .map_err(|_| AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error"))?;

        if let Some(db_session) = db_session {
            let mut session_update = db_session.into_active_model();
            
            session_update.token = Set(Some(jwt.clone()));
            session_update.user_id = Set(Some(db_user.id));
            session_update.guest_id = Set(None);

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
                unique_id: Set(request_user.unique_id.clone()),
                ..Default::default()
            };

            let _ = new_session.save(&database)
                .await
                .map_err(|_| AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error"
                ))?;
        }

        // Remove the Guest record, because we made an user record from guest
        if let Some(guest_record) = Guests::find()
        .filter(guest::Column::UniqueId.eq(&request_user.unique_id))
        .one(&database)
        .await
        .map_err(|_| AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error"))? {
                let delete_result = guest_record.into_active_model()
                    .delete(&database)
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
    
    /*
        We wamt to create a bran new account.
        0. After frontend the request data is normilizid
        1. Validate the request data 
        2. Crete an user
        3. Create a new record in the user_logs
        4. Create a new record in the user_roles
        5. make a session and sumbit the token as a response
    */

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

        // Remove the Guest record, because we made an user record from guest
        if let Some(guest_record) = Guests::find()
        .filter(guest::Column::UniqueId.eq(&req_user.unique_id))
        .one(&database)
        .await
        .map_err(|_| AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error"))? {
                let delete_result = guest_record.into_active_model()
                    .delete(&database)
                    .await
                    .map_err(|_| AppError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal Server Error"
                    ))?;
            }
        
        // if user is real and password is correct
        let jwt = create_token()?;
        log::debug!("NO SUCH EMAIL WAS FOUND");
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

        log::debug!("NEW USER WAS SAVED TO THE DATABASE");

        let new_user_id = new_user_result.id.unwrap();

        // Create a record in user_logs
        let utc = Utc::now();
        let dtwtz = FixedOffset::east(0)
            .with_ymd_and_hms(utc.year(), utc.month(), utc.day(), utc.hour(), utc.minute(), utc.second())
            .unwrap();

        let new_user_logs = user_logs::ActiveModel {
            user_id: Set(new_user_id),
            created_at: Set(dtwtz),
            last_login: Set(dtwtz),
            modified_at: Set(dtwtz),
            ..Default::default()
        };

        let new_user_logs_result = new_user_logs.save(&database).await
            .map_err(|_| AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error"))?;

        log::debug!("NEW USER_LOGS WAS SAVED TO THE DATABASE");

        // Create a record in user_roles
        let new_user_roles = user_roles::ActiveModel {
            // role_id 1 mean => customer
            role_id: Set(1),
            user_id: Set(new_user_id),
            ..Default::default()
        };

        let new_user_roles_result = new_user_roles.save(&database).await
            .map_err(|_| AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error"))?;

        log::debug!("NEW USER_ROLES WAS SAVED TO THE DATABASE");


        // Create the session
        let new_session = sessions::ActiveModel {
            token: Set(Some(jwt.clone())),
            unique_id: Set(req_user.unique_id),
            user_id: Set(Some(new_user_id)),
            ..Default::default()
        };

        new_session.save(&database).await
            .map_err(|_| AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error"))?;

        log::debug!("NEW SESSION WAS SAVED TO THE DATABASE");

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