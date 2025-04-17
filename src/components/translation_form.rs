use dioxus::prelude::*;
use crate::enums::translations::Translations;
use crate::components::form::{TextareaField, ChoiceOptions};
use crate::components::paragraph_list::{Paragraph as ListParagraph, ParagraphList};
use crate::components::story_content::Choice;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Paragraph {
    pub id: String,
    pub index: usize,
    #[serde(default)]
    pub chapter_id: String,
    pub texts: Vec<Text>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Text {
    pub lang: String,
    pub paragraphs: String,
    pub choices: Vec<Choice>,
}

#[derive(Props, Clone, PartialEq)]
pub struct TranslationFormProps {
    t: Translations,
    paragraphs: String,
    new_caption: String,
    new_goto: String,
    extra_captions: Vec<String>,
    extra_gotos: Vec<String>,
    show_extra_options: Vec<()>,
    paragraphs_error: bool,
    new_caption_error: bool,
    new_goto_error: bool,
    available_paragraphs: Vec<ListParagraph>,
    selected_paragraph: Option<Paragraph>,
    on_paragraphs_change: EventHandler<String>,
    on_new_caption_change: EventHandler<String>,
    on_new_goto_change: EventHandler<String>,
    on_extra_caption_change: EventHandler<(usize, String)>,
    on_extra_goto_change: EventHandler<(usize, String)>,
    on_add_choice: EventHandler<()>,
    on_remove_choice: EventHandler<usize>,
    on_submit: EventHandler<()>,
    on_paragraph_select: EventHandler<String>,
}

#[component]
pub fn TranslationForm(props: TranslationFormProps) -> Element {
    let mut is_paragraph_open = use_signal(|| false);
    let mut paragraph_search_query = use_signal(|| String::new());
    
    let paragraphs = Arc::new(props.paragraphs.clone());
    let new_caption = Arc::new(props.new_caption.clone());
    let new_goto = Arc::new(props.new_goto.clone());
    let extra_captions = props.extra_captions.clone();
    let extra_gotos = props.extra_gotos.clone();
    let _show_extra_options = props.show_extra_options.clone();
    let paragraphs_error = props.paragraphs_error;
    let new_caption_error = props.new_caption_error;
    let new_goto_error = props.new_goto_error;
    let available_paragraphs = props.available_paragraphs.clone();
    let selected_paragraph = props.selected_paragraph.clone();
    let t = props.t.clone();

    let is_form_valid = {
        let paragraphs = paragraphs.clone();
        let new_caption = new_caption.clone();
        let new_goto = new_goto.clone();
        let selected_paragraph = selected_paragraph.clone();
        use_memo(move || {
            !paragraphs.trim().is_empty() &&
            !new_caption.trim().is_empty() &&
            !new_goto.trim().is_empty() &&
            selected_paragraph.is_some()
        })
    };

    rsx! {
        div { 
            class: "max-w-3xl mx-auto p-8 bg-white dark:bg-gray-800 rounded-lg shadow-lg border border-gray-100 dark:border-gray-700",
            div { class: "space-y-4",
                // 段落選擇器
                ParagraphList {
                    label: props.t.paragraph,
                    value: props.selected_paragraph.as_ref().map(|p| p.id.clone()).unwrap_or_else(|| "選擇段落".to_string()),
                    paragraphs: props.available_paragraphs.clone(),
                    is_open: *is_paragraph_open.read(),
                    search_query: paragraph_search_query.read().to_string(),
                    on_toggle: move |_| {
                        let current = *is_paragraph_open.read();
                        is_paragraph_open.set(!current);
                    },
                    on_search: move |query| paragraph_search_query.set(query),
                    on_select: move |id| {
                        props.on_paragraph_select.call(id);
                        is_paragraph_open.set(false);
                    },
                    has_error: false,
                    t: props.t.clone(),
                }

                // 段落內容欄位
                TextareaField {
                    label: t.paragraph_content,
                    placeholder: t.paragraph_content,
                    value: paragraphs.to_string(),
                    required: true,
                    has_error: paragraphs_error,
                    rows: 5,
                    on_input: move |event: FormEvent| {
                        let value = event.value().clone();
                        props.on_paragraphs_change.call(value);
                    },
                    on_blur: move |_| {}
                }

                // 選項設定
                ChoiceOptions {
                    t: t.clone(),
                    new_caption: new_caption.to_string(),
                    new_goto: new_goto.to_string(),
                    new_action_type: String::new(),
                    new_action_key: None,
                    new_action_value: None,
                    extra_captions: extra_captions,
                    extra_gotos: extra_gotos,
                    extra_action_types: Vec::new(),
                    extra_action_keys: Vec::new(),
                    extra_action_values: Vec::new(),
                    new_caption_error: new_caption_error,
                    new_goto_error: new_goto_error,
                    available_paragraphs: available_paragraphs,
                    on_new_caption_change: props.on_new_caption_change,
                    on_new_goto_change: props.on_new_goto_change,
                    on_new_action_type_change: move |_| {},
                    on_new_action_key_change: move |_| {},
                    on_new_action_value_change: move |_| {},
                    on_extra_caption_change: props.on_extra_caption_change,
                    on_extra_goto_change: props.on_extra_goto_change,
                    on_extra_action_type_change: move |_| {},
                    on_extra_action_key_change: move |_| {},
                    on_extra_action_value_change: move |_| {},
                    on_add_choice: props.on_add_choice,
                    on_remove_choice: props.on_remove_choice
                }

                // 提交按鈕
                button {
                    class: "w-full px-4 py-2 text-sm font-medium text-white bg-green-600 rounded-md hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 disabled:opacity-50 disabled:cursor-not-allowed",
                    disabled: !*is_form_valid.read(),
                    onclick: move |_| props.on_submit.call(()),
                    "{t.submit}"
                }
            }
        }
    }
} 