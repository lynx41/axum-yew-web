mod catalog_settings;

use catalog_settings::CatalogSettings;

use yew::{function_component, html, Html};
use crate::components::{
    footer::Footer,
    header::Header
};


#[function_component(CategoryPerfume)]
pub fn category_perfume() -> Html {
    html! {

        <>

        <Header />

        <div class="catalog">
            <div class="layout layout_wuth_sidebar">
            // Perfume category block
                <CatalogSettings />

            </div>
        </div>
        
        <Footer />

        </>

    }
}