use crate::views::{
    home::Home,
    cabinet::Cabinet,
    auth::Auth,
    not_found::NotFound
};

use yew::{Html, html};
use yew_router::Routable;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/cabinet")]
    Cabinet,
    #[at("/auth")]
    Auth,
    #[not_found]
    #[at("/404")]
    NotFound
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Cabinet => html! { <Cabinet /> },
        Route::Auth => html! { <Auth /> },
        Route::NotFound => html! { <NotFound /> }
    }
}