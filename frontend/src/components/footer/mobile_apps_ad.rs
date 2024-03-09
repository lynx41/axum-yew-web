use yew::{function_component, html, Html};
use yew_i18n::use_translation;

use crate::routes::Prop;

#[function_component(MobileAppsAd)]
pub fn mobile_apps_ad(prop: &Prop) -> Html {
    
    let mut i18n = use_translation();

    i18n.set_translation_language(&prop.selected_language);

    html! {
        <section class="footer-section">
            <div class="layout">
                <div class="footer-stores">
                    <h3 class="footer-stores__heading">
                        { i18n.t("Download our apps") }
                    </h3>

                    <ul class="footer-stores__buttons">
                        
                        <li class="footer-stores__item">
                            <a class="footer-stores__button" href="#play.google.com/store" aria-label={ i18n.t("GooglePlayAriaLabel") }>
                                <img loading="lazy" alt="Google Play" src={ i18n.t("GooglePlayIconUrl") } />
                            </a>
                        </li>

                        <li class="footer-stores__item">
                            <a class="footer-stores__button" href="#app-store/" aria-label={ i18n.t("AppStoreAriaLabel") }>
                                <img loading="lazy" alt="AppStore" src={ i18n.t("AppStoreIconUrl") } />
                            </a>
                        </li>

                    </ul>
                </div>
            </div>
        </section>
    }
}