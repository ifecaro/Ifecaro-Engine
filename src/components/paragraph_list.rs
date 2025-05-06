use dioxus::prelude::*;
use crate::components::dropdown::Dropdown;
use crate::enums::translations::Translations;

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
    pub t: Translations,
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
    let untranslated_text = props.t.untranslated;

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
            search_placeholder: props.t.search_paragraph,
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