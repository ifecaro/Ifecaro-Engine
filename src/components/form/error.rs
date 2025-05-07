use dioxus::prelude::*;
use dioxus_i18n::t;

#[derive(Props, Clone, PartialEq)]
pub struct ErrorProps {
    pub message: String,
}

pub fn Error(props: ErrorProps) -> Element {
    rsx! {
        p { class: "mt-1 text-sm text-red-500",
            {props.message}
        }
    }
} 