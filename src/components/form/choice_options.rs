use dioxus::prelude::*;
use crate::enums::translations::Translations;
use crate::components::form::InputField;
use dioxus::events::FormEvent;

#[derive(Props, Clone, PartialEq)]
pub struct ChoiceOptionsProps {
    t: Translations,
    new_caption: String,
    new_goto: String,
    extra_captions: Vec<String>,
    extra_gotos: Vec<String>,
    new_caption_error: bool,
    new_goto_error: bool,
    on_new_caption_change: EventHandler<FormEvent>,
    on_new_goto_change: EventHandler<FormEvent>,
    on_extra_caption_change: EventHandler<(usize, FormEvent)>,
    on_extra_goto_change: EventHandler<(usize, FormEvent)>,
    on_add_choice: EventHandler<()>,
}

#[component]
pub fn ChoiceOptions(props: ChoiceOptionsProps) -> Element {
    rsx! {
        div { class: "space-y-4",
            label { class: "block text-gray-700 dark:text-gray-300 text-sm font-semibold mb-3",
                "{props.t.options}"
            }
            div { class: "space-y-6",
                div { class: "grid grid-cols-2 gap-6",
                    InputField {
                        label: props.t.option_text,
                        placeholder: props.t.option_text,
                        value: props.new_caption,
                        required: true,
                        has_error: props.new_caption_error,
                        on_input: props.on_new_caption_change,
                        on_blur: move |_: FocusEvent| {}
                    }
                    InputField {
                        label: props.t.goto_target,
                        placeholder: props.t.goto_target,
                        value: props.new_goto,
                        required: true,
                        has_error: props.new_goto_error,
                        on_input: props.on_new_goto_change,
                        on_blur: move |_: FocusEvent| {}
                    }
                }
            }
        }

        {props.extra_captions.iter().enumerate().map(|(i, _)| {
            rsx! {
                div { class: "space-y-4",
                    div { class: "grid grid-cols-2 gap-6",
                        InputField {
                            label: props.t.option_text,
                            placeholder: props.t.option_text,
                            value: props.extra_captions[i].clone(),
                            required: false,
                            has_error: false,
                            on_input: move |evt| props.on_extra_caption_change.call((i, evt)),
                            on_blur: move |_: FocusEvent| {}
                        }
                        InputField {
                            label: props.t.goto_target,
                            placeholder: props.t.goto_target,
                            value: props.extra_gotos[i].clone(),
                            required: false,
                            has_error: false,
                            on_input: move |evt| props.on_extra_goto_change.call((i, evt)),
                            on_blur: move |_: FocusEvent| {}
                        }
                    }
                }
            }
        })}

        button {
            class: "px-6 py-2.5 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors duration-200 font-medium",
            onclick: move |_| props.on_add_choice.call(()),
            "{props.t.add}"
        }
    }
} 