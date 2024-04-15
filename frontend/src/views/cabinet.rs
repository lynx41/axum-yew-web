use std::{borrow::BorrowMut, ops::Deref, rc::Rc};

use crate::{
    components::{footer::Footer, header::Header, props::IsAuth, utils::client_context::ClientContext}, stores::language::{get_selected_langauge, get_supported_languages}, Route};

use yew::{function_component, html, platform::spawn_local, use_context, use_effect_with, use_mut_ref, use_state, Html, Properties};
use yew_router::components::Redirect;
use gloo::{console::log, net::http::Request, storage::{LocalStorage, Storage}};


#[function_component(Cabinet)]
pub fn cabinet() -> Html {

    let client_context = use_context::<Rc<ClientContext>>().unwrap();

    let cabinet_response = use_state(|| String::new());

    // making a unique_session_ID - Usefull inly for guests
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
            
            // <p>{&*cabinet_response.deref()}</p>
            <p>{"Cabinet"}</p>
            <p>{client_context.is_auth.to_string()}</p>
            
            <Footer />
        </>
    }
}