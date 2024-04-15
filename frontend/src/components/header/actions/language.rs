// use crate::{routes::Route, stores::language::LangSelector};

use std::{ops::Deref, rc::Rc};

use gloo::storage::{LocalStorage, Storage};
use strum::IntoEnumIterator;
use web_sys::MouseEvent;
use yew::{function_component, html, html_nested, use_context, Callback, Html, Properties};
use yew_router::hooks::use_navigator;


// list of languages can be changed in /src/stores/language.rs
use crate::{components::utils::client_context::ClientContext, routes::Route, stores::language::{get_selected_langauge, DEFAULT_LANGUAGE, get_supported_languages, LANGUAGE_KEY}};


#[function_component(Languages)]
pub fn languages() -> Html {
    
    let client_context = use_context::<Rc<ClientContext>>().unwrap();

    // let selected_language = get_selected_langauge();
    // let languages_lst = get_supported_languages();

    
    html! {
        <li class="header-actions__item header-actions__item--language">
            
            <div class="header-actions__component">
                <ul class="lang lang-header">

                    {
                        client_context.supported_languages.iter().map(|language| {
                            html_nested!{
                                <LangItem
                                object_language={language.clone()}
                                current_language={client_context.selected_language.deref().clone()} />
                            }
                        }).collect::<Html>()
                    }

                </ul>
            </div>
        </li>
    }
}


#[derive(Properties, PartialEq, Debug, Clone)]
pub struct LangItemProps {
    pub object_language: String,
    pub current_language: String,
}


#[function_component(LangItem)]
pub fn lang_item(props: &LangItemProps) -> Html {
    // let navigator = use_navigator().unwrap();
    let lang = props.object_language.clone();
    let client_context = use_context::<Rc<ClientContext>>().unwrap();

    let onclick = {
        let lang = lang.clone();
        let client_context = client_context.clone();
        // let navigator = navigator.clone();

        Callback::from(move |_: MouseEvent| {
            let lang = lang.clone();
            let client_context = client_context.clone();

            let _ = LocalStorage::set(LANGUAGE_KEY, &lang);
            client_context.selected_language.set(lang);
    
            // navigator.push(&Route::NotFound);
        })
    };

    if props.object_language == props.current_language {
        html! {
            <li class="lang__item lang-header__item lang-header__item_state_active">
                <span class="lang__link lang__link--active ">
                    {props.object_language.to_string()}
                </span>
            </li>
        }
    } else {
        html! {
            <li class="lang__item lang-header__item">
                <a class="lang__link lang__link--possible" href={format!("#lang:{}",props.object_language.clone())}
                        onclick={onclick} >
                    <img alt={props.object_language.to_string().clone()}
                        src={format!("https://localhost:5000/public/system/images/locales/{}.svg", props.object_language)} />
                    {props.object_language.to_string()}
                </a>
            </li>
        }
    }
}