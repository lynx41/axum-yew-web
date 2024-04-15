// use std::fmt;

// use gloo::storage::LocalStorage;
// use gloo::storage::Storage;
// use serde::{Deserialize, Serialize};
// use strum::EnumIter;
// use strum::IntoEnumIterator;
// use yewdux::prelude::*;

use gloo::storage::{LocalStorage, Storage};

pub const LANGUAGE_KEY: &str = "Language";
pub const DEFAULT_LANGUAGE: &str = LANG_ENG;
pub const LANG_UA: &str = "Ua";
pub const LANG_ENG: &str = "Eng";
const LANGUAGES_LIST: [&'static str; 2] = [ LANG_UA, LANG_ENG ];

pub fn get_supported_languages() -> Vec<&'static str> {
    LANGUAGES_LIST.to_vec()
}


pub fn get_selected_langauge() -> String {
    match LocalStorage::get(LANGUAGE_KEY) {
        Ok(lang) => lang,
        Err(_) => {
            // on first user visit
            let default_lang = LANG_ENG;
            
            let _ = LocalStorage::set(LANGUAGE_KEY, default_lang);
            default_lang.to_owned()
        }
    }
}





// pub const LANGUAGE_KEY: &str = "Language";

// #[derive(Clone, Serialize, Deserialize, Store, PartialEq, Eq, Debug, Default, EnumIter)]
// #[store(storage = "local", storage_tab_sync)]
// // NAME NEW LANGUAGES as svg images in GIT_REPO/public/system/images/locales/
// pub enum LangSelector {
//     #[default]
//     UA,
//     ENG,
// }

// impl LangSelector {
//     pub fn as_str(&self) -> &'static str {
//         match self {
//             LangSelector::ENG => "ENG",
//             LangSelector::UA => "UA",
//         }
//     }
// }

// impl fmt::Display for LangSelector {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// pub fn get_selected_langauge() -> String {
//     match LocalStorage::get::<LangSelector>(LANGUAGE_KEY) {
//         Ok(language) => language.to_string(),
//         Err(_) => {
//             let _ = LocalStorage::set(LANGUAGE_KEY, LangSelector::default());
//             LangSelector::default().to_string()
//         }
//     }
// }

// pub fn get_supported_languages() -> Vec<&'static str> {
//     LangSelector::iter()
//         .map(|language| language.as_str())
//         .collect()
// }