mod language;
mod orders;
mod notifications;

use language::Languages;
use orders::Orders;
use notifications::Notifications;

use yew::{function_component, html, Html};


#[function_component(Actions)]
pub fn actions() -> Html {
    html! {
        <ul class="header-actions">
        
        // default language can be changed in /src/stores/language.rs
        <Languages />

        // <Help /> Only for non-auth users

        // !!! Next components visible only for auth users
        
        <Orders />

        // <Bonuses />

        <Notifications />

        </ul>
    }
}