use crate::{State, utils::cors::cors};

mod home;
mod profile;
mod users;

use home::home;
use profile::profile;
use tower_http::services::ServeDir;
use users::login;
use crate::utils::guard::guard;

use axum::{Router, routing::{get, post}, middleware};

pub async fn routes(state: State) -> Router {

    Router::new()

        // auth needed
        .route("/profile", get(profile))
        .route_layer(middleware::from_fn_with_state(state.clone(), guard))

        // no auth needed
        .route("/login", post(login))
        .route("/", get(home))

        .with_state(state.clone())

        .nest_service("/public", ServeDir::new("../public"))
        .layer(cors())
}