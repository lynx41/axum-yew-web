use crate::views::{
    home::Home,
    cabinet::Cabinet,
};

use crate::stores::language::{get_selected_langauge, get_supported_languages};

use yew::{html, Html};
use yew_router::{components::Redirect, Routable};


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/cabinet")]
    Cabinet,
    // #[at("/auth")]
    // Auth,
    #[not_found]
    #[at("/404")]
    NotFound
}

pub fn switch(route: Route) -> Html {
    
    match route {
        Route::Home => html! {
            // Free access
            <Home />
        },

        Route::Cabinet => html! {
            // Login required
            <Cabinet />
        },

        // If 404
        Route::NotFound => html! { 
            <Redirect<Route> to={Route::Home} />
        }
    }
}