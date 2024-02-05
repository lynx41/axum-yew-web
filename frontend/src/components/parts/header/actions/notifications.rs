use yew::{function_component, html, Html};

#[function_component(Notifications)]
pub fn notifications() -> Html {
    
    html! {
        <li class="header-actions__item header-actions__item--notifications">
            <div class="header-actions__component">
                <a class="header__button" href="#cabinet/communications">
                    <svg aria-hidden="true">
                        <use href="icon-simple-bell">
                            <symbol id="icon-simple-bell" viewBox="0 0 24 24">
                                <g clip-path="url(#icon-simple-bell_clip0_2109_100)">
                                    <path fill-rule="evenodd" clip-rule="evenodd" d="M13 1C13 0.447715 12.5523 0 12 0C11.4477 0 11 0.447715 11 1V2.10589C7.52435 2.68435 4.79942 5.56929 4.52087 9.19051L4.05177 15.2888C4.03291 15.534 3.95137 15.7702 3.81498 15.9748L2.16795 18.4453C1.96338 18.7522 1.94431 19.1467 2.11833 19.4719C2.29234 19.797 2.63121 20 3 20H8.12602C8.57006 21.7252 10.1362 23 12 23C13.8638 23 15.4299 21.7252 15.874 20H21C21.3688 20 21.7077 19.797 21.8817 19.4719C22.0557 19.1467 22.0366 18.7522 21.8321 18.4453L20.2185 16.0249C20.0602 15.7874 19.9655 15.5133 19.9436 15.2288L19.4353 8.62076C19.1664 5.12501 16.4216 2.37445 13 2.03523V1ZM12 21C11.2597 21 10.6134 20.5978 10.2676 20H13.7324C13.3866 20.5978 12.7403 21 12 21ZM19.1315 18H4.86853L5.47908 17.0842C5.80556 16.5945 6.00074 16.0291 6.04588 15.4422L6.51498 9.3439C6.74693 6.32847 9.2614 4 12.2857 4C14.9876 4 17.234 6.08022 17.4412 8.77415L17.9495 15.3822C17.9977 16.0084 18.206 16.6117 18.5544 17.1343L19.1315 18Z"></path>
                                </g>
                                <defs></defs>
                            </symbol>
                        </use>
                    </svg>
                </a>
            </div>
        </li>
    }
}