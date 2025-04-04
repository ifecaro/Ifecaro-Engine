use dioxus::prelude::*;
use crate::enums::translations::Translations;
use crate::components::form::{InputField, TextareaField, ChoiceOptions};
use crate::components::paragraph_list::Paragraph;
use std::sync::Arc;

#[derive(Props, Clone, PartialEq)]
pub struct ParagraphFormProps {
    t: Translations,
    choice_id: String,
    paragraphs: String,
    new_caption: String,
    new_goto: String,
    extra_captions: Vec<String>,
    extra_gotos: Vec<String>,
    show_extra_options: Vec<()>,
    choice_id_error: bool,
    paragraphs_error: bool,
    new_caption_error: bool,
    new_goto_error: bool,
    available_paragraphs: Vec<Paragraph>,
    on_choice_id_change: EventHandler<String>,
    on_paragraphs_change: EventHandler<String>,
    on_new_caption_change: EventHandler<String>,
    on_new_goto_change: EventHandler<String>,
    on_extra_caption_change: EventHandler<(usize, String)>,
    on_extra_goto_change: EventHandler<(usize, String)>,
    on_add_choice: EventHandler<()>,
    on_submit: EventHandler<()>,
}

#[component]
pub fn ParagraphForm(props: ParagraphFormProps) -> Element {
    let choice_id = Arc::new(props.choice_id.clone());
    let paragraphs = Arc::new(props.paragraphs.clone());
    let new_caption = Arc::new(props.new_caption.clone());
    let new_goto = Arc::new(props.new_goto.clone());
    let extra_captions = props.extra_captions.clone();
    let extra_gotos = props.extra_gotos.clone();
    let show_extra_options = props.show_extra_options.clone();
    let choice_id_error = props.choice_id_error;
    let paragraphs_error = props.paragraphs_error;
    let new_caption_error = props.new_caption_error;
    let new_goto_error = props.new_goto_error;
    let available_paragraphs = props.available_paragraphs.clone();
    let t = props.t.clone();

    let is_form_valid = {
        let choice_id = choice_id.clone();
        let paragraphs = paragraphs.clone();
        let new_caption = new_caption.clone();
        let new_goto = new_goto.clone();
        use_memo(move || {
            !choice_id.trim().is_empty() &&
            !paragraphs.trim().is_empty() &&
            !new_caption.trim().is_empty() &&
            !new_goto.trim().is_empty()
        })
    };

    rsx! {
        div { class: "space-y-8",
            // Choice ID 欄位
            div { class: "flex-1",
                InputField {
                    label: t.choice_id.clone(),
                    placeholder: t.choice_id.clone(),
                    value: choice_id.to_string(),
                    required: true,
                    has_error: choice_id_error,
                    on_input: props.on_choice_id_change,
                    on_blur: move |_| {}
                }
            }

            // 段落內容欄位
            TextareaField {
                label: t.paragraph.clone(),
                placeholder: t.paragraph.clone(),
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
                extra_captions: extra_captions,
                extra_gotos: extra_gotos,
                new_caption_error: new_caption_error,
                new_goto_error: new_goto_error,
                available_paragraphs: available_paragraphs,
                on_new_caption_change: props.on_new_caption_change,
                on_new_goto_change: props.on_new_goto_change,
                on_extra_caption_change: props.on_extra_caption_change,
                on_extra_goto_change: props.on_extra_goto_change,
                on_add_choice: props.on_add_choice
            }

            // 提交按鈕
            button {
                class: "w-full px-6 py-3 bg-green-500 text-white rounded-md hover:bg-green-600 transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed font-medium text-lg",
                disabled: !*is_form_valid.read(),
                onclick: move |_| props.on_submit.call(()),
                "{t.submit}"
            }
        }
    }
} 