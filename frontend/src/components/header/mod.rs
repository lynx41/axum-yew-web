pub mod button;
pub mod logo;
pub mod catalog;
pub mod search_bar;

pub mod actions;

use gloo::{console::{debug, log}, net::http::Request, storage::{LocalStorage, Storage}};
use yew::{function_component, html, use_effect_with, use_mut_ref, use_state, Callback, Html, MouseEvent, Properties};

use crate::components::props::HeaderProps;


#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {

    let user_onclick = props.user_btn_onclick.clone();
    let user_btn_onclick = Callback::from(move |e: MouseEvent| {
        user_onclick.emit(e);
    });

    html! {
        <header>
            <div class="layout">
                <div class="header-layout">
                
                    // header button
                    <button::Button />

                    // header logo
                    <logo::Logo />

                    // header catalog
                    <catalog::Catalog />

                    // header search bar
                    <search_bar::SearchBar />

                    // header actions
                    <actions::Actions
                        selected_language={props.selected_language.clone()}
                        supported_languages={props.supported_languages.clone()}
                        user_btn_onclick={props.user_btn_onclick.clone()}
                        is_auth={props.is_auth.clone()}
                    />
                    
                </div>
            </div>
        </header>
    }
}