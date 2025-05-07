use dioxus::prelude::*;
use dioxus_i18n::t;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    pub r#type: String,
    pub on_click: EventHandler<()>,
    pub disabled: bool,
    pub children: Element,
    pub class: Option<String>,
}

pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            r#type: props.r#type,
            onclick: move |_| props.on_click.call(()),
            disabled: props.disabled,
            class: props.class.unwrap_or_else(|| "w-full px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-md focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500".to_string()),
            {props.children}
        }
    }
} 