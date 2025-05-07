use dioxus::prelude::*;
use dioxus_i18n::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CheckboxProps {
    pub label: String,
    pub checked: bool,
    pub on_change: EventHandler<bool>,
    pub disabled: bool,
}

pub fn Checkbox(props: CheckboxProps) -> Element {
    rsx! {
        div { class: "flex items-center",
            input {
                r#type: "checkbox",
                checked: props.checked,
                onchange: move |e| props.on_change.call(e.value().parse().unwrap_or(false)),
                disabled: props.disabled,
                class: "h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded",
            }
            label { class: "ml-2 block text-sm text-gray-900 dark:text-gray-300",
                {props.label}
            }
        }
    }
} 