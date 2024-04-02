pub mod button;
pub mod logo;
pub mod catalog;
pub mod search_bar;

pub mod actions;

use gloo::{console::{debug, log}, net::http::Request, storage::{LocalStorage, Storage}};
use yew::{function_component, html, use_effect_with, use_mut_ref, use_state, Html, Properties};

use crate::components::props::HeaderProps;


#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {

    // let is_auth = use_state(|| false);
    // // let mut is_auth = false;
    
    // wasm_bindgen_futures::spawn_local(async move {
    //     let is_auth = is_auth.clone();

    //     if let Ok(token) = LocalStorage::get::<String>("Token") {
    //         let url = "https://localhost:5000/verify_token";
            
    //         let fetched_response = Request::get(url)
    //             .header("Authorization", &format!("Bearer {}", token))
    //             .send()
    //             .await;

    //         log!("Have token");

    //         match fetched_response {
    //             Ok(_) => {
    //                 is_auth.set(true);
    //                 log!("good token");
    //             },
    //             Err(_) => {
    //                 is_auth.set(true);
    //                 log!("bad token");
    //             }
    //         }
    //     } else { log!("NO token") };
    // });

    // let auth = *is_auth.clone().deref();

    // use_effect_with(
    //     (),
    //     {
    //         let is_auth = is_auth.clone();
    //         move |()| {
    //             let is_auth = is_auth.clone();
    //             wasm_bindgen_futures::spawn_local(async move {
    //                 let url = "https://localhost:5000";

    //                 if let Ok(token) = LocalStorage::get::<String>("Token") {
    //                     let fetched_response = Request::get(url)
    //                         .header("Authorization", &format!("Bearer {}", token))
    //                         .send()
    //                         .await;

    //                     match fetched_response {
    //                         Ok(_) => { is_auth.set(true); },
    //                         Err(_) => { }
    //                     }
    //                 };
    //             })
    //         }
    //     }
    // );

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
                        is_auth={props.is_auth.clone()}
                    />
                    
                </div>
            </div>
        </header>
    }
}