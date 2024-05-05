use crate::views::{
    cabinet::Cabinet,
    home::Home,
    auth_guard::AuthGuard,
    layouts::category_perfume::CategoryPerfume,
};

use yew::{html, Html};
use yew_router::{components::Redirect, Routable};


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/cabinet")]
    Cabinet,
    #[at("/perfume")]
    CategoryParfume,
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
            <AuthGuard <Cabinet> />
        },

        Route::CategoryParfume => html! {
            <CategoryPerfume />
        },

        Route::NotFound => html! { 
            // If 404
            <Redirect<Route> to={Route::Home} />
        }
    }
}