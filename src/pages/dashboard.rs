use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::prelude::*;
use crate::constants::config::config::{BASE_API_URL, SETTINGS};
use crate::enums::translations::DashboardTranslations;
use crate::components::toast::Toast;

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

    let add_choice = move |_| {
        show_extra_options.write().push(());
        extra_captions.write().push(String::new());
        extra_gotos.write().push(String::new());
    };

    let handle_submit = move |evt: Event<FormData>| {
        evt.stop_propagation();
        
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
                                    choice_id.set("".to_string());
                                    paragraphs.set("".to_string());
                                    choices.write().clear();
                                    new_caption.set("".to_string());
                                    new_goto.set("".to_string());
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

    rsx! {
        crate::pages::layout::Layout { 
            title: "Dashboard",
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
                
                div { class: "mb-6",
                    label { class: "block text-gray-700 text-sm font-bold mb-2",
                        "{t.choice_id}"
                    }
                    input {
                        class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                        r#type: "text",
                        placeholder: "{t.choice_id}",
                        value: "{choice_id}",
                        oninput: move |evt| choice_id.set(evt.value().clone())
                    }
                }

                div { class: "mb-6",
                    label { class: "block text-gray-700 text-sm font-bold mb-2",
                        "{t.paragraph}"
                    }
                    textarea {
                        class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                        rows: "4",
                        placeholder: "{t.paragraph}",
                        value: "{paragraphs}",
                        oninput: move |evt| paragraphs.set(evt.value().clone())
                    }
                }

                div { class: "mb-6",
                    label { class: "block text-gray-700 text-sm font-bold mb-2",
                        "{t.options}"
                    }
                    div { class: "flex gap-2 mb-2",
                        input {
                            class: "shadow appearance-none border rounded flex-1 py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                            r#type: "text",
                            placeholder: "{t.option_text}",
                            value: "{new_caption}",
                            oninput: move |evt| new_caption.set(evt.value().clone())
                        }
                        input {
                            class: "shadow appearance-none border rounded flex-1 py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                            r#type: "text",
                            placeholder: "{t.goto_target}",
                            value: "{new_goto}",
                            oninput: move |evt| new_goto.set(evt.value().clone())
                        }
                    }
                    
                    {show_extra_options.read().iter().enumerate().map(|(i, _)| rsx!(
                        div { 
                            class: "flex gap-2 mb-2",
                            key: "{i}",
                            input {
                                class: "shadow appearance-none border rounded flex-1 py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                                r#type: "text",
                                placeholder: "{t.option_text}",
                                value: "{extra_captions.read()[i]}",
                                oninput: move |evt| {
                                    let mut captions = extra_captions.write();
                                    captions[i] = evt.value().clone();
                                }
                            }
                            input {
                                class: "shadow appearance-none border rounded flex-1 py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                                r#type: "text",
                                placeholder: "{t.goto_target}",
                                value: "{extra_gotos.read()[i]}",
                                oninput: move |evt| {
                                    let mut gotos = extra_gotos.write();
                                    gotos[i] = evt.value().clone();
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
                    class: "bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline",
                    r#type: "submit",
                    "{t.submit}"
                }
            }
        }
    }
}
