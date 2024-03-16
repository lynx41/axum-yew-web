use yew::{function_component, html, Html};

#[function_component(Logo)]
pub fn logo() -> Html {
    html! {
        <a class="header-logo" title="My logo" href="#">
            <picture>
                <source media="(min-width: 1280px)" srcset="https://localhost:5000/public/system/images/qiCF4i01.svg" />
                <source media="(min-width: 240px)" srcset="https://localhost:5000/public/system/images/XfP9di01.svg" />
                <img alt="BilobaByte Logo" loading="lazy" src="https://localhost:5000/public/system/images/qiCF4i01.svg" />
            </picture>
        </a>
    }
}