use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub password: String,
    pub unique_id: String,
}

impl User {
    pub fn from(
        email: impl Into<String>,
        password: impl Into<String>,
        unique_id: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            password: password.into(),
            unique_id: unique_id.into(),
        }
    }
}