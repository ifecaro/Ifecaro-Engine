use dioxus::prelude::{component, dioxus_core, Element, Props, rsx, IntoDynNode, dioxus_elements, fc_to_builder, GlobalSignal, Readable};

#[derive(Props, Clone, PartialEq)]
pub struct TitleProps {
    title: Option<&'static str>,
    children: Element,
}

#[component]
pub fn Layout(props: TitleProps) -> Element {
    rsx! {
        div { class: "h-screen bg-cover bg-center pt-16 px-16",
            if let Some(title) = props.title {
                crate::components::title::Title { title }
            }
            {props.children}
        }
    }
}
