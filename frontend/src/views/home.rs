use std::ops::Deref;

use crate::components::{
    header::Header,
    footer::Footer,
};

use crate::stores::language::{get_selected_langauge, get_supported_languages};
use crate::components::props::IsAuth;


use gloo::net::http::Request;
use gloo::storage::{LocalStorage, Storage};
use yew::platform::spawn_local;
use yew::{hook, use_effect_with, use_state, HtmlResult};
use yew::{function_component, html,Html};
use yew::suspense::{Suspense, SuspensionResult};


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

    html! {
        <>
            <Header
                selected_language={selected_language.clone()}
                supported_languages={supported_languages.clone()}
                is_auth={is_auth.deref().clone()}
            />
            
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