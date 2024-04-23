use std::borrow::Borrow;
use std::ops::Deref;
use std::rc::Rc;

use crate::components::utils::client_context::ClientContext;
use crate::components::{
    header::Header,
    footer::Footer,
};

use crate::stores::language::get_selected_langauge;
use crate::components::props::IsAuth;
use crate::views::layouts::home::MainPage;

use gloo::net::http::Request;
use gloo::storage::{LocalStorage, Storage};
use yew::platform::spawn_local;
use yew::{use_context, use_effect_with, use_state};
use yew::{function_component, html,Html};


#[function_component(Home)]
pub fn home() -> Html {
    
    let client_context = use_context::<Rc<ClientContext>>().unwrap();

    // config
    // let selected_language = get_selected_langauge();
    // let supported_languages = get_supported_languages();

    // let is_auth = use_state(|| IsAuth::Unknown);

    // Check if the user is authenticated or not
    // use_effect_with(
    //     (),
    //     {
    //         let is_auth = is_auth.clone();
    //         move |()| {
    //             let is_auth = is_auth.clone();
    //             spawn_local(async move {
                
    //                 if let Ok(token) = LocalStorage::get::<String>("Token") {
    //                     let fetched_response = Request::get("https://localhost:5000/verify_token")
    //                         .header("Authorization", &format!("Bearer {}", token))
    //                         .send()
    //                         .await;
                        
    //                     match fetched_response {
    //                         Ok(r) => {
    //                             if r.status() == 200 {
    //                                 is_auth.set(IsAuth::Yes)
    //                             } else {
    //                                 let _ = LocalStorage::delete("Token");
    //                                 is_auth.set(IsAuth::No);
    //                             }
    //                         },
                            
    //                         Err(_) => { is_auth.set(IsAuth::No); }
    //                     }
    //                 } else {
    //                     is_auth.set(IsAuth::No);
    //                 }
    //             });
    //         }
    //     }
    // );

    // making a unique_session_ID
    // use_effect_with(
    //     (),
    //     {
    //         move |()| {
    //             spawn_local(async move {
                    
    //                 // if a first visit gen a new session
    //                 if let Ok(unique_id) = LocalStorage::get::<String>("UniqueID") {
    //                     let fetched_response: String = Request::post("https://localhost:5000/validate_unique_session")
    //                         .header("UniqueID", &unique_id)
    //                         .send()
    //                         .await
    //                         .unwrap()
    //                         .json()
    //                         .await
    //                         .unwrap();

    //                 let _ = LocalStorage::set("UniqueID", fetched_response);

    //                 } else {
    //                     let fetched_response: String = Request::get("https://localhost:5000/create_unique_session")
    //                         .send()
    //                         .await
    //                         .unwrap()
    //                         .json()
    //                         .await
    //                         .unwrap();

    //                     let _ = LocalStorage::set("UniqueID", fetched_response);
    //                 }
    //             });
    //         }
    //     }
    // );

    html! {
        <>

        <Header />
        
        <MainPage />
        
        // Home page doesn't need the Footer, because it has it inside the aside of MainPage 
        // <Footer />

        </>
    }
}