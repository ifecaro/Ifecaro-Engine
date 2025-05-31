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

    // 將 Paragraph 轉換為 DisplayParagraph
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

    // 找到當前選中的段落
    let selected_preview = props.paragraphs.iter()
        .find(|p| p.id == props.value)
        .map(convert_to_display)
        .map(|p| p.display_text)
        .unwrap_or_else(|| props.value.clone());

    // 過濾並轉換段落
    let display_paragraphs: Vec<DisplayParagraph> = props.paragraphs.iter()
        .map(convert_to_display)
        .filter(|p| p.display_text.to_lowercase().contains(&props.search_query.to_lowercase()))
        .collect();

    rsx! {
        Dropdown {
            label: props.label,
            value: selected_preview,
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
            class: props.class,
            search_placeholder: t!("search_paragraph"),
            button_class: None,
            label_class: None,
            dropdown_class: "",
            search_input_class: "",
            option_class: "",
            disabled: props.disabled,
            required: props.required,
        }
    }
}

#[component]
pub fn MultiSelectParagraphList(props: MultiSelectParagraphListProps) -> Element {
    let untranslated_text = t!("untranslated");

    // 將 Paragraph 轉換為 DisplayParagraph
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

    // 過濾出可選的段落（排除已選的）
    let available_paragraphs: Vec<DisplayParagraph> = props.paragraphs.iter()
        .filter(|p| !props.selected_ids.contains(&p.id))
        .map(convert_to_display)
        .filter(|p| p.display_text.to_lowercase().contains(&props.search_query.to_lowercase()))
        .collect();

    // 顯示值：始終顯示「目標段落」
    let display_value = t!("select_paragraph").to_string();
    
    // 檢查是否有可用段落，如果沒有就停用選單
    let is_disabled = props.disabled || props.paragraphs.is_empty();

    rsx! {
        div {
            class: format!("space-y-2 {}", props.class),
            
            // 手動顯示標籤
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
            
            // 已選段落的標籤顯示（僅在有選中項目時顯示）
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
                                            title: "{p.display_text}",
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

            // 主要的下拉選單 - 不顯示標籤，因為我們已經手動顯示了
            Dropdown {
                label: String::new(), // 空字符串，不顯示標籤
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