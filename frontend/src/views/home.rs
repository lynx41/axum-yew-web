use std::ops::Deref;

use crate::components::assemblies::{
    header::Header,
    footer::Footer,
};

use gloo::net::http::Request;
use yew::{
    function_component, html,
    suspense::{use_future, UseFutureHandle},Html,
};

#[function_component(Home)]
pub fn home() -> Html {
    
    let url = "https://localhost:5000";

    let text: Result<UseFutureHandle<String>, yew::suspense::Suspension> = use_future(
        || async {
            Request::get(url)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap()
        }
    );

    let text = match text {
        Ok(ref val) => val.to_string(),
        Err(ref failed) => { String::from("[Loading]") }
    };

    
    html! {
        <>
        <Header />
        
        <div>
            <p>{text.deref()}</p>
        </div>
        
        <Footer />
        </>
    }
}