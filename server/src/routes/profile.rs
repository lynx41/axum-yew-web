use axum::Json;

pub async fn profile() -> Json<String> {
    Json::from("Welcome to your profile".to_owned())
}