#[cfg(target_arch = "wasm32")]
use console_error_panic_hook;
#[cfg(target_arch = "wasm32")]
use tracing_wasm;
#[cfg(target_arch = "wasm32")]
use {log::Level, log::LevelFilter, log::Log, log::Metadata, log::Record, web_sys::console};

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
    contexts::{
        chapter_context::ChapterProvider, language_context::LanguageProvider,
        paragraph_context::ParagraphProvider, settings_context::SettingsContext,
        story_context::StoryContext,
    },
    enums::route::Route,
};
use dioxus::prelude::*;
use dioxus::web;
use dioxus::web::launch::launch_cfg;
use dioxus_toastr::{ToastProvider};

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

#[cfg(target_arch = "wasm32")]
struct DomLogger;

#[cfg(target_arch = "wasm32")]
static DOM_LOGGER: DomLogger = DomLogger;

#[cfg(target_arch = "wasm32")]
impl Log for DomLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let message = format!("[{}] {}", record.level(), record.args());

        log_dom!("{message}");

        match record.level() {
            Level::Error => console::error_1(&message.into()),
            Level::Warn => console::warn_1(&message.into()),
            Level::Info => console::info_1(&message.into()),
            _ => console::log_1(&message.into()),
        }
    }

    fn flush(&self) {}
}

#[cfg(target_arch = "wasm32")]
fn init_dom_logger() {
    let _ = log::set_logger(&DOM_LOGGER).map(|()| log::set_max_level(LevelFilter::Trace));
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        tracing_wasm::set_as_global_default();
        init_dom_logger();
    }

    // 這裡一定要指定 root id = "app-root"
    let cfg = web::Config::new().rootname("app-root");
    launch_cfg(App, cfg);
}

#[component]
fn App() -> Element {
    let _settings_context = use_context_provider(|| Signal::new(SettingsContext::default()));

    #[cfg(target_arch = "wasm32")]
    {
        rsx! {
            ToastProvider {
                LanguageProvider {
                    ChapterProvider {
                        ParagraphProvider {
                            StoryProvider {
                                Router::<Route> {}
                            }
                        }
                    }
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        rsx! {
            div {
                class: "min-h-screen flex items-center justify-center bg-gray-100 text-gray-900 dark:bg-gray-900 dark:text-gray-100",
                "Ifecaro is intended to run in a WebAssembly target."
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
