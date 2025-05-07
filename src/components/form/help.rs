use dioxus::prelude::*;
use dioxus_i18n::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HelpProps {
    pub message: String,
}

pub fn Help(props: HelpProps) -> Element {
    rsx! {
        p { class: "mt-1 text-sm text-gray-500 dark:text-gray-400",
            {props.message}
        }
    }
} 