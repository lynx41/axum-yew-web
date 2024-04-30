use std::rc::Rc;

use yew::{function_component, html, use_context, Callback, Html, MouseEvent};

use crate::components::utils::client_context::ClientContext;
use yew_i18n::{use_translation, I18nProvider};

#[function_component(MainAuth)]
pub fn main_auth() -> Html {
    
    let client_context = use_context::<Rc<ClientContext>>().unwrap();
    let mut i18n = use_translation();

    i18n.set_translation_language(&client_context.selected_language);

    let onclick = {
        let client_context = client_context.clone();
        Callback::from(move |e: MouseEvent| {
            client_context.modal_auth_display.set(true);
        })
    };

    html! {
        <div class="main-auth">
            <h3 class="main-auth__heading">{ i18n.t("heading") }</h3>
            <p class="main-auth__caption">{ i18n.t("caption") }</p>
            <button class="button button--navy button--small main-auth__button" type="button" onclick={onclick}>
                { i18n.t("button") }
            </button>
        </div>
    }
}