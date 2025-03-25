use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::prelude::*;
use crate::constants::config::config::{BASE_API_URL, SETTINGS};
use crate::enums::translations::Translations;
use crate::components::toast::Toast;
use crate::components::form::{InputField, TextareaField};
use dioxus::events::{FormEvent, FocusEvent};

#[derive(Props, Clone, PartialEq)]
pub struct DashboardProps {
    lang: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Text {
    lang: String,
    paragraphs: String,
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Choice {
    caption: String,
    goto: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Paragraph {
    index: usize,
    choice_id: String,
    texts: Vec<Text>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Data {
    items: Vec<Paragraph>,
}

#[component]
pub fn Dashboard(props: DashboardProps) -> Element {
    let mut choices = use_signal(|| Vec::<Choice>::new());
    let mut choice_id = use_signal(|| String::new());
    let mut paragraphs = use_signal(|| String::new());
    let mut new_caption = use_signal(|| String::new());
    let mut new_goto = use_signal(|| String::new());
    let mut extra_captions = use_signal(|| Vec::<String>::new());
    let mut extra_gotos = use_signal(|| Vec::<String>::new());
    let mut show_extra_options = use_signal(|| Vec::<()>::new());
    let mut show_toast = use_signal(|| false);
    let mut toast_visible = use_signal(|| false);
    let t = Translations::get(&props.lang);

    // 新增錯誤提示的signals
    let mut choice_id_error = use_signal(|| false);
    let mut paragraphs_error = use_signal(|| false);
    let mut new_caption_error = use_signal(|| false);
    let mut new_goto_error = use_signal(|| false);

    let add_choice = move |_| {
        show_extra_options.write().push(());
        extra_captions.write().push(String::new());
        extra_gotos.write().push(String::new());
    };

    // 修改驗證函數的參數類型
    let validate_field = |value: &str, error_signal: &mut Signal<bool>| {
        if value.trim().is_empty() {
            error_signal.set(true);
        } else {
            error_signal.set(false);
        }
    };

    let handle_submit = move |evt: Event<FormData>| {
        evt.stop_propagation();
        
        if choice_id.read().trim().is_empty() {
            return;
        }
        
        if paragraphs.read().trim().is_empty() {
            return;
        }
        
        if new_caption.read().trim().is_empty() || new_goto.read().trim().is_empty() {
            return;
        }

        let mut all_choices = Vec::new();
        
        if !new_caption.read().is_empty() && !new_goto.read().is_empty() {
            all_choices.push(Choice {
                caption: new_caption.read().clone(),
                goto: new_goto.read().clone(),
            });
        }

        for i in 0..extra_captions.read().len() {
            let caption = &extra_captions.read()[i];
            let goto = &extra_gotos.read()[i];
            if !caption.is_empty() && !goto.is_empty() {
                all_choices.push(Choice {
                    caption: caption.clone(),
                    goto: goto.clone(),
                });
            }
        }

        let text = Text {
            lang: props.lang.clone(),
            paragraphs: paragraphs.read().clone(),
            choices: all_choices,
        };

        spawn_local(async move {
            let client = reqwest::Client::new();
            let url = format!("{}{}", BASE_API_URL, SETTINGS);
            
            match client.get(&url).send().await {
                Ok(response) => {
                    if let Ok(data) = response.json::<Data>().await {
                        let max_index = data.items.iter()
                            .map(|item| item.index)
                            .max()
                            .unwrap_or(0);
                        
                        let record = Paragraph {
                            index: max_index + 1,
                            choice_id: choice_id.read().clone(),
                            texts: vec![text],
                        };

                        match client.post(&url).json(&record).send().await {
                            Ok(response) => {
                                if response.status().is_success() {
                                    choice_id.set(String::new());
                                    paragraphs.set(String::new());
                                    choices.write().clear();
                                    new_caption.set(String::new());
                                    new_goto.set(String::new());
                                    extra_captions.write().clear();
                                    extra_gotos.write().clear();
                                    show_extra_options.write().clear();
                                    show_toast.set(true);
                                    
                                    let mut toast_visible = toast_visible.clone();
                                    spawn_local(async move {
                                        let window = web_sys::window().unwrap();
                                        let promise = js_sys::Promise::new(&mut |resolve, _| {
                                            window
                                                .set_timeout_with_callback_and_timeout_and_arguments_0(
                                                    &resolve,
                                                    50,
                                                )
                                                .unwrap();
                                        });
                                        let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                                        toast_visible.set(true);
                                    });
                                    
                                    let mut show_toast = show_toast.clone();
                                    let mut toast_visible = toast_visible.clone();
                                    spawn_local(async move {
                                        let window = web_sys::window().unwrap();
                                        let promise = js_sys::Promise::new(&mut |resolve, _| {
                                            window
                                                .set_timeout_with_callback_and_timeout_and_arguments_0(
                                                    &resolve,
                                                    2700,
                                                )
                                                .unwrap();
                                        });
                                        let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                                        
                                        toast_visible.set(false);
                                        
                                        let promise = js_sys::Promise::new(&mut |resolve, _| {
                                            window
                                                .set_timeout_with_callback_and_timeout_and_arguments_0(
                                                    &resolve,
                                                    300,
                                                )
                                                .unwrap();
                                        });
                                        let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                                        show_toast.set(false);
                                    });
                                }
                            }
                            Err(_) => {}
                        }
                    }
                }
                Err(_) => {}
            }
        });
    };

    // 檢查所有必填欄位是否都已填寫
    let is_form_valid = move || {
        !choice_id.read().trim().is_empty() &&
        !paragraphs.read().trim().is_empty() &&
        !new_caption.read().trim().is_empty() &&
        !new_goto.read().trim().is_empty()
    };
    
    rsx! {
        crate::pages::layout::Layout { 
            title: Some("Dashboard"),
            {show_toast.read().then(|| {
                rsx!(
                    Toast {
                        visible: *toast_visible.read(),
                        message: t.submit_success.to_string()
                    }
                )
            })}
            form { 
                class: "max-w-3xl mx-auto p-8 bg-white dark:bg-gray-800 rounded-lg shadow-lg border border-gray-100 dark:border-gray-700",
                onsubmit: handle_submit,
                "onsubmit": "event.preventDefault();",
                
                div { class: "space-y-8",
                    InputField {
                        label: t.choice_id,
                        placeholder: t.choice_id,
                        value: choice_id.read().to_string(),
                        required: true,
                        has_error: *choice_id_error.read(),
                        on_input: move |evt: FormEvent| {
                            choice_id.set(evt.value().clone());
                            validate_field(&evt.value(), &mut choice_id_error);
                        },
                        on_blur: move |evt: FocusEvent| validate_field(&choice_id.read(), &mut choice_id_error)
                    }

                    div { class: "space-y-2",
                        TextareaField {
                            label: t.paragraph,
                            placeholder: t.paragraph,
                            value: paragraphs.read().to_string(),
                            required: true,
                            has_error: *paragraphs_error.read(),
                            rows: 6,
                            on_input: move |evt: FormEvent| {
                                paragraphs.set(evt.value().clone());
                                validate_field(&evt.value(), &mut paragraphs_error);
                            },
                            on_blur: move |evt: FocusEvent| validate_field(&paragraphs.read(), &mut paragraphs_error)
                        }
                    }

                    div { class: "space-y-4",
                        label { class: "block text-gray-700 dark:text-gray-300 text-sm font-semibold mb-3",
                            "{t.options}"
                        }
                        div { class: "space-y-6",
                            div { class: "grid grid-cols-2 gap-6",
                                InputField {
                                    label: t.option_text,
                                    placeholder: t.option_text,
                                    value: new_caption.read().to_string(),
                                    required: true,
                                    has_error: *new_caption_error.read(),
                                    on_input: move |evt: FormEvent| {
                                        new_caption.set(evt.value().clone());
                                        validate_field(&evt.value(), &mut new_caption_error);
                                    },
                                    on_blur: move |evt: FocusEvent| validate_field(&new_caption.read(), &mut new_caption_error)
                                }
                                InputField {
                                    label: t.goto_target,
                                    placeholder: t.goto_target,
                                    value: new_goto.read().to_string(),
                                    required: true,
                                    has_error: *new_goto_error.read(),
                                    on_input: move |evt: FormEvent| {
                                        new_goto.set(evt.value().clone());
                                        validate_field(&evt.value(), &mut new_goto_error);
                                    },
                                    on_blur: move |evt: FocusEvent| validate_field(&new_goto.read(), &mut new_goto_error)
                                }
                            }
                        }
                    }

                    {show_extra_options.read().iter().enumerate().map(|(i, _)| {
                        rsx! {
                            div { class: "space-y-4",
                                div { class: "grid grid-cols-2 gap-6",
                                    InputField {
                                        label: t.option_text,
                                        placeholder: t.option_text,
                                        value: extra_captions.read()[i].clone(),
                                        required: false,
                                        has_error: false,
                                        on_input: move |evt: FormEvent| {
                                            let mut captions = extra_captions.write();
                                            captions[i] = evt.value().clone();
                                        },
                                        on_blur: move |_: FocusEvent| {}
                                    }
                                    InputField {
                                        label: t.goto_target,
                                        placeholder: t.goto_target,
                                        value: extra_gotos.read()[i].clone(),
                                        required: false,
                                        has_error: false,
                                        on_input: move |evt: FormEvent| {
                                            let mut gotos = extra_gotos.write();
                                            gotos[i] = evt.value().clone();
                                        },
                                        on_blur: move |_: FocusEvent| {}
                                    }
                                }
                            }
                        }
                    })}

                    div { class: "flex flex-col space-y-4",
                        button {
                            class: "px-6 py-2.5 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors duration-200 font-medium",
                            onclick: add_choice,
                            "{t.add}"
                        }

                        button {
                            class: "w-full px-6 py-3 bg-green-500 text-white rounded-md hover:bg-green-600 transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed font-medium text-lg",
                            r#type: "submit",
                            disabled: !is_form_valid(),
                            "{t.submit}"
                        }
                    }
                }
            }
        }
    }
}