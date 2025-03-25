use dioxus::prelude::*;
use dioxus::events::{FormEvent, FocusEvent};

#[component]
pub fn TextareaField(
    label: &'static str,
    placeholder: &'static str,
    value: String,
    required: bool,
    has_error: bool,
    rows: u32,
    on_input: EventHandler<FormEvent>,
    on_blur: EventHandler<FocusEvent>,
) -> Element {
    rsx! {
        div { 
            class: "mb-6",
            {(!label.is_empty()).then(|| rsx!(
                label { 
                    class: "block text-gray-700 dark:text-gray-300 text-sm font-bold mb-2",
                    span { "{label}" }
                    {required.then(|| rsx!(
                        span { class: "text-red-500 ml-1", "*" }
                    ))}
                }
            ))}
            textarea {
                class: {
                    let base_classes = "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-gray-300 dark:bg-gray-700 dark:border-gray-600 leading-tight focus:outline-none focus:shadow-outline dark:focus:border-gray-500";
                    if has_error {
                        format!("{} border-red-500", base_classes)
                    } else {
                        base_classes.to_string()
                    }
                },
                required: required,
                placeholder: "{placeholder}",
                rows: "{rows}",
                value: "{value}",
                onblur: move |evt| on_blur.call(evt),
                oninput: move |evt| on_input.call(evt)
            }
            {has_error.then(|| rsx!(
                div { 
                    class: "text-red-500 text-sm mt-1",
                    "請填寫此欄位"
                }
            ))}
        }
    }
} 