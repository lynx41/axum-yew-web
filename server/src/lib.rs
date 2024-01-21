mod routes;
mod utils;
mod database;

use axum_server::tls_rustls::RustlsConfig;
use routes::routes;

use std::net::SocketAddr;

use clap::Parser;

use axum::{ServiceExt, extract::FromRef};
use sea_orm::{Database, DatabaseConnection};
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;

#[derive(Clone, FromRef)]
pub struct State {
    pub main: DatabaseConnection,
}

// Setup CLI setup with clap

#[derive(Parser, Debug, Clone)]
pub struct Opt {
    // LOG LEVEL
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,
    
    // ADDR
    #[clap(short = 'a', long = "addr", default_value = "0.0.0.0")]
    addr: String,
    
    // PORT
    #[clap(short = 'p', long = "port", default_value = "5000")]
    port: String,

    // Where frontend files are located (./dist)
    #[clap(long = "static-dir", default_value = "./dist")]
    static_dir: String,

    // CERT.PEM
    #[clap(long = "cert-path", default_value = "../cert.pem")]
    cert_path: String,

    // KEY.PEM
    #[clap(long = "key-path", default_value = "../key.pem")]
    key_path: String,
}

pub async fn run(database_url: &str) {
    
    let opt = Opt::parse();

    // console logging : On
    tracing_subscriber::fmt::init();

    let main_database = Database::connect(database_url).await.unwrap();
    let state = State { main: main_database };

    let sock_addr: SocketAddr = format!("{}:{}",
        opt.addr, opt.port).parse().unwrap();

    // TLS
    let config = RustlsConfig::from_pem_file(
        opt.cert_path,
        opt.key_path,
    )
        .await
        .expect("Failed to load Cert and Key pem files");

    // NormalizePathLayer allowes `...\user` and `...\user\` be the same path
    let app = NormalizePathLayer::trim_trailing_slash()
        .layer(routes(state).await);

    log::info!("LISTENING on https://{}", &sock_addr);

    // TLS support
    axum_server::bind_rustls(sock_addr, config)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start the server");
}