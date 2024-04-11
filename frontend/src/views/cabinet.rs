use std::{borrow::BorrowMut, ops::Deref};

use crate::{
    components::{footer::Footer, header::Header, props::IsAuth}, stores::language::{get_selected_langauge, get_supported_languages}, Route};

use yew::{function_component, html, platform::spawn_local, use_effect_with, use_mut_ref, use_state, Html};
use yew_router::components::Redirect;
use gloo::{net::http::Request, storage::{LocalStorage, Storage}};


#[function_component(Cabinet)]
pub fn cabinet() -> Html {
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

    // making a unique_session_ID
    use_effect_with(
        (),
        {
            move |()| {
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
            }
        }
    );




    if *is_auth.deref() != IsAuth::Yes {
        html! { <Redirect<Route> to={Route::Home} /> }
    } else {
        html! {
            <>
                <Header
                    selected_language={selected_language.clone()}
                    supported_languages={supported_languages.clone()}
                    is_auth={is_auth.deref().clone()}
                />
                
                <p>{"Cabinet page"}</p> 
                
                <Footer 
                    selected_language={selected_language.clone()}
                    supported_languages={supported_languages.clone()}
                />
            </>
        }
    }
}