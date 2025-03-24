use dioxus::prelude::{dioxus_elements, rsx, Element, IntoDynNode, Props, component, dioxus_core, GlobalSignal, Readable};
use dioxus::hooks::use_context;
use dioxus::signals::Signal;

#[derive(Props, Clone, PartialEq)]
pub struct TitleProps {
    title: &'static str
}

#[derive(Clone)]
struct TitleTranslations {
    dashboard: &'static str,
    story: &'static str,
}

impl TitleTranslations {
    fn get(lang: &str) -> Self {
        match lang {
            "en" => Self {
                dashboard: "Dashboard",
                story: "Story",
            },
            "zh-TW" => Self {
                dashboard: "儀表板",
                story: "故事",
            },
            _ => Self::get("en"),
        }
    }
}

#[component]
pub fn Title(props: TitleProps) -> Element {
    let lang = use_context::<Signal<&str>>();
    let t = TitleTranslations::get(lang());

    let title = match props.title {
        "Dashboard" => t.dashboard,
        "Story" => t.story,
        _ => props.title,
    };

    rsx! {
        h1 { class: "text-5xl pt-4 pb-8 dark:text-white", "{title}" }
    }
}