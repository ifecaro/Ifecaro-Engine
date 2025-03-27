use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InputFieldProps {
    label: String,
    placeholder: String,
    value: String,
    required: bool,
    has_error: bool,
    on_input: EventHandler<String>,
    on_blur: EventHandler<()>,
}

#[component]
pub fn InputField(props: InputFieldProps) -> Element {
    let error_class = if props.has_error { "border-red-500" } else { "" };
    
    rsx! {
        div { class: "space-y-2",
            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                "{props.label}"
            }
            input {
                class: "block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-green-500 dark:bg-gray-700 dark:text-white {error_class}",
                r#type: "text",
                placeholder: "{props.placeholder}",
                required: props.required,
                value: "{props.value}",
                oninput: move |evt| props.on_input.call(evt.value().to_string()),
                onblur: move |_| props.on_blur.call(())
            }
            {props.has_error.then(|| {
                rsx! {
                    p { class: "text-sm text-red-500", "此欄位為必填" }
                }
            })}
        }
    }
} 