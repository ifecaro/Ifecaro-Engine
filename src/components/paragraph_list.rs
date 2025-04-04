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
        Dropdown {
            label: props.label.clone(),
            value: props.value.clone(),
            options: props.paragraphs.clone(),
            is_open: props.is_open,
            search_query: props.search_query.clone(),
            on_toggle: props.on_toggle,
            on_search: props.on_search,
            on_select: move |paragraph: Paragraph| {
                props.on_select.call(paragraph.id);
            },
            display_fn: display_paragraph
        }
    }
} 