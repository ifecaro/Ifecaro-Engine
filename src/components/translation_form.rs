use dioxus::prelude::*;
use crate::components::form::{TextareaField, ChoiceOptions};
use crate::components::paragraph_list::{Paragraph as ParagraphListItem, ParagraphList};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use dioxus_i18n::t;
use dioxus::events::FormEvent;
use crate::contexts::chapter_context::Chapter;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ParagraphChoice {
    Complex {
        to: String,
        #[serde(rename = "type")]
        type_: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        same_page: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        time_limit: Option<u32>,
    },
    Simple(String),
}

impl ParagraphChoice {
    pub fn get_to(&self) -> String {
        match self {
            ParagraphChoice::Complex { to, .. } => to.clone(),
            ParagraphChoice::Simple(text) => text.clone(),
        }
    }

    pub fn get_type(&self) -> String {
        match self {
            ParagraphChoice::Complex { type_, .. } => type_.clone(),
            ParagraphChoice::Simple(_) => "goto".to_string(),
        }
    }

    pub fn get_key(&self) -> Option<String> {
        match self {
            ParagraphChoice::Complex { key, .. } => key.clone(),
            ParagraphChoice::Simple(_) => None,
        }
    }

    pub fn get_value(&self) -> Option<serde_json::Value> {
        match self {
            ParagraphChoice::Complex { value, .. } => value.clone(),
            ParagraphChoice::Simple(_) => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Paragraph {
    pub id: String,
    #[serde(default)]
    pub chapter_id: String,
    pub texts: Vec<Text>,
    pub choices: Vec<ParagraphChoice>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Text {
    pub lang: String,
    pub paragraphs: String,
    pub choices: Vec<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct TranslationFormProps {
    paragraphs: String,
    new_caption: String,
    new_goto: String,
    extra_captions: Vec<String>,
    extra_gotos: Vec<String>,
    show_extra_options: Vec<()>,
    paragraphs_error: bool,
    new_caption_error: bool,
    new_goto_error: bool,
    available_paragraphs: Vec<ParagraphListItem>,
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
    
    let available_chapters = use_signal(|| Vec::<Chapter>::new());
    let selected_language = use_signal(|| String::new());
    let choice_paragraphs = use_signal(|| Vec::<ParagraphListItem>::new());

    let paragraphs = Arc::new(props.paragraphs.clone());
    let new_caption = Arc::new(props.new_caption.clone());
    let new_goto = Arc::new(props.new_goto.clone());
    let _extra_captions = props.extra_captions.clone();
    let _extra_gotos = props.extra_gotos.clone();
    let _show_extra_options = props.show_extra_options.clone();
    let paragraphs_error = props.paragraphs_error;
    let _new_caption_error = props.new_caption_error;
    let _new_goto_error = props.new_goto_error;
    let _available_paragraphs = props.available_paragraphs.clone();
    let selected_paragraph = props.selected_paragraph.clone();

    let mut choices = use_signal(|| Vec::<(String, Vec<String>, String, Option<String>, Option<serde_json::Value>, String, bool, Option<u32>, Option<String>, String)>::new());
    let mut action_type_open = use_signal(|| vec![false]);

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
                // Paragraph selector
                ParagraphList {
                    label: t!("select_paragraph"),
                    value: props.selected_paragraph.as_ref().map(|p| p.id.clone()).unwrap_or_else(|| t!("select_paragraph")),
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
                    show_selected_chip: false,
                }

                // Paragraph content field
                TextareaField {
                    label: Box::leak(t!("paragraph_content").into_boxed_str()),
                    placeholder: Box::leak(t!("paragraph_content").into_boxed_str()),
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

                // Choice options
                ChoiceOptions {
                    choices: choices.read().clone(),
                    on_choice_change: move |(index, field, value): (usize, String, String)| {
                        let mut choices_write = choices.write();
                        match field.as_str() {
                            "caption" => choices_write[index].0 = value,
                            "goto" => choices_write[index].1 = value.split(',').map(|s| s.trim().to_string()).collect(),
                            "action_type" => choices_write[index].2 = value,
                            "action_key" => choices_write[index].3 = Some(value),
                            "action_value" => choices_write[index].4 = Some(serde_json::Value::String(value)),
                            "target_chapter" => choices_write[index].5 = value,
                            "time_limit" => choices_write[index].7 = value.parse::<u32>().ok(),
                            "timeout_to" => choices_write[index].8 = if value.trim().is_empty() { None } else { Some(value) },
                            "timeout_target_chapter" => choices_write[index].9 = value,
                            _ => {}
                        }
                    },
                    on_choice_add_paragraph: move |(index, paragraph_id): (usize, String)| {
                        let mut choices_write = choices.write();
                        if let Some(choice) = choices_write.get_mut(index) {
                            if !choice.1.contains(&paragraph_id) {
                                choice.1.push(paragraph_id);
                            }
                        }
                    },
                    on_choice_remove_paragraph: move |(index, paragraph_id): (usize, String)| {
                        let mut choices_write = choices.write();
                        if let Some(choice) = choices_write.get_mut(index) {
                            choice.1.retain(|id| id != &paragraph_id);
                        }
                    },
                    on_add_choice: move |_| {
                        choices.write().push((
                            String::new(),
                            Vec::<String>::new(),
                            String::new(),
                            None,
                            None,
                            String::new(),
                            false,
                            None,
                            None,
                            String::new(),
                        ));
                    },
                    on_remove_choice: move |index| {
                        choices.write().remove(index);
                    },
                    available_chapters: available_chapters.read().clone(),
                    selected_language: selected_language.read().clone(),
                    choice_paragraphs: vec![choice_paragraphs.read().clone()],
                    choice_chapters_open: vec![false],
                    choice_chapters_search: vec![String::new()],
                    choice_paragraphs_open: vec![false],
                    choice_paragraphs_search: vec![String::new()],
                    on_chapter_toggle: move |_| {},
                    on_chapter_search: move |_| {},
                    on_paragraph_toggle: move |_| {},
                    on_paragraph_search: move |_| {},
                    timeout_chapter_open: vec![false],
                    timeout_chapter_search: vec![String::new()],
                    timeout_paragraphs_open: vec![false],
                    timeout_paragraphs_search: vec![String::new()],
                    timeout_paragraphs: vec![Vec::new()],
                    on_timeout_chapter_toggle: move |_| {},
                    on_timeout_chapter_search: move |_| {},
                    on_timeout_paragraph_toggle: move |_| {},
                    on_timeout_paragraph_search: move |_| {},
                    action_type_open: action_type_open.read().clone(),
                    on_action_type_toggle: move |index| {
                        let mut current = action_type_open.read().clone();
                        if let Some(is_open) = current.get_mut(index) {
                            *is_open = !(*is_open as bool);
                        }
                        action_type_open.set(current);
                    },
                }

                // Submit button
                button {
                    class: "w-full px-4 py-2 text-sm font-medium text-white bg-green-600 rounded-md hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 disabled:opacity-50 disabled:cursor-not-allowed",
                    disabled: !*is_form_valid.read(),
                    onclick: move |_| props.on_submit.call(()),
                    {t!("submit")}
                }
            }
        }
    }
} 