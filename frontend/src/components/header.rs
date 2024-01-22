use yew::{function_component, html, Html};

#[function_component(Header)]
pub fn header() -> Html {
    
    html! {
        <>
        <header>
            <div class="layout">
                <div class="header-layout">
                    
                    // Burger Icon
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

                    // Logo
                    <a class="header-logo" title="My logo" href="#">
                        <picture>
                            <source media="(min-width: 1280px)" srcset="https://localhost:5000/public/system/images/qiCF4i01.svg" />
                            <source media="(min-width: 240px)" srcset="https://localhost:5000/public/system/images/XfP9di01.svg" />
                            <img alt="BilobaByte Logo" loading="lazy" src="https://localhost:5000/public/system/images/qiCF4i01.svg" />
                        </picture>
                    </a>

                </div>
            </div>
        </header>
        </>
    }
}