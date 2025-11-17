use dioxus::events::{FocusEvent, FormEvent};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn TextareaField(
    label: &'static str,
    placeholder: &'static str,
    value: String,
    required: bool,
    has_error: bool,
    rows: u32,
    auto_resize: Option<bool>,
    on_input: EventHandler<FormEvent>,
    on_blur: EventHandler<FocusEvent>,
) -> Element {
    let auto_resize = auto_resize.unwrap_or(false);
    let line_count = if auto_resize {
        // Calculate line count, considering line breaks and extra buffer space
        let lines = value.lines().count().max(1);
        // Add one buffer line to avoid content being cut off
        (lines + 1).max(rows as usize) as u32
    } else {
        rows
    };

    rsx! {
        div {
            class: "mb-6",
            {(!label.is_empty()).then(|| rsx!(
                label {
                    class: "block text-gray-700 dark:text-gray-300 text-sm font-medium mb-2",
                    span { "{label}" }
                    {required.then(|| rsx!(
                        span { class: "text-red-500 ml-1", "*" }
                    ))}
                }
            ))}
            textarea {
                class: {
                    let mut base_classes = "shadow appearance-none border rounded-lg w-full py-2.5 px-4 text-sm text-gray-700 dark:text-gray-300 dark:bg-gray-700 dark:border-gray-600 leading-tight focus:outline-none focus:shadow-outline dark:focus:border-gray-500".to_string();

                    if auto_resize {
                        // Remove overflow-hidden, only keep resize-none
                        base_classes.push_str(" resize-none");
                    }

                    if has_error {
                        format!("{} border-red-500", base_classes)
                    } else {
                        base_classes
                    }
                },
                required: required,
                placeholder: "{placeholder}",
                rows: line_count,
                value: "{value}",
                onblur: move |evt| on_blur.call(evt),
                oninput: move |evt| on_input.call(evt)
            }
            {has_error.then(|| rsx!(
                div {
                    class: "text-red-500 text-sm mt-1",
                    "{t!(\"please_fill_in_this_field\")}"
                }
            ))}
        }
    }
}
