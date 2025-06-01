use dioxus::prelude::*;
use dioxus_i18n::t;
use crate::components::form::{InputField, ActionTypeSelector};
use crate::components::paragraph_list::{Paragraph, MultiSelectParagraphList};
use crate::contexts::chapter_context::Chapter;

#[derive(Props, Clone, PartialEq)]
pub struct ChoiceOptionsProps {
    pub choices: Vec<(String, Vec<String>, String, Option<String>, Option<serde_json::Value>, String, bool, Option<u32>)>,
    pub on_choice_change: EventHandler<(usize, String, String)>,
    pub on_choice_add_paragraph: EventHandler<(usize, String)>,
    pub on_choice_remove_paragraph: EventHandler<(usize, String)>,
    pub on_add_choice: EventHandler<()>,
    pub on_remove_choice: EventHandler<usize>,
    pub available_chapters: Vec<Chapter>,
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
        // Paragraph options title area
        div {
            class: "flex items-center justify-between mb-6",
            h3 {
                class: "text-lg font-semibold text-gray-900 dark:text-gray-100",
                {t!("options")}
            }
            // Add option button (desktop: on the right side of title, mobile: hidden)
            button {
                class: "hidden lg:inline-flex items-center px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-colors duration-200",
                onclick: move |_| props.on_add_choice.call(()),
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    class: "w-4 h-4 mr-2",
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
                {t!("add_option")}
            }
        }
        
        // Render all options
        {props.choices.iter().enumerate().map(|(index, (caption, goto_list, action_type, action_key, action_value, target_chapter, same_page, time_limit))| {
            // Check if action type is empty (None)
            let is_action_disabled = action_type.is_empty();
            
            rsx! {
                div {
                    key: "{index}",
                    div {
                        class: "relative border-2 border-gray-200 dark:border-gray-600 rounded-lg mb-8",
                        div {
                            class: "p-4 space-y-4",
                            // Option title row and delete button
                            div {
                                class: "flex items-center justify-between mb-4",
                                div {
                                    class: "text-sm font-medium text-gray-700 dark:text-gray-300",
                                    {format!("{} {}", t!("option"), index + 1)}
                                }
                                // Delete button (desktop: right-aligned, mobile: hidden)
                                button {
                                    class: "hidden lg:block px-3 py-1 text-sm text-red-600 dark:text-red-400 hover:text-red-700 dark:hover:text-red-300 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg transition-colors duration-200",
                                    onclick: move |_| props.on_remove_choice.call(index),
                                    {t!("delete_option")}
                                }
                            }
                            
                            // First row: title, target chapter, target paragraph (desktop: side by side, mobile: vertical)
                            div {
                                class: "grid grid-cols-1 lg:grid-cols-3 gap-4",
                                // Option title input field
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
                                // Target chapter selector
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
                                    on_select: move |chapter: Chapter| {
                                        props.on_choice_change.call((index, "target_chapter".to_string(), chapter.id.clone()));
                                    },
                                    has_error: false,
                                    selected_language: props.selected_language.clone(),
                                }
                                // Multi-select target paragraph selector
                                MultiSelectParagraphList {
                                    label: t!("goto_target"),
                                    selected_ids: goto_list.clone(),
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
                                        props.on_choice_add_paragraph.call((index, id));
                                    },
                                    on_remove: move |id| {
                                        props.on_choice_remove_paragraph.call((index, id));
                                    },
                                    has_error: false,
                                    selected_language: props.selected_language.clone(),
                                }
                            }
                            
                            // Action related fields
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

                                    div {
                                        class: if is_action_disabled { "opacity-50" } else { "" },
                                        InputField {
                                            label: t!("action_key"),
                                            placeholder: t!("action_key"),
                                            value: action_key.clone().unwrap_or_default(),
                                            required: false,
                                            has_error: false,
                                            disabled: is_action_disabled,
                                            on_input: move |value: String| {
                                                if !is_action_disabled {
                                                    props.on_choice_change.call((index, "action_key".to_string(), value));
                                                }
                                            },
                                            on_blur: move |_| {}
                                        }
                                    }

                                    div {
                                        class: if is_action_disabled { "opacity-50" } else { "" },
                                        InputField {
                                            label: t!("action_value"),
                                            placeholder: t!("action_value"),
                                            value: action_value.clone().map(|v| match v {
                                                serde_json::Value::String(s) => s.clone(),
                                                _ => v.to_string()
                                            }).unwrap_or_default(),
                                            required: false,
                                            has_error: false,
                                            disabled: is_action_disabled,
                                            on_input: move |value: String| {
                                                if !is_action_disabled {
                                                    props.on_choice_change.call((index, "action_value".to_string(), value));
                                                }
                                            },
                                            on_blur: move |_| {}
                                        }
                                    }
                                }
                            }
                            // same_page checkbox
                            div {
                                class: "flex items-center mt-2",
                                input {
                                    r#type: "checkbox",
                                    checked: *same_page,
                                    onchange: move |evt| {
                                        let checked = evt.value() == "true";
                                        props.on_choice_change.call((index, "same_page".to_string(), checked.to_string()));
                                    },
                                    class: "form-checkbox h-4 w-4 text-blue-600 transition-transform transition-opacity duration-150 ease-in-out will-change-transform will-change-opacity",
                                }
                                label {
                                    class: "ml-2 text-sm text-gray-700 dark:text-gray-300",
                                    {t!("same_page")}
                                }
                            }
                            // Time limit input field
                            InputField {
                                label: t!("time_limit_seconds"),
                                value: time_limit.map(|v| v.to_string()).unwrap_or_default(),
                                on_input: move |value| {
                                    props.on_choice_change.call((index, "time_limit".to_string(), value));
                                },
                                placeholder: t!("time_limit_seconds"),
                                has_error: false,
                                required: false,
                                on_blur: move |_| {},
                            }
                            
                            // Delete button (mobile: shown at the end, desktop: hidden)
                            button {
                                class: "lg:hidden w-full mt-2 px-4 py-2 text-sm text-red-600 dark:text-red-400 hover:text-red-700 dark:hover:text-red-300 transition-colors duration-200",
                                onclick: move |_| props.on_remove_choice.call(index),
                                {t!("delete_option")}
                            }
                        }
                    }
                }
            }
        })}

        // Add option button (mobile: shown at the bottom, desktop: hidden)
        button {
            class: "lg:hidden w-full px-4 py-3 border-2 border-dashed border-gray-300 dark:border-gray-600 rounded-lg text-gray-600 dark:text-gray-400 hover:border-gray-400 dark:hover:border-gray-500 hover:text-gray-700 dark:hover:text-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors duration-200",
            onclick: move |_| props.on_add_choice.call(()),
            {t!("add_option")}
        }
    }
}