mod content;
mod sidebar;

use content::Content;
use sidebar::Sidebar;

use yew::{function_component, html, Html};


#[function_component(MainPage)]
pub fn main_page() -> Html {
  
  
    html! {

        <div class="main-page">
            <div class="layout layout_wuth_sidebar">

                <Content />

                <Sidebar />

            </div>
        </div>

    }
}