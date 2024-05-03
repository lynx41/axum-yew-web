mod components;
mod views;
mod stores;
mod routes;

use std::rc::Rc;

use crate::{components::{props::IsAuth, utils::client_context::ClientContext}, routes::{switch, Route}, stores::language::{get_selected_langauge, get_supported_languages}};

use gloo::{console::log, net::http::Request, storage::{LocalStorage, Storage}};
use yew::{function_component, html, platform::spawn_local, use_effect_with, use_state, ContextProvider, Html, HtmlResult, Suspense};
use yew_router::{BrowserRouter, Switch};

const UNIQUE_ID_KEY: &str = "UniqueID";

#[function_component(ContextualApp)]
fn contextual_app() -> HtmlResult {

    let selected_language = use_state(|| get_selected_langauge());

    let supported_languages = get_supported_languages();

    let is_auth = use_state(|| IsAuth::Unknown);

    let modal_auth_display = use_state(|| false);

    use_effect_with(
        (),
        {
            let is_auth = is_auth.clone();

            move |()| {
                let is_auth = is_auth.clone();

                spawn_local(async move {
                    let is_auth = is_auth.clone();

                    if let Ok(token) = LocalStorage::get::<String>("Token") {
                        let fetched_response = Request::get("https://localhost:5000/cabinet")
                            .header("Authorization", &format!("Bearer {}", token))
                            .send()
                            .await;

                        if let Ok(response) = fetched_response {
                            if response.status() == 200 {
                                is_auth.set(IsAuth::Yes);
                            } else {
                                // if token is bad
                                is_auth.set(IsAuth::No);
                                LocalStorage::delete("Token");
                            }
                        } else {
                            // if error during response
                            is_auth.set(IsAuth::No);
                        }
                    } else {
                        // if no token was found
                        is_auth.set(IsAuth::No);
                    }
                });
            }
        }
    );

    let context = Rc::new(ClientContext {
        selected_language,
        supported_languages,
        is_auth,
        modal_auth_display
    });

    // Check if the person that visited your site has an UniqueId and its valid
    use_effect_with(
        (),
        {
            move |()| {
                spawn_local(async move {

                    match LocalStorage::get::<String>(UNIQUE_ID_KEY) {
                        Ok(unique_id) => {
                            // Client has UniqueID, need to validate the data.
                            let fetched_reponse: String = Request::post("https://localhost:5000/validate_unique_session")
                                .header(UNIQUE_ID_KEY, &unique_id)
                                .send()
                                .await
                                .unwrap()
                                .json()
                                .await
                                .unwrap();
                            
                            let _ = LocalStorage::set(UNIQUE_ID_KEY, fetched_reponse);                        
                        },
                        Err(_) => {
                            // It's client's first visit, create a new UniqueID
                            let fetched_response: String = Request::get("https://localhost:5000/create_unique_session")
                                .send()
                                .await
                                .unwrap()
                                .json()
                                .await
                                .unwrap();

                            let _ = LocalStorage::set(UNIQUE_ID_KEY, fetched_response);
                        },
                    }

                })
            }
        }
    );

    Ok(html! {
        <ContextProvider<Rc<ClientContext>> context={context}>
            <Switch<Route> render={switch} />
        </ContextProvider<Rc<ClientContext>>>
    })
}



#[function_component(App)]
pub fn app() -> Html {

    html! {
        <BrowserRouter>
        
            <Suspense fallback={html!{ <h3>{"Loading"}</h3> }}>
                <ContextualApp />
            </Suspense>
        
        </BrowserRouter>
    }
}