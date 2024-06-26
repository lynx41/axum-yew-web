mod custom_input;
mod countries;
mod validate_input;

use std::{ops::Deref, rc::Rc};

use gloo::{console::log, net::{http::Request, websocket::events}, storage::{LocalStorage, Storage}, utils::document};
use serde_json::{json, to_value};
use shared::models::user::User;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{js_sys::JSON, HtmlInputElement};
use yew::{function_component, html, platform::spawn_local, use_context, use_effect, use_effect_with, use_mut_ref, use_state, Callback, Event, Html, MouseEvent, Properties};
use yew_router::hooks::use_navigator;

use crate::{components::{props::IsAuth, utils::client_context::ClientContext}, routes::Route};

use self::validate_input::{validate_email, validate_password};


#[derive(Debug, PartialEq)]
pub enum AuthTemplate {
    Login,
    Register,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<MouseEvent>,
}

#[function_component(ModalWindowAuth)]
pub fn modal_window_auth(props: &Props) -> Html {

    let client_context = use_context::<Rc<ClientContext>>().unwrap().clone();

    let navigator = use_navigator().unwrap();

    let is_active = use_mut_ref(|| false);
    let auth_template = use_state(|| AuthTemplate::Login);
    let is_eye_icon_active = use_state(|| false);
    
    let email_is_valid = use_state(|| false);
    let email_handler = use_state(|| String::new());

    let password_is_valid = use_state(|| false);
    let password_handler = use_state(|| String::new());
    let password_handler_repeat = use_state(|| String::new());

    let on_toggle_password = {
        let is_eye_icon_active = is_eye_icon_active.clone();
        Callback::from(move |_| {
            is_eye_icon_active.set(!*is_eye_icon_active.deref());
        })
    };

    // if the user wants to change the template from login to register and vice versa
    let switch_template = {
        let auth_template = auth_template.clone();
        Callback::from(move |_: MouseEvent| {
            if *auth_template.deref() == AuthTemplate::Login {
                auth_template.set(AuthTemplate::Register);
            } else { auth_template.set(AuthTemplate::Login) }
        })
    };


    // if the user wants to close the auth modal
    let onclick = props.onclick.clone();
    let onclick_handler_close = {
        let onclick = onclick.clone();
        Callback::from(move |e: MouseEvent| {
            onclick.emit(e);
        })
    };
    
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

    let email_onchange = {
        let email_is_valid = email_is_valid.clone();
        let email_handler = email_handler.clone();

        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            
            if validate_email(&value) {
                email_is_valid.set(true);
                email_handler.set(value);
            } else {
                email_is_valid.set(false);
            }
        })
    };

    let password_onchange = {
        let password_is_valid = password_is_valid.clone();
        let password_handler = password_handler.clone();
        let auth_template = auth_template.clone();
        let password_handler_repeat = password_handler_repeat.clone();

        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            
            if validate_password(&value) {
                if *auth_template.deref() == AuthTemplate::Register {
                    if value == *password_handler_repeat.deref() {
                        password_handler.set(value);
                        password_is_valid.set(true);
                    }
                } else {
                    password_handler.set(value);
                    password_is_valid.set(true);
                }
            } else {
                password_is_valid.set(false);
            }
        })
    };

    let password_repeat_onchange = {
        let password_handler_repeat = password_handler_repeat.clone();

        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            password_handler_repeat.set(value);
        })
    };

    let send_btn_onclick = {
        let navigator = navigator.clone();
        let onclick = onclick.clone();
        let client_context = client_context.clone();
        
        let email_is_valid = email_is_valid.clone();
        let password_is_valid = password_is_valid.clone();
        let auth_template = auth_template.clone();

        let email_handler = email_handler.clone();
        let password_handler = password_handler.clone();

        Callback::from(move |e: MouseEvent| {
            let navigator = navigator.clone();
            let onclick = onclick.clone();
            let client_context = client_context.clone();

            if *email_is_valid.deref() && *password_is_valid.deref() {

                // If inputs are valid, than depending on the AuthTemplate you can send requests

                spawn_local(async move {

                    // if a first visit gen a new session
                    if let Ok(unique_id) = LocalStorage::get::<String>("UniqueID") {
                        let fetched_response: String = Request::post("https://localhost:5000/validate_unique_session")
                            .header("UniqueID", &unique_id)
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();

                    let _ = LocalStorage::set("UniqueID", fetched_response);

                    } else {
                        let fetched_response: String = Request::get("https://localhost:5000/create_unique_session")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();

                        let _ = LocalStorage::set("UniqueID", fetched_response);
                    }
                });


                let user = User {
                    email: email_handler.deref().clone(),
                    password: password_handler.deref().clone(),
                    unique_id: LocalStorage::get("UniqueID").unwrap(),
                };


                if *auth_template.deref() == AuthTemplate::Login {
                    // Call the login API
                    spawn_local(async move {
                        let client_context = client_context.clone();
                        let onclick = onclick.clone();

                        let fetched_response = Request::post("https:/localhost:5000/login")
                            .json(&user)
                            .unwrap()
                            .send()
                            .await
                            .unwrap()
                            .json::<String>()
                            .await;
                        
                        match fetched_response {
                            Ok(token) => {
                                log!("1_ok!!!!");
                                let _ = LocalStorage::set("Token", token);
                                
                                // Everything was done successfully - login
                                navigator.push(&Route::Home);
                                onclick.emit(e);
                                client_context.is_auth.set(IsAuth::Yes);
                            },
                            Err(_) => { log!("1_ERRR") }
                        }
                    })

                } else {
                    // Call the Register API
                    spawn_local(async move {
                        let client_context = client_context.clone();
                        let onclick = onclick.clone();

                        let fetched_response = Request::post("https:/localhost:5000/register")
                            .json(&user)
                            .unwrap()
                            .send()
                            .await
                            .unwrap()
                            .json::<String>()
                            .await;
                        
                        match fetched_response {
                            Ok(token) => {
                                let _ = LocalStorage::set("Token", token);

                                // Everything was done successfully - register
                                navigator.push(&Route::Home);
                                onclick.emit(e);
                                client_context.is_auth.set(IsAuth::Yes);
                            },
                            Err(_) => { log!("2_ERRR") }
                        }
                    })

                }
            }
        })
    };

    html! {

        <div id="auth-modal" class="single-modal-window">
            <div class="modal_background modal_background_show_animation" role="button" onclick={is_unactive_onclick}>
                
                // The modal form itself
                <div class="modal__holder modal_holder_show_animation modal__holder--small-medium" onclick={is_active_onclick}>
                    
                    // CONTENT HEADER
                    <div class="modal__header">
                        if {*auth_template.deref() == AuthTemplate::Login} {
                            <h3 class="modal__heading">{"Вхід"}</h3>
                        } else {
                            <h3 class="modal__heading">{"Реєстрація"}</h3>
                        }
                        
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
                    
                    // CONTENT FORM
                    <div class="modal__content">
                        <div class="auth-form" role="form">

                            <div class="email">
                                <label class="form__label" for="email">{"Ел. пошта"}</label>
                                <input id="email" class="email__input" type="text" _size_medium="" onchange={email_onchange} />
                            </div>

                            <div class="password">
                                <label class="form__label" for="password">{"Пароль"}</label>
                                <div class="form__row_with_button">
                                    <input id="password" class="password__input" type={ if *is_eye_icon_active.deref() { "text" } else { "password" } } _size_medium="" onchange={password_onchange} />
                                    <button class="button_type_link form__toggle-password" type="button" aria-hidden="true" onclick={on_toggle_password}>
                                        if {*is_eye_icon_active.deref()} {
                                            <svg width="24" height="24" aria-hidden="true">
                                                <use href="#icon-eye-blind">
                                                    <symbol viewBox="0 0 24 24" id="icon-eye-blind">
                                                        <path clip-rule="evenodd" d="m7.66041 17.7539-2.95332 2.9533-1.41421-1.4142 2.62823-2.6283c-1.64498-1.1692-3.02375-2.5341-3.89946-3.4843-.62302-.6759-.62302-1.6849 0-2.3608 1.83891-1.99523 5.89614-5.8196 9.97835-5.8196 1.485 0 2.9668.50613 4.3397 1.24619l2.9532-2.95322 1.4142 1.41421-2.6281 2.62814c1.6449 1.16921 3.0236 2.53413 3.8993 3.48428.623.6759.623 1.6849 0 2.3608-1.8389 1.9952-5.8961 5.8196-9.9783 5.8196-1.485 0-2.96676-.5061-4.33959-1.2461zm1.48789-1.4879c.9892.4669 1.9548.734 2.8517.734 1.4498 0 3.0794-.698 4.6978-1.81 1.4879-1.0223 2.7781-2.2641 3.6471-3.19-.869-.9259-2.1592-2.16765-3.6471-3.19003-.0185-.01271-.037-.02537-.0555-.03797l-2.71 2.71c.0441.1652.0677.3389.0677.518 0 1.1046-.8954 2-2 2-.1791 0-.3528-.0236-.518-.0677zm3.3698-6.1982c-.1653-.0442-.3389-.0678-.5181-.0678-1.1046 0-2 .8954-2 2 0 .1792.0236.3528.0678.5181l-2.71001 2.71c-.01853-.0127-.03706-.0253-.05559-.0381-1.48794-1.0223-2.77813-2.2641-3.64714-3.19.86901-.9259 2.1592-2.16765 3.64714-3.19003 1.61835-1.11198 3.2479-1.80997 4.6978-1.80997.8969 0 1.8625.2671 2.8518.73408z" fill-rule="evenodd"></path>
                                                    </symbol>
                                                </use>
                                            </svg>
                                        } else {
                                            <svg width="24" height="24" aria-hidden="true">
                                                <use href="#icon-eye">
                                                    <symbol viewBox="0 0 24 24" id="icon-eye">
                                                        <g>
                                                            <path d="m12 14c1.1046 0 2-.8954 2-2s-.8954-2-2-2-2 .8954-2 2 .8954 2 2 2z"></path>
                                                            <path clip-rule="evenodd" d="m21.9783 10.8196c.623.6759.623 1.6849 0 2.3608-1.8389 1.9952-5.8961 5.8196-9.9783 5.8196-4.08221 0-8.13944-3.8244-9.97835-5.8196-.62302-.6759-.62302-1.6849 0-2.3608 1.83891-1.99523 5.89614-5.8196 9.97835-5.8196 4.0822 0 8.1394 3.82437 9.9783 5.8196zm-5.2805-2.00963c1.4879 1.02238 2.7781 2.26413 3.6471 3.19003-.869.9259-2.1592 2.1677-3.6471 3.19-1.6184 1.112-3.248 1.81-4.6978 1.81-1.4499 0-3.07945-.698-4.6978-1.81-1.48794-1.0223-2.77813-2.2641-3.64714-3.19.86901-.9259 2.1592-2.16765 3.64714-3.19003 1.61835-1.11198 3.2479-1.80997 4.6978-1.80997 1.4498 0 3.0794.69799 4.6978 1.80997z" fill-rule="evenodd"></path>
                                                        </g>
                                                    </symbol>
                                                </use>
                                            </svg>
                                        }
                                    </button>
                                </div>
                            </div>

                            if {*auth_template.deref() == AuthTemplate::Register} {
                                <div class="password_repeat">
                                    <label class="form__label" for="password">{"Повторіть пароль"}</label>
                                    <div class="form__row_with_button">
                                        <input id="password_repeat" class="password__input" type={ if *is_eye_icon_active.deref() { "text" } else { "password" } } _size_medium="" onchange={password_repeat_onchange} />
                                    </div>
                                </div>
                            } else {
                                <button class="button button--medium button--link link-button link-button__bottom">{"Нагадати пароль"}</button>
                            }

                            <div class="indentation__bottom"></div>

                            <button class="button button--medium button--green submit-button submit-button_bottom" type="submit" onclick={send_btn_onclick}>{"Продовжити"}</button>
                    
                            
                    
                            <button class="button button--medium button--link link-button link-button__bottom" onclick={switch_template} >{
                                if {*auth_template.deref() == AuthTemplate::Register} { "Авторизуватися" } else { "Зареєструватися" }
                            }</button>

                        </div>
                    </div>

                </div>

            </div>
        </div>
        
    }
}