use dioxus::prelude::*;
use crate::components::dropdown::Dropdown;

#[derive(Debug, Clone, PartialEq)]
pub struct Paragraph {
    pub id: String,
    pub preview: String,
}

#[allow(dead_code)]
fn display_paragraph(paragraph: &Paragraph) -> String {
    paragraph.preview.clone()
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
}

#[component]
pub fn ParagraphList(props: ParagraphListProps) -> Element {
    // 找到當前選中的段落
    let selected_preview = props.paragraphs.iter()
        .find(|p| p.id == props.value)
        .map(|p| p.preview.clone())
        .unwrap_or_else(|| props.value.clone());

    // 過濾段落
    let filtered_paragraphs = props.paragraphs.iter()
        .filter(|paragraph| {
            paragraph.preview.to_lowercase().contains(&props.search_query.to_lowercase())
        })
        .cloned()
        .collect::<Vec<_>>();

    // 定義顯示函數
    let display_paragraph = |paragraph: &Paragraph| paragraph.preview.clone();

    rsx! {
        Dropdown {
            label: props.label,
            value: selected_preview,
            options: filtered_paragraphs,
            is_open: props.is_open,
            search_query: props.search_query,
            on_toggle: props.on_toggle,
            on_search: props.on_search,
            on_select: move |paragraph: Paragraph| {
                props.on_select.call(paragraph.id);
                props.on_toggle.call(());
            },
            display_fn: display_paragraph,
            has_error: props.has_error,
            class: props.class,
            search_placeholder: "搜尋段落...",
            button_class: "",
            dropdown_class: "",
            search_input_class: "",
            option_class: "",
            disabled: props.disabled,
        }
    }
} 