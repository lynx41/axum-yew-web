use std::collections::HashMap;

use serde_json::Value;

use crate::stores::language::{LANG_ENG, LANG_UA};

pub fn categories_loc() -> HashMap<String, Value> {

    let graphical_tables_link = "#GraphicalTablets";
    let parfumery_link = "#Parfumery";

    HashMap::from([
        (
            LANG_ENG.to_owned(),
            serde_json::json!({
                "Graphical Tables": "Graphical tablets",
                "Parfumery": "Parfumery",
                "Graphical Tables link": graphical_tables_link,
                "Parfumery link": parfumery_link
            })
        ),
        (
            LANG_UA.to_owned(),
            serde_json::json!({
                "Graphical Tables": "Графічні планшети",
                "Parfumery": "Парфумерія",
                "Graphical Tables link": graphical_tables_link,
                "Parfumery link": parfumery_link
            })
        )
    ])
}

pub fn help_centre_loc() -> HashMap<String, Value> {

    HashMap::from([
        (
            LANG_ENG.to_owned(),
            serde_json::json!({
                "title": "Help Centre"
            })
        ),
        (
            LANG_UA.to_owned(),
            serde_json::json!({
                "title": "Центр допомоги"
            })
        )
    ])
}

pub fn main_auth_loc() -> HashMap<String, Value> {

    HashMap::from([
        (
            LANG_ENG.to_owned(),
            serde_json::json!({
                "heading": "Welcome!",
                "caption": "Sign in to receive personalized bonuses and discounts.",
                "button": "Continue"
            })
        ),
        (
            LANG_UA.to_owned(),
            serde_json::json!({
                "heading": "Ласкаво просимо!",
                "caption": "Увійдіть, щоб отримувати персональні бонуси та знижки",
                "button": "Продовжити"
            })
        )
    ])
}