use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components::dropdown::Dropdown;

#[derive(Debug, Clone, PartialEq)]
pub struct Paragraph {
    pub id: String,
    pub preview: String,
}

fn display_paragraph(paragraph: &Paragraph) -> String {
    paragraph.preview.clone()
}

#[derive(Props, Clone, PartialEq)]
pub struct ParagraphListProps {
    label: String,
    value: String,
    paragraphs: Vec<Paragraph>,
    is_open: bool,
    search_query: String,
    on_toggle: EventHandler<()>,
    on_search: EventHandler<String>,
    on_select: EventHandler<String>,
}

#[component]
pub fn ParagraphList(props: ParagraphListProps) -> Element {
    rsx! {
        div { class: "relative",
            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                "{props.label}"
            }
            div {
                class: "relative",
                button {
                    class: "w-full px-4 py-2 text-left text-sm bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 hover:bg-gray-50 dark:hover:bg-gray-600 transition-colors duration-150",
                    onclick: move |_| props.on_toggle.call(()),
                    div { class: "flex justify-between items-center",
                        span { class: "block truncate text-gray-900 dark:text-gray-100",
                            "{props.value}"
                        }
                        span { class: "ml-2 pointer-events-none text-gray-500 dark:text-gray-400",
                            if props.is_open {
                                "▲"
                            } else {
                                "▼"
                            }
                        }
                    }
                }
                div {
                    class: if props.is_open {
                        "absolute z-10 mt-1 w-full bg-white dark:bg-gray-700 shadow-lg max-h-60 rounded-md py-1 text-base overflow-auto focus:outline-none sm:text-sm border border-gray-200 dark:border-gray-600"
                    } else {
                        "hidden"
                    },
                    div { class: "sticky top-0 z-10 bg-white dark:bg-gray-700 px-3 py-2 border-b border-gray-200 dark:border-gray-600",
                        input {
                            class: "w-full px-3 py-2 text-sm bg-gray-50 dark:bg-gray-600 border border-gray-300 dark:border-gray-500 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900 dark:text-gray-100 placeholder-gray-500 dark:placeholder-gray-400",
                            placeholder: "搜尋...",
                            value: "{props.search_query}",
                            oninput: move |event: FormEvent| props.on_search.call(event.value().clone())
                        }
                    }
                    div { class: "py-1",
                        {props.paragraphs.iter()
                            .filter(|paragraph| {
                                paragraph.preview.to_lowercase().contains(&props.search_query.to_lowercase())
                            })
                            .map(|paragraph| {
                                let id = paragraph.id.clone();
                                rsx! {
                                    div {
                                        class: "cursor-pointer select-none relative py-2 pl-3 pr-9 hover:bg-gray-100 dark:hover:bg-gray-600 text-gray-900 dark:text-gray-100 transition-colors duration-150",
                                        onclick: move |_| props.on_select.call(id.clone()),
                                        span { class: "block truncate",
                                            "{paragraph.preview}"
                                        }
                                    }
                                }
                            })}
                    }
                }
            }
        }
    }
} 