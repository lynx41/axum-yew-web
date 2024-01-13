use crate::State;

mod home;

use home::home;

use axum::{Router, routing::get};

pub async fn routes(state: State) -> Router {

    Router::new()
        .route("/", get(home))

        .with_state(state.clone())
}