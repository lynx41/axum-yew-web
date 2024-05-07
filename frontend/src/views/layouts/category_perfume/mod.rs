mod catalog_settings;
mod section;

use catalog_settings::CatalogSettings;

use yew::{function_component, html, Html};
use crate::components::{
    footer::Footer,
    header::Header
};

use section::Content;

#[function_component(CategoryPerfume)]
pub fn category_perfume() -> Html {
    html! {

        <>

        <Header />

        <div class="catalog">
            
            <div class="layout">
                <CatalogSettings />
            </div>

            <div class="layout layout_with_sidebar">
                
                // Section
                <Content />


                // Aside
            
            </div>

        </div>
        
        <Footer />

        </>

    }
}