mod language;

use language::Languages;


use yew::{function_component, html, Html};


#[function_component(Actions)]
pub fn actions() -> Html {
    html! {
        <ul class="header-actions">
        
        // default language can be changed in /src/stores/language.rs
        <Languages />

        </ul>
    }
}