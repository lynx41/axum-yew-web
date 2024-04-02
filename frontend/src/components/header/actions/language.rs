use crate::{routes::Route, stores::language::LangSelector};

use gloo::storage::{LocalStorage, Storage};
use strum::IntoEnumIterator;
use web_sys::MouseEvent;
use yew::{function_component, html, html_nested, Callback, Html, Properties};
use yew_router::hooks::use_navigator;


// list of languages can be changed in /src/stores/language.rs
pub const LANGUAGE_KEY: &str = "Language";


#[function_component(Languages)]
pub fn languages() -> Html {
    
    // On Load locales must be set (if not exists)
    let current_lang = match LocalStorage::get::<LangSelector>(LANGUAGE_KEY) {
        Err(_) => {
            let _ = LocalStorage::set(LANGUAGE_KEY, LangSelector::default());
            LangSelector::default()
        },
        Ok(lang) => lang
    };
    
    html! {
        <li class="header-actions__item header-actions__item--language">
            
            <div class="header-actions__component">
                <ul class="lang lang-header">

                    {
                        LangSelector::iter().map(|language| {
                            // map body
                            html_nested!{
                                <LangItem
                                object_language={language}
                                current_language={current_lang.clone()} />
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
    pub object_language: LangSelector,
    pub current_language: LangSelector,
}


#[function_component(LangItem)]
pub fn lang_item(props: &LangItemProps) -> Html {
    let navigator = use_navigator().unwrap();
    let lang = props.object_language.clone();

    let onclick = Callback::from(move |_: MouseEvent| {
        let lang = lang.clone();
        let navigator = navigator.clone();

        let _ = LocalStorage::set(LANGUAGE_KEY, lang);
        navigator.push(&Route::NotFound);
    });

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