use yew::{Html, html, function_component};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! { 
        <p>{"404"}</p>
     }
}