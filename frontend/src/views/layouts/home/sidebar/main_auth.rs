use yew::{function_component, html, Html};

#[function_component(MainAuth)]
pub fn main_auth() -> Html {
    html! {
        <div class="main-auth">
            <h3 class="main-auth__heading">{"Welcome!"}</h3>
            <p class="main-auth__caption">{"Log in to get personal bonuses and sales"}</p>
            <button class="button button--navy button--small main-auth__button" type="button">
                {"Log in"}
            </button>
        </div>
    }
}