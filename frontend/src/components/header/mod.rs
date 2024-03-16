pub mod button;
pub mod logo;
pub mod catalog;
pub mod search_bar;

pub mod actions;

use gloo::console::debug;
use yew::{function_component, html, Html};


#[function_component(Header)]
pub fn header() -> Html {

    html! {
        <header>
            <div class="layout">
                <div class="header-layout">
                
                    // header button
                    <button::Button />

                    // header logo
                    <logo::Logo />

                    // header catalog
                    <catalog::Catalog />

                    // header search bar
                    <search_bar::SearchBar />

                    // header actions
                    <actions::Actions />
                    
                </div>
            </div>
        </header>
    }
}