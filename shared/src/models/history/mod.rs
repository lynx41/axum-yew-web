use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HistoryUpdate {
    pub unique_id: String,
    pub item_id: String
}