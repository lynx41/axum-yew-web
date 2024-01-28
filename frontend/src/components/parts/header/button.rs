use yew::{function_component, html, Html};

#[function_component(Button)]
pub fn button() -> Html {
    
    html! {
        <div class="header-menu">
            <button class="header-bth">
                <svg>
                    <use href="#icon-menu">
                        <symbol id="icon-menu" viewBox="0 0 24 24">
                            <g>
                                <rect height="2" rx="1" width="22" x="1" y="3"></rect>
                                <rect height="2" rx="1" width="22" x="1" y="11"></rect>
                                <rect height="2" rx="1" width="22" x="1" y="19"></rect>
                            </g>
                        </symbol>
                    </use>
                </svg>
            </button>
        </div>
    }
}