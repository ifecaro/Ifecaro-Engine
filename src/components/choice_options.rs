use dioxus::prelude::*;
use crate::components::dropdown::Dropdown;
use crate::components::chapter_selector::ChapterSelector;
use crate::components::paragraph_list::ParagraphList;
use crate::enums::translations::Translations;
use crate::components::translation_form::{Chapter, ChapterTitle};

#[derive(Props, Clone, PartialEq)]
pub struct ChoiceOptionsProps {
    pub t: Translations,
    pub choices: Vec<(String, String, String, Option<String>, Option<serde_json::Value>, String)>,
    pub on_choice_change: EventHandler<(usize, String, String)>,
    pub on_add_choice: EventHandler<()>,
    pub on_remove_choice: EventHandler<usize>,
    pub available_chapters: Vec<Chapter>,
    pub selected_language: String,
    pub choice_chapters_open: Vec<bool>,
    pub choice_chapters_search: Vec<String>,
    pub choice_paragraphs_open: Vec<bool>,
    pub choice_paragraphs_search: Vec<String>,
    pub choice_paragraphs: Vec<Vec<crate::components::paragraph_list::Paragraph>>,
    pub on_chapter_toggle: EventHandler<usize>,
    pub on_chapter_search: EventHandler<(usize, String)>,
    pub on_paragraph_toggle: EventHandler<usize>,
    pub on_paragraph_search: EventHandler<(usize, String)>,
    pub action_type_open: Vec<bool>,
    pub on_action_type_toggle: EventHandler<usize>,
}

#[component]
pub fn ChoiceOptions(props: ChoiceOptionsProps) -> Element {
    rsx! {
        div {
            class: "space-y-4",
            for (index, choice) in props.choices.iter().enumerate() {
                div {
                    class: "p-4 bg-gray-50 dark:bg-gray-800 rounded-lg space-y-4",
                    // 選項標題
                    div {
                        class: "flex items-center space-x-4",
                        div {
                            class: "flex-1",
                            label {
                                class: "block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1",
                                "{props.t.choice_caption}"
                            }
                            input {
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 focus:border-transparent bg-white dark:bg-gray-700 text-gray-900 dark:text-white",
                                r#type: "text",
                                value: "{choice.0}",
                                oninput: move |ev| {
                                    props.on_choice_change.call((index, "caption".to_string(), ev.value.clone()));
                                }
                            }
                        }
                        button {
                            class: "p-2 text-gray-400 hover:text-gray-500 dark:hover:text-gray-300",
                            onclick: move |_| props.on_remove_choice.call(index),
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                class: "h-5 w-5",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                                }
                            }
                        }
                    }

                    // 目標章節選擇器
                    div {
                        class: "w-full",
                        ChapterSelector {
                            label: props.t.target_chapter,
                            value: choice.5.clone(),
                            chapters: props.available_chapters.clone(),
                            is_open: props.choice_chapters_open[index],
                            search_query: props.choice_chapters_search[index].clone(),
                            on_toggle: move |_| props.on_chapter_toggle.call(index),
                            on_search: move |query| props.on_chapter_search.call((index, query)),
                            on_select: move |chapter: Chapter| {
                                props.on_choice_change.call((index, "target_chapter".to_string(), chapter.id.clone()));
                            },
                            has_error: false,
                            selected_language: props.selected_language.clone(),
                            button_class: None,
                            label_class: None,
                        }
                    }

                    // 目標段落選擇器
                    div {
                        class: "w-full",
                        ParagraphList {
                            label: props.t.target_paragraph,
                            value: choice.1.clone(),
                            paragraphs: props.choice_paragraphs[index].clone(),
                            is_open: props.choice_paragraphs_open[index],
                            search_query: props.choice_paragraphs_search[index].clone(),
                            on_toggle: move |_| props.on_paragraph_toggle.call(index),
                            on_search: move |query| props.on_paragraph_search.call((index, query)),
                            on_select: move |id| {
                                props.on_choice_change.call((index, "goto".to_string(), id));
                            },
                            has_error: false,
                            t: props.t.clone(),
                            selected_language: props.selected_language.clone(),
                            button_class: None,
                            label_class: None,
                        }
                    }

                    // 動作類型選擇器
                    div {
                        class: "w-full",
                        label {
                            class: "block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1",
                            "{props.t.action_type}"
                        }
                        div {
                            class: "relative",
                            button {
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 focus:border-transparent bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-left",
                                onclick: move |_| props.on_action_type_toggle.call(index),
                                "{choice.2}"
                            }
                            if props.action_type_open[index] {
                                div {
                                    class: "absolute z-10 mt-1 w-full bg-white dark:bg-gray-700 shadow-lg rounded-md py-1",
                                    div {
                                        class: "cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-600 px-4 py-2",
                                        onclick: move |_| {
                                            props.on_choice_change.call((index, "action_type".to_string(), "goto".to_string()));
                                            props.on_action_type_toggle.call(index);
                                        },
                                        "goto"
                                    }
                                    div {
                                        class: "cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-600 px-4 py-2",
                                        onclick: move |_| {
                                            props.on_choice_change.call((index, "action_type".to_string(), "set".to_string()));
                                            props.on_action_type_toggle.call(index);
                                        },
                                        "set"
                                    }
                                }
                            }
                        }
                    }

                    // 動作鍵值
                    if choice.2 == "set" {
                        div {
                            class: "grid grid-cols-2 gap-4",
                            div {
                                class: "w-full",
                                label {
                                    class: "block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1",
                                    "{props.t.action_key}"
                                }
                                input {
                                    class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 focus:border-transparent bg-white dark:bg-gray-700 text-gray-900 dark:text-white",
                                    r#type: "text",
                                    value: "{choice.3.clone().unwrap_or_default()}",
                                    oninput: move |ev| {
                                        props.on_choice_change.call((index, "action_key".to_string(), ev.value.clone()));
                                    }
                                }
                            }
                            div {
                                class: "w-full",
                                label {
                                    class: "block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1",
                                    "{props.t.action_value}"
                                }
                                input {
                                    class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 focus:border-transparent bg-white dark:bg-gray-700 text-gray-900 dark:text-white",
                                    r#type: "text",
                                    value: "{choice.4.clone().and_then(|v| v.as_str().map(|s| s.to_string())).unwrap_or_default()}",
                                    oninput: move |ev| {
                                        props.on_choice_change.call((index, "action_value".to_string(), ev.value.clone()));
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 添加選項按鈕
            button {
                class: "w-full mt-4 px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500",
                onclick: move |_| props.on_add_choice.call(()),
                "{props.t.add_choice}"
            }
        }
    }
} 