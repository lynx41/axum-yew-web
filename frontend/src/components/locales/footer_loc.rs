use std::collections::HashMap;

use serde_json::Value;

use crate::stores::language::LangSelector;

pub fn inner_top_loc() -> HashMap<String, Value> {

    HashMap::from([
        (
            LangSelector::ENG.to_string(),
            serde_json::json!({
                "title": "Call center working hours"
            })
        ),
        (
            LangSelector::UA.to_string(),
            serde_json::json!({
                "title": "Графік роботи Call-центра"
            })
        )
    ])
}

pub fn inner_bot_loc() -> HashMap<String, Value> {
    HashMap::from([

    ])
}

pub fn mobile_apps_ad() -> HashMap<String, Value> {
    HashMap::from([
        (
            LangSelector::UA.to_string(),
            serde_json::json!({
            "Download our apps": "Завантажуй наші застосунки",
            "GooglePlayIconUrl": "https://localhost:5000/public/system/images/locales/footer/google-play-badge-ua.svg",
            "AppStoreIconUrl": "https://localhost:5000/public/system/images/locales/footer/app-store-badge-ua.svg",
            "GooglePlayAriaLabel": "Застосунок для Android",
            "AppStoreAriaLabel": "Застосунок для IOS",
            })
        ),
        (
            LangSelector::ENG.to_string(),
            serde_json::json!({
                "Download our apps": "Download our apps",
                "GooglePlayIconUrl": "https://localhost:5000/public/system/images/locales/footer/google-play-badge-en.svg",
                "AppStoreIconUrl": "https://localhost:5000/public/system/images/locales/footer/app-store-badge-en.svg",
                "GooglePlayAriaLabel": "Android application",
                "AppStoreAriaLabel": "IOS application"
            })
        )
    ])
}