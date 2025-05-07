use dioxus::prelude::*;
use dioxus_i18n::t;
use crate::components::form::{InputField, ActionTypeSelector};
use crate::components::paragraph_list::{Paragraph, ParagraphList};

#[derive(Props, Clone, PartialEq)]
pub struct ChoiceOptionsProps {
    pub choices: Vec<(String, String, String, Option<String>, Option<serde_json::Value>, String)>,
    pub on_choice_change: EventHandler<(usize, String, String)>,
    pub on_add_choice: EventHandler<()>,
    pub on_remove_choice: EventHandler<usize>,
    pub available_chapters: Vec<crate::pages::dashboard::Chapter>,
    pub selected_language: String,
    pub choice_chapters_open: Vec<bool>,
    pub choice_chapters_search: Vec<String>,
    pub choice_paragraphs_open: Vec<bool>,
    pub choice_paragraphs_search: Vec<String>,
    pub choice_paragraphs: Vec<Vec<Paragraph>>,
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
        // 渲染所有選項
        {props.choices.iter().enumerate().map(|(index, (caption, goto, action_type, action_key, action_value, target_chapter))| {
            rsx! {
                div {
                    key: "{index}",
                    div {
                        class: "text-sm font-medium text-gray-700 dark:text-gray-300 mb-4 pl-4",
                        span {
                            {format!("{} {}", t!("option"), index + 1)}
                        }
                    }
                    div {
                        class: "relative border-2 border-gray-200 dark:border-gray-600 rounded-lg mb-8",
                        div {
                            class: "p-4 space-y-4",
                            // 標題輸入框
                            InputField {
                                label: t!("caption"),
                                value: caption.clone(),
                                on_input: move |value| {
                                    props.on_choice_change.call((index, "caption".to_string(), value));
                                },
                                placeholder: t!("caption"),
                                has_error: false,
                                required: true,
                                on_blur: move |_| {},
                            }
                            // 目標章節選擇器
                            crate::components::chapter_selector::ChapterSelector {
                                label: t!("target_chapter"),
                                value: target_chapter.clone(),
                                chapters: props.available_chapters.clone(),
                                is_open: props.choice_chapters_open.get(index).copied().unwrap_or(false),
                                search_query: props.choice_chapters_search.get(index).cloned().unwrap_or_default(),
                                on_toggle: {
                                    let on_chapter_toggle = props.on_chapter_toggle.clone();
                                    move |_| on_chapter_toggle.call(index)
                                },
                                on_search: {
                                    let on_chapter_search = props.on_chapter_search.clone();
                                    move |query| on_chapter_search.call((index, query))
                                },
                                on_select: move |chapter: crate::pages::dashboard::Chapter| {
                                    props.on_choice_change.call((index, "target_chapter".to_string(), chapter.id.clone()));
                                },
                                has_error: false,
                                selected_language: props.selected_language.clone(),
                            }
                            // 目標段落選擇器
                            ParagraphList {
                                label: t!("goto_target"),
                                value: goto.clone(),
                                paragraphs: props.choice_paragraphs.get(index).cloned().unwrap_or_default(),
                                is_open: props.choice_paragraphs_open.get(index).copied().unwrap_or(false),
                                search_query: props.choice_paragraphs_search.get(index).cloned().unwrap_or_default(),
                                on_toggle: {
                                    let on_paragraph_toggle = props.on_paragraph_toggle.clone();
                                    move |_| on_paragraph_toggle.call(index)
                                },
                                on_search: {
                                    let on_paragraph_search = props.on_paragraph_search.clone();
                                    move |query| on_paragraph_search.call((index, query))
                                },
                                on_select: move |id| {
                                    props.on_choice_change.call((index, "goto".to_string(), id));
                                },
                                has_error: false,
                                selected_language: props.selected_language.clone(),
                            }
                            // Action 相關欄位
                            div {
                                class: "border-t border-gray-200 dark:border-gray-700 mt-4 pt-4",
                                div {
                                    class: "text-sm font-medium text-gray-700 dark:text-gray-300 mb-4",
                                    {t!("action_settings")}
                                }
                                div {
                                    class: "grid grid-cols-1 lg:grid-cols-3 gap-4",
                                    div {
                                        class: "relative",
                                        ActionTypeSelector {
                                            label: t!("action_type"),
                                            value: action_type.clone(),
                                            is_open: props.action_type_open.get(index).copied().unwrap_or(false),
                                            on_toggle: {
                                                let on_action_type_toggle = props.on_action_type_toggle.clone();
                                                move |_| on_action_type_toggle.call(index)
                                            },
                                            on_select: {
                                                move |value| {
                                                    props.on_choice_change.call((index, "action_type".to_string(), value));
                                                    props.on_action_type_toggle.call(index);
                                                }
                                            },
                                            has_error: false,
                                            required: false,
                                        }
                                    }

                                    InputField {
                                        label: t!("action_key"),
                                        placeholder: t!("action_key"),
                                        value: action_key.clone().unwrap_or_default(),
                                        required: false,
                                        has_error: false,
                                        on_input: move |value: String| {
                                            props.on_choice_change.call((index, "action_key".to_string(), value));
                                        },
                                        on_blur: move |_| {}
                                    }

                                    InputField {
                                        label: t!("action_value"),
                                        placeholder: t!("action_value"),
                                        value: action_value.clone().map(|v| match v {
                                            serde_json::Value::String(s) => s.clone(),
                                            _ => v.to_string()
                                        }).unwrap_or_default(),
                                        required: false,
                                        has_error: false,
                                        on_input: move |value: String| {
                                            props.on_choice_change.call((index, "action_value".to_string(), value));
                                        },
                                        on_blur: move |_| {}
                                    }
                                }
                            }
                            
                            // 刪除按鈕
                            button {
                                class: "w-full mt-2 px-4 py-2 text-sm text-red-600 dark:text-red-400 hover:text-red-700 dark:hover:text-red-300 transition-colors duration-200",
                                onclick: move |_| props.on_remove_choice.call(index),
                                {t!("delete_option")}
                            }
                        }
                    }
                }
            }
        })}

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
                span { {t!("add_option")} }
            }
        }
    }
}