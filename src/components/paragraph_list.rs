use dioxus::prelude::*;
use crate::components::dropdown::Dropdown;
use dioxus_i18n::t;

#[derive(Debug, Clone, PartialEq)]
pub struct Paragraph {
    pub id: String,
    pub preview: String,
    pub has_translation: bool,
}

#[derive(Props, Clone, PartialEq)]
pub struct ParagraphListProps {
    pub label: String,
    pub value: String,
    pub paragraphs: Vec<Paragraph>,
    pub is_open: bool,
    pub search_query: String,
    pub on_toggle: EventHandler<()>,
    pub on_search: EventHandler<String>,
    pub on_select: EventHandler<String>,
    #[props(default = false)]
    pub has_error: bool,
    #[props(default = String::new())]
    pub class: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default = false)]
    pub required: bool,
    #[props(default = String::new())]
    pub selected_language: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct MultiSelectParagraphListProps {
    pub label: String,
    pub selected_ids: Vec<String>,
    pub paragraphs: Vec<Paragraph>,
    pub is_open: bool,
    pub search_query: String,
    pub on_toggle: EventHandler<()>,
    pub on_search: EventHandler<String>,
    pub on_select: EventHandler<String>,
    pub on_remove: EventHandler<String>,
    #[props(default = false)]
    pub has_error: bool,
    #[props(default = String::new())]
    pub class: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default = false)]
    pub required: bool,
    #[props(default = String::new())]
    pub selected_language: String,
}

#[derive(Debug, Clone, PartialEq)]
struct DisplayParagraph {
    id: String,
    display_text: String,
}

#[component]
pub fn ParagraphList(props: ParagraphListProps) -> Element {
    let untranslated_text = t!("untranslated");

    // Convert Paragraph to DisplayParagraph
    let convert_to_display = |p: &Paragraph| {
        let display_text = if !p.has_translation {
            format!("（{}）{}", untranslated_text, p.preview)
        } else {
            p.preview.clone()
        };
        DisplayParagraph {
            id: p.id.clone(),
            display_text,
        }
    };

    // Selected paragraph (if any) in converted form
    let selected_paragraph = props.paragraphs
        .iter()
        .find(|p| p.id == props.value)
        .map(convert_to_display);

    // Filter and convert paragraphs for dropdown option list
    let display_paragraphs: Vec<DisplayParagraph> = props
        .paragraphs
        .iter()
        .map(convert_to_display)
        .filter(|p| p.display_text.to_lowercase().contains(&props.search_query.to_lowercase()))
        .collect();

    // Fallback button text – keep it consistent with MultiSelect list
    let button_text = if selected_paragraph.is_some() {
        t!("select_paragraph").to_string()
    } else {
        t!("select_paragraph").to_string()
    };

    // Disable dropdown when no paragraphs available or externally disabled
    let is_disabled = props.disabled || props.paragraphs.is_empty();

    rsx! {
        div {
            class: format!("space-y-2 {}", props.class),

            // Manual label (so that we can control styling like MultiSelect)
            if !props.label.is_empty() {
                label {
                    class: format!("block text-sm font-medium mb-2 {}",
                        if props.has_error {
                            "text-red-700 dark:text-red-400"
                        } else {
                            "text-gray-700 dark:text-gray-300"
                        }
                    ),
                    {props.label}
                    if props.required {
                        span { class: "text-red-500 ml-1", "*" }
                    }
                }
            }

            // Selected paragraph preview chip (similar style as MultiSelect chips)
            if let Some(p) = selected_paragraph {
                div {
                    class: "mb-2",
                    div {
                        class: "text-xs text-gray-500 dark:text-gray-400 mb-2",
                        {t!("selected_paragraphs")}
                    }
                    div {
                        class: "flex flex-wrap gap-2",
                        div {
                            class: "inline-flex items-center bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 text-xs px-2 py-1 rounded-full",
                            span {
                                class: "truncate max-w-32",
                                title: "{p.display_text.clone()}",
                                {p.display_text.clone()}
                            }
                            button {
                                class: "ml-1 text-blue-600 dark:text-blue-300 hover:text-blue-800 dark:hover:text-blue-100",
                                onclick: move |_| {
                                    // Clear selection by sending empty string
                                    props.on_select.call(String::new());
                                },
                                "×"
                            }
                        }
                    }
                }
            }

            // Dropdown for selecting paragraph
            Dropdown {
                label: String::new(), // we rendered label manually
                value: button_text,
                options: display_paragraphs,
                is_open: props.is_open,
                search_query: props.search_query,
                on_toggle: props.on_toggle,
                on_search: props.on_search,
                on_select: move |paragraph: DisplayParagraph| {
                    props.on_select.call(paragraph.id);
                    props.on_toggle.call(());
                },
                display_fn: |p: &DisplayParagraph| p.display_text.clone(),
                has_error: props.has_error,
                class: String::new(),
                search_placeholder: t!("search_paragraph"),
                button_class: None,
                label_class: None,
                dropdown_class: "",
                search_input_class: "",
                option_class: "",
                disabled: is_disabled,
                required: props.required,
            }
        }
    }
}

#[component]
pub fn MultiSelectParagraphList(props: MultiSelectParagraphListProps) -> Element {
    let untranslated_text = t!("untranslated");

    // Convert Paragraph to DisplayParagraph
    let convert_to_display = |p: &Paragraph| {
        let display_text = if !p.has_translation {
            format!("（{}）{}", untranslated_text, p.preview)
        } else {
            p.preview.clone()
        };
        DisplayParagraph {
            id: p.id.clone(),
            display_text,
        }
    };

    // Filter out selectable paragraphs (excluding already selected ones)
    let available_paragraphs: Vec<DisplayParagraph> = props.paragraphs.iter()
        .filter(|p| !props.selected_ids.contains(&p.id))
        .map(convert_to_display)
        .filter(|p| p.display_text.to_lowercase().contains(&props.search_query.to_lowercase()))
        .collect();

    // Display value: always show "Target Paragraph"
    let display_value = t!("select_paragraph").to_string();
    
    // Check if there are available paragraphs, disable menu if none
    let is_disabled = props.disabled || props.paragraphs.is_empty();

    rsx! {
        div {
            class: format!("space-y-2 {}", props.class),
            
            // Manually display label
            if !props.label.is_empty() {
                label {
                    class: format!("block text-sm font-medium mb-2 {}",
                        if props.has_error {
                            "text-red-700 dark:text-red-400"
                        } else {
                            "text-gray-700 dark:text-gray-300"
                        }
                    ),
                    {props.label}
                    if props.required {
                        span { class: "text-red-500 ml-1", "*" }
                    }
                }
            }
            
            // Selected paragraphs label display (only show when there are selected items)
            if !props.selected_ids.is_empty() {
                div {
                    class: "mb-2",
                    div {
                        class: "text-xs text-gray-500 dark:text-gray-400 mb-2",
                        {format!("{} ({})", t!("selected_paragraphs"), props.selected_ids.len())}
                    }
                    div {
                        class: "flex flex-wrap gap-2",
                        {props.selected_ids.iter().enumerate().map(|(i, id)| {
                            let paragraph = props.paragraphs.iter()
                                .find(|p| &p.id == id)
                                .map(convert_to_display);
                            
                            if let Some(p) = paragraph {
                                rsx! {
                                    div {
                                        key: "{i}",
                                        class: "inline-flex items-center bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 text-xs px-2 py-1 rounded-full",
                                        span {
                                            class: "truncate max-w-32",
                                            title: "{p.display_text.clone()}",
                                            {p.display_text.clone()}
                                        }
                                        button {
                                            class: "ml-1 text-blue-600 dark:text-blue-300 hover:text-blue-800 dark:hover:text-blue-100",
                                            onclick: {
                                                let id = id.clone();
                                                move |_| {
                                                    props.on_remove.call(id.clone());
                                                }
                                            },
                                            "×"
                                        }
                                    }
                                }
                            } else {
                                rsx! { div { key: "{i}" } }
                            }
                        })}
                    }
                }
            }

            // Main dropdown menu - don't show label since we already manually displayed it
            Dropdown {
                label: String::new(), // Empty string, don't show label
                value: display_value,
                options: available_paragraphs,
                is_open: props.is_open,
                search_query: props.search_query,
                on_toggle: props.on_toggle,
                on_search: props.on_search,
                on_select: move |paragraph: DisplayParagraph| {
                    props.on_select.call(paragraph.id);
                },
                display_fn: |p: &DisplayParagraph| p.display_text.clone(),
                has_error: props.has_error,
                class: String::new(),
                search_placeholder: t!("search_paragraph"),
                button_class: None,
                label_class: None,
                dropdown_class: "",
                search_input_class: "",
                option_class: "",
                disabled: is_disabled,
                required: props.required,
            }
        }
    }
} 