use reqwest::Method;
use tower_http::cors::{CorsLayer, Any};

pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_headers(Any)
        .allow_origin(Any)
}