use std::collections::HashMap;

use gloo::{console::log, storage::{LocalStorage, Storage}};
use serde_json::Value;
use web_sys::console::log;
use yew::{function_component, html, Html};

use yew_i18n::{use_translation, I18nProvider};

use crate::stores::language::LangSelector;


#[function_component(InnerBot)]
pub fn inner_bot() -> Html {
    
    // let mut i18n = use_translation();

    let selected_langauge = LocalStorage::get::<LangSelector>("Language")
        .unwrap();

    log!(format!("{}", selected_langauge));


    // Landing page (Configuration and provider)

    let supported_languages = vec!["ENG", "UA"];

    let mut translations = HashMap::new();

    translations.insert(
        "ENG".to_string(),
        serde_json::json!({
            "my_text_field": "Hello"
        })
    );

    translations.insert(
        "UA".to_string(),
        serde_json::json!({
            "my_text_field": "Привіт"
        })
    );


    // Inside component (hook to access the i18n context in my component)

    // let mut i18n = use_translation();

    // i18n.set_translation_language("ENG");


    html! {
        
        <div class="footer-top__links" place="footer">
            
            // Block 1 "About the company"
            <div class="footer-links">
                // Box Title
                <div class="footer-links__item footer-links__item--heading">
                    <h3 class="footer-links__label">{"Інформацію про компанію"}</h3>
                </div>
                
                // Box table
                <ul class="footer-links__list">
                    
                    // About us
                    <li class="footer-links__item">
                        <a href="#about_company">{"Про нас"}</a>
                    </li>

                    // Terms and Conditions of Use
                    <li class="footer-links__item">
                        <a href="#terms_and_conditions">{"Умови користування сайту"}</a>
                    </li> 
                    
                    <li class="footer-links__item">
                        <a href="#terms_and_conditions">{"Умови користування сайту"}</a>
                    </li> 

                </ul>

                // 
            </div>

            // Block 2 "Help"
            <div class="footer-links">
                // Box Title
                <div class="footer-links__item footer-links__item--heading">
                    <h3 class="footer-links__label">{"Інформацію про компанію"}</h3>
                </div>
                
                // Box table
                <ul class="footer-links__list">
                    
                    // About us
                    <li class="footer-links__item">
                        <a href="#about_company">{"Про нас"}</a>
                    </li>

                    // Terms and Conditions of Use
                    <li class="footer-links__item">
                        <a href="#terms_and_conditions">{"Умови користування сайту"}</a>
                    </li> 
                    
                </ul>

                // 
            </div>

            // Block 3 "Services"
            <div class="footer-links">
                // Box Title
                <div class="footer-links__item footer-links__item--heading">
                    <h3 class="footer-links__label">{"Інформацію про компанію"}</h3>
                </div>
                
                // Box table
                <ul class="footer-links__list">
                    
                    // About us
                    <li class="footer-links__item">
                        <a href="#about_company">{"Про нас"}</a>
                    </li>

                    // Terms and Conditions of Use
                    <li class="footer-links__item">
                        <a href="#terms_and_conditions">{"Умови користування сайту"}</a>
                    </li> 
                    
                </ul>

                // 
            </div>

            // Block 4 "Partnership"
            <div class="footer-links">
                // Box Title
                <div class="footer-links__item footer-links__item--heading">
                    <h3 class="footer-links__label">{"Інформацію про компанію"}</h3>
                </div>
                
                // Box table
                <ul class="footer-links__list">
                    
                    // About us
                    <li class="footer-links__item">
                        <a href="#about_company">{"Про нас"}</a>
                    </li>

                    // Terms and Conditions of Use
                    <li class="footer-links__item">
                        <a href="#terms_and_conditions">{"Умови користування сайту"}</a>
                    </li> 
                    
                    <li class="footer-links__item">
                        <a href="#test_one">
                            
                            {"Hi"}
                            // { i18n.t("my_text_field") }
                        </a>

                        <I18nProvider supported_languages={supported_languages} translations={translations}>
                            <MyComponent />
                        </I18nProvider>
                    </li>

                </ul>

                // 
            </div>
        
        </div>
    }
}


#[function_component(MyComponent)]
pub fn my_component() -> Html {
    let mut i18n = use_translation();

    // This shit will be sended from the root component
    let selected_langauge = LocalStorage::get::<LangSelector>("Language")
        .unwrap()
        .to_string();

    i18n.set_translation_language(&selected_langauge);

    html! {
        <a href="#test_two">{ i18n.t("my_text_field") }</a>
    }
}