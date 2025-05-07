use dioxus::prelude::*;
use dioxus_i18n::t;

#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {
    pub label: String,
    pub value: String,
    pub options: Vec<(String, String)>,
    pub on_change: EventHandler<String>,
    pub has_error: bool,
}

pub fn Select(props: SelectProps) -> Element {
    rsx! {
        div { class: "space-y-1",
            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                {props.label}
            }
            select {
                class: "w-full px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 dark:text-white",
                value: props.value.clone(),
                onchange: move |e| props.on_change.call(e.value().to_string()),
                for (value, label) in props.options.iter() {
                    option {
                        value: value.clone(),
                        {label.clone()}
                    }
                }
            }
            if props.has_error {
                p { class: "mt-1 text-sm text-red-500",
                    {t!("required-field")}
                }
            }
        }
    }
} 