use axum::{response::Response, TypedHeader, headers::{Authorization, authorization::Bearer}, middleware::Next, http::Request, extract::State};
use log::info;
use reqwest::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use tracing::debug;

use crate::database::guest::{Entity as Guests, self};

use crate::{
    database::sessions::{Entity as Sessions, self},
    utils::app_error::AppError,
    utils::jwt::is_valid};

pub async fn guard<B>(
    State(database): State<DatabaseConnection>,
    TypedHeader(header): TypedHeader<Authorization<Bearer>>,
    mut request: Request<B>,
    next: Next<B>
) -> Result<Response, AppError> {

    // Get token
    let token = header.token().to_owned();

    // Try to find a user with the token
    if let Some(session) = Sessions::find()
        .filter(sessions::Column::Token.eq(&token))
        .one(&database)
        .await
        .map_err(|_| AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error"))? {

                // there's some session
                if is_valid(&token).is_err() {
                    // token is bad, create a guest and bind this session to the guest insted of user
                    let new_guest = guest::ActiveModel {
                        unique_id: Set(session.unique_id.clone()),
                        ..Default::default()
                    };

                    let saved_guest = new_guest.save(&database)
                        .await
                        .map_err(|_| AppError::new(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Internal Server Error"))?;


                    let mut active_session = session.into_active_model();
                    active_session.user_id = Set(None);
                    active_session.guest_id = Set(Some(saved_guest.id.unwrap()));

                    let _ = active_session.save(&database)
                    .await
                    .map_err(|_| AppError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal Server Error"))?; 

                    return Err(AppError::new(
                            StatusCode::UNAUTHORIZED,
                            "You are not autorized."));
                } else {
                    // Keep this, so it cash if user doens't exist
                    request.extensions_mut().insert(session);

                    Ok(next.run(request).await)
                }
            } else {
                // no session was found
                return Err(AppError::new(
                    StatusCode::UNAUTHORIZED,
                    "You are not autorized."));
            }
}