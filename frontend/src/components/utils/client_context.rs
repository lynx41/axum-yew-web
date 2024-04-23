use std::{rc::Rc, sync::Arc};

use yew::UseStateHandle;

use crate::components::props::IsAuth;

#[derive(Clone, Debug, PartialEq)]
pub struct ClientContext {
    pub selected_language: UseStateHandle<String>,
    pub supported_languages: Vec<&'static str>,
    pub is_auth: UseStateHandle<IsAuth>
}

pub struct UtilsContext {
    pub modal_auth_display: UseStateHandle<bool>,
}