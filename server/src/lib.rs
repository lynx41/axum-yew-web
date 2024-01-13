mod routes;

use routes::routes;

use std::net::SocketAddr;

use axum::{ServiceExt, extract::FromRef};
use dotenvy_macro::dotenv;
use sea_orm::{Database, DatabaseConnection};
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;

#[derive(Clone, FromRef)]
pub struct State {
    pub main: DatabaseConnection,
}


pub async fn run(database_url: &str) {
    
    // enable console logging
    tracing_subscriber::fmt::init();

    let main_database = Database::connect(database_url).await.unwrap();
    let state = State { main: main_database };

    let sock_addr: SocketAddr = format!("{}:{}",
        dotenv!("SERVER_ADDR"), dotenv!("SERVER_PORT")).parse().unwrap();

    // NormalizePathLayer allowes `...\user` and `...\user\` be the same path
    let app = NormalizePathLayer::trim_trailing_slash()
        .layer(routes(state).await);

    log::info!("LISTENING on http://{}", &sock_addr);

    axum::Server::bind(&sock_addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start the server, please check the config");
}