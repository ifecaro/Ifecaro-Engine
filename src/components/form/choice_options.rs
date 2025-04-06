use dioxus::prelude::*;
use crate::enums::translations::Translations;
use crate::components::form::InputField;
use crate::components::paragraph_list::{Paragraph, ParagraphList};

#[derive(Props, Clone, PartialEq)]
pub struct ChoiceOptionsProps {
    t: Translations,
    new_caption: String,
    new_goto: String,
    new_action_type: String,
    new_action_key: Option<String>,
    new_action_value: Option<serde_json::Value>,
    extra_captions: Vec<String>,
    extra_gotos: Vec<String>,
    extra_action_types: Vec<String>,
    extra_action_keys: Vec<Option<String>>,
    extra_action_values: Vec<Option<serde_json::Value>>,
    new_caption_error: bool,
    new_goto_error: bool,
    available_paragraphs: Vec<Paragraph>,
    on_new_caption_change: EventHandler<String>,
    on_new_goto_change: EventHandler<String>,
    on_new_action_type_change: EventHandler<String>,
    on_new_action_key_change: EventHandler<Option<String>>,
    on_new_action_value_change: EventHandler<Option<serde_json::Value>>,
    on_extra_caption_change: EventHandler<(usize, String)>,
    on_extra_goto_change: EventHandler<(usize, String)>,
    on_extra_action_type_change: EventHandler<(usize, String)>,
    on_extra_action_key_change: EventHandler<(usize, Option<String>)>,
    on_extra_action_value_change: EventHandler<(usize, Option<serde_json::Value>)>,
    on_add_choice: EventHandler<()>,
}

#[component]
pub fn ChoiceOptions(props: ChoiceOptionsProps) -> Element {
    let mut is_goto_open = use_signal(|| false);
    let mut goto_search_query = use_signal(|| String::new());
    let mut extra_goto_open = use_signal(|| false);
    let mut extra_goto_search = use_signal(|| String::new());
    let mut should_scroll = use_signal(|| false);
    let t = props.t.clone();

    // 監聽 should_scroll 的變化
    use_effect(move || {
        if *should_scroll.read() {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let scroll_height = document.document_element().unwrap().scroll_height() as f64;
            let _ = window.scroll_with_x_and_y(0.0, scroll_height);
            should_scroll.set(false);
        }
        
        (move || {})()
    });

    // 找到目標段落的預覽文字
    let target_preview = props.available_paragraphs.iter()
        .find(|p| p.id == props.new_goto)
        .map(|p| p.preview.clone())
        .unwrap_or_else(|| props.new_goto.clone());

    rsx! {
            // 主要選項（選項 1）
            div { 
                class: "border-2 border-gray-200 dark:border-gray-600 rounded-lg overflow-hidden",
                div {
                    class: "bg-gray-100 dark:bg-gray-600 px-4 py-3 border-b border-gray-200 dark:border-gray-500",
                    div {
                        class: "text-lg font-medium text-gray-900 dark:text-white",
                        "Option 1"
                    }
                }
                div {
                    class: "p-4 bg-white dark:bg-gray-800",
                    div {
                        class: "space-y-4",
                        div {
                            class: "space-y-4",
                            InputField {
                                label: t.caption,
                                placeholder: t.caption,
                                value: props.new_caption,
                                required: true,
                                has_error: props.new_caption_error,
                                on_input: props.on_new_caption_change,
                                on_blur: move |_| {}
                            }

                            div { class: "flex items-center space-x-4",
                                div { class: "flex-1",
                                    ParagraphList {
                                        label: t.goto_target,
                                        value: target_preview,
                                        paragraphs: props.available_paragraphs.clone().into_iter()
                                            .filter(|p| p.id != props.available_paragraphs[0].id)
                                            .collect(),
                                        is_open: *is_goto_open.read(),
                                        search_query: goto_search_query.read().to_string(),
                                        on_toggle: move |_| {
                                            let current = *is_goto_open.read();
                                            is_goto_open.set(!current);
                                        },
                                        on_search: move |query| goto_search_query.set(query),
                                        on_select: move |id: String| {
                                            props.on_new_goto_change.call(id);
                                            is_goto_open.set(false);
                                            goto_search_query.set(String::new());
                                        }
                                    }
                                }
                            }
                        }

                        // Action 相關欄位
                        div {
                            class: "border-t border-gray-200 dark:border-gray-700 mt-4 pt-4",
                            div {
                                class: "text-sm font-medium text-gray-700 dark:text-gray-300 mb-4",
                                "Action Settings"
                            }
                            div {
                                class: "grid grid-cols-1 lg:grid-cols-3 gap-4",
                                div {
                                    class: "relative",
                                    label { 
                                        class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                                        "Action Type"
                                    }
                                    select {
                                        class: "block w-full px-4 py-2.5 text-base bg-gray-50 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-blue-500 focus:border-blue-500 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                        value: props.new_action_type.clone(),
                                        onchange: move |evt| props.on_new_action_type_change.call(evt.value().clone()),
                                        option {
                                            value: "",
                                            selected: props.new_action_type.is_empty(),
                                            "None"
                                        }
                                        option {
                                            value: "setting",
                                            selected: props.new_action_type == "setting",
                                            "Setting"
                                        }
                                    }
                                }

                                InputField {
                                    label: "Action Key",
                                    placeholder: "Enter action key (optional)",
                                    value: props.new_action_key.clone().unwrap_or_default(),
                                    required: false,
                                    has_error: false,
                                    on_input: move |value: String| {
                                        props.on_new_action_key_change.call(if value.is_empty() { None } else { Some(value) });
                                    },
                                    on_blur: move |_| {}
                                }

                                InputField {
                                    label: "Action Value",
                                    placeholder: "Enter action value (optional)",
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

            // 額外選項
            {props.extra_captions.iter().enumerate().map(|(i, caption)| {
                let caption = caption.clone();
                let goto = props.extra_gotos[i].clone();
                let action_type = props.extra_action_types[i].clone();
                let action_key = props.extra_action_keys[i].clone();
                let action_value = props.extra_action_values[i].clone();
                let i = i.clone();
                
                let target_preview = props.available_paragraphs.iter()
                    .find(|p| p.id == goto)
                    .map(|p| p.preview.clone())
                    .unwrap_or_else(|| goto.clone());
                
                rsx! {
                    div { 
                        class: "border-2 border-gray-200 dark:border-gray-600 rounded-lg overflow-hidden",
                        div {
                            class: "bg-gray-100 dark:bg-gray-600 px-4 py-3 border-b border-gray-200 dark:border-gray-500",
                            div {
                                class: "text-lg font-medium text-gray-900 dark:text-white",
                                "Option {i + 2}"
                            }
                        }
                        div {
                            class: "p-4 bg-white dark:bg-gray-800",
                            div {
                                class: "space-y-4",
                                div {
                                    class: "space-y-4",
                                    InputField {
                                        label: t.caption,
                                        placeholder: t.caption,
                                        value: caption,
                                        required: true,
                                        has_error: false,
                                        on_input: move |value| props.on_extra_caption_change.call((i, value)),
                                        on_blur: move |_| {}
                                    }

                                    div { class: "space-y-2",
                                        ParagraphList {
                                            label: t.goto_target,
                                            value: target_preview,
                                            paragraphs: props.available_paragraphs.clone().into_iter()
                                                .filter(|p| p.id != props.available_paragraphs[0].id)
                                                .collect(),
                                            is_open: *extra_goto_open.read(),
                                            search_query: extra_goto_search.read().to_string(),
                                            on_toggle: move |_| {
                                                let current = *extra_goto_open.read();
                                                extra_goto_open.set(!current);
                                            },
                                            on_search: move |query| extra_goto_search.set(query),
                                            on_select: move |id: String| {
                                                props.on_extra_goto_change.call((i, id));
                                                extra_goto_open.set(false);
                                                extra_goto_search.set(String::new());
                                            }
                                        }
                                    }
                                }

                                // 額外選項的 Action 相關欄位
                                div {
                                    class: "border-t border-gray-200 dark:border-gray-700 mt-4 pt-4",
                                    div {
                                        class: "text-sm font-medium text-gray-700 dark:text-gray-300 mb-4",
                                        "Action Settings"
                                    }
                                    div {
                                        class: "grid grid-cols-1 lg:grid-cols-3 gap-4",
                                        div {
                                            class: "relative",
                                            label { 
                                                class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                                                "Action Type"
                                            }
                                            select {
                                                class: "block w-full px-4 py-2.5 text-base bg-gray-50 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-blue-500 focus:border-blue-500 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                                value: action_type.clone(),
                                                onchange: move |evt| props.on_extra_action_type_change.call((i, evt.value().clone())),
                                                option {
                                                    value: "",
                                                    selected: action_type.is_empty(),
                                                    "None"
                                                }
                                                option {
                                                    value: "setting",
                                                    selected: action_type == "setting",
                                                    "Setting"
                                                }
                                            }
                                        }

                                        InputField {
                                            label: "Action Key",
                                            placeholder: "Enter action key (optional)",
                                            value: action_key.unwrap_or_default(),
                                            required: false,
                                            has_error: false,
                                            on_input: move |value: String| {
                                                props.on_extra_action_key_change.call((i, if value.is_empty() { None } else { Some(value) }));
                                            },
                                            on_blur: move |_| {}
                                        }

                                        InputField {
                                            label: "Action Value",
                                            placeholder: "Enter action value (optional)",
                                            value: action_value.map(|v| v.to_string()).unwrap_or_default(),
                                            required: false,
                                            has_error: false,
                                            on_input: move |value: String| {
                                                props.on_extra_action_value_change.call((i, if value.is_empty() { None } else { Some(serde_json::Value::String(value)) }));
                                            },
                                            on_blur: move |_| {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            })}

        // 新增選項按鈕
        button {
            class: "border-2 border-gray-200 dark:border-gray-600 rounded-lg overflow-hidden h-full flex flex-col hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors duration-200",
            onclick: move |_| {
                props.on_add_choice.call(());
                should_scroll.set(true);
            },
            div {
                class: "bg-gray-100 dark:bg-gray-600 px-4 py-3 border-b border-gray-200 dark:border-gray-500",
                div {
                    class: "text-lg font-medium text-gray-900 dark:text-white",
                    "Add Option"
                }
            }
            div {
                class: "flex-1 flex items-center justify-center p-4 bg-white dark:bg-gray-800",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    class: "h-8 w-8 text-gray-500 dark:text-gray-400",
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
            }
        }
    }
}