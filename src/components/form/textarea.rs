use dioxus::prelude::*;
use dioxus_i18n::t;

#[derive(Props, Clone, PartialEq)]
pub struct TextareaProps {
    pub label: String,
    pub value: String,
    pub on_change: EventHandler<String>,
    pub has_error: bool,
    pub placeholder: Option<String>,
    pub rows: Option<u32>,
}

pub fn Textarea(props: TextareaProps) -> Element {
    rsx! {
        div { class: "space-y-1",
            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                {props.label}
            }
            textarea {
                class: "w-full px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 dark:text-white",
                placeholder: props.placeholder.clone().unwrap_or_default(),
                rows: props.rows.unwrap_or(3),
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