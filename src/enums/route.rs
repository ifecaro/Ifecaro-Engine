use dioxus::prelude::*;
use crate::{
    pages::{dashboard::Dashboard, page_not_found::PageNotFound, story::Story},
    Layout,
    Home,
};

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
    PageNotFound { route: Vec<String> }
}

impl Route {
    pub fn default_language() -> String {
        "zh-TW".to_string()
    }
}
