use dioxus::prelude::*;
use crate::{
    pages::{dashboard::Dashboard, page_not_found::PageNotFound, story::Story},
    Layout,
    Home,
};
use web_sys::window;

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
        // 嘗試獲取瀏覽器語言
        if let Some(window) = window() {
            let navigator = window.navigator();
            
            // 使用 navigator.language 屬性獲取瀏覽器語言
            if let Some(language) = navigator.language() {
                // 檢查是否在支持的語言列表中
                let supported_languages = ["zh-TW", "zh-CN", "en-US", "en-GB", "es-ES", "es-CL"];
                
                // 檢查完整語言代碼
                if supported_languages.contains(&language.as_str()) {
                    return language;
                }
                
                // 檢查語言代碼前綴
                for lang in supported_languages.iter() {
                    if language.starts_with(&lang[..2]) {
                        return lang.to_string();
                    }
                }
            }
        }
        
        // 如果無法獲取瀏覽器語言或不在支持列表中，默認為英語
        "en-US".to_string()
    }
}
