use std::{fmt::format, ops::Deref};

use gloo::console::log;
use web_sys::{HtmlBaseElement, HtmlElement, HtmlHeadElement, HtmlInputElement};
use yew::{function_component, html, use_node_ref, use_state, use_state_eq, Callback, Event, Html, MouseEvent, TargetCast};

#[function_component(CatalogSettings)]
pub fn catalog_settings() -> Html {

    let order_types = [
        "Avg. Customer Review".to_owned(),
        "Price: Low to High".to_owned(),
        "Price: High to Low".to_owned(),
        "Newest Arrivals".to_owned(),
        "Best Sellers".to_owned(),
    ];

    // emit the variable from select html change
    let selected_order_type = use_state_eq(|| order_types[0].clone());
    let selected_node_ref = use_node_ref();

    let on_select_changed = {
        let selected_node_ref = selected_node_ref.clone();
        let selected_order_type = selected_order_type.clone();
        Callback::from(move |_| {
            if let Some(input) = selected_node_ref.cast::<HtmlInputElement>() {
                selected_order_type.set(input.value());
                // log!(input.value());
                log!(input.value());
            }
        })
    };

    html! {
        
        <div class="layout-settings">
            <div class="catalog-settings">
            
            // Filter button mobile

            // Selected filters mobile
            
            // Sort desktop
            <div class="catalog-settings__sorting">
                <select class="select-css" ref={selected_node_ref} onchange={on_select_changed}>
                    {
                        for order_types.iter().map(|name| {
                            html! { <option selected={&*selected_order_type == name}> { name }</option> }
                        })
                    }
                </select>
            </div>
            
            // <p>{&*selected_element.clone().deref()}</p>

            </div>
        </div>
    }
}