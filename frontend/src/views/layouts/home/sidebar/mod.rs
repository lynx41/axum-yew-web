mod menu_category;
mod help_centre;
mod main_auth;
mod apps_ads;

use std::{ops::Deref, rc::Rc};

use menu_category::MenuCategories;
use help_centre::HelpCentre;
use main_auth::MainAuth;
use apps_ads::AppsAds;

use yew::{function_component, html, use_context, Html, Properties};

use crate::components::{props::IsAuth, utils::client_context::ClientContext};



#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let client_context = use_context::<Rc<ClientContext>>().unwrap();

    // CATEGORIES GOES HERE, 
    html! {

        <aside class="sidebar sidebar_type_main">
            <div class="menu-wrapper menu-wrapper_state_static">

                <MenuCategories />

                // Help centre menu
                <HelpCentre />

                // If not loged in, welcome with login button
                if { *client_context.is_auth.deref() == IsAuth::No } {
                    <MainAuth />
                }

                // Install our apps
                <AppsAds />


            </div>
        </aside>
    }
}