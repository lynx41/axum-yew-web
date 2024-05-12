use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct PerfumeQuery {
    pub brand: Option<String>,
    pub seasonality: Option<String>,
    pub volume: Option<String>,
    pub class: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PerfumeTile {
    // image url
    pub tile_picture_src: String,
    // the goods id
    pub product_page_src: i32,
    pub old_price: Option<i32>,
    pub price: i32,
    pub title: String,
}

impl PerfumeTile {
    pub fn from(
        tile_picture_src: String,
        product_page_src: i32,
        old_price: Option<i32>,
        price: i32,
        title: String
    ) -> Self {
        Self {
            tile_picture_src,
            product_page_src,
            old_price,
            price,
            title
        }
    }
}