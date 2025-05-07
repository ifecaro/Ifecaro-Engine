use dioxus::prelude::*;
use dioxus_i18n::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LabelProps {
    pub for_id: String,
    pub required: bool,
    pub children: Element,
}

pub fn Label(props: LabelProps) -> Element {
    rsx! {
        label {
            r#for: props.for_id,
            class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
            {props.children}
            if props.required {
                span { class: "text-red-500 ml-1", "*" }
            }
        }
    }
} 