use yew::{function_component, html, Html};

#[function_component(Stores)]
pub fn stores() -> Html {
    html! {
        <section class="footer-section">
            <div class="layout">
                <div class="footer-stores">
                    <h3 class="footer-stores__heading">
                        {"Завантажуй наші застосунки"}
                    </h3>

                    <ul class="footer-stores__buttons">
                        
                        <li class="footer-stores__item">
                            <a class="footer-stores__button" href="#play.google.com/store" aria-label="Застосунок для Android">
                                <img loading="lazy" alt="Google Play" src="https://localhost:5000/public/system/images/locales/footer/google-play-badge-en.svg" />
                            </a>
                        </li>

                        <li class="footer-stores__item">
                            <a class="footer-stores__button" href="#app-store/" aria-label="Застосунок для IOS">
                                <img loading="lazy" alt="AppStore" src="https://localhost:5000/public/system/images/locales/footer/app-store-badge-en.svg" />
                            </a>
                        </li>

                    </ul>
                </div>
            </div>
        </section>
    }
}