use crate::{
    layout::Layout,
    pages::{
        auth::{InviteCheckEmail, InviteRequest, Login, Register},
        dashboard::Dashboard,
        home::Home,
        page_not_found::PageNotFound,
        story::Story,
    },
};
use dioxus::prelude::*;
use smallvec::SmallVec;
use web_sys::window;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},

    #[route("/:lang")]
    #[route("/staging/:lang")]
    Story { lang: String },

    #[route("/:lang/dashboard")]
    Dashboard { lang: String },

    #[route("/:lang/invite")]
    InviteRequest { lang: String },

    #[route("/:lang/invite/check-email")]
    InviteCheckEmail { lang: String },

    #[route("/:lang/register")]
    Register { lang: String },

    #[route("/:lang/login")]
    Login { lang: String },

    #[route("/:..route")]
    PageNotFound { route: SmallVec<[String; 8]> },
}

impl Route {
    pub fn canonical_language(lang: &str) -> String {
        crate::i18n::canonical_language_code(lang).to_string()
    }

    pub fn with_canonical_language(&self) -> Option<Self> {
        match self {
            Route::Story { lang } => {
                let canonical = Self::canonical_language(lang);
                (canonical != *lang).then_some(Route::Story { lang: canonical })
            }
            Route::Dashboard { lang } => {
                let canonical = Self::canonical_language(lang);
                (canonical != *lang).then_some(Route::Dashboard { lang: canonical })
            }
            Route::InviteRequest { lang } => {
                let canonical = Self::canonical_language(lang);
                (canonical != *lang).then_some(Route::InviteRequest { lang: canonical })
            }
            Route::InviteCheckEmail { lang } => {
                let canonical = Self::canonical_language(lang);
                (canonical != *lang).then_some(Route::InviteCheckEmail { lang: canonical })
            }
            Route::Register { lang } => {
                let canonical = Self::canonical_language(lang);
                (canonical != *lang).then_some(Route::Register { lang: canonical })
            }
            Route::Login { lang } => {
                let canonical = Self::canonical_language(lang);
                (canonical != *lang).then_some(Route::Login { lang: canonical })
            }
            Route::Home {} | Route::PageNotFound { .. } => None,
        }
    }

    pub fn default_language() -> String {
        // Try to get browser language
        if let Some(window) = window() {
            let navigator = window.navigator();

            // Use navigator.language property to get browser language
            if let Some(language) = navigator.language() {
                let canonical_language = Self::canonical_language(&language);
                if canonical_language != language {
                    return canonical_language;
                }

                // Check if it's in the list of supported languages
                let supported_languages = crate::components::language_selector::AVAILABLE_LANGUAGES
                    .iter()
                    .map(|lang| lang.code)
                    .collect::<Vec<_>>();

                // Check complete language code
                if supported_languages.contains(&language.as_str()) {
                    return language;
                }

                // Check language code prefix
                for lang in supported_languages.iter() {
                    if language.starts_with(&lang[..2]) {
                        return lang.to_string();
                    }
                }
            }
        }

        // If unable to get browser language or not in supported list, default to English
        "en-US".to_string()
    }
}
