use crate::views::{
    home::Home,
    profile::Profile,
    auth::Auth,
    not_found::NotFound
};

use yew::{Html, html};
use yew_router::Routable;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/profile")]
    Profile,
    #[at("/auth")]
    Auth,
    #[not_found]
    #[at("/404")]
    NotFound
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Profile => html! { <Profile /> },
        Route::Auth => html! { <Auth /> },
        Route::NotFound => html! { <NotFound /> }
    }
}