use dioxus::prelude::*;
use dioxus_i18n::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DropdownProps<T: Clone + PartialEq> {
    pub label: String,
    pub value: String,
    pub options: Vec<T>,
    pub is_open: bool,
    pub search_query: String,
    pub on_toggle: EventHandler<()>,
    pub on_search: EventHandler<String>,
    pub on_select: EventHandler<T>,
    pub display_fn: fn(&T) -> String,
    pub has_error: bool,
    pub class: String,
    pub search_placeholder: String,
    pub button_class: Option<String>,
    pub label_class: Option<String>,
    pub dropdown_class: String,
    pub search_input_class: String,
    pub option_class: String,
    pub disabled: bool,
    pub required: bool,
}

pub fn Dropdown<T: Clone + PartialEq + 'static>(props: DropdownProps<T>) -> Element {
    let t = use_translation();

    rsx! {
        div { class: "space-y-1",
            label { class: props.label_class.unwrap_or_else(|| "block text-sm font-medium text-gray-700 dark:text-gray-300".to_string()),
                {props.label}
            }
            div { class: "relative",
                button {
                    r#type: "button",
                    class: props.button_class.unwrap_or_else(|| "w-full px-3 py-2 text-left text-sm border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 dark:text-white".to_string()),
                    onclick: move |_| props.on_toggle.call(()),
                    disabled: props.disabled,
                    {props.value}
                }
                if props.is_open {
                    div { class: format!("absolute z-10 w-full mt-1 bg-white dark:bg-gray-700 rounded-md shadow-lg {}", props.dropdown_class),
                        div { class: "p-2",
                            input {
                                r#type: "text",
                                class: format!("w-full px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 dark:text-white {}", props.search_input_class),
                                placeholder: props.search_placeholder,
                                value: props.search_query,
                                onchange: move |e| props.on_search.call(e.value.clone()),
                            }
                        }
                        div { class: "max-h-60 overflow-auto",
                            for option in props.options.iter() {
                                button {
                                    class: format!("w-full px-3 py-2 text-left text-sm hover:bg-gray-100 dark:hover:bg-gray-600 {}", props.option_class),
                                    onclick: move |_| props.on_select.call(option.clone()),
                                    {(props.display_fn)(option)}
                                }
                            }
                        }
                    }
                }
            }
            if props.has_error {
                p { class: "mt-1 text-sm text-red-500",
                    t!("required-field")
                }
            }
        }
    }
} 