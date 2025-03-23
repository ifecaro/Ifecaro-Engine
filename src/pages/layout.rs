use dioxus::prelude::{component, dioxus_core, Element, Props, rsx, IntoDynNode, dioxus_elements, fc_to_builder, GlobalSignal, Readable};

#[derive(Props, Clone, PartialEq)]
pub struct TitleProps {
    title: Option<String>,
    children: Element,
}

#[component]
pub fn Layout(props: TitleProps) -> Element {
    rsx! {
        div { class: "h-screen bg-cover bg-center pt-16 px-16",
            if props.title.is_some() {
                {
                    rsx! {
                        crate::components::title::Title { title: props.title.unwrap() }
                    }
                }
            }
            {props.children}
        }
    }
}
