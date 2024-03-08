use yew::{function_component, html, Html};

use crate::components::parts::footer;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class="app-footer">
        
            <div class="footer-wrapper">

                <footer::stores::Stores />

                <footer class="footer">
                
                    <div class="layout">
                    
                        <div class="footer-top">

                            <footer::inner_top::InnerTop />

                            <footer::inner_bot::InnerBot />

                        </div>

                    </div>

                </footer>

            </div>

        </div>


    }
}