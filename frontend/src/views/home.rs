use std::ops::Deref;
use std::rc::Rc;

use crate::components::{
    header::Header,
    footer::Footer,
};

use crate::stores::language::{get_selected_langauge, get_supported_languages};
use crate::components::props::IsAuth;


use gloo::console::log;
use gloo::net::http::Request;
use gloo::storage::{LocalStorage, Storage};
use gloo::utils::document;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::platform::spawn_local;
use yew::{hook, use_effect_with, use_state, Callback, HtmlResult, MouseEvent};
use yew::{function_component, html,Html};
use yew::suspense::{Suspense, SuspensionResult};

use crate::components::modal_windows::modal_auth::ModalWindowAuth;

#[function_component(Home)]
pub fn home() -> Html {
    
    // config
    let selected_language = get_selected_langauge();
    let supported_languages = get_supported_languages();

    let is_auth = use_state(|| IsAuth::Unknown);

    // Check if the user is authenticated or not
    use_effect_with(
        (),
        {
            let is_auth = is_auth.clone();
            move |()| {
                let is_auth = is_auth.clone();
                spawn_local(async move {
                
                    if let Ok(token) = LocalStorage::get::<String>("Token") {
                        let fetched_response = Request::get("https://localhost:5000/verify_token")
                            .header("Authorization", &format!("Bearer {}", token))
                            .send()
                            .await;
                        
                        match fetched_response {
                            Ok(r) => {
                                if r.status() == 500 {
                                    is_auth.set(IsAuth::No);
                                } else {
                                    is_auth.set(IsAuth::Yes);
                                }
                            },
                            
                            Err(_) => { is_auth.set(IsAuth::No); }
                        }
                    } else {
                        is_auth.set(IsAuth::No);
                    }
                });
            }
        }
    );

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
        Callback::from(move |_| {
            modal_auth_display.set(false);
        })
    };

    // // Close the auth modal windows on click outside the modal

    // {
    //     let modal_auth_display = modal_auth_display.clone();
    //     let close = Rc::new(Closure::<dyn Fn()>::new({
    //         move || {
    //             // modal_auth_display.set(false);
    //             log!("TRUE");
    //         }
    //     }));

    //     use_effect_with((),
    //         move |_| {
    //             // let body = document().body().unwrap();
    //             let body = document().
                
    //             body.add_event_listener_with_callback("click", (*close).as_ref().unchecked_ref())
    //                 .unwrap();
    //             let close_close = close.clone();
    //             move || {
    //                 body.remove_event_listener_with_callback("click", (*close_close).as_ref().unchecked_ref())
    //                     .unwrap();
    //             }
    //         }
    //     );
    // }



    html! {
        <>
        
        <Header
            selected_language={selected_language.clone()}
            supported_languages={supported_languages.clone()}
            user_btn_onclick={user_btn_onclick.clone()}
            is_auth={is_auth.deref().clone()}
        />
        
        if *modal_auth_display.deref() {
            <ModalWindowAuth onclick={close_modal_auth.clone()} />
        }

        <div>
            <p>{"HOME PAGE"}</p>
            <p>{is_auth.to_string()}</p>
        </div>
        
        <Footer 
            selected_language={selected_language.clone()}
            supported_languages={supported_languages.clone()}
        />
        
        </>
    }
}