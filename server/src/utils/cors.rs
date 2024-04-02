use reqwest::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, Method};
use tower_http::cors::{CorsLayer, Any};

pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_origin(Any)
}