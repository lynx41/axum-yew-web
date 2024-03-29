use yew::{function_component, html, Html};
use yew_router::components::Link;

use crate::routes::Route;

#[function_component(Logo)]
pub fn logo() -> Html {
    html! {
        
        <Link<Route> to={Route::Home}>
            <a class="header-logo" title="My logo">
                <picture>
                    <source media="(min-width: 1280px)" srcset="https://localhost:5000/public/system/images/header/big_logo.svg" />
                    <source media="(min-width: 240px)" srcset="https://localhost:5000/public/system/images/header/small_logo.svg" />
                    <img alt="BilobaByte Logo" loading="lazy" src="https://localhost:5000/public/system/images/header/big_logo.svg" />
                </picture>
                </a>
        </Link<Route>>
        
        // <a class="header-logo" title="My logo" href="#">
        //     <picture>
        //         <source media="(min-width: 1280px)" srcset="https://localhost:5000/public/system/images/header/big_logo.svg" />
        //         <source media="(min-width: 240px)" srcset="https://localhost:5000/public/system/images/header/small_logo.svg" />
        //         <img alt="BilobaByte Logo" loading="lazy" src="https://localhost:5000/public/system/images/header/big_logo.svg" />
        //     </picture>
        // </a>
    }
}