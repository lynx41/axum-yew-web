use crate::{State, utils::cors::cors};

mod home;
mod profile;
mod users;
mod verify_token;
mod unique_session;
mod categories;

use home::home;
use profile::profile;
use tower_http::services::ServeDir;
use users::{login, logout, register};
use verify_token::verify_token;
use unique_session::{create_unique_session, validate_unique_session};
use categories::perfume::perfume;
use crate::utils::guard::guard;

use axum::{Router, routing::{get, post}, middleware};

pub async fn routes(state: State) -> Router {

    Router::new()

        // auth needed
        .route("/cabinet", get(profile))
        // auth needed utils
        .route("/verify_token", get(verify_token))
        .route_layer(middleware::from_fn_with_state(state.clone(), guard))

        // no auth needed
        .route("/perfume", get(perfume))

        // no auth needed utils
        .route("/create_unique_session", get(create_unique_session))
        .route("/validate_unique_session", post(validate_unique_session))
        .route("/register", post(register))
        .route("/login", post(login))
        
        // home
        .route("/", get(home))

        .with_state(state.clone())

        // place where images are located
        .nest_service("/public", ServeDir::new("../public"))

        .layer(cors())
}