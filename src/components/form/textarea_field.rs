use dioxus::prelude::*;

#[component]
pub fn TextareaField(
    label: String,
    placeholder: String,
    value: String,
    required: bool,
    has_error: bool,
    rows: u32,
    on_input: EventHandler<FormEvent>,
    on_blur: EventHandler<FocusEvent>,
) -> Element {
    rsx! {
        div { class: "mb-6",
            label { class: "block text-gray-700 text-sm font-bold mb-2",
                span { "{label}" }
                {required.then(|| rsx!(
                    span { class: "text-red-500 ml-1", "*" }
                ))}
            }
            textarea {
                class: {
                    let mut classes = "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline".to_string();
                    if has_error {
                        classes.push_str(" border-red-500");
                    }
                    classes
                },
                required: required,
                rows: "{rows}",
                placeholder: "{placeholder}",
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