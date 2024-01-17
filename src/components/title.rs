use dioxus::prelude::{dioxus_elements, rsx, Element, GlobalAttributes, IntoDynNode, Props, Scope};

#[derive(Props)]
pub struct TitleProps<'a> {
    title: &'a str,
}

#[allow(non_snake_case)]
pub fn Title<'a>(cx: Scope<'a, TitleProps<'a>>) -> Element {
    cx.render(rsx! {
        h1 { class: "text-5xl px-16 py-4", {cx.props.title} }
    })
}
