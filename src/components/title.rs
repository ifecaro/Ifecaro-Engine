use dioxus::prelude::{dioxus_elements, rsx, Element, IntoDynNode, Props, component, dioxus_core, GlobalSignal, Readable};

#[derive(Props, Clone, PartialEq)]
pub struct TitleProps {
    title: &'static str
}

#[component]
pub fn Title(props:TitleProps) -> Element {
    rsx! {
        h1 { class: "text-5xl pt-4 pb-8 dark:text-white", {props.title} }
    }
}