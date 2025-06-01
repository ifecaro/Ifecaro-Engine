use dioxus::prelude::*;
use dioxus_i18n::t;

#[derive(Props, Clone, PartialEq)]
pub struct InputFieldProps {
    label: String,
    placeholder: String,
    value: String,
    required: bool,
    has_error: bool,
    on_input: EventHandler<String>,
    on_blur: EventHandler<()>,
    children: Option<Element>,
    #[props(default = false)]
    disabled: bool,
}

#[component]
pub fn InputField(props: InputFieldProps) -> Element {
    let error_class = if props.has_error { "border-red-500" } else { "" };
    let disabled_class = if props.disabled { "bg-gray-100 dark:bg-gray-600 cursor-not-allowed" } else { "" };
    
    rsx! {
        div { class: "space-y-2",
            label { 
                class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                span { "{props.label}" }
                {if props.required {
                    rsx! {
                        span {
                            class: "text-red-500 ml-1",
                            "*"
                        }
                    }
                } else {
                    rsx! {}
                }}
            }
            div { class: "flex items-center space-x-3",
                input {
                    class: "flex-1 block w-full px-4 py-2.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-green-500 dark:bg-gray-700 dark:text-white {error_class} {disabled_class}",
                    r#type: "text",
                    placeholder: "{props.placeholder}",
                    required: props.required,
                    disabled: props.disabled,
                    value: "{props.value}",
                    oninput: move |evt| {
                        if !props.disabled {
                            props.on_input.call(evt.value().to_string())
                        }
                    },
                    onblur: move |_| {
                        if !props.disabled {
                            props.on_blur.call(())
                        }
                    }
                }
                if let Some(children) = &props.children {
                    {children}
                }
            }
        }
        {props.has_error.then(|| {
            rsx! {
                p { class: "mt-1 text-sm text-red-500", "{t!(\"this_field_is_required\")}" }
            }
        })}
    }
} 