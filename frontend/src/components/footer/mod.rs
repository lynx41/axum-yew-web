mod inner_top;
mod inner_bot;
mod mobile_apps_ad;
mod bottom;

use std::ops::Deref;
use std::rc::Rc;

use crate::components::props::FooterProps;
use crate::components::footer::{
    inner_top::InnerTop,
    inner_bot::InnerBot,
    mobile_apps_ad::MobileAppsAd,
    bottom::Bottom,
};

use crate::components::locales::footer_loc;
use crate::components::utils::client_context::ClientContext;

use yew::{function_component, html, use_context, Html};
use yew_i18n::{use_translation, I18nProvider};


#[function_component(Footer)]
// pub fn footer(props: &FooterProps) -> Html {
pub fn footer() -> Html {
    
    let client_context = use_context::<Rc<ClientContext>>().unwrap();

    let mobile_apps_ad_loc = footer_loc::mobile_apps_ad();
    // Footer TOP 
    let inner_top_translations = footer_loc::inner_top_loc();
    let inner_bot_translations = footer_loc::inner_bot_loc();
    // Footer BOT
    let bot_translations = footer_loc::bottom_loc();

    html! {
        
        <div class="app-footer">
        
            <div class="footer-wrapper">

                <I18nProvider supported_languages={client_context.supported_languages.clone()} translations={mobile_apps_ad_loc}>
                    <MobileAppsAd selected_language={client_context.selected_language.deref().clone()} />
                </I18nProvider>
                
                <footer class="footer">
                
                    <div class="layout">

                        // Footer top
                        <div class="footer-top">

                            <I18nProvider supported_languages={client_context.supported_languages.clone()} translations={inner_top_translations}>
                                    <InnerTop selected_language={client_context.selected_language.deref().clone()} />
                            </I18nProvider>
                            

                            <I18nProvider supported_languages={client_context.supported_languages.clone()} translations={inner_bot_translations}>
                                    <InnerBot selected_language={client_context.selected_language.deref().clone()} />
                            </I18nProvider>

                        </div>

                        // Footer bot
                        <div class="footer-bottom">
                        
                            <I18nProvider supported_languages={client_context.supported_languages.clone()} translations={bot_translations}>
                                <Bottom selected_language={client_context.selected_language.deref().clone()} />
                            </I18nProvider>

                        </div>

                    </div>

                </footer>

            </div>

        </div>

    }
}