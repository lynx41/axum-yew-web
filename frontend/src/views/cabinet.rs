use std::{borrow::BorrowMut, ops::Deref, rc::Rc};

use crate::{
    components::{footer::Footer, header::Header, props::IsAuth, utils::client_context::ClientContext}, stores::language::{get_selected_langauge, get_supported_languages}, Route};

use yew::{function_component, html, platform::spawn_local, use_context, use_effect_with, use_mut_ref, use_state, Html, Properties};
use yew_router::components::Redirect;
use gloo::{console::log, net::http::Request, storage::{LocalStorage, Storage}};


#[function_component(Cabinet)]
pub fn cabinet() -> Html {

    let client_context = use_context::<Rc<ClientContext>>().unwrap();

   
    html! {
        <>
            <Header />
            
            <p>{"Cabinet"}</p>
            
            <Footer />
        </>
    }
}