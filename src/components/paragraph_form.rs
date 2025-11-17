use crate::components::form::{ChoiceOptions, TextareaField};
use crate::components::paragraph_list::Paragraph;
use crate::contexts::chapter_context::Chapter;
use crate::models::impacts::Impact;
use dioxus::prelude::*;
use dioxus_i18n::t;
use std::sync::Arc;

#[derive(Props, Clone, PartialEq)]
pub struct ParagraphFormProps {
    paragraphs: String,
    new_caption: String,
    new_goto: String,
    extra_captions: Vec<String>,
    extra_gotos: Vec<String>,
    show_extra_options: Vec<()>,
    paragraphs_error: bool,
    new_caption_error: bool,
    new_goto_error: bool,
    available_paragraphs: Vec<Paragraph>,
    on_paragraphs_change: EventHandler<String>,
    on_new_caption_change: EventHandler<String>,
    on_new_goto_change: EventHandler<String>,
    on_extra_caption_change: EventHandler<(usize, String)>,
    on_extra_goto_change: EventHandler<(usize, String)>,
    on_add_choice: EventHandler<()>,
    on_remove_choice: EventHandler<usize>,
    on_submit: EventHandler<()>,
    choices: Vec<(
        String,
        Vec<String>,
        String,
        Option<String>,
        Option<serde_json::Value>,
        String,
        bool,
        Option<u32>,
        Option<String>,
        String,
        Vec<Impact>,
    )>,
    available_chapters: Vec<Chapter>,
    selected_language: String,
    choice_paragraphs: Vec<Paragraph>,
    choice_chapters_open: Vec<bool>,
    choice_chapters_search: Vec<String>,
    choice_paragraphs_open: Vec<bool>,
    choice_paragraphs_search: Vec<String>,
    on_chapter_toggle: EventHandler<()>,
    on_chapter_search: EventHandler<()>,
    on_paragraph_toggle: EventHandler<()>,
    on_paragraph_search: EventHandler<()>,
    action_type_open: Vec<bool>,
    on_action_type_toggle: EventHandler<usize>,
}

#[component]
pub fn ParagraphForm(props: ParagraphFormProps) -> Element {
    let mut choices = use_signal(|| {
        Vec::<(
            String,
            Vec<String>,
            String,
            Option<String>,
            Option<serde_json::Value>,
            String,
            bool,
            Option<u32>,
            Option<String>,
            String,
            Vec<Impact>,
        )>::new()
    });
    let available_chapters = use_signal(|| Vec::<Chapter>::new());
    let selected_language = use_signal(|| String::new());
    let choice_paragraphs = use_signal(|| Vec::<Paragraph>::new());
    let mut action_type_open = use_signal(|| vec![false]);

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

    let is_form_valid = {
        let paragraphs = paragraphs.clone();
        let new_caption = new_caption.clone();
        let new_goto = new_goto.clone();
        use_memo(move || {
            !paragraphs.trim().is_empty()
                && !new_caption.trim().is_empty()
                && !new_goto.trim().is_empty()
        })
    };

    rsx! {
        div { class: "space-y-8",
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

            // Option settings
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
                        "same_page" => choices_write[index].6 = value == "true",
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
                        Vec::new(),
                    ));
                },
                on_remove_choice: move |index| {
                    choices.write().remove(index);
                },
                characters: Vec::new(),
                relationships: Vec::new(),
                on_impacts_change: move |(index, impacts): (usize, Vec<Impact>)| {
                    if let Some(choice) = choices.write().get_mut(index) {
                        choice.10 = impacts.clone();
                    }
                },
                available_chapters: available_chapters.read().clone(),
                selected_language: selected_language.read().clone(),
                choice_paragraphs: vec![choice_paragraphs.read().clone()],
                choice_chapters_open: vec![false],
                choice_chapters_search: vec![String::new()],
                choice_paragraphs_open: vec![false],
                choice_paragraphs_search: vec![String::new()],
                on_chapter_toggle: move |_idx: usize| {},
                on_chapter_search: move |_pair: (usize, String)| {},
                on_paragraph_toggle: move |_idx: usize| {},
                on_paragraph_search: move |_pair: (usize, String)| {},
                timeout_chapter_open: vec![false],
                timeout_chapter_search: vec![String::new()],
                timeout_paragraphs_open: vec![false],
                timeout_paragraphs_search: vec![String::new()],
                timeout_paragraphs: vec![Vec::new()],
                on_timeout_chapter_toggle: move |_idx: usize| {},
                on_timeout_chapter_search: move |_pair: (usize, String)| {},
                on_timeout_paragraph_toggle: move |_idx: usize| {},
                on_timeout_paragraph_search: move |_pair: (usize, String)| {},
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
                class: "w-full px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed",
                disabled: !*is_form_valid.read(),
                onclick: move |_| props.on_submit.call(()),
                {t!("submit")}
            }
        }
    }
}
