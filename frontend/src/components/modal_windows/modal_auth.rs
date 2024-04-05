use std::rc::Rc;

use gloo::{console::log, net::websocket::events, utils::document};
use wasm_bindgen::{closure::Closure, JsCast};
use yew::{function_component, html, use_effect, use_effect_with, use_mut_ref, use_state, Callback, Html, MouseEvent, Properties};


#[derive(Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<MouseEvent>, 
}

#[function_component(ModalWindowAuth)]
pub fn modal_window_auth(props: &Props) -> Html {

    let is_active = use_mut_ref(|| false);

    // if the user want to close the auth modal
    let onclick = props.onclick.clone();
    let onclick_handler_close = Callback::from(move |e: MouseEvent| {
        onclick.emit(e);
    });


    // if user clicks on auth form everything if fine
    let is_active_onclick = {
        let is_active = is_active.clone();
        Callback::from(move |_: MouseEvent| {
            *is_active.borrow_mut() = true;
        })
    };

    // but when the user clicks outside of the form, the form dissapears
    let is_unactive_onclick = {
        let is_active = is_active.clone();
        let onclick = props.onclick.clone();
        Callback::from(move |e: MouseEvent| {
            if !*is_active.borrow() {
                onclick.emit(e);
            } else {
                *is_active.borrow_mut() = false;
            }
        })
    };

    html! {

        <div id="auth-modal" class="single-modal-window">
            <div class="modal_background modal_background_show_animation" role="button" onclick={is_unactive_onclick}>
                
                // The modal form itself
                <div class="modal__holder modal_holder_show_animation modal__holder--small-medium" onclick={is_active_onclick}>
                
                    <div class="modal__header">
                        <h3 class="modal__heading">{"Вхід"}</h3>
                        <button class="modal__close" type="button" aria-label="Закрити модальне вікно" onclick={onclick_handler_close}>
                            <svg pointer-events="none">
                                <use href="#icon-close-modal">
                                    <symbol viewBox="0 0 24 24" id="icon-close-modal">
                                        <path d="m18.295 7.11511c.3894-.38936.3894-1.02064 0-1.41-.3893-.38936-1.0206-.38936-1.41 0l-4.885 4.88499-4.88499-4.88499c-.38936-.38936-1.02063-.38936-1.41 0-.38936.38936-.38936 1.02064 0 1.41l4.88499 4.88499-4.88499 4.885c-.38936.3894-.38936 1.0206 0 1.41.38937.3894 1.02064.3894 1.41 0l4.88499-4.885 4.885 4.885c.3894.3894 1.0207.3894 1.41 0 .3894-.3894.3894-1.0206 0-1.41l-4.885-4.885z"></path>
                                    </symbol>
                                </use>
                            </svg>
                        </button>
                    </div>

                    <div class="modal__content">
                        <p>{"FUTURE AUTH FORM"}</p>
                    </div>

                </div>

            </div>
        </div>
        
    }
}