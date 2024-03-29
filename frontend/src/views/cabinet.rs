use yew::{function_component, Html, html};

use crate::{components::{footer::Footer, header::Header}, routes::Props};

#[function_component(Cabinet)]
pub fn cabinet(props: &Props) -> Html {
    html! {
        <>
            <Header />
            
            <p>{"Cabinet page"}</p> 
            
            <Footer 
                selected_language={props.selected_language.clone()}
                supported_languages={props.supported_languages.clone()} 
            />
        </>
    }
}