use std::fmt;
use std::default;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use strum::EnumIter;
use yewdux::prelude::*;
use yewdux::store;


#[derive(Clone, Serialize, Deserialize, Store, PartialEq, Eq, Debug, Default, EnumIter)]
#[store(storage = "local", storage_tab_sync)]
// NAME NEW LANGUAGES as svg images in GIT_REPO/public/system/images/locales/
pub enum LangSelector {
    #[default]
    UA,
    ENG,
}

impl LangSelector {
    pub fn as_str(&self) -> &'static str {
        match self {
            LangSelector::ENG => "ENG",
            LangSelector::UA => "UA",
        }
    }
}

impl fmt::Display for LangSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


// #[derive(Clone, Serialize, Deserialize, Store, PartialEq, Eq, Debug, Default)]
// // #[store(storage = "local", storage_tab_sync, listener(LogListener))]
// #[store(storage = "local", storage_tab_sync)]
// pub struct Language {
//     pub language: LangSelector,
// }

// struct LogListener;

// impl Listener for LogListener {
//     type Store = Language;

//     fn on_change(&mut self, _cx: &yewdux::Context, state: Rc<Self::Store>) {
//         yewdux::log::info!("Language changed to {}", state.language);
//     }
// }
