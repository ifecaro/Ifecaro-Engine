use dioxus::prelude::*;
use crate::enums::translations::Translations;
use crate::components::form::InputField;
use crate::components::paragraph_list::{Paragraph, ParagraphList};
use wasm_bindgen_futures::spawn_local;
use web_sys::console;

#[derive(Props, Clone, PartialEq)]
pub struct ChoiceOptionsProps {
    t: Translations,
    new_caption: String,
    new_goto: String,
    extra_captions: Vec<String>,
    extra_gotos: Vec<String>,
    new_caption_error: bool,
    new_goto_error: bool,
    available_paragraphs: Vec<Paragraph>,
    on_new_caption_change: EventHandler<String>,
    on_new_goto_change: EventHandler<String>,
    on_extra_caption_change: EventHandler<(usize, String)>,
    on_extra_goto_change: EventHandler<(usize, String)>,
    on_add_choice: EventHandler<()>,
}

#[component]
pub fn ChoiceOptions(props: ChoiceOptionsProps) -> Element {
    let mut is_goto_open = use_signal(|| false);
    let mut goto_search_query = use_signal(|| String::new());
    let mut extra_goto_open = use_signal(|| false);
    let mut extra_goto_search = use_signal(|| String::new());

    let extra_captions = props.extra_captions.clone();
    let extra_gotos = props.extra_gotos.clone();
    let t = props.t.clone();

    // 找到目標段落的預覽文字
    let target_preview = props.available_paragraphs.iter()
        .find(|p| p.id == props.new_goto)
        .map(|p| p.preview.clone())
        .unwrap_or_else(|| props.new_goto.clone());

    rsx! {
        div { 
            class: "max-w-3xl mx-auto p-8 bg-white dark:bg-gray-800 rounded-lg shadow-lg border border-gray-100 dark:border-gray-700",
            div { class: "space-y-4",
                InputField {
                    label: t.option_text.clone(),
                    placeholder: t.option_text.clone(),
                    value: props.new_caption,
                    required: true,
                    has_error: props.new_caption_error,
                    on_input: props.on_new_caption_change,
                    on_blur: move |_| {}
                }

                div { class: "space-y-2",
                    div { class: "flex items-center space-x-4",
                        div { class: "flex-1",
                            ParagraphList {
                                label: t.goto_target.clone(),
                                value: target_preview,
                                paragraphs: props.available_paragraphs.clone().into_iter()
                                    .filter(|p| p.id != props.available_paragraphs[0].id)
                                    .collect(),
                                is_open: *is_goto_open.read(),
                                search_query: goto_search_query.read().to_string(),
                                on_toggle: move |_| {
                                    let current = *is_goto_open.read();
                                    is_goto_open.set(!current);
                                },
                                on_search: move |query| goto_search_query.set(query),
                                on_select: move |id: String| {
                                    props.on_new_goto_change.call(id);
                                    is_goto_open.set(false);
                                    goto_search_query.set(String::new());
                                }
                            }
                        }
                        div { class: "flex-shrink-0 mt-8",
                            button {
                                class: "inline-flex items-center justify-center w-10 h-10 text-sm font-medium text-white bg-gray-700 rounded-lg hover:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-600 transition-colors duration-200 shadow-sm",
                                onclick: move |_| props.on_add_choice.call(()),
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    class: "h-5 w-5",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        d: "M12 4v16m8-8H4"
                                    }
                                }
                            }
                        }
                    }
                    
                    {props.new_goto_error.then(|| {
                        rsx! {
                            p { class: "text-sm text-red-500", "此欄位為必填" }
                        }
                    })}
                }

                {extra_captions.iter().enumerate().map(|(i, caption)| {
                    let label = format!("{} {}", t.option_text, i + 2);
                    let goto_label = format!("{} {}", t.goto_target, i + 2);
                    let caption = caption.clone();
                    let goto = extra_gotos[i].clone();
                    let i = i.clone();
                    
                    // 找到目標段落的預覽文字
                    let target_preview = props.available_paragraphs.iter()
                        .find(|p| p.id == goto)
                        .map(|p| p.preview.clone())
                        .unwrap_or_else(|| goto.clone());
                    
                    rsx! {
                        div { class: "space-y-4",
                            InputField {
                                label: label.clone(),
                                placeholder: t.option_text.clone(),
                                value: caption,
                                required: true,
                                has_error: false,
                                on_input: move |value| props.on_extra_caption_change.call((i, value)),
                                on_blur: move |_| {}
                            }

                            div { class: "space-y-2",
                                ParagraphList {
                                    label: goto_label.clone(),
                                    value: target_preview,
                                    paragraphs: props.available_paragraphs.clone().into_iter()
                                        .filter(|p| p.id != props.available_paragraphs[0].id)
                                        .collect(),
                                    is_open: *extra_goto_open.read(),
                                    search_query: extra_goto_search.read().to_string(),
                                    on_toggle: move |_| {
                                        let current = *extra_goto_open.read();
                                        extra_goto_open.set(!current);
                                    },
                                    on_search: move |query| extra_goto_search.set(query),
                                    on_select: move |id: String| {
                                        props.on_extra_goto_change.call((i, id));
                                        extra_goto_open.set(false);
                                        extra_goto_search.set(String::new());
                                    }
                                }
                            }
                        }
                    }
                })}
            }
        }
    }
}