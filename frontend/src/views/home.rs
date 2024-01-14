use std::ops::Deref;

use gloo::{net::http::Request, console::log};
use web_sys::{Event, MouseEvent};
use yew::{Html, html, function_component, Callback, use_state, use_effect, use_effect_with};

#[function_component(Home)]
pub fn home() -> Html {
    
    let text = use_state(|| String::new());
    {
        let text = text.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let url = "https://localhost:5000";

            let fetched_text = Request::get(url)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            text.set(fetched_text);
        })
    }
    
    html! {
        <div>
            <p>{text.deref()}</p>
        </div>
    }
}