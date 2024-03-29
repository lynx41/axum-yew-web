use yew::{function_component, html, Html};
use yew_router::components::Link;

use crate::routes::Route;


#[function_component(Orders)]
pub fn orders() -> Html {
    html! {
        <li class="header-actions__item header-actions__item--orders">
            <div class="header-actions__component">
                <Link<Route> to={Route::Cabinet}>
                    <a class="header__button" href="#cabinet/orders">
                        <svg aria-hidden="true">
                            <use href="#icon-orders">
                                <symbol id="icon-orders" viewBox="0 0 24 24">
                                    <g>
                                        <path d="m8 7c0 .55228-.44769 1-1 1s-1-.44772-1-1 .44769-1 1-1 1 .44772 1 1z"></path>
                                        <path d="m7 12c.55231 0 1-.4477 1-1s-.44769-1-1-1-1 .4477-1 1 .44769 1 1 1z"></path>
                                        <path d="m8 15c0 .5523-.44769 1-1 1s-1-.4477-1-1 .44769-1 1-1 1 .4477 1 1z"></path>
                                        <path d="m11 6c-.5523 0-1 .44772-1 1s.4477 1 1 1h6c.5523 0 1-.44772 1-1s-.4477-1-1-1z"></path>
                                        <path d="m10 11c0-.5523.4477-1 1-1h6c.5523 0 1 .4477 1 1s-.4477 1-1 1h-6c-.5523 0-1-.4477-1-1z"></path>
                                        <path d="m11 14c-.5523 0-1 .4477-1 1s.4477 1 1 1h6c.5523 0 1-.4477 1-1s-.4477-1-1-1z"></path>
                                        <path clip-rule="evenodd" d="m2 6c0-2.20914 1.79083-4 4-4h12c2.2092 0 4 1.79086 4 4v12c0 2.2091-1.7908 4-4 4h-12c-2.20917 0-4-1.7909-4-4zm4-2c-1.10455 0-2 .89543-2 2v12c0 1.1046.89545 2 2 2h12c1.1046 0 2-.8954 2-2v-12c0-1.10457-.8954-2-2-2z" fill-rule="evenodd"></path>
                                    </g>
                                </symbol>
                            </use>
                        </svg>
                    </a>
                </Link<Route>>
            </div>
        </li>
    }
}