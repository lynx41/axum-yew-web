use axum::Json;

pub async fn home() -> Json<String> {
    Json::from("Welcome".to_owned())
}