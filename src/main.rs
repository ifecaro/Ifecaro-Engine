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
    },
};

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    provide_context(Signal::new(SettingsContext::default()));
    rsx! {
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

