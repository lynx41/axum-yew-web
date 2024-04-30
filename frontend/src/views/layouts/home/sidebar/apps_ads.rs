use std::rc::Rc;

use yew::{function_component, html, use_context, Html};
use yew_i18n::use_translation;

use crate::components::utils::client_context::ClientContext;




#[function_component(AppsAds)]
pub fn apps_ads() -> Html {
    
    let client_context = use_context::<Rc<ClientContext>>().unwrap();
    let mut i18n = use_translation();

    i18n.set_translation_language(&client_context.selected_language);
    
    html! {
        <div class="main-stores">
            <h4 class="main-sidebar__heading">{ i18n.t("Download our apps") }</h4>
            
            <a class="main-stores__button" title={ i18n.t("AppStoreAriaLabel") } href="#app-store/">
                <img loading="lazy" alt="AppStore" src={ i18n.t("AppStoreIconUrl") } />
            </a>

            <a class="main-stores__button" title={ i18n.t("GooglePlayAriaLabel") } href="#play.google.com/store">
                <img loading="lazy" alt="Google Play" src={ i18n.t("GooglePlayIconUrl") } />
            </a>
        </div>
    }
}