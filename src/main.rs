#[cfg(target_arch = "wasm32")]
use console_error_panic_hook;
#[cfg(target_arch = "wasm32")]
use tracing_wasm;

mod components;
mod constants;
mod contexts;
mod enums;
mod hooks;
mod i18n;
mod layout;
mod models;
mod pages;
mod services;
mod utils;

use crate::{
    components::toast::ToastContainer,
    contexts::{
        chapter_context::ChapterProvider, language_context::LanguageProvider,
        paragraph_context::ParagraphProvider, settings_context::SettingsContext,
        story_context::StoryContext, toast_context::ToastManager,
    },
    enums::route::Route,
};
use dioxus::prelude::*;
use dioxus::web;
use dioxus::web::launch::launch_cfg;

#[cfg(target_arch = "wasm32")]
fn append_log_line(msg: &str) {
    use web_sys::window;

    let Some(window) = window() else { return };
    let Some(document) = window.document() else { return };
    let Some(el) = document.get_element_by_id("debug-log") else { return };

    // 很土法，但簡單：舊內容 + <br> + 新的一行文字
    let current = el.inner_html();
    let new_html = if current.is_empty() || current == "(DOM log)" {
        format!("{msg}")
    } else {
        format!("{current}<br>{msg}")
    };

    el.set_inner_html(&new_html);
}

// 小小 macro，方便在任何地方呼叫
macro_rules! log_dom {
    ($($t:tt)*) => {{
        #[cfg(target_arch = "wasm32")]
        {
            append_log_line(&format!($($t)*));
        }
    }};
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        log_dom!("✅ WASM main started");
        console_error_panic_hook::set_once();
        tracing_wasm::set_as_global_default();
        wasm_logger::init(wasm_logger::Config::default());
        log_dom!("✅ WASM main started 2");
    }

    // 這裡一定要指定 root id = "app-root"
    let cfg = web::Config::new().rootname("app-root");
    launch_cfg(App, cfg);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(ToastManager::new()));
    provide_context(Signal::new(SettingsContext::default()));

    #[cfg(target_arch = "wasm32")]
    log_dom!("✅ App component entered");

    rsx! {
        LanguageProvider {
            ChapterProvider {
                ParagraphProvider {
                    StoryProvider {
                        Router::<Route> {}
                        ToastContainer {}
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct StoryProviderProps {
    children: Element,
}

#[component]
fn StoryProvider(props: StoryProviderProps) -> Element {
    use_context_provider(|| Signal::new(StoryContext::new()));
    rsx! {
        {props.children}
    }
}
