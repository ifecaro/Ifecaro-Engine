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

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        tracing_wasm::set_as_global_default();
        wasm_logger::init(wasm_logger::Config::default());
    }
    launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(ToastManager::new()));
    provide_context(Signal::new(SettingsContext::default()));
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
