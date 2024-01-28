use gloo::console::debug;
use yew::{function_component, html, Html};

use crate::components::parts::header;

#[function_component(Header)]
pub fn header() -> Html {

    html! {
        <header>
            <div class="layout">
                <div class="header-layout">
                
                    // header button
                    <header::button::Button />

                    // header logo
                    <header::logo::Logo />

                    // header catalog
                    <header::catalog::Catalog />

                    // header search bar
                    <header::search_bar::SearchBar />

                    // header actions
                    <header::actions::Actions />
                    
                </div>
            </div>
        </header>
    }
}