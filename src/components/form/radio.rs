use dioxus::prelude::*;
use dioxus_i18n::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct RadioProps {
    pub label: String,
    pub value: String,
    pub checked: bool,
    pub on_change: EventHandler<String>,
    pub disabled: bool,
}

pub fn Radio(props: RadioProps) -> Element {
    rsx! {
        div { class: "flex items-center",
            input {
                r#type: "radio",
                value: props.value,
                checked: props.checked,
                onchange: move |e| props.on_change.call(e.value().to_string()),
                disabled: props.disabled,
                class: "h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300",
            }
            label { class: "ml-2 block text-sm text-gray-900 dark:text-gray-300",
                {props.label}
            }
        }
    }
} 