use dioxus::prelude::*;
use dioxus_i18n::t;

#[derive(Props, Clone, PartialEq)]
pub struct TitleProps {
    title: &'static str,
}

#[component]
pub fn Title(props: TitleProps) -> Element {
    let title = match props.title {
        "Dashboard" => t!("dashboard"),
        "Story" => t!("story"),
        _ => props.title.to_string(),
    };

    rsx! {
        h1 { class: "text-5xl pt-4 pb-8 dark:text-white", "{title}" }
    }
}
