#[cfg(target_arch = "wasm32")]
use console_error_panic_hook;
#[cfg(target_arch = "wasm32")]
use tracing_wasm;

mod components;
mod enums;
mod i18n;
mod layout;
mod pages;
mod contexts;
mod constants;
mod models;
mod services;

use dioxus::prelude::*;
use crate::{
    enums::route::Route,
    contexts::{
        language_context::LanguageProvider,
        settings_context::SettingsContext,
        chapter_context::ChapterProvider,
        paragraph_context::ParagraphProvider,
        story_context::provide_story_context,
        toast_context::provide_toast_context,
    },
    components::toast::ToastContainer,
};

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        tracing_wasm::set_as_global_default();
    }
    launch(App);
}

#[component]
fn App() -> Element {
    provide_toast_context();
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
    provide_story_context();
    rsx! {
        {props.children}
    }
}

