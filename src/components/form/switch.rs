use dioxus::prelude::*;
use dioxus_i18n::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SwitchProps {
    pub label: String,
    pub checked: bool,
    pub on_change: EventHandler<bool>,
    pub disabled: bool,
}

pub fn Switch(props: SwitchProps) -> Element {
    rsx! {
        div { class: "flex items-center",
            button {
                r#type: "button",
                role: "switch",
                "aria-checked": props.checked,
                onclick: move |_| props.on_change.call(!props.checked),
                disabled: props.disabled,
                class: format!(
                    "relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 {}",
                    if props.checked {
                        "bg-blue-600"
                    } else {
                        "bg-gray-200 dark:bg-gray-700"
                    }
                ),
                span {
                    "aria-hidden": "true",
                    class: format!(
                        "pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {}",
                        if props.checked {
                            "translate-x-5"
                        } else {
                            "translate-x-0"
                        }
                    ),
                }
            }
            label { class: "ml-3 block text-sm text-gray-900 dark:text-gray-300",
                {props.label}
            }
        }
    }
} 