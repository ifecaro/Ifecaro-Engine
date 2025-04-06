use dioxus::prelude::*;
use crate::enums::translations::Translations;
use crate::components::form::{TextareaField, ChoiceOptions};
use crate::components::paragraph_list::Paragraph;
use std::sync::Arc;

#[derive(Props, Clone, PartialEq)]
pub struct ParagraphFormProps {
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
    available_paragraphs: Vec<Paragraph>,
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
    let t = props.t.clone();

    let is_form_valid = {
        let paragraphs = paragraphs.clone();
        let new_caption = new_caption.clone();
        let new_goto = new_goto.clone();
        use_memo(move || {
            !paragraphs.trim().is_empty() &&
            !new_caption.trim().is_empty() &&
            !new_goto.trim().is_empty()
        })
    };

    rsx! {
        div { class: "space-y-8",
            // 段落內容欄位
            TextareaField {
                label: t.paragraph,
                placeholder: t.paragraph,
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
                class: "w-full px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed",
                disabled: !*is_form_valid.read(),
                onclick: move |_| props.on_submit.call(()),
                {t.submit}
            }
        }
    }
} 