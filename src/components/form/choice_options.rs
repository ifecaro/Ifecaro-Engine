use dioxus::prelude::*;
use crate::enums::translations::Translations;
use crate::components::form::{InputField, ActionTypeSelector};
use crate::components::paragraph_list::{Paragraph, ParagraphList};

#[derive(Props, Clone, PartialEq)]
pub struct ChoiceOptionsProps {
    pub t: Translations,
    pub new_caption: String,
    pub new_goto: String,
    pub new_action_type: String,
    pub new_action_key: Option<String>,
    pub new_action_value: Option<serde_json::Value>,
    pub extra_captions: Vec<String>,
    pub extra_gotos: Vec<String>,
    pub extra_action_types: Vec<String>,
    pub extra_action_keys: Vec<Option<String>>,
    pub extra_action_values: Vec<Option<serde_json::Value>>,
    pub new_caption_error: bool,
    pub new_goto_error: bool,
    pub available_paragraphs: Vec<Paragraph>,
    pub on_new_caption_change: EventHandler<String>,
    pub on_new_goto_change: EventHandler<String>,
    pub on_new_action_type_change: EventHandler<String>,
    pub on_new_action_key_change: EventHandler<Option<String>>,
    pub on_new_action_value_change: EventHandler<Option<serde_json::Value>>,
    pub on_extra_caption_change: EventHandler<(usize, String)>,
    pub on_extra_goto_change: EventHandler<(usize, String)>,
    pub on_extra_action_type_change: EventHandler<(usize, String)>,
    pub on_extra_action_key_change: EventHandler<(usize, Option<String>)>,
    pub on_extra_action_value_change: EventHandler<(usize, Option<serde_json::Value>)>,
    pub on_add_choice: EventHandler<()>,
    pub on_remove_choice: EventHandler<usize>,
}

#[component]
pub fn ChoiceOptions(props: ChoiceOptionsProps) -> Element {
    let t = props.t.clone();
    let mut is_goto_open = use_signal(|| false);
    let mut goto_search_query = use_signal(|| String::new());
    let mut is_action_type_open = use_signal(|| false);

    rsx! {
        // 主要選項（選項 1）
        div {
            div {
                class: "text-sm font-medium text-gray-700 dark:text-gray-300 mb-4",
                span {
                    "{t.option} 1"
                }
            }

            div { 
                class: "relative border-2 border-gray-200 dark:border-gray-600 rounded-lg",
                div {
                    class: "p-4 space-y-4",
                    // 選項1標題
                    // 標題輸入框
                    InputField {
                        label: t.caption,
                        value: props.new_caption.clone(),
                        on_input: move |value| props.on_new_caption_change.call(value),
                        placeholder: t.caption,
                        has_error: props.new_caption_error,
                        required: true,
                        on_blur: move |_| {},
                    }
                    // 目標段落選擇器
                    ParagraphList {
                        label: props.t.goto_target,
                        value: props.new_goto.clone(),
                        paragraphs: props.available_paragraphs.clone(),
                        is_open: *is_goto_open.read(),
                        search_query: goto_search_query.read().to_string(),
                        on_toggle: move |_| {
                            let current = *is_goto_open.read();
                            is_goto_open.set(!current);
                        },
                        on_search: move |query| goto_search_query.set(query),
                        on_select: move |id| {
                            props.on_new_goto_change.call(id);
                            is_goto_open.set(false);
                        },
                        has_error: props.new_goto_error,
                        t: props.t.clone(),
                    }
                    // Action 相關欄位
                    div {
                        class: "border-t border-gray-200 dark:border-gray-700 mt-4 pt-4",
                        div {
                            class: "text-sm font-medium text-gray-700 dark:text-gray-300 mb-4",
                            "{t.action_settings}"
                        }
                        div {
                            class: "grid grid-cols-1 lg:grid-cols-3 gap-4",
                            div {
                                class: "relative",
                                ActionTypeSelector {
                                    label: t.action_type,
                                    value: props.new_action_type.clone(),
                                    is_open: *is_action_type_open.read(),
                                    on_toggle: move |_| {
                                        let current = *is_action_type_open.read();
                                        is_action_type_open.set(!current);
                                    },
                                    on_select: move |value| {
                                        props.on_new_action_type_change.call(value);
                                        is_action_type_open.set(false);
                                    },
                                    has_error: false,
                                    required: false,
                                }
                            }

                            InputField {
                                label: t.action_key,
                                placeholder: t.action_key,
                                value: props.new_action_key.clone().unwrap_or_default(),
                                required: false,
                                has_error: false,
                                on_input: move |value: String| {
                                    props.on_new_action_key_change.call(if value.is_empty() { None } else { Some(value) });
                                },
                                on_blur: move |_| {}
                            }

                            InputField {
                                label: t.action_value,
                                placeholder: t.action_value,
                                value: props.new_action_value.clone().map(|v| v.to_string()).unwrap_or_default(),
                                required: false,
                                has_error: false,
                                on_input: move |value: String| {
                                    props.on_new_action_value_change.call(if value.is_empty() { None } else { Some(serde_json::Value::String(value)) });
                                },
                                on_blur: move |_| {}
                            }
                        }
                    }
                }
            }
        }

        // 新增選項按鈕
        button {
            class: "w-full mt-4 border-2 border-dashed border-gray-200 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors duration-200 flex items-center justify-center",
            onclick: move |_| props.on_add_choice.call(()),
            div {
                class: "flex items-center space-x-2 text-gray-500 dark:text-gray-400 p-4",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    class: "h-5 w-5",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    stroke_width: "2",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M12 4v16m8-8H4"
                    }
                }
                span { "{t.add_option}" }
            }
        }

        // 額外選項
        {props.extra_captions.iter().enumerate().map(|(index, caption)| {
            let goto = props.extra_gotos.get(index).cloned().unwrap_or_default();
            let action_type = props.extra_action_types.get(index).cloned().unwrap_or_default();
            let action_key = props.extra_action_keys.get(index).cloned().unwrap_or_default();
            let action_value = props.extra_action_values.get(index).cloned().unwrap_or_default();
            let mut is_extra_goto_open = use_signal(|| false);
            let mut extra_goto_search_query = use_signal(|| String::new());
            let mut is_extra_action_type_open = use_signal(|| false);

            rsx! {
                div {
                    div {
                        class: "text-sm font-medium text-gray-700 dark:text-gray-300 mb-4",
                        span {
                            "{t.option} {index + 2}"
                        }
                    }
                    div {
                        key: "{index}",
                        class: "relative border-2 border-gray-200 dark:border-gray-600 rounded-lg mt-4",
                        div {
                            class: "p-4 space-y-4",
                            // 標題輸入框
                            InputField {
                                label: t.caption,
                                value: caption.clone(),
                                on_input: move |value| props.on_extra_caption_change.call((index, value)),
                                placeholder: t.caption,
                                has_error: false,
                                required: true,
                                on_blur: move |_| {},
                            }
                            // 目標段落選擇器
                            ParagraphList {
                                label: props.t.goto_target,
                                value: goto.clone(),
                                paragraphs: props.available_paragraphs.clone(),
                                is_open: *is_extra_goto_open.read(),
                                search_query: extra_goto_search_query.read().to_string(),
                                on_toggle: move |_| {
                                    let current = *is_extra_goto_open.read();
                                    is_extra_goto_open.set(!current);
                                },
                                on_search: move |query| extra_goto_search_query.set(query),
                                on_select: move |id| {
                                    props.on_extra_goto_change.call((index, id));
                                    is_extra_goto_open.set(false);
                                },
                                has_error: false,
                                t: props.t.clone(),
                            }
                            // Action 相關欄位
                            div {
                                class: "border-t border-gray-200 dark:border-gray-700 mt-4 pt-4",
                                div {
                                    class: "text-sm font-medium text-gray-700 dark:text-gray-300 mb-4",
                                    "{t.action_settings}"
                                }
                                div {
                                    class: "grid grid-cols-1 lg:grid-cols-3 gap-4",
                                    div {
                                        class: "relative",
                                        ActionTypeSelector {
                                            label: t.action_type,
                                            value: action_type.clone(),
                                            is_open: *is_extra_action_type_open.read(),
                                            on_toggle: move |_| {
                                                let current = *is_extra_action_type_open.read();
                                                is_extra_action_type_open.set(!current);
                                            },
                                            on_select: move |value| {
                                                props.on_extra_action_type_change.call((index, value));
                                                is_extra_action_type_open.set(false);
                                            },
                                            has_error: false,
                                            required: false,
                                        }
                                    }

                                    InputField {
                                        label: t.action_key,
                                        placeholder: t.action_key,
                                        value: action_key.clone().unwrap_or_default(),
                                        required: false,
                                        has_error: false,
                                        on_input: move |value: String| {
                                            props.on_extra_action_key_change.call((index, if value.is_empty() { None } else { Some(value) }));
                                        },
                                        on_blur: move |_| {}
                                    }

                                    InputField {
                                        label: t.action_value,
                                        placeholder: t.action_value,
                                        value: action_value.clone().map(|v| v.to_string()).unwrap_or_default(),
                                        required: false,
                                        has_error: false,
                                        on_input: move |value: String| {
                                            props.on_extra_action_value_change.call((index, if value.is_empty() { None } else { Some(serde_json::Value::String(value)) }));
                                        },
                                        on_blur: move |_| {}
                                    }
                                }
                            }
                            // 刪除按鈕
                            button {
                                class: "w-full mt-2 px-4 py-2 text-sm text-red-600 dark:text-red-400 hover:text-red-700 dark:hover:text-red-300 transition-colors duration-200",
                                onclick: move |_| props.on_remove_choice.call(index),
                                "{t.delete_option}"
                            }
                        }
                    }
                }
            }
        })}
    }
}