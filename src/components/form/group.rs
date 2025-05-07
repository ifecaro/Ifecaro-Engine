use dioxus::prelude::*;
use dioxus_i18n::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GroupProps {
    pub label: String,
    pub children: Element,
}

pub fn Group(props: GroupProps) -> Element {
    rsx! {
        div { class: "space-y-4",
            h3 { class: "text-lg font-medium text-gray-900 dark:text-white",
                {props.label}
            }
            div { class: "space-y-4",
                {props.children}
            }
        }
    }
} 