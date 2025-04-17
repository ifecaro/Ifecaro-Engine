use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use crate::enums::translations::Translations;
use crate::components::form::{TextareaField, ChoiceOptions};
use dioxus::events::FormEvent;
use crate::components::story_content::{Choice, Action};
use crate::components::dropdown::Dropdown;
use crate::components::translation_form::{Paragraph, Text};
use crate::components::chapter_selector::ChapterSelector;
use dioxus::hooks::use_context;
use crate::contexts::language_context::LanguageState;
use std::cell::RefCell;
use std::thread_local;
use crate::components::language_selector::{Language, AVAILABLE_LANGUAGES};
use std::env;
use crate::constants::config::{BASE_API_URL, PARAGRAPHS, CHAPTERS};
use web_sys::window;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

thread_local! {
    static CURRENT_LANGUAGE: RefCell<String> = RefCell::new(String::from("zh-TW"));
}

#[derive(Props, Clone, PartialEq)]
pub struct DashboardProps {
    pub lang: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {
    pub items: Vec<Paragraph>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Chapter {
    pub id: String,
    pub titles: Vec<ChapterTitle>,
    pub order: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChapterTitle {
    pub lang: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Collection {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub collection_type: String,
    pub system: bool,
    pub schema: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollectionsData {
    pub items: Vec<Collection>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChapterData {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub chapter_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChaptersData {
    pub items: Vec<ChapterData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChapterInfo {
    pub id: String,
    pub titles: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemData {
    pub id: String,
    pub key: String,
    #[serde(rename = "value")]
    pub value_raw: serde_json::Value,
    #[serde(rename = "collectionId")]
    pub collection_id: String,
    #[serde(rename = "collectionName")]
    pub collection_name: String,
    pub created: String,
    pub updated: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemDataResponse {
    pub items: Vec<SystemData>,
    pub page: i32,
    #[serde(rename = "perPage")]
    pub per_page: i32,
    #[serde(rename = "totalItems")]
    pub total_items: i32,
    #[serde(rename = "totalPages")]
    pub total_pages: i32,
}

#[allow(dead_code)]
struct ChoiceOption {
    id: String,
    preview: String,
}

fn display_language(lang: &&Language) -> String {
    lang.name.to_string()
}

#[allow(non_snake_case)]
pub fn Dashboard(_props: DashboardProps) -> Element {
    let language_state = use_context::<Signal<LanguageState>>();
    let current_lang = language_state.read().current_language.clone();
    
    // 更新 thread_local 變量
    CURRENT_LANGUAGE.with(|lang| {
        *lang.borrow_mut() = current_lang.clone();
    });

    // 在語言變更時更新 thread_local 變量
    use_effect(move || {
        CURRENT_LANGUAGE.with(|lang| {
            *lang.borrow_mut() = language_state.read().current_language.clone();
        });
        
        (move || {})()
    });

    let mut paragraphs = use_signal(|| String::new());
    let mut new_caption = use_signal(|| String::new());
    let mut new_goto = use_signal(|| String::new());
    let mut extra_captions = use_signal(|| Vec::<String>::new());
    let mut extra_gotos = use_signal(|| Vec::<String>::new());
    let mut show_extra_options = use_signal(|| Vec::<()>::new());
    let mut show_toast = use_signal(|| false);
    let mut toast_visible = use_signal(|| false);
    let _init_done = use_signal(|| false);
    let mut is_open = use_signal(|| false);
    let mut search_query = use_signal(|| String::new());
    let mut is_paragraph_open = use_signal(|| false);
    let mut paragraph_search_query = use_signal(|| String::new());
    let mut available_paragraphs = use_signal(|| Vec::<crate::components::paragraph_list::Paragraph>::new());
    let available_chapters = use_signal(|| Vec::<Chapter>::new());
    let mut selected_chapter = use_signal(|| String::new());
    let mut is_chapter_open = use_signal(|| false);
    let mut chapter_search_query = use_signal(|| String::new());
    let mut selected_paragraph = use_signal(|| None::<Paragraph>);
    let mut is_edit_mode = use_signal(|| false);
    let mut paragraph_data = use_signal(|| Vec::<Paragraph>::new());
    let t = Translations::get(&current_lang);
    let mut should_scroll = use_signal(|| false);
    let mut paragraph_language = use_signal(|| current_lang.clone());

    let mut paragraphs_error = use_signal(|| false);
    let mut new_caption_error = use_signal(|| false);
    let mut new_goto_error = use_signal(|| false);
    let mut chapter_error = use_signal(|| false);
    let has_loaded = use_signal(|| false);
    let error_toast_visible = use_signal(|| false);

    let mut new_action_type = use_signal(|| String::new());
    let mut new_action_key = use_signal(|| None::<String>);
    let mut new_action_value = use_signal(|| None::<serde_json::Value>);
    let mut extra_action_types = use_signal(|| Vec::<String>::new());
    let mut extra_action_keys = use_signal(|| Vec::<Option<String>>::new());
    let mut extra_action_values = use_signal(|| Vec::<Option<serde_json::Value>>::new());

    let mut show_error_toast = use_signal(|| false);
    let mut error_message = use_signal(|| String::new());

    let mut update_paragraph_previews = move || {
        let selected_lang = paragraph_language.read().clone();
        
        if paragraph_data.read().is_empty() {
            return;
        }
        
        let paragraphs: Vec<crate::components::paragraph_list::Paragraph> = paragraph_data.read().iter()
            .map(|item| {
                // 取得段落的第一行作為預覽
                let preview = if let Some(text) = item.texts.first() {
                    // 使用第一個可用的翻譯的第一行
                    text.paragraphs.lines().next().unwrap_or("").to_string()
                } else {
                    // 如果沒有任何翻譯，顯示段落 ID
                    format!("[{}]", item.id)
                };
                
                crate::components::paragraph_list::Paragraph {
                    id: item.id.clone(),
                    preview,
                }
            })
            .collect();
        
        available_paragraphs.set(paragraphs);
    };

    // 載入章節列表
    use_effect(move || {
        let chapters_url = format!("{}{}", BASE_API_URL, CHAPTERS);
        let client = reqwest::Client::new();
        let mut available_chapters = available_chapters.clone();
        
        wasm_bindgen_futures::spawn_local(async move {
            let auth_token = env::var("AUTH_TOKEN").unwrap_or_else(|_| "YOUR_AUTH_TOKEN".to_string());
            match client.get(&chapters_url)
                .header("Authorization", format!("Bearer {}", auth_token))
                .send()
                .await {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<serde_json::Value>().await {
                            Ok(chapters_data) => {
                                if let Some(items) = chapters_data.get("items").and_then(|i| i.as_array()) {
                                    let chapters: Vec<Chapter> = items.iter()
                                        .filter_map(|item| {
                                            let id = item.get("id")?.as_str()?.to_string();
                                            let titles = item.get("titles")?.as_array()?
                                                .iter()
                                                .filter_map(|title_obj| {
                                                    let lang = title_obj.get("lang")?.as_str()?.to_string();
                                                    let title = title_obj.get("title")?.as_str()?.to_string();
                                                    Some(ChapterTitle { lang, title })
                                                })
                                                .collect();
                                            let order = item.get("order")?.as_i64().unwrap_or(0) as i32;
                                            
                                            Some(Chapter { id, titles, order })
                                        })
                                        .collect();
                                    
                                    // 按 order 排序
                                    let mut sorted_chapters = chapters;
                                    sorted_chapters.sort_by(|a, b| a.order.cmp(&b.order));
                                    
                                    available_chapters.set(sorted_chapters);
                                }
                            }
                            Err(_) => {}
                        }
                    }
                }
                Err(_) => {}
            }
        });
        
        (move || {})()
    });

    // 載入段落數據
    use_effect(move || {
        let paragraphs_url = format!("{}{}", BASE_API_URL, PARAGRAPHS);
        let client = reqwest::Client::new();
        let mut has_loaded = has_loaded.clone();
        let mut paragraph_data = paragraph_data.clone();
        let mut update_paragraph_previews = update_paragraph_previews.clone();

        wasm_bindgen_futures::spawn_local(async move {
            // 載入段落
            let auth_token = env::var("AUTH_TOKEN").unwrap_or_else(|_| "YOUR_AUTH_TOKEN".to_string());
            match client.get(&paragraphs_url)
                .header("Authorization", format!("Bearer {}", auth_token))
                .send()
                .await {
                Ok(response) => {
                    match response.json::<Data>().await {
                        Ok(data) => {
                            paragraph_data.set(data.items.clone());
                            update_paragraph_previews();
                            has_loaded.set(true);
                        }
                        Err(_e) => {}
                    }
                }
                Err(_) => {}
            }
        });
        
        (move || {})()
    });

    let filtered_languages = use_memo(move || {
        let query = search_query.read().to_lowercase();
        AVAILABLE_LANGUAGES.iter()
            .filter(|l| {
                l.name.to_lowercase().contains(&query) || 
                l.code.to_lowercase().contains(&query)
            })
            .collect::<Vec<_>>()
    });

    let _dropdown_class = use_memo(move || {
        if *is_open.read() {
            "translate-y-0 opacity-100"
        } else {
            "-translate-y-2 opacity-0 pointer-events-none"
        }
    });

    let current_language = use_memo(move || {
        AVAILABLE_LANGUAGES.iter()
            .find(|l| l.code == *paragraph_language.read())
            .map(|l| l.name)
            .unwrap_or("繁體中文")
    });

    let is_form_valid = use_memo(move || {
        // 檢查主要欄位
        let main_fields_valid = !paragraphs.read().trim().is_empty() &&
            !new_caption.read().trim().is_empty() &&
            !new_goto.read().trim().is_empty();

        // 檢查額外選項（只有在有額外選項時才檢查）
        let extra_choices_valid = if !extra_captions.read().is_empty() {
            extra_captions.read().iter().zip(extra_gotos.read().iter())
                .all(|(caption, goto)| !caption.trim().is_empty() && !goto.trim().is_empty())
        } else {
            true
        };

        main_fields_valid && extra_choices_valid
    });

    let has_changes = use_memo(move || {
        if *is_edit_mode.read() {
            // 如果是編輯模式，檢查是否有任何欄位被修改
            let paragraphs_changed = paragraphs.read().to_string() != selected_paragraph.read().as_ref().map(|p| p.texts.iter().find(|t| t.lang == *paragraph_language.read()).map(|t| t.paragraphs.clone()).unwrap_or_default()).unwrap_or_default();
            let choices_changed = !new_caption.read().trim().is_empty() || !new_goto.read().trim().is_empty() || !extra_captions.read().is_empty() || !extra_gotos.read().is_empty();
            paragraphs_changed || choices_changed
        } else {
            // 如果是新翻譯，只要有任何內容就表示有變化
            true  // 在新增模式下，我們總是認為有變更，因為這是一個新的段落
        }
    });

    let validate_field = |value: &str, error_signal: &mut Signal<bool>| {
        if value.trim().is_empty() {
            error_signal.set(true);
        } else {
            error_signal.set(false);
        }
    };

    let handle_submit = move |_| {
        if !*is_form_valid.read() {
            return;
        }
        
        let mut choices = Vec::new();
        let current_lang = language_state.read().current_language.clone();
        
        let main_choice = Choice {
                caption: new_caption.read().clone(),
            action: Action {
                type_: new_action_type.read().clone(),
                key: new_action_key.read().clone(),
                value: new_action_value.read().clone(),
                to: new_goto.read().clone(),
            },
        };
        choices.push(main_choice);

        for ((((caption, goto), action_type), action_key), action_value) in extra_captions.read().iter()
            .zip(extra_gotos.read().iter())
            .zip(extra_action_types.read().iter())
            .zip(extra_action_keys.read().iter())
            .zip(extra_action_values.read().iter())
        {
            let choice = Choice {
                    caption: caption.clone(),
                action: Action {
                    type_: action_type.clone(),
                    key: action_key.clone(),
                    value: action_value.clone(),
                    to: goto.clone(),
                },
            };
            choices.push(choice);
        }

        let text = Text {
            lang: paragraph_language.read().clone(),
            paragraphs: paragraphs.read().clone(),
            choices: choices.clone(),
        };

        spawn_local(async move {
            let client = reqwest::Client::new();
            
            // 檢查是否選擇了章節
            if selected_chapter.read().is_empty() {
                error_message.set("請選擇章節".to_string());
                show_error_toast.set(true);
                let mut error_toast_visible = error_toast_visible.clone();
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
                    error_toast_visible.set(true);

                    // 3秒後隱藏 toast
                    let promise = js_sys::Promise::new(&mut |resolve, _| {
                        window
                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                &resolve,
                                3000,
                            )
                            .unwrap();
                    });
                    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                    error_toast_visible.set(false);
                });
                return;
            }
            
            // 計算新段落的 index
            let max_index = paragraph_data.read().iter()
                .map(|p| p.index)
                .max()
                .unwrap_or(0);
            let new_index = max_index + 1;
            
            // 建立新的段落資料
            let chapter_id = selected_chapter.read().clone();
            
            // 建立新的段落資料
            let new_paragraph = serde_json::json!({
                "chapter_id": chapter_id,
                "index": new_index,
                "texts": [text]
            });
            
            // 發布到段落集合
            let paragraphs_url = format!("{}{}", BASE_API_URL, PARAGRAPHS);
            
            match client.post(&paragraphs_url)
                .json(&new_paragraph)
                .send()
                .await {
                Ok(response) => {
                    let status = response.status();
                    if status.is_success() {
                        // 重新載入段落資料
                        let paragraphs_url = format!("{}{}", BASE_API_URL, PARAGRAPHS);
                        match client.get(&paragraphs_url)
                            .send()
                            .await {
                            Ok(response) => {
                                if response.status().is_success() {
                                    match response.json::<Data>().await {
                                        Ok(data) => {
                                            paragraph_data.set(data.items);
                                            update_paragraph_previews();
                                        }
                                        Err(_) => {}
                                    }
                                }
                            }
                            Err(_) => {}
                        }

                        paragraphs.set(String::new());
                        choices.clear();
                        new_caption.set(String::new());
                        new_goto.set(String::new());
                        new_action_type.set(String::new());
                        new_action_key.set(None);
                        new_action_value.set(None);
                        extra_captions.write().clear();
                        extra_gotos.write().clear();
                        extra_action_types.write().clear();
                        extra_action_keys.write().clear();
                        extra_action_values.write().clear();
                        show_extra_options.write().clear();
                        selected_chapter.set(String::new());
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

                            // 3秒後隱藏 toast
                            let promise = js_sys::Promise::new(&mut |resolve, _| {
                                window
                                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                                        &resolve,
                                        3000,
                                    )
                                    .unwrap();
                            });
                            let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                            toast_visible.set(false);
                        });
                    } else {
                        match response.text().await {
                            Ok(error_text) => {
                                error_message.set(format!("伺服器錯誤: {}", error_text));
                                show_error_toast.set(true);
                                let mut error_toast_visible = error_toast_visible.clone();
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
                                    error_toast_visible.set(true);

                                    // 3秒後隱藏 toast
                                    let promise = js_sys::Promise::new(&mut |resolve, _| {
                                        window
                                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                                &resolve,
                                                3000,
                                            )
                                            .unwrap();
                                    });
                                    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                                    error_toast_visible.set(false);
                                });
                            }
                            Err(_) => {
                                error_message.set(format!("伺服器錯誤: {}", status));
                                show_error_toast.set(true);
                                let mut error_toast_visible = error_toast_visible.clone();
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
                                    error_toast_visible.set(true);

                                    // 3秒後隱藏 toast
                                    let promise = js_sys::Promise::new(&mut |resolve, _| {
                                        window
                                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                                &resolve,
                                                3000,
                                            )
                                            .unwrap();
                                    });
                                    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                                    error_toast_visible.set(false);
                                });
                            }
                        }
                    }
                }
                Err(e) => {
                    error_message.set(format!("網路錯誤: {}", e));
                    show_error_toast.set(true);
                    let mut error_toast_visible = error_toast_visible.clone();
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
                        error_toast_visible.set(true);

                        // 3秒後隱藏 toast
                        let promise = js_sys::Promise::new(&mut |resolve, _| {
                            window
                                .set_timeout_with_callback_and_timeout_and_arguments_0(
                                    &resolve,
                                    3000,
                                )
                                .unwrap();
                        });
                        let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                        error_toast_visible.set(false);
                    });
                }
            }
        });
    };

    let handle_add_translation = {
        let language_state = language_state.clone();
        move |_| {
            if let Some(paragraph) = selected_paragraph.read().as_ref() {
                let current_lang = language_state.read().current_language.clone();
                let mut updated_texts = paragraph.texts.clone();

                let mut choices = Vec::new();
                
                let main_choice = Choice {
                    caption: new_caption.read().clone(),
                    action: Action {
                        type_: new_action_type.read().clone(),
                        key: new_action_key.read().clone(),
                        value: new_action_value.read().clone(),
                        to: new_goto.read().clone(),
                    },
                };
                choices.push(main_choice);

                for ((((caption, goto), action_type), action_key), action_value) in extra_captions.read().iter()
                    .zip(extra_gotos.read().iter())
                    .zip(extra_action_types.read().iter())
                    .zip(extra_action_keys.read().iter())
                    .zip(extra_action_values.read().iter())
                {
                    let choice = Choice {
                        caption: caption.clone(),
                        action: Action {
                            type_: action_type.clone(),
                            key: action_key.clone(),
                            value: action_value.clone(),
                            to: goto.clone(),
                        },
                    };
                    choices.push(choice);
                }

                updated_texts.push(Text {
                    lang: paragraph_language.read().clone(),
                    paragraphs: paragraphs.read().clone(),
                    choices: choices.clone(),
                });

                let updated_paragraph = serde_json::json!({
                    "texts": updated_texts
                });

                let client = reqwest::Client::new();
                let paragraphs_url = format!("{}{}/{}", BASE_API_URL, PARAGRAPHS, paragraph.id);

                spawn_local(async move {
                    match client.patch(&paragraphs_url)
                        .json(&updated_paragraph)
                        .send()
                        .await {
                        Ok(response) => {
                            if response.status().is_success() {
                                paragraphs.set(String::new());
                                choices.clear();
                                new_caption.set(String::new());
                                new_goto.set(String::new());
                                new_action_type.set(String::new());
                                new_action_key.set(None);
                                new_action_value.set(None);
                                extra_captions.write().clear();
                                extra_gotos.write().clear();
                                extra_action_types.write().clear();
                                extra_action_keys.write().clear();
                                extra_action_values.write().clear();
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

                                    // 3秒後隱藏 toast
                                    let promise = js_sys::Promise::new(&mut |resolve, _| {
                                        window
                                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                                &resolve,
                                                3000,
                                            )
                                            .unwrap();
                                    });
                                    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                                    toast_visible.set(false);
                                });
                            }
                        }
                        Err(_) => {}
                    }
                });
            }
        }
    };

    let mut handle_paragraph_select = move |id: String| {
        // 從完整的段落資料中尋找選中的段落
        if let Some(paragraph) = paragraph_data.read().iter().find(|p| p.id == id) {
            selected_paragraph.set(Some(paragraph.clone()));
            
            // 使用選擇的語言而不是界面語言
            let selected_lang = paragraph_language.read().clone();

            // 檢查是否有已存在的翻譯，使用精確匹配
            if let Some(existing_text) = paragraph.texts.iter().find(|text| text.lang == selected_lang) {
                // 填充段落內容
                paragraphs.set(existing_text.paragraphs.clone());
                
                // 填充選項
                if !existing_text.choices.is_empty() {
                    // 設置第一個選項
                    new_caption.set(existing_text.choices[0].caption.clone());
                    new_goto.set(existing_text.choices[0].action.to.clone());
                    new_action_type.set(existing_text.choices[0].action.type_.clone());
                    new_action_key.set(existing_text.choices[0].action.key.clone());
                    new_action_value.set(existing_text.choices[0].action.value.clone());
                    
                    // 設置額外選項
                    let mut captions = Vec::new();
                    let mut gotos = Vec::new();
                    let mut action_types = Vec::new();
                    let mut action_keys = Vec::new();
                    let mut action_values = Vec::new();
                    let mut options = Vec::new();
                    
                    for choice in existing_text.choices.iter().skip(1) {
                        captions.push(choice.caption.clone());
                        gotos.push(choice.action.to.clone());
                        action_types.push(choice.action.type_.clone());
                        action_keys.push(choice.action.key.clone());
                        action_values.push(choice.action.value.clone());
                        options.push(());
                    }
                    
                    // 確保所有向量都有相同的長度
                    let len = captions.len();
                    if len > 0 {
                        extra_captions.set(captions);
                        extra_gotos.set(gotos);
                        extra_action_types.set(action_types);
                        extra_action_keys.set(action_keys);
                        extra_action_values.set(action_values);
                        show_extra_options.set(options);
                    } else {
                        // 如果沒有額外選項，清空所有向量
                        extra_captions.set(Vec::new());
                        extra_gotos.set(Vec::new());
                        extra_action_types.set(Vec::new());
                        extra_action_keys.set(Vec::new());
                        extra_action_values.set(Vec::new());
                        show_extra_options.set(Vec::new());
                    }
                } else {
                    // 如果沒有選項，清空所有向量
                    new_caption.set(String::new());
                    new_goto.set(String::new());
                    new_action_type.set(String::new());
                    new_action_key.set(None);
                    new_action_value.set(None);
                    extra_captions.set(Vec::new());
                    extra_gotos.set(Vec::new());
                    extra_action_types.set(Vec::new());
                    extra_action_keys.set(Vec::new());
                    extra_action_values.set(Vec::new());
                    show_extra_options.set(Vec::new());
                }
            } else {
                // 如果沒有當前語言的翻譯，清空所有欄位
                paragraphs.set(String::new());
                new_caption.set(String::new());
                new_goto.set(String::new());
                new_action_type.set(String::new());
                new_action_key.set(None);
                new_action_value.set(None);
                extra_captions.set(Vec::new());
                extra_gotos.set(Vec::new());
                extra_action_types.set(Vec::new());
                extra_action_keys.set(Vec::new());
                extra_action_values.set(Vec::new());
                show_extra_options.set(Vec::new());
            }
        }
    };

    let handle_add_choice = move |_| {
        show_extra_options.write().push(());
        extra_captions.write().push(String::new());
        extra_gotos.write().push(String::new());
        extra_action_types.write().push(String::new());
        extra_action_keys.write().push(None);
        extra_action_values.write().push(None);
        should_scroll.set(true);
    };

    let handle_remove_choice = move |index: usize| {
        let mut captions = extra_captions.write();
        let mut gotos = extra_gotos.write();
        let mut action_types = extra_action_types.write();
        let mut action_keys = extra_action_keys.write();
        let mut action_values = extra_action_values.write();
        let mut options = show_extra_options.write();

        captions.remove(index);
        gotos.remove(index);
        action_types.remove(index);
        action_keys.remove(index);
        action_values.remove(index);
        options.remove(index);
    };

    // 處理 toast 顯示
    use_effect(move || {
        if *show_toast.read() {
            toast_visible.set(true);
            let window = window().unwrap();
            let closure = Closure::wrap(Box::new(move || {
                toast_visible.set(false);
                show_toast.set(false);
            }) as Box<dyn FnMut()>);
            
            let timeout = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                3000,
            ).unwrap();
            
            (move || {
                window.clear_timeout_with_handle(timeout);
                closure.forget(); // 防止 closure 被過早釋放
            })()
        }
    });

    rsx! {
        crate::pages::layout::Layout { 
            title: Some("Dashboard"),
            div { 
                class: "min-h-screen bg-gray-50 dark:bg-gray-900",
                // Toast 區域
                div {
                    class: "fixed bottom-4 right-4 z-50",
                    // 成功 Toast
                    div {
                        class: format!("bg-green-500 text-white px-6 py-3 rounded-lg shadow-lg transition-all duration-300 transform {}",
                            if *toast_visible.read() {
                                "translate-y-0 opacity-100"
                            } else {
                                "translate-y-2 opacity-0 hidden"
                            }
                        ),
                        "{t.submit_success}"
                    }
                    // 錯誤 Toast
                    div {
                        class: format!("bg-red-500 text-white px-6 py-3 rounded-lg shadow-lg transition-all duration-300 transform {}",
                            if *error_toast_visible.read() {
                                "translate-y-0 opacity-100"
                            } else {
                                "translate-y-2 opacity-0 hidden"
                            }
                        ),
                        "{error_message.read()}"
                    }
                }
                div {
                    class: "w-full mx-auto",
                    // 主要內容區域
                    div { 
                        class: "space-y-4 bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700",
                        // 表單區域
                        div {
                            class: "p-3 sm:p-4 md:p-6 lg:p-8 space-y-4 sm:space-y-6",
                            // 語言和章節選擇區域
                            div { 
                                class: "grid grid-cols-1 md:grid-cols-2 gap-4 sm:gap-6 lg:gap-8",
                                // 語言選擇
                                div {
                                    class: "w-full",
                                    Dropdown {
                                        label: t.select_language,
                                        value: current_language.read().to_string(),
                                        options: filtered_languages.read().clone(),
                                        is_open: *is_open.read(),
                                        search_query: search_query.read().to_string(),
                                        on_toggle: move |_| {
                                            let current = *is_open.read();
                                            is_open.set(!current);
                                        },
                                        on_search: move |query| search_query.set(query),
                                        on_select: move |lang: &Language| {
                                            let current_lang = lang.code.to_string();
                                            paragraph_language.set(current_lang.clone());
                                            update_paragraph_previews();
                                            is_open.set(false);
                                            search_query.set(String::new());
                                        },
                                        display_fn: display_language,
                                        has_error: false,
                                        search_placeholder: t.search_language,
                                    }
                                }

                                // 章節選擇器
                                div {
                                    class: "w-full",
                                    ChapterSelector {
                                        key: format!("chapter-dropdown-{}", paragraph_language.read()),
                                        label: t.select_chapter,
                                        value: selected_chapter.read().clone(),
                                        chapters: available_chapters.read().clone(),
                                        is_open: *is_chapter_open.read(),
                                        search_query: chapter_search_query.read().to_string(),
                                        on_toggle: move |_| {
                                            let current = *is_chapter_open.read();
                                            is_chapter_open.set(!current);
                                        },
                                        on_search: move |query| chapter_search_query.set(query),
                                        on_select: move |chapter: Chapter| {
                                            selected_chapter.set(chapter.id.clone());
                                            is_chapter_open.set(false);
                                            chapter_search_query.set(String::new());
                                            validate_field(&chapter.id, &mut chapter_error);
                                        },
                                        has_error: *chapter_error.read(),
                                    }
                                }
                            }

                            // 編輯/新增段落區域
                            div {
                                class: "pt-6 border-t border-gray-200 dark:border-gray-700",
                                div { 
                                    class: "w-full",
                                    div {
                                        class: "flex flex-col sm:flex-row items-start sm:items-end space-y-2 sm:space-y-0 sm:space-x-4",
                                        div { 
                                            class: "w-full sm:flex-1",
                                            crate::components::paragraph_list::ParagraphList {
                                                label: t.select_paragraph,
                                                value: selected_paragraph.read().as_ref().map(|p| p.id.clone()).unwrap_or(t.select_paragraph.to_string()),
                                                paragraphs: available_paragraphs.read().clone(),
                                                is_open: *is_paragraph_open.read(),
                                                search_query: paragraph_search_query.read().to_string(),
                                                on_toggle: move |_| {
                                                    if *is_edit_mode.read() {
                                                        let current = *is_paragraph_open.read();
                                                        is_paragraph_open.set(!current);
                                                    }
                                                },
                                                on_search: move |query| {
                                                    if *is_edit_mode.read() {
                                                        paragraph_search_query.set(query);
                                                    }
                                                },
                                                on_select: move |id| {
                                                    if *is_edit_mode.read() {
                                                        handle_paragraph_select(id);
                                                    }
                                                },
                                                has_error: false,
                                                disabled: !*is_edit_mode.read(),
                                                t: t.clone(),
                                            }
                                        }

                                        button {
                                            class: "w-full md:w-10 h-10 inline-flex items-center justify-center rounded-lg text-white bg-blue-600 hover:bg-blue-700 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-500 dark:hover:bg-blue-600 dark:focus:ring-blue-800",
                                            onclick: move |_| {
                                                let current_mode = *is_edit_mode.read();
                                                is_edit_mode.set(!current_mode);
                                                if current_mode {
                                                    // 退出編輯模式時清空所有欄位
                                                    paragraphs.set(String::new());
                                                    new_caption.set(String::new());
                                                    new_goto.set(String::new());
                                                    new_action_type.set(String::new());
                                                    new_action_key.set(None);
                                                    new_action_value.set(None);
                                                    extra_captions.write().clear();
                                                    extra_gotos.write().clear();
                                                    extra_action_types.write().clear();
                                                    extra_action_keys.write().clear();
                                                    extra_action_values.write().clear();
                                                    show_extra_options.write().clear();
                                                    selected_paragraph.set(None);
                                                }
                                            },
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
                                                    d: if *is_edit_mode.read() {
                                                        "M12 4v16m8-8H4"
                                                    } else {
                                                        "M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // 段落內容區域
                            div { 
                                class: "w-full",
                                TextareaField {
                                    label: t.paragraph_content,
                                    placeholder: t.paragraph_content,
                                    value: paragraphs.read().to_string(),
                                    required: true,
                                    has_error: *paragraphs_error.read(),
                                    rows: 5,
                                    on_input: move |event: FormEvent| {
                                        let value = event.value().clone();
                                        paragraphs.set(value.clone());
                                        validate_field(&value, &mut paragraphs_error);
                                    },
                                    on_blur: move |_| validate_field(&paragraphs.read(), &mut paragraphs_error)
                                }
                            }

                            // 選項區域
                            div {
                                class: "w-full",
                                ChoiceOptions {
                                    t: t.clone(),
                                    new_caption: new_caption.read().to_string(),
                                    new_goto: new_goto.read().to_string(),
                                    new_action_type: new_action_type.read().clone(),
                                    new_action_key: new_action_key.read().clone(),
                                    new_action_value: new_action_value.read().clone(),
                                    extra_captions: extra_captions.read().clone(),
                                    extra_gotos: extra_gotos.read().clone(),
                                    extra_action_types: extra_action_types.read().clone(),
                                    extra_action_keys: extra_action_keys.read().clone(),
                                    extra_action_values: extra_action_values.read().clone(),
                                    new_caption_error: *new_caption_error.read(),
                                    new_goto_error: *new_goto_error.read(),
                                    available_paragraphs: available_paragraphs.read().clone(),
                                    on_new_caption_change: move |value: String| {
                                        new_caption.set(value.clone());
                                        validate_field(&value, &mut new_caption_error);
                                    },
                                    on_new_goto_change: move |value: String| {
                                        new_goto.set(value.clone());
                                        validate_field(&value, &mut new_goto_error);
                                    },
                                    on_new_action_type_change: move |value| new_action_type.set(value),
                                    on_new_action_key_change: move |value| new_action_key.set(value),
                                    on_new_action_value_change: move |value| new_action_value.set(value),
                                    on_extra_caption_change: move |(i, value): (usize, String)| {
                                        let mut captions = extra_captions.write();
                                        captions[i] = value;
                                    },
                                    on_extra_goto_change: move |(i, value): (usize, String)| {
                                        let mut gotos = extra_gotos.write();
                                        gotos[i] = value;
                                    },
                                    on_extra_action_type_change: move |(i, value): (usize, String)| {
                                        let mut types = extra_action_types.write();
                                        types[i] = value;
                                    },
                                    on_extra_action_key_change: move |(i, value): (usize, Option<String>)| {
                                        let mut keys = extra_action_keys.write();
                                        keys[i] = value;
                                    },
                                    on_extra_action_value_change: move |(i, value): (usize, Option<serde_json::Value>)| {
                                        let mut values = extra_action_values.write();
                                        values[i] = value;
                                    },
                                    on_add_choice: handle_add_choice,
                                    on_remove_choice: handle_remove_choice
                                }
                            }
                        }

                        // 提交按鈕區域
                        div {
                            class: "px-4 sm:px-6 py-4 bg-gray-50 dark:bg-gray-700/50 border-t border-gray-200 dark:border-gray-700",
                            button {
                                class: "w-full inline-flex justify-center items-center px-4 sm:px-6 py-2.5 sm:py-3 bg-green-600 text-white rounded-lg hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed font-medium text-base sm:text-lg shadow-sm",
                                disabled: !*is_form_valid.read() || (*is_edit_mode.read() && selected_paragraph.read().is_none()) || !*has_changes.read(),
                                onclick: move |_| {
                                    if *is_edit_mode.read() {
                                        handle_add_translation(());
                                    } else {
                                        handle_submit(());
                                    }
                                },
                                "{t.submit}"
                            }
                        }
                    }
                }
            }
        }
    }
}