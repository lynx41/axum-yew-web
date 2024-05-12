mod catalog_settings;
mod section;
mod filters;
pub mod item_page;

use std::ops::Deref;

use catalog_settings::CatalogSettings;

use gloo::{console::log, utils::window};
use yew::{function_component, html, use_state, Callback, Html};
use crate::components::{
    footer::Footer,
    header::Header
};

use section::Content;
use filters::Filters;

#[function_component(CategoryPerfume)]
pub fn category_perfume() -> Html {
    
    let href_query = use_state(|| None);
    let updated = use_state(|| false);

    let onchange = {
        let href_query = href_query.clone();
        let updated = updated.clone();
        Callback::from(move |href: Option<String>| {
            href_query.set(href);
            updated.set(true);
        })
    };

    let update_was_loaded = {
        let updated = updated.clone();

        Callback::from(move |b: bool| {
            if b {
                updated.set(!b);
                log!("WAS UPDATED");
            }
        })
    };

    html! {

        <>

        <Header />

        <div class="catalog">
            
            <div class="layout">
                <CatalogSettings />
            </div>

            <div class="layout layout_with_sidebar">
                
                // Section
                <Content 
                    href={href_query.deref().to_owned()}
                    updated={updated.deref().to_owned()}
                    on_update={update_was_loaded.clone()} />

                // Aside
                <Filters emit_href={onchange} />

            </div>

        </div>
        
        <Footer />

        </>

    }
}