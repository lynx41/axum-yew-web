use std::rc::Rc;

use yew::{function_component, html, use_context, Html};
use yew_i18n::{use_translation, I18nProvider};

use crate::components::utils::client_context::ClientContext;

#[function_component(MainLinks)]
pub fn main_links() -> Html {

    let client_context = use_context::<Rc<ClientContext>>().unwrap();
    let mut i18n = use_translation();

    i18n.set_translation_language(&client_context.selected_language);
    
    html! {

        <div class="main-sidebar__block main-links">

            // block - about us
            <div class="side-links">
                <div class="side-links__item side-links__item--heading">
                    <h3 class="side-links__label">{ i18n.t("Information about the company") }</h3>
                </div>
                <ul class="side-links__list">
                    
                    <li class="side-links__item">
                        <a href="#About us">{ i18n.t("About us") }</a>
                    </li>
                
                    <li class="side-links__item">
                        <a href="#Terms and Conditions">{ i18n.t("Terms and Conditions") }</a>
                    </li> 
                    
                    <li class="side-links__item">
                        <a href="#Job opportunities">{ i18n.t("Job opportunities") }</a>
                    </li> 

                    <li class="side-links__item">
                        <a href="#Contacts">{ i18n.t("Contacts") }</a>
                    </li> 

                    <li class="side-links__item">
                        <a href="#All categories">{ i18n.t("All categories") }</a>
                    </li> 

                </ul>
            </div>

            // block - help
            <div class="side-links">
                <div class="side-links__item side-links__item--heading">
                    <h3 class="side-links__label">{ i18n.t("Help") }</h3>
                </div>
                <ul class="side-links__list">
                    
                <li class="side-links__item">
                    <a href="#Delivery and payment">{ i18n.t("Delivery and payment") }</a>
                </li>

                <li class="side-links__item">
                    <a href="#Credit">{ i18n.t("Credit") }</a>
                </li> 
                
                <li class="side-links__item">
                    <a href="#Warranty">{ i18n.t("Warranty") }</a>
                </li> 

                <li class="side-links__item">
                    <a href="#Return of goods">{ i18n.t("Return of goods") }</a>
                </li> 

                <li class="side-links__item">
                    <a href="#Service centers">{ i18n.t("Service centers") }</a>
                </li> 

                </ul>
            </div>

            // block - Services
            <div class="side-links">
                <div class="side-links__item side-links__item--heading">
                    <h3 class="side-links__label">{ i18n.t("Services") }</h3>
                </div>
                <ul class="side-links__list">
                    
                    <li class="side-links__item">
                        <a href="#Bonus account">{ i18n.t("Bonus account") }</a>
                    </li>

                    <li class="side-links__item">
                        <a href="#Gift certificates">{ i18n.t("Gift certificates") }</a>
                    </li>

                    <li class="side-links__item">
                        <a href="#Exchange">{ i18n.t("Exchange") }</a>
                    </li>

                    <li class="side-links__item">
                        <a href="#For corporate clients">{ i18n.t("For corporate clients") }</a>
                    </li>

                </ul>
            </div>

            // block - Partnership
            <div class="side-links">
                <div class="side-links__item side-links__item--heading">
                    <h3 class="side-links__label">{ i18n.t("Partners") }</h3>
                </div>
                <ul class="side-links__list">
                    
                    <li class="side-links__item">
                        <a href="#Sell on HSLV">{ i18n.t("Sell on HSLV") }</a>
                    </li>

                    <li class="side-links__item">
                        <a href="#Cooperation with us">{ i18n.t("Cooperation with us") }</a>
                    </li>

                    <li class="side-links__item">
                        <a href="#Franchising">{ i18n.t("Franchising") }</a>
                    </li>

                    <li class="side-links__item">
                        <a href="#Premises for rent">{ i18n.t("Premises for rent") }</a>
                    </li>

                </ul>
            </div>

        </div>
    }
}