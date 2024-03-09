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
        (
            LangSelector::ENG.to_string(),
            serde_json::json!({
                "Information about the company": "Information about the company",
                "About us": "About us",
                "Terms and Conditions": "Terms and Conditions",
                "Job opportunities": "Job opportunities",
                "Contacts": "Contacts",
                "All categories": "All categories",
                "Help": "Help",
                "Delivery and payment": "Delivery and payment",
                "Credit": "Credit",
                "Warranty": "Warranty",
                "Return of goods": "Return of goods",
                "Service centers": "Service centers",
                "Services": "Services",
                "Bonus account": "Bonus account",
                "Gift certificates": "Gift certificates",
                "Exchange": "Exchange",
                "For corporate clients": "For corporate clients",
                "Partners": "Partners",
                "Sell on Biloba": "Sell on Biloba",
                "Cooperation with us": "Cooperation with us",
                "Franchising": "Franchising",
                "Premises for rent": "Premises for rent",
            })
        ),
        (
            LangSelector::UA.to_string(),
            serde_json::json!({
                "Information about the company": "Інформація про компанію",
                "About us": "Про нас",
                "Terms and Conditions": "Умови використання сайту",
                "Job opportunities": "Вакансії",
                "Contacts": "Контакти",
                "All categories": "Усі категорії",
                "Help": "Допомога",
                "Delivery and payment": "Доставка та оплата",
                "Credit": "Кредит",
                "Warranty": "Гарантія",
                "Return of goods": "Повернення товару",
                "Service centers": "Сервісні центри",
                "Services": "Сервіси",
                "Bonus account": "Бонусний рахунок",
                "Gift certificates": "Подарункові сертифікати",
                "Exchange": "Обмін",
                "For corporate clients": "Корпоративним клієнтам",
                "Partners": "Партнерам",
                "Sell on Biloba": "Продавати на Білобі",
                "Cooperation with us": "Співпраця з нами",
                "Franchising": "Франчайзинг",
                "Premises for rent": "Оренда приміщень",
            })
        )
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