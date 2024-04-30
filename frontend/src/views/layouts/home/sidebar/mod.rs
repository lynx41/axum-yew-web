mod menu_category;
mod help_centre;
mod main_auth;
mod apps_ads;
mod main_links;

use std::{ops::Deref, rc::Rc};

use menu_category::MenuCategories;
use help_centre::HelpCentre;
use main_auth::MainAuth;
use apps_ads::AppsAds;
use main_links::MainLinks;

use yew::{function_component, html, use_context, Html, Properties};
use crate::components::locales::{footer_loc, sidebar_loc};
use crate::components::{props::IsAuth, utils::client_context::ClientContext};
use yew_i18n::{use_translation, I18nProvider};


#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let client_context = use_context::<Rc<ClientContext>>().unwrap();
    
    let categories_loc = sidebar_loc::categories_loc();
    let help_centre_loc = sidebar_loc::help_centre_loc();
    let auth_loc = sidebar_loc::main_auth_loc();
    let mobile_apps_ad_loc = footer_loc::mobile_apps_ad();
    let links_loc = footer_loc::inner_bot_loc();
    
    // CATEGORIES GOES HERE

    html! {

        <aside class="sidebar sidebar_type_main">
            <div class="menu-wrapper menu-wrapper_state_static">

                <I18nProvider supported_languages={client_context.supported_languages.clone()} translations={categories_loc}>
                    <MenuCategories />
                </I18nProvider> 

                // Help centre menu
                <I18nProvider supported_languages={client_context.supported_languages.clone()} translations={help_centre_loc}>
                    <HelpCentre />
                </I18nProvider> 
                
                // If not loged in, welcome with login button
                if { *client_context.is_auth.deref() == IsAuth::No } {
                    <I18nProvider supported_languages={client_context.supported_languages.clone()} translations={auth_loc}>
                        <MainAuth />
                    </I18nProvider> 
                }

                // Install our apps
                <I18nProvider supported_languages={client_context.supported_languages.clone()} translations={mobile_apps_ad_loc}>
                    <AppsAds />
                </I18nProvider>

                // From the footer
                <I18nProvider supported_languages={client_context.supported_languages.clone()} translations={links_loc}>
                    <MainLinks />
                </I18nProvider>
            </div>
        </aside>
    }
}