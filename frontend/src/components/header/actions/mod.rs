mod language;
mod orders;
mod notifications;
mod cart;
mod user;


use std::{borrow::BorrowMut, ops::Deref, rc::Rc};

use gloo::{net::http::Request, storage::{LocalStorage, Storage}, console::log};
use language::Languages;
use orders::Orders;
use notifications::Notifications;
use cart::Cart;
use user::User;

use yew::{function_component, html, use_context, use_effect_with, use_mut_ref, use_state, Callback, Html, MouseEvent, Properties};

use crate::components::{props::{HeaderActions, IsAuth}, utils::client_context::ClientContext};


#[function_component(Actions)]
// pub fn actions(props: &HeaderActions) -> Html {
pub fn actions() -> Html {

    let client_context = use_context::<Rc<ClientContext>>().unwrap();

    html! {
        <ul class="header-actions">
        
        // default language can be changed in /src/stores/language.rs
        // visible for everyone
        <Languages />


        // development in plans, but for now 
        // visible only for loggedls
        // <Help /> Only for non-auth users


        // can be viewed only by GUESTS
        if {*client_context.is_auth.deref() == IsAuth::No} {
            <User />
        }


        // development in plans, but for now 
        // visible only for AUTHORIZED
        // <Bonuses />


        // redirects to special page of all user's orders
        // visible only for AUTHORIZED
        if {*client_context.is_auth.deref() == IsAuth::Yes} {
            <Orders />
        }


        // visible only for AUTHORIZED
        if {*client_context.is_auth.deref() == IsAuth::Yes} {
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