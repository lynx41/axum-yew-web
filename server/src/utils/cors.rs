use std::str::FromStr;

use axum::http::HeaderName;
use reqwest::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, Method};
use tower_http::cors::{CorsLayer, Any};

pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE, HeaderName::from_str("UniqueID").unwrap()])
        .allow_origin(Any)
}