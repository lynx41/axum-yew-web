use yew::{function_component, html, Html};

#[function_component(Catalog)]
pub fn catalog() -> Html {
    html! {
        <div class="header-catalog">
            <button class="button button--medium button--with-icon menu__toggle">
                <i class="fa fa-th-large" aria-hidden="true"></i>
                {"Каталог"}
            </button>
        </div>
    }
}