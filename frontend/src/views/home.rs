use std::ops::Deref;

use crate::components::assemblies::{
    header::Header,
    // footer::Footer,
};

use crate::components::footer::Footer;
use crate::routes::Props;

use gloo::console::log;
use gloo::net::http::Request;
use yew::{
    function_component, html,
    suspense::{use_future, UseFutureHandle},Html,
};


#[function_component(Home)]
pub fn home(props: &Props) -> Html {
    
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

    // it calculates everything twice during render and after render is fully ready.
    // so to render only once you need to make your calculations on OK variant
    let text = match text {
        Ok(ref val) => {
            // Here goes your calcualtions
            // log!(format!("{:?}", props.supported_languages));
            val.to_string()
        },
        Err(ref failed) => { String::from("[Loading]") }
    };


    html! {
        <>
        <Header />
        
        <div>
            <p>{text.deref()}</p>
        </div>
        
        <Footer 
            selected_language={props.selected_language.clone()}
            supported_languages={props.supported_languages.clone()} 
        />
        
        </>
    }
}