pub mod button;
pub mod logo;
pub mod catalog;
pub mod search_bar;

pub mod actions;

use std::ops::Deref;

use yew::{function_component, html, use_state, Callback, Html, MouseEvent};

use crate::components::props::HeaderProps;

use crate::components::modal_windows::modal_auth::ModalWindowAuth;

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {

    


    // Check if user clicked on the 'User' button to auth
    let modal_auth_display = use_state(|| false);

    let user_btn_onclick = {
        let modal_auth_display = modal_auth_display.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            modal_auth_display.set(true);
        })
    };

    // If the user wants to close the modal auth window by icon
    let close_modal_auth = {
        let modal_auth_display = modal_auth_display.clone();
        Callback::from(move |_: MouseEvent| {
            modal_auth_display.set(false);
        })
    };

    html! {
        <>
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
                        user_btn_onclick={user_btn_onclick.clone()}
                        is_auth={props.is_auth.clone()}
                    />
                    
                </div>
            </div>
        </header>

        if *modal_auth_display.deref() {
            <ModalWindowAuth onclick={close_modal_auth.clone()} />
        }

        </>
    }
}