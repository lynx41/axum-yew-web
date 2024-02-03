use std::ops::Deref;

use crate::{routes::Route, stores::language::LangSelector};

use gloo::{console::debug, storage::{LocalStorage, Storage}};
use strum::IntoEnumIterator;
use web_sys::{js_sys::global, Event, MouseEvent};
use yew::{function_component, html, html_nested, scheduler::push, use_effect, use_effect_with, use_force_update, use_state, Callback, Hook, Html, Properties};
use yew_router::{components::Redirect, hooks::{use_navigator, use_route}, navigator::Navigator, AnyRoute};
// use yewdux::{dispatch::Dispatch, functional::use_store};
use yewdux::{dispatch, prelude::use_store, use_selector, Dispatch, Store};


const LANGUAGE_KEY: &str = "Language";


#[function_component(Languages)]
pub fn languages() -> Html {
    // let (store, dispatch) = use_store::<LangSelector>();
    
    // Check for local language file
    let chosen_language = match LocalStorage::get::<LangSelector>(LANGUAGE_KEY) {
        Err(error) => {
            LocalStorage::set(LANGUAGE_KEY, LangSelector::default());
            LangSelector::default()
        },
        Ok(language) => language
    };

    // Changing the language from child element
    fn callback_with_lang(language: LangSelector) -> Callback<MouseEvent> {
        Callback::from(move |_| {
            LocalStorage::set(LANGUAGE_KEY, language.clone());
        })
    }


    html! {
        <li class="header-actions__item header-actions__item--langauge">
            
            <div class="header-actions__component">
                <ul class="lang lang-header">

                    {
                        LangSelector::iter().map(|language| {
                            // map body
                            if language == chosen_language {
                                html! {
                                    <li class="lang__item lang-header__item lang-header__item_state_active">
                                        <span class="lang__link lang__link--active">
                                            {language.to_string()}
                                        </span>
                                    </li>
                                }
                            } else {
                                html! {
                                    <li class="lang__item lang-header__item">
                                        <a class="lang__link" href={format!("#lang:{}",language.clone())}
                                                onclick={callback_with_lang(language.clone())} >
                                            <img alt={language.to_string().clone()} src="https://localhost:5000/public/system/images/flag-ua.svg" />
                                            {language.to_string()}
                                        </a>
                                    </li>
                                }
                            }

                            // return as html
                        }).collect::<Html>()
                    }

                    // {
                    //     LangSelector::iter().map(|language| {                            
                    //         html_nested! { <LangItem language={language} handle_onclick={onclick.clone()} /> }
                    //     }).collect::<Html>()
                    // }
                </ul>
            </div>
        </li>
    }
}


// #[derive(Properties, PartialEq, Debug, Clone)]
// pub struct LangProps {
//     pub language: LangSelector,
//     handle_onclick: Callback<LangSelector>
// }

// #[function_component(LangItem)]
// pub fn lang_item(props: &LangProps) -> Html {
//     // Why this needs to be allocated for every language?
//     let chosen_language = LocalStorage::get(LANGUAGE_KEY).unwrap();

//     // It's hard to debug
//     let handle_onclick = props.handle_onclick.clone();

//     let onclick = Callback::from(move |_| {
//         handle_onclick.emit(props.language.clone());
//     });

//     if props.language == chosen_language {
        
//         html! {
//             <li class="lang__item lang-header__item lang-header__item_state_active">
//                 <span class="lang__link lang__link--active">
//                     {props.language.to_string()}
//                 </span>
//             </li>
//         }

//     } else {

//         html! {
//             <li class="lang__item lang-header__item">
//                 <a class="lang__link" href={format!("#lang:{}",props.language)} onclick={onclick} >
//                     <img alt={props.language.to_string()} src="https://localhost:5000/public/system/images/flag-ua.svg" />
//                     {props.language.to_string()}
//                 </a>
//             </li>
//         }
//     }
// }