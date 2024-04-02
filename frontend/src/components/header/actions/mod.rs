mod language;
mod orders;
mod notifications;
mod cart;
mod user;


use std::{borrow::BorrowMut, ops::Deref};

use gloo::{net::http::Request, storage::{LocalStorage, Storage}, console::log};
use language::Languages;
use orders::Orders;
use notifications::Notifications;
use cart::Cart;
use user::User;

use yew::{function_component, html, use_effect_with, use_mut_ref, use_state, Html};

use crate::components::props::{HeaderProps, IsAuth};


#[function_component(Actions)]
pub fn actions(props: &HeaderProps) -> Html {

    html! {
        <ul class="header-actions">
        
        // default language can be changed in /src/stores/language.rs
        // visible for everyone
        <Languages />


        // development in plans, but for now 
        // visible only for loggedls
        // <Help /> Only for non-auth users


        // can be viewed only by GUESTS
        if {props.is_auth == IsAuth::No} {
            <User />
        }


        // development in plans, but for now 
        // visible only for AUTHORIZED
        // <Bonuses />


        // redirects to special page of all user's orders
        // visible only for AUTHORIZED
        if {props.is_auth == IsAuth::Yes} {
            <Orders />
        }


        // visible only for AUTHORIZED
        if {props.is_auth == IsAuth::Yes} {
            <Notifications />
        }

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