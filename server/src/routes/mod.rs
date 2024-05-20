use crate::{State, utils::cors::cors};

mod home;
mod profile;
mod users;
mod verify_token;
mod unique_session;
mod categories;
mod history;
mod wish_list;

use home::home;
use profile::profile;
use tower_http::services::ServeDir;
use users::{login, logout, register};
use verify_token::verify_token;
use unique_session::{create_unique_session, validate_unique_session};
use categories::perfume::perfume;
use categories::recomendations::perfume_suggestions;
use wish_list::{wish_list_add, wish_list_check, wish_list_delete};
use history::{guest::guest_history, user::user_history};
use crate::utils::guard::guard;

use axum::{middleware, routing::{delete, get, post}, Router};

use self::categories::perfume_id::perfume_id;

pub async fn routes(state: State) -> Router {

    Router::new()

        // auth needed
        .route("/cabinet", get(profile))
        // auth needed utils
        .route("/wish_list_check", post(wish_list_check))
        .route("/wish_list_add", post(wish_list_add))
        .route("/wish_list_remove", delete(wish_list_delete))
        
        .route("/user_save_history", post(user_history))
        .route("/verify_token", get(verify_token))
        .route_layer(middleware::from_fn_with_state(state.clone(), guard))

        // no auth needed
        .route("/get_suggestions", get(perfume_suggestions))
        .route("/perfume/:id", get(perfume_id))
        .route("/perfume", get(perfume))

        // no auth needed utils
        .route("/guest_save_history", post(guest_history))
        .route("/create_unique_session", get(create_unique_session))
        .route("/validate_unique_session", post(validate_unique_session))
        .route("/logout", get(logout))
        .route("/register", post(register))
        .route("/login", post(login))
        
        // home
        .route("/", get(home))

        .with_state(state.clone())

        // place where images are located
        .nest_service("/public", ServeDir::new("../public"))

        .layer(cors())
}