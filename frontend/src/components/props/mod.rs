use yew::{Callback, MouseEvent, Properties};


#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub selected_language: String,
    pub supported_languages: Vec<&'static str>,
    pub is_auth: IsAuth,
}

#[derive(Properties, PartialEq)]
pub struct FooterProps {
    pub selected_language: String,
    pub supported_languages: Vec<&'static str>,
}

#[derive(Properties, PartialEq)]
pub struct HeaderActions {
    pub selected_language: String,
    pub supported_languages: Vec<&'static str>,
    pub user_btn_onclick: Callback<MouseEvent>,
    pub is_auth: IsAuth,
}

#[derive(Properties, PartialEq)]
pub struct SelectedLangauge {
    pub selected_language: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct PageProps {
    pub selected_language: String,
    pub supported_languages: Vec<&'static str>,
    pub is_auth: IsAuth,
}

#[derive(PartialEq, Clone, Debug, Default)]
pub enum IsAuth {
    #[default]
    Unknown,
    Yes,
    No,
}

impl std::fmt::Display for IsAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::No => write!(f, "No"),
            Self::Yes => write!(f, "Yes")
        }
    }
}