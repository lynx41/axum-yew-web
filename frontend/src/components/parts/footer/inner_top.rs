use yew::{function_component, html, Html};

#[function_component(InnerTop)]
pub fn inner_top() -> Html {
    html! {

        <div class="footer-top">
            // Socials
            <div class="footer-top__social">
                <a class="button button--small button--with-icon button--link footer-top__schedule" href="#contacts/">
                    <svg viewBox="0 0 512 512" aria-hidden="true" xmlns="http://www.w3.org/2000/svg">
                        <path d="M464 256A208 208 0 1 1 48 256a208 208 0 1 1 416 0zM0 256a256 256 0 1 0 512 0A256 256 0 1 0 0 256zM232 120V256c0 8 4 15.5 10.7 20l96 64c11 7.4 25.9 4.4 33.3-6.7s4.4-25.9-6.7-33.3L280 243.2V120c0-13.3-10.7-24-24-24s-24 10.7-24 24z"/>
                    </svg>
                    {"Графік роботи Call-центра"}
                </a>
            </div>
        </div>

    }
}