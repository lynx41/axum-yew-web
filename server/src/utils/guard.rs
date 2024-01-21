use axum::{response::Response, TypedHeader, headers::{Authorization, authorization::Bearer}, middleware::Next, http::Request, extract::State};
use reqwest::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};

use crate::{
    database::users::{Entity as Users, self},
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
    let (_, token) = token.split_once(' ').unwrap();
    let token = token.to_owned();

    log::debug!("{}", token);

    // Try to find a user with the token
    let user = Users::find()
        .filter(users::Column::Token.eq(&token))
        .one(&database)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"))?;

    // Validating token after getting from the adtabase to obsfucate that the token is wrong
    is_valid(&token)?;

    // Check for result
    let Some(user) = user else {
        return Err(AppError::new(StatusCode::UNAUTHORIZED, "You are not autorized."))
    };

    // Keep this, so it cash if user doens't exist
    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}