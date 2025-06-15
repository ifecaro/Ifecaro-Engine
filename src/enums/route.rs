use dioxus::prelude::*;
use crate::{
    pages::{dashboard::Dashboard, page_not_found::PageNotFound, story::Story, home::Home},
    layout::Layout,
};
use web_sys::window;
use smallvec::SmallVec;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    
    #[route("/:lang")]
    Story { lang: String },
    
    #[route("/:lang/dashboard")]
    Dashboard { lang: String },
    
    #[route("/:..route")]
    PageNotFound { route: SmallVec<[String; 8]> }
}

impl Route {
    pub fn default_language() -> String {
        // Try to get browser language
        if let Some(window) = window() {
            let navigator = window.navigator();
            
            // Use navigator.language property to get browser language
            if let Some(language) = navigator.language() {
                // Check if it's in the list of supported languages
                let supported_languages = ["zh-TW", "zh-CN", "en-US", "en-GB", "es-ES", "es-CL"];
                
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
