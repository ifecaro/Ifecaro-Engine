use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::prelude::*;
use crate::constants::config::config::{BASE_API_URL, SETTINGS};
use crate::enums::translations::DashboardTranslations;
use crate::components::toast::Toast;
use crate::components::form::{InputField, TextareaField};
use dioxus::events::{FormEvent, FocusEvent};

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

#[derive(Clone)]
struct Translations {
    choice_id: &'static str,
    paragraph: &'static str,
    options: &'static str,
    option_text: &'static str,
    goto_target: &'static str,
    add: &'static str,
    submit: &'static str,
    submit_success: &'static str,
}

impl Translations {
    fn get(lang: &str) -> Self {
        match lang {
            "en" => Self {
                choice_id: "Choice ID",
                paragraph: "Paragraph",
                options: "Options",
                option_text: "Option Text",
                goto_target: "Go to Target",
                add: "Add",
                submit: "Submit",
                submit_success: "Successfully submitted!",
            },
            "zh-TW" => Self {
                choice_id: "Choice ID",
                paragraph: "段落",
                options: "選項",
                option_text: "選項文字",
                goto_target: "跳轉目標",
                add: "新增",
                submit: "送出",
                submit_success: "資料送出成功！",
            },
            // Default to English
            _ => Self::get("en"),
        }
    }
}

#[component]
pub fn Dashboard() -> Element {
    let mut choices = use_signal(Vec::<Choice>::new);
    let mut choice_id = use_signal(String::new);
    let mut paragraphs = use_signal(String::new);
    let mut new_caption = use_signal(String::new);
    let mut new_goto = use_signal(String::new);
    let mut extra_captions = use_signal(Vec::<String>::new);
    let mut extra_gotos = use_signal(Vec::<String>::new);
    let mut show_extra_options = use_signal(Vec::<()>::new);
    let mut show_toast = use_signal(|| false);
    let mut toast_visible = use_signal(|| false);
    let lang = use_context::<Signal<&str>>();
    let t = DashboardTranslations::get(lang());

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
            lang: lang().to_string(),
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
                class: "max-w-2xl mx-auto p-6 bg-white rounded-lg shadow-md",
                onsubmit: handle_submit,
                "onsubmit": "event.preventDefault();",
                
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

                TextareaField {
                    label: t.paragraph,
                    placeholder: t.paragraph,
                    value: paragraphs.read().to_string(),
                    required: true,
                    has_error: *paragraphs_error.read(),
                    rows: 4,
                    on_input: move |evt: FormEvent| {
                        paragraphs.set(evt.value().clone());
                        validate_field(&evt.value(), &mut paragraphs_error);
                    },
                    on_blur: move |evt: FocusEvent| validate_field(&paragraphs.read(), &mut paragraphs_error)
                }

                div { class: "mb-6",
                    label { class: "block text-gray-700 text-sm font-bold mb-2",
                        span { "{t.options}" }
                        span { class: "text-red-500 ml-1", "*" }
                    }
                    div { class: "flex gap-2 mb-2",
                        div { class: "flex-1",
                            InputField {
                                label: "",
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
                        }
                        div { class: "flex-1",
                            InputField {
                                label: "",
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
                    
                    {show_extra_options.read().iter().enumerate().map(|(i, _)| rsx!(
                        div { 
                            class: "flex gap-2 mb-2",
                            key: "{i}",
                            div { class: "flex-1",
                                InputField {
                                    label: "",
                                    placeholder: t.option_text,
                                    value: extra_captions.read()[i].clone(),
                                    required: false,
                                    has_error: false,
                                    on_input: move |evt: FormEvent| {
                                        let mut captions = extra_captions.write();
                                        captions[i] = evt.value().clone();
                                    },
                                    on_blur: move |_| {}
                                }
                            }
                            div { class: "flex-1",
                                InputField {
                                    label: "",
                                    placeholder: t.goto_target,
                                    value: extra_gotos.read()[i].clone(),
                                    required: false,
                                    has_error: false,
                                    on_input: move |evt: FormEvent| {
                                        let mut gotos = extra_gotos.write();
                                        gotos[i] = evt.value().clone();
                                    },
                                    on_blur: move |_| {}
                                }
                            }
                        }
                    ))}

                    div { class: "flex justify-end",
                        button {
                            class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline",
                            r#type: "button",
                            onclick: add_choice,
                            "{t.add}"
                        }
                    }
                }

                button {
                    class: {
                        let base_classes = "font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline";
                        if is_form_valid() {
                            format!("bg-green-500 hover:bg-green-700 text-white {}", base_classes)
                        } else {
                            format!("bg-gray-300 text-gray-200 cursor-not-allowed {}", base_classes)
                        }
                    },
                    r#type: "submit",
                    disabled: !is_form_valid(),
                    "{t.submit}"
                }
            }
        }
    }
}
