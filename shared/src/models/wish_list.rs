use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WishListRequest {
    pub token: String,
    pub item_id: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WishListStatus {
    NotWished,
    Wished,
    Unknown
}