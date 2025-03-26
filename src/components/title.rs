use dioxus::prelude::*;
use dioxus::hooks::use_context;
use crate::contexts::language_context::LanguageState;
use crate::enums::translations::Translations;

#[derive(Props, Clone, PartialEq)]
pub struct TitleProps {
    title: &'static str
}

#[component]
pub fn Title(props: TitleProps) -> Element {
    let state = use_context::<Signal<LanguageState>>();
    let t = Translations::get(&state.read().current_language);

    let title = match props.title {
        "Dashboard" => t.dashboard,
        "Story" => t.story,
        _ => props.title,
    };

    rsx! {
        h1 { class: "text-5xl pt-4 pb-8 dark:text-white", "{title}" }
    }
}