use dioxus::prelude::{
    dioxus_elements, fc_to_builder, rsx, Element, GlobalAttributes, IntoDynNode, Props, Scope,
};

#[derive(Props)]
pub struct TitleProps<'a> {
    title: Option<&'a str>,
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Layout<'a>(cx: Scope<'a, TitleProps<'a>>) -> Element {
    cx.render(rsx! {

        div { class: "h-screen bg-cover bg-center pt-16 px-16",
            if cx.props.title.is_some() {
                rsx!(crate::components::title::Title { title: cx.props.title.unwrap() })
            }
            {&cx.props.children}
        }
    })
}
