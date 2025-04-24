use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TitleProps {
    title: Option<&'static str>,
    children: Element,
}

#[component]
pub fn Layout(props: TitleProps) -> Element {
    rsx! {
        div { 
            class: "min-h-screen bg-fixed bg-cover bg-center pt-4 sm:pt-8 lg:pt-16 px-2 sm:px-6 md:px-12 lg:px-24",
            if let Some(title) = props.title {
                crate::components::title::Title { title }
            }
            {props.children}
        }
    }
}
