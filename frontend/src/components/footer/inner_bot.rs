use std::collections::HashMap;

use gloo::{console::log, storage::{LocalStorage, Storage}};
use serde_json::Value;
use web_sys::console::log;
use yew::{function_component, html, Html};

use yew_i18n::{use_translation, I18nProvider};

use crate::{routes::Prop, stores::language::LangSelector};


#[function_component(InnerBot)]
pub fn inner_bot(prop: &Prop) -> Html {

    let mut i18n = use_translation();

    i18n.set_translation_language(&prop.selected_language);

    html! {
        
        <div class="footer-top__links" place="footer">
            
            // Block 1 "About the company"
            <div class="footer-links">
                // Box Title
                <div class="footer-links__item footer-links__item--heading">
                    <h3 class="footer-links__label">{ i18n.t("Information about the company") }</h3>
                </div>
                
                // Box table
                <ul class="footer-links__list">
                    
                    <li class="footer-links__item">
                        <a href="#About us">{ i18n.t("About us") }</a>
                    </li>

                    <li class="footer-links__item">
                        <a href="#Terms and Conditions">{ i18n.t("Terms and Conditions") }</a>
                    </li> 
                    
                    <li class="footer-links__item">
                        <a href="#Job opportunities">{ i18n.t("Job opportunities") }</a>
                    </li> 

                    <li class="footer-links__item">
                        <a href="#Contacts">{ i18n.t("Contacts") }</a>
                    </li> 

                    <li class="footer-links__item">
                        <a href="#All categories">{ i18n.t("All categories") }</a>
                    </li> 

                </ul>

            </div>

            // Block 2 "Help"
            <div class="footer-links">
                // Box Title
                <div class="footer-links__item footer-links__item--heading">
                    <h3 class="footer-links__label">{ i18n.t("Help") }</h3>
                </div>
                
                // Box table
                <ul class="footer-links__list">
                    
                    <li class="footer-links__item">
                        <a href="#Delivery and payment">{ i18n.t("Delivery and payment") }</a>
                    </li>

                    <li class="footer-links__item">
                        <a href="#Credit">{ i18n.t("Credit") }</a>
                    </li> 
                    
                    <li class="footer-links__item">
                        <a href="#Warranty">{ i18n.t("Warranty") }</a>
                    </li> 

                    <li class="footer-links__item">
                        <a href="#Return of goods">{ i18n.t("Return of goods") }</a>
                    </li> 

                    <li class="footer-links__item">
                        <a href="#Service centers">{ i18n.t("Service centers") }</a>
                    </li> 

                </ul>

                // 
            </div>

            // Block 3 "Services"
            <div class="footer-links">
                // Box Title
                <div class="footer-links__item footer-links__item--heading">
                    <h3 class="footer-links__label">{ i18n.t("Services") }</h3>
                </div>
                
                // Box table
                <ul class="footer-links__list">

                    <li class="footer-links__item">
                        <a href="#Bonus account">{ i18n.t("Bonus account") }</a>
                    </li>

                    <li class="footer-links__item">
                        <a href="#Gift certificates">{ i18n.t("Gift certificates") }</a>
                    </li>

                    <li class="footer-links__item">
                        <a href="#Exchange">{ i18n.t("Exchange") }</a>
                    </li>

                    <li class="footer-links__item">
                        <a href="#For corporate clients">{ i18n.t("For corporate clients") }</a>
                    </li>

                </ul>

            </div>

            // Block 4 "Partnership"
            <div class="footer-links">
                // Box Title
                <div class="footer-links__item footer-links__item--heading">
                    <h3 class="footer-links__label">{ i18n.t("Partners") }</h3>
                </div>
                
                // Box table
                <ul class="footer-links__list">
                    
                    <li class="footer-links__item">
                        <a href="#Sell on Biloba">{ i18n.t("Sell on Biloba") }</a>
                    </li>

                    <li class="footer-links__item">
                        <a href="#Cooperation with us">{ i18n.t("Cooperation with us") }</a>
                    </li>

                    <li class="footer-links__item">
                        <a href="#Franchising">{ i18n.t("Franchising") }</a>
                    </li>

                    <li class="footer-links__item">
                        <a href="#Premises for rent">{ i18n.t("Premises for rent") }</a>
                    </li>
                    
                </ul>

            </div>
        
        </div>
    }
}