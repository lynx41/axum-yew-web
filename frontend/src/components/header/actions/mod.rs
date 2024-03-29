mod language;
mod orders;
mod notifications;
mod cart;
mod user;


use language::Languages;
use orders::Orders;
use notifications::Notifications;
use cart::Cart;
use user::User;

use yew::{function_component, html, Html};


#[function_component(Actions)]
pub fn actions() -> Html {
    html! {
        <ul class="header-actions">
        
        // default language can be changed in /src/stores/language.rs
        // visible for everyone
        <Languages />


        // development in plans, but for now 
        // visible only for loggedls
        // <Help /> Only for non-auth users


        // visible only for non-logged
        <User />


        // development in plans, but for now 
        // visible only for logged
        // <Bonuses />


        // redirects to special page of all user's orders
        // visible only for logged
        <Orders />


        // visible only for logged
        <Notifications />


        // development in plans, but for now 
        // visible for everyone
        // <Comparison />

        // development in plans, but for now 
        // visible only for logged
        // <WishList />


        // visible for everyone
        <Cart />

        </ul>
    }
}