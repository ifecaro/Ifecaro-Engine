use dioxus::prelude::*;
use dioxus_i18n::t;

#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    pub label: String,
    pub value: String,
    pub on_change: EventHandler<String>,
    pub has_error: bool,
    pub r#type: Option<String>,
    pub placeholder: Option<String>,
}

pub fn Input(props: InputProps) -> Element {
    rsx! {
        div { class: "space-y-1",
            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                {props.label}
            }
            input {
                r#type: props.r#type.clone().unwrap_or_else(|| "text".to_string()),
                class: "w-full px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 dark:text-white",
                placeholder: props.placeholder.clone().unwrap_or_default(),
                value: props.value,
                onchange: move |e| props.on_change.call(e.value().to_string()),
            }
            if props.has_error {
                p { class: "mt-1 text-sm text-red-500",
                    {t!("required-field")}
                }
            }
        }
    }
} 