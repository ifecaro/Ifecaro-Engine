use dioxus::prelude::{dioxus_elements, rsx, Element, IntoDynNode, Props, component, dioxus_core, GlobalSignal, Readable};
use dioxus::hooks::use_context;
use dioxus::signals::Signal;
use crate::enums::translations::Translations;

#[derive(Props, Clone, PartialEq)]
pub struct TitleProps {
    title: &'static str
}

#[component]
pub fn Title(props: TitleProps) -> Element {
    let lang = use_context::<Signal<&str>>();
    let t = Translations::get(lang());

    let title = match props.title {
        "Dashboard" => t.dashboard,
        "Story" => t.story,
        _ => props.title,
    };

    rsx! {
        h1 { class: "text-5xl pt-4 pb-8 dark:text-white", "{title}" }
    }
}