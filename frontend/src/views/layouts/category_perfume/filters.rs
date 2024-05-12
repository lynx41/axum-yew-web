use std::{collections::HashMap, ops::Deref};

use gloo::{console::log, history::query::ToQuery, net::http::QueryParams, storage::{LocalStorage, Storage}, utils::window};
use serde::{Deserialize, Serialize};
use shared::models::categories::perfume::PerfumeQuery;
use web_sys::HtmlElement;
use yew::{function_component, html, use_mut_ref, use_state, Callback, Html, MouseEvent, Properties, TargetCast};
use yew_router::{hooks::{use_location, use_navigator}, navigator};
use yew::html::IntoPropValue;

use crate::routes::Route;

// fn set_query_params() {
//     set_query_param("Brand", "Avi,Bak");
// }

// fn set_query_param(param_name: &str, param_value: &str) {
    
//     let location = use_location().;
    
//     // let location = gloo::utils::window().location();
//     // let search = if let Ok(the_search) = location {
        
//     // }
    
// }

fn checkbox_change(e: MouseEvent) {
    let target = e.target_unchecked_into::<HtmlElement>();
    let class_name = target.class_name();
    if class_name.contains("checkbox-filter__link--checked") {
        target.set_class_name("checkbox-filter__link");
    } else {
        target.set_class_name("checkbox-filter__link checkbox-filter__link--checked");
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub emit_href: Callback<Option<String>>
}

#[function_component(Filters)]
pub fn filters(props: &Props) -> Html {


    // let filter_params = use_state(|| FilterParams::new());


    let brand_query = use_state(|| HashMap::<i32, String>::new());
    let seasonality_query = use_state(|| HashMap::<i32, String>::new());
    let volume_query = use_state(|| HashMap::<i32, String>::new());
    let class_query = use_state(|| HashMap::<i32, String>::new());

    let filters_update = use_state(|| false);

    // Update url with an applied filters
    let navigator = use_navigator().unwrap();

    if *filters_update.deref() {
        let query = 
        PerfumeQuery {
            brand: {
                if !brand_query.is_empty() { Some(brand_query.iter().map(|(_, name)| { name.to_owned() }).collect::<Vec<String>>().join(",")) } else { None }
            },
            seasonality: {
                if !seasonality_query.is_empty() { Some(seasonality_query.iter().map(|(_, name)| { name.to_owned() }).collect::<Vec<String>>().join(",")) } else { None }
            },
            volume: {
                if !volume_query.is_empty() { Some(volume_query.iter().map(|(_, name)| { name.to_owned() }).collect::<Vec<String>>().join(",")) } else { None }
            },
            class: {
                if !class_query.is_empty() { Some(class_query.iter().map(|(_, name)| { name.to_owned() }).collect::<Vec<String>>().join(",")) } else { None }
            }
        };

        let _ = LocalStorage::set("LocalQuery", &query);
        let _ = navigator.push_with_query(&Route::CategoryParfume, &query);

        let href = window().location().href().unwrap();
        log!(&href);
        let href_query = href.split_terminator("?").nth(1);

        
        match href_query {
            Some(q) => { props.emit_href.emit(Some(q.to_owned())) },
            None => { props.emit_href.emit(None) }
        }

        filters_update.set(false);
    }



    // checkbox onclick
    let onclick_1 = {
        let brand_query = brand_query.clone();
        let filters_update = filters_update.clone();
        Callback::from(move |e: MouseEvent| {

            // write to storage, the storage will be sended to backend
            checkbox_change(e);

            let id = 1;
            let mut query = brand_query.deref().to_owned();

            if query.contains_key(&id) {
                let _ = query.remove(&id);
            } else {
                let _ = query.insert(id, "Kilian".to_owned());
            }

            brand_query.set(query);
            filters_update.set(true);
        })
    };

    let onclick_2 = {
        let brand_query = brand_query.clone();
        let filters_update = filters_update.clone();
        Callback::from(move |e: MouseEvent| {

            // write to storage, the storage will be sended to backend
            checkbox_change(e);

            let id = 2;
            let mut query = brand_query.deref().to_owned();

            if query.contains_key(&id) {
                let _ = query.remove(&id);
            } else {
                let _ = query.insert(id, "Carolina Herrera".to_owned());
            }

            brand_query.set(query);
            filters_update.set(true);
        })
    };

    let onclick_3 = {
        let brand_query = brand_query.clone();
        let filters_update = filters_update.clone();
        Callback::from(move |e: MouseEvent| {

            // write to storage, the storage will be sended to backend
            checkbox_change(e);

            let id = 3;
            let mut query = brand_query.deref().to_owned();

            if query.contains_key(&id) {
                let _ = query.remove(&id);
            } else {
                let _ = query.insert(id, "Montale".to_owned());
            }

            brand_query.set(query);
            filters_update.set(true);
        })
    };

    let onclick_4 = {
        let seasonality_query = seasonality_query.clone();
        let filters_update = filters_update.clone();
        Callback::from(move |e: MouseEvent| {

            // write to storage, the storage will be sended to backend
            checkbox_change(e);

            let id = 1;
            let mut query = seasonality_query.deref().to_owned();

            if query.contains_key(&id) {
                let _ = query.remove(&id);
            } else {
                let _ = query.insert(id, "Spring".to_owned());
            }

            seasonality_query.set(query);
            filters_update.set(true);
        })
    };

    let onclick_5 = {
        let seasonality_query = seasonality_query.clone();
        let filters_update = filters_update.clone();
        Callback::from(move |e: MouseEvent| {

            // write to storage, the storage will be sended to backend
            checkbox_change(e);

            let id = 2;
            let mut query = seasonality_query.deref().to_owned();

            if query.contains_key(&id) {
                let _ = query.remove(&id);
            } else {
                let _ = query.insert(id, "Winter".to_owned());
            }

            seasonality_query.set(query);
            filters_update.set(true);
        })
    };

    let onclick_6 = {
        let seasonality_query = seasonality_query.clone();
        let filters_update = filters_update.clone();
        Callback::from(move |e: MouseEvent| {

            // write to storage, the storage will be sended to backend
            checkbox_change(e);

            let id = 3;
            let mut query = seasonality_query.deref().to_owned();

            if query.contains_key(&id) {
                let _ = query.remove(&id);
            } else {
                let _ = query.insert(id, "Summer".to_owned());
            }

            seasonality_query.set(query);
            filters_update.set(true);
        })
    };

    let onclick_7 = {
        let seasonality_query = seasonality_query.clone();
        let filters_update = filters_update.clone();
        Callback::from(move |e: MouseEvent| {

            // write to storage, the storage will be sended to backend
            checkbox_change(e);

            let id = 4;
            let mut query = seasonality_query.deref().to_owned();

            if query.contains_key(&id) {
                let _ = query.remove(&id);
            } else {
                let _ = query.insert(id, "Autumn".to_owned());
            }

            seasonality_query.set(query);
            filters_update.set(true);
        })
    };

    let onclick_8 = {
        let volume_query = volume_query.clone();
        let filters_update = filters_update.clone();
        Callback::from(move |e: MouseEvent| {

            // write to storage, the storage will be sended to backend
            checkbox_change(e);

            let id = 1;
            let mut query = volume_query.deref().to_owned();

            if query.contains_key(&id) {
                let _ = query.remove(&id);
            } else {
                let _ = query.insert(id, "100".to_owned());
            }

            volume_query.set(query);
            filters_update.set(true);
        })
    };

    let onclick_9 = {
        let volume_query = volume_query.clone();
        let filters_update = filters_update.clone();
        Callback::from(move |e: MouseEvent| {

            // write to storage, the storage will be sended to backend
            checkbox_change(e);

            let id = 2;
            let mut query = volume_query.deref().to_owned();

            if query.contains_key(&id) {
                let _ = query.remove(&id);
            } else {
                let _ = query.insert(id, "20".to_owned());
            }

            volume_query.set(query);
            filters_update.set(true);
        })
    };

    let onclick_10 = {
        let volume_query = volume_query.clone();
        let filters_update = filters_update.clone();
        Callback::from(move |e: MouseEvent| {

            // write to storage, the storage will be sended to backend
            checkbox_change(e);

            let id = 3;
            let mut query = volume_query.deref().to_owned();

            if query.contains_key(&id) {
                let _ = query.remove(&id);
            } else {
                let _ = query.insert(id, "30".to_owned());
            }

            volume_query.set(query);
            filters_update.set(true);
        })
    };

    let onclick_11 = {
        let class_query = class_query.clone();
        let filters_update = filters_update.clone();
        Callback::from(move |e: MouseEvent| {

            // write to storage, the storage will be sended to backend
            checkbox_change(e);

            let id = 1;
            let mut query = class_query.deref().to_owned();

            if query.contains_key(&id) {
                let _ = query.remove(&id);
            } else {
                let _ = query.insert(id, "Mass".to_owned());
            }

            class_query.set(query);
            filters_update.set(true);
        })
    };

    let onclick_12 = {
        let class_query = class_query.clone();
        let filters_update = filters_update.clone();
        Callback::from(move |e: MouseEvent| {

            // write to storage, the storage will be sended to backend
            checkbox_change(e);

            let id = 2;
            let mut query = class_query.deref().to_owned();

            if query.contains_key(&id) {
                let _ = query.remove(&id);
            } else {
                let _ = query.insert(id, "Elite".to_owned());
            }

            class_query.set(query);
            filters_update.set(true);
        })
    };

    let onclick_13 = {
        let class_query = class_query.clone();
        let filters_update = filters_update.clone();
        Callback::from(move |e: MouseEvent| {

            // write to storage, the storage will be sended to backend
            checkbox_change(e);

            let id = 3;
            let mut query = class_query.deref().to_owned();

            if query.contains_key(&id) {
                let _ = query.remove(&id);
            } else {
                let _ = query.insert(id, "Special".to_owned());
            }

            class_query.set(query);
            filters_update.set(true);
        })
    };

    html! {

        <aside class="sidebar">
            // sidebar-block with search-bar | when disabled has the role sidebar-block_state_collapsed
            
            // Sidebar block - Brand

            <div class="sidebar-block">
                <button class="sidebar-block__toggle">
                    <span class="sidebar-block_toggle-title">
                        {"Бренд"}
                        // <span class="sidebar-block__toggle-quantity">{"1"}</span>
                    </span>

                    <svg class="sidebar-block__toggle-icon">
                        <use href="#icon-angle-left">
                            <symbol viewBox="0 0 24 24" id="icon-angle-left">
                                <path clip-rule="evenodd" d="m16.726 21.6877c-.3799.401-1.0128.4181-1.4137.0383l-10.26633-9.726 10.26633-9.72595c.4009-.37984 1.0338-.36273 1.4137.0382.3798.40094.3627 1.03387-.0383 1.4137l-8.73367 8.27405 8.73367 8.274c.401.3799.4181 1.0128.0383 1.4137z" fill-rule="evenodd"></path>
                            </symbol>
                        </use>
                    </svg>
                </button>

                <div class="sidebar-block__inner" style="overflow-x: hidden;">
                    // <div class="sidebar-search">
                    //     <input class="sidebar-search__input" placeholder="Пошук" />
                    // </div>

                    <div class="scrollbar__layout">
                        <div class="scrollbar__inner" style="width: 251px">
                            <div style="width: 233px;">
                                <ul class="checkbox-filter">
                                    
                                    <li class="checkbox-filter__item" onclick={onclick_1}>
                                        <a class="checkbox-filter__link">{"KIlian"}</a>
                                    </li>

                                    <li class="checkbox-filter__item" onclick={onclick_2}>
                                        <a class="checkbox-filter__link">{"Carolina Herrera"}</a>
                                    </li>

                                    <li class="checkbox-filter__item" onclick={onclick_3}>
                                        <a class="checkbox-filter__link">{"Montale"}</a>
                                    </li>


                                    // {
                                    //     for (0..75).into_iter().map(|_| {
                                    //         html! { 
                                    //             <li class="checkbox-filter__item">
                                    //                 <a class="checkbox-filter__link">{"First 1"}</a>
                                    //             </li>
                                    //         }
                                    //     })
                                    // }

                                    // <li class="checkbox-filter__item">
                                    //     <a class="checkbox-filter__link checkbox-filter__link--checked">{"Second 2"}</a>
                                    // </li>

                                </ul>
                            </div>
                        </div>

                        // <div class="scrollbar__holder" style="right: 0px">
                        //     <div class="scrollbar__path">
                        //         <div class="scrollbar__slider" style="height: 69.7785px; transform: translateY(0px);"></div>
                        //     </div>
                        // </div>
                    </div>
                </div>

            </div>


            // Sidebar block - Seasonality

            <div class="sidebar-block">
                <button class="sidebar-block__toggle">
                    <span class="sidebar-block_toggle-title">
                        {"Сезонність"}
                        // <span class="sidebar-block__toggle-quantity">{"1"}</span>
                    </span>

                    <svg class="sidebar-block__toggle-icon">
                        <use href="#icon-angle-left">
                            <symbol viewBox="0 0 24 24" id="icon-angle-left">
                                <path clip-rule="evenodd" d="m16.726 21.6877c-.3799.401-1.0128.4181-1.4137.0383l-10.26633-9.726 10.26633-9.72595c.4009-.37984 1.0338-.36273 1.4137.0382.3798.40094.3627 1.03387-.0383 1.4137l-8.73367 8.27405 8.73367 8.274c.401.3799.4181 1.0128.0383 1.4137z" fill-rule="evenodd"></path>
                            </symbol>
                        </use>
                    </svg>
                </button>

                <div class="sidebar-block__inner" style="overflow-x: hidden;">

                    <div class="scrollbar__layout">
                        <div class="scrollbar__inner" style="width: 251px">
                            <div style="width: 233px;">
                                <ul class="checkbox-filter">
                                    
                                    <li class="checkbox-filter__item" onclick={onclick_4}>
                                        <a class="checkbox-filter__link">{"Весна"}</a>
                                    </li>

                                    <li class="checkbox-filter__item" onclick={onclick_5}>
                                        <a class="checkbox-filter__link">{"Зима"}</a>
                                    </li>

                                    <li class="checkbox-filter__item" onclick={onclick_6}>
                                        <a class="checkbox-filter__link">{"Літо"}</a>
                                    </li>

                                    <li class="checkbox-filter__item" onclick={onclick_7}>
                                        <a class="checkbox-filter__link">{"Осінь"}</a>
                                    </li>

                                </ul>
                            </div>
                        </div>

                    </div>
                </div>

            </div>


            // Sidebar block - Volume
            // Sidebar block - Seasonality

            <div class="sidebar-block">
                <button class="sidebar-block__toggle">
                    <span class="sidebar-block_toggle-title">
                        {"Ємність"}
                        // <span class="sidebar-block__toggle-quantity">{"1"}</span>
                    </span>

                    <svg class="sidebar-block__toggle-icon">
                        <use href="#icon-angle-left">
                            <symbol viewBox="0 0 24 24" id="icon-angle-left">
                                <path clip-rule="evenodd" d="m16.726 21.6877c-.3799.401-1.0128.4181-1.4137.0383l-10.26633-9.726 10.26633-9.72595c.4009-.37984 1.0338-.36273 1.4137.0382.3798.40094.3627 1.03387-.0383 1.4137l-8.73367 8.27405 8.73367 8.274c.401.3799.4181 1.0128.0383 1.4137z" fill-rule="evenodd"></path>
                            </symbol>
                        </use>
                    </svg>
                </button>

                <div class="sidebar-block__inner" style="overflow-x: hidden;">

                    <div class="scrollbar__layout">
                        <div class="scrollbar__inner" style="width: 251px">
                            <div style="width: 233px;">
                                <ul class="checkbox-filter">
                                    
                                    <li class="checkbox-filter__item" onclick={onclick_8}>
                                        <a class="checkbox-filter__link">{"100"}</a>
                                    </li>

                                    <li class="checkbox-filter__item" onclick={onclick_9}>
                                        <a class="checkbox-filter__link">{"20"}</a>
                                    </li>

                                    <li class="checkbox-filter__item" onclick={onclick_10}>
                                        <a class="checkbox-filter__link">{"30"}</a>
                                    </li>

                                </ul>
                            </div>
                        </div>

                    </div>
                </div>

            </div>


            // Sidebar block - Class
            <div class="sidebar-block">
                <button class="sidebar-block__toggle">
                    <span class="sidebar-block_toggle-title">
                        {"Клас"}
                        // <span class="sidebar-block__toggle-quantity">{"1"}</span>
                    </span>

                    <svg class="sidebar-block__toggle-icon">
                        <use href="#icon-angle-left">
                            <symbol viewBox="0 0 24 24" id="icon-angle-left">
                                <path clip-rule="evenodd" d="m16.726 21.6877c-.3799.401-1.0128.4181-1.4137.0383l-10.26633-9.726 10.26633-9.72595c.4009-.37984 1.0338-.36273 1.4137.0382.3798.40094.3627 1.03387-.0383 1.4137l-8.73367 8.27405 8.73367 8.274c.401.3799.4181 1.0128.0383 1.4137z" fill-rule="evenodd"></path>
                            </symbol>
                        </use>
                    </svg>
                </button>

                <div class="sidebar-block__inner" style="overflow-x: hidden;">

                    <div class="scrollbar__layout">
                        <div class="scrollbar__inner" style="width: 251px">
                            <div style="width: 233px;">
                                <ul class="checkbox-filter">
                                    
                                    <li class="checkbox-filter__item" onclick={onclick_11}>
                                        <a class="checkbox-filter__link">{"Мас-маркет"}</a>
                                    </li>

                                    <li class="checkbox-filter__item" onclick={onclick_12}>
                                        <a class="checkbox-filter__link">{"Елітні"}</a>
                                    </li>

                                    <li class="checkbox-filter__item" onclick={onclick_13}>
                                        <a class="checkbox-filter__link">{"Нішові"}</a>
                                    </li>

                                </ul>
                            </div>
                        </div>

                    </div>
                </div>

            </div>

        </aside>

    }
}