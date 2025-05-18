use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use dioxus::events::FormEvent;
use crate::components::dropdown::Dropdown;
use crate::components::chapter_selector::ChapterSelector;
use dioxus::hooks::use_context;
use crate::contexts::language_context::LanguageState;
use crate::contexts::chapter_context::ChapterState;
use crate::contexts::paragraph_context::{ParagraphState, Paragraph as ContextParagraph, Text as ContextText, ParagraphChoice as ContextParagraphChoice};
use std::cell::RefCell;
use std::thread_local;
use crate::components::language_selector::{Language, AVAILABLE_LANGUAGES};
use crate::constants::config::{BASE_API_URL, PARAGRAPHS};
use std::rc::Rc;
use crate::components::paragraph_list::Paragraph as ParagraphListParagraph;
use dioxus_i18n::t;
use crate::components::form::{TextareaField, ChoiceOptions};
use gloo_timers::callback::Timeout;
use crate::contexts::chapter_context::Chapter;

thread_local! {
    static CURRENT_LANGUAGE: RefCell<String> = RefCell::new(String::from("zh-TW"));
}

#[derive(Props, Clone, PartialEq)]
pub struct DashboardProps {
    pub lang: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {
    pub items: Vec<ContextParagraph>,
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
    let chapter_state = use_context::<Signal<ChapterState>>();
    let paragraph_state = use_context::<Signal<ParagraphState>>();
    let current_lang = language_state.read().current_language.clone();
    
    // 更新 thread_local 變量
    CURRENT_LANGUAGE.with(|lang| {
        *lang.borrow_mut() = current_lang.clone();
    });

    // 初始化 paragraph_language 為當前界面語言
    let mut paragraph_language = use_signal(|| current_lang.clone());

    // 在語言變更時更新 thread_local 變量和 paragraph_language
    use_effect(move || {
        let current_lang = language_state.read().current_language.clone();
        CURRENT_LANGUAGE.with(|lang| {
            *lang.borrow_mut() = current_lang.clone();
        });
        paragraph_language.set(current_lang);
        
        (move || {})()
    });

    let mut paragraphs = use_signal(|| String::new());
    let mut choices = use_signal(|| {
        let mut initial_choices = Vec::new();
        initial_choices.push((
            String::new(),
            String::new(),
            String::new(),
            None,
            None,
            String::new(),
            false,
            None,
        ));
        initial_choices
    });
    let mut choice_chapters_open = use_signal(|| vec![false]);
    let mut choice_chapters_search = use_signal(|| vec![String::new()]);
    let mut choice_paragraphs_open = use_signal(|| vec![false]);
    let mut choice_paragraphs_search = use_signal(|| vec![String::new()]);
    let mut choice_paragraphs = use_signal(|| vec![Vec::<crate::components::paragraph_list::Paragraph>::new()]);
    let mut action_type_open = use_signal(|| vec![false]);
    let _show_extra_options = use_signal(|| Vec::<()>::new());
    let mut show_toast = use_signal(|| false);
    let mut toast_visible = use_signal(|| false);
    let mut toast_animating_out = use_signal(|| false);
    let mut error_toast_visible = use_signal(|| false);
    let mut error_toast_animating_out = use_signal(|| false);
    let _init_done = use_signal(|| false);
    let mut is_open = use_signal(|| false);
    let mut search_query = use_signal(|| String::new());
    let mut is_paragraph_open = use_signal(|| false);
    let mut paragraph_search_query = use_signal(|| String::new());
    let mut available_paragraphs = use_signal(|| Vec::<crate::components::paragraph_list::Paragraph>::new());
    let _target_chapter_paragraphs = use_signal(|| Vec::<crate::components::paragraph_list::Paragraph>::new());
    let mut selected_chapter = use_signal(|| String::new());
    let mut is_chapter_open = use_signal(|| false);
    let mut chapter_search_query = use_signal(|| String::new());
    let mut selected_paragraph = use_signal(|| None::<ContextParagraph>);
    let mut is_edit_mode = use_signal(|| false);
    let _has_loaded = use_signal(|| paragraph_state.read().loaded);
    let mut _should_scroll = use_signal(|| false);
    let _target_chapter = use_signal(|| String::new());
    let _extra_target_chapters = use_signal(|| Vec::<String>::new());

    // 新增三個獨立的章節選單狀態
    let _header_chapter = use_signal(|| String::new());
    let _header_chapter_open = use_signal(|| false);
    let _header_chapter_search = use_signal(|| String::new());
    
    let _first_choice_chapter = use_signal(|| String::new());
    let _first_choice_chapter_open = use_signal(|| false);
    let _first_choice_chapter_search = use_signal(|| String::new());
    
    let _extra_choice_chapter = use_signal(|| String::new());
    let _extra_choice_chapter_open = use_signal(|| false);
    let _extra_choice_chapter_search = use_signal(|| String::new());

    let mut paragraphs_error = use_signal(|| false);
    let mut chapter_error = use_signal(|| false);

    let _new_action_type = use_signal(|| String::new());
    let _new_action_key = use_signal(|| None::<String>);
    let _new_action_value = use_signal(|| None::<serde_json::Value>);
    let _extra_action_types = use_signal(|| Vec::<String>::new());
    let _extra_action_keys = use_signal(|| Vec::<Option<String>>::new());
    let _extra_action_values = use_signal(|| Vec::<Option<serde_json::Value>>::new());
    let _extra_target_chapters = use_signal(|| Vec::<String>::new());

    let show_error_toast = use_signal(|| false);
    let error_message = use_signal(|| String::new());
    let _paragraph_previews = use_signal(|| Vec::<crate::components::paragraph_list::Paragraph>::new());

    let update_paragraph_previews = Rc::new(RefCell::new(move || {
        let selected_language = if let Ok(lang) = paragraph_language.try_read() { lang.clone() } else { return; };
        let selected_chapter_id = if let Ok(chap) = selected_chapter.try_read() { chap.clone() } else { return; };
        if selected_language.is_empty() || selected_chapter_id.is_empty() {
            if let Ok(mut ap) = available_paragraphs.try_write() { ap.clear(); }
            return;
        }

        // 從context中獲取段落數據
        let chapter_paragraphs = paragraph_state.read().get_by_chapter(&selected_chapter_id);
        
        // 將段落分成兩組
        let (translated_paragraphs, untranslated_paragraphs): (Vec<_>, Vec<_>) = chapter_paragraphs
            .iter()
            .map(|p| {
                let has_translation = p.texts.iter().any(|text| text.lang == selected_language);
                let preview = p.texts
                    .iter()
                    .find(|text| text.lang == selected_language)
                    .or_else(|| p.texts.iter().find(|text| text.lang == "en-US" || text.lang == "en-GB"))
                    .or_else(|| p.texts.first())
                    .map(|text| {
                        match text.paragraphs.lines().next() {
                            Some(line) => line.to_string(),
                            None => String::new(),
                        }
                    })
                    .unwrap_or_default();
                (crate::components::paragraph_list::Paragraph {
                    id: p.id.clone(),
                    preview,
                    has_translation,
                }, has_translation)
            })
            .partition(|(_, has_translation)| *has_translation);
        // 合併段落，先放有翻譯的，再放沒有翻譯的
        let mut all_paragraphs = translated_paragraphs.into_iter().map(|(p, _)| p).collect::<Vec<_>>();
        all_paragraphs.extend(untranslated_paragraphs.into_iter().map(|(p, _)| p));
        if let Ok(mut ap) = available_paragraphs.try_write() {
            *ap = all_paragraphs;
        }
    }));

    // 處理 toast 顯示
    use_effect(move || {
        if *show_toast.read() {
            toast_visible.set(true);
            toast_animating_out.set(false);
            let mut toast_visible = toast_visible.clone();
            let mut toast_animating_out = toast_animating_out.clone();
            let mut show_toast = show_toast.clone();
            // 3 秒後自動隱藏
            Timeout::new(3000, move || {
                // 先設置為正在退場
                toast_animating_out.set(true);
                // 等待退場動畫完成（400ms）後再隱藏
                Timeout::new(500, move || {
                    toast_visible.set(false);
                    toast_animating_out.set(false);
                    show_toast.set(false);
                }).forget();
            }).forget();
        }
    });

    // 處理錯誤 toast 顯示
    use_effect(move || {
        if *show_error_toast.read() {
            error_toast_visible.set(true);
            error_toast_animating_out.set(false);
            let mut error_toast_visible = error_toast_visible.clone();
            let mut error_toast_animating_out = error_toast_animating_out.clone();
            let mut show_error_toast = show_error_toast.clone();
            // 3 秒後自動隱藏
            Timeout::new(3000, move || {
                // 先設置為正在退場
                error_toast_animating_out.set(true);
                // 等待退場動畫完成（400ms）後再隱藏
                Timeout::new(500, move || {
                    error_toast_visible.set(false);
                    error_toast_animating_out.set(false);
                    show_error_toast.set(false);
                }).forget();
            }).forget();
        }
    });

    // 在章節選擇變更後更新可用段落列表
    {
        let update_paragraph_previews = update_paragraph_previews.clone();
        use_effect(move || {
            let _ = selected_chapter.read().clone();
            if let Ok(mut cb) = update_paragraph_previews.try_borrow_mut() {
                (*cb)();
            }
            
            (move || {})()
        });
    }

    // 在語言變更時更新段落預覽
    {
        let update_paragraph_previews = update_paragraph_previews.clone();
        use_effect(move || {
            let _ = paragraph_language.read().clone();
            if let Ok(mut cb) = update_paragraph_previews.try_borrow_mut() {
                (*cb)();
            }
            
            (move || {})()
        });
    }

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
        if let Ok(lang_code) = paragraph_language.try_read() {
            AVAILABLE_LANGUAGES.iter()
                .find(|l| l.code == *lang_code)
                .map(|l| l.name)
                .unwrap_or("繁體中文")
        } else {
            "繁體中文"
        }
    });

    let is_form_valid = use_memo(move || {
        let main_fields_valid = if let (Ok(p), Ok(c)) = (paragraphs.try_read(), selected_chapter.try_read()) {
            !p.trim().is_empty() && !c.is_empty()
        } else {
            false
        };
        let has_any_choices = if let Ok(choices) = choices.try_read() {
            !choices.is_empty()
        } else {
            false
        };
        let choices_valid = if let Ok(choices) = choices.try_read() {
            choices.iter().all(|(_choice_text, to, type_, _key, _value, _target_chapter, _same_page, _time_limit)| {
                let has_content = !to.trim().is_empty() || !type_.trim().is_empty();
                if has_content {
                    !to.trim().is_empty()
                } else {
                    true
                }
            })
        } else {
            false
        };
        main_fields_valid && (!has_any_choices || choices_valid)
    });

    let has_changes = use_memo(move || {
        if let Ok(edit_mode) = is_edit_mode.try_read() {
            if *edit_mode {
                let paragraphs_changed = if let (Ok(p), Ok(sel_para), Ok(lang)) = (paragraphs.try_read(), selected_paragraph.try_read(), paragraph_language.try_read()) {
                    p.to_string() != sel_para.as_ref()
                        .map(|p| p.texts.iter().find(|t| t.lang == *lang)
                            .map(|t| t.paragraphs.clone())
                            .unwrap_or_default())
                        .unwrap_or_default()
                } else {
                    false
                };
                let has_option_changes = if let (Ok(sel_para), Ok(lang), Ok(new_choices)) = (selected_paragraph.try_read(), paragraph_language.try_read(), choices.try_read()) {
                    if let Some(paragraph) = sel_para.as_ref() {
                        let current_choices = &paragraph.texts.iter().find(|t| t.lang == *lang)
                            .map(|t| t.choices.clone())
                            .unwrap_or_default();
                        current_choices.len() != new_choices.len() ||
                            current_choices.iter().zip(new_choices.iter()).any(|(old_choice, (choice_text, to, type_, _key, _value, _target_chapter, _same_page, _time_limit))| {
                                let has_content = !choice_text.trim().is_empty() || !to.trim().is_empty() || !type_.trim().is_empty();
                                if has_content {
                                    old_choice != choice_text || !to.is_empty() || !type_.is_empty()
                                } else {
                                    false
                                }
                            })
                    } else {
                        false
                    }
                } else {
                    false
                };
                paragraphs_changed || has_option_changes
            } else {
                let has_paragraph = if let (Ok(p), Ok(c)) = (paragraphs.try_read(), selected_chapter.try_read()) {
                    !p.trim().is_empty() && !c.is_empty()
                } else {
                    false
                };
                let has_valid_choices = if let Ok(choices) = choices.try_read() {
                    choices.iter().any(|(_choice_text, to, type_, _key, _value, _target_chapter, _same_page, _time_limit)| {
                        let has_content = !to.trim().is_empty() || !type_.trim().is_empty();
                        if has_content {
                            !to.trim().is_empty()
                        } else {
                            false
                        }
                    })
                } else {
                    false
                };
                has_paragraph || has_valid_choices
            }
        } else {
            false
        }
    });

    let validate_field = |value: &str, error_signal: &mut Signal<bool>| {
        if value.trim().is_empty() {
            error_signal.set(true);
        } else {
            error_signal.set(false);
        }
    };

    let handle_choice_change = move |(index, field, value): (usize, String, String)| {
        let mut current_choices = choices.read().clone();
        
        if let Some(choice) = current_choices.get_mut(index) {
            match field.as_str() {
                "caption" => {
                    choice.0 = value;
                },
                "goto" => {
                    choice.1 = value;
                },
                "action_type" => {
                    choice.2 = value;
                },
                "action_key" => {
                    choice.3 = Some(value);
                },
                "action_value" => {
                    choice.4 = Some(serde_json::Value::String(value));
                },
                "target_chapter" => {
                    choice.5 = value.clone();
                    let mut current = choice_chapters_open.read().clone();
                    if let Some(is_open) = current.get_mut(index) {
                        *is_open = false;
                    }
                    choice_chapters_open.set(current);
                    choice.1 = String::new();
                    choices.set(current_choices.clone());
                    let mut current_paragraphs = choice_paragraphs.read().clone();
                    while current_paragraphs.len() <= index {
                        current_paragraphs.push(Vec::new());
                    }
                    if !value.is_empty() {
                        let selected_lang = paragraph_language.read().clone();
                        // 從 context 中獲取段落
                        let chapter_paragraphs = paragraph_state.read().get_by_chapter(&value);
                        let filtered_paragraphs = chapter_paragraphs.iter()
                            .map(|item| {
                                let has_translation = item.texts.iter().any(|text| text.lang == selected_lang);
                                let preview = item.texts.iter()
                                    .find(|t| t.lang == selected_lang)
                                    .or_else(|| item.texts.iter().find(|t| t.lang == "en-US" || t.lang == "en-GB"))
                                    .or_else(|| item.texts.first())
                                    .map(|text| {
                                        match text.paragraphs.lines().next() {
                                            Some(line) => line.to_string(),
                                            None => String::new(),
                                        }
                                    })
                                    .unwrap_or_default();
                                crate::components::paragraph_list::Paragraph {
                                    id: item.id.clone(),
                                    preview,
                                    has_translation,
                                }
                            })
                            .collect::<Vec<_>>();
                        current_paragraphs[index] = filtered_paragraphs;
                    } else {
                        current_paragraphs[index] = Vec::new();
                    }
                    choice_paragraphs.set(current_paragraphs);
                    return;
                },
                "same_page" => {
                    choice.6 = value == "true";
                },
                "time_limit" => {
                    choice.7 = value.parse::<u32>().ok();
                },
                _ => {}
            }
        }
        choices.set(current_choices);
    };

    let mut handle_paragraph_select = {
        let mut selected_paragraph = selected_paragraph.clone();
        let paragraph_state = paragraph_state.clone();
        let paragraph_language = paragraph_language.clone();
        let mut paragraphs = paragraphs.clone();
        let mut choices = choices.clone();
        let mut action_type_open = action_type_open.clone();
        let mut choice_chapters_open = choice_chapters_open.clone();
        let mut choice_paragraphs_open = choice_paragraphs_open.clone();
        let mut choice_paragraphs_search = choice_paragraphs_search.clone();
        let mut choice_paragraphs = choice_paragraphs.clone();
        let available_paragraphs = available_paragraphs.clone();
        
        move |index: usize| {
            let available_paragraphs = available_paragraphs.read();
            if let Some(paragraph) = available_paragraphs.get(index) {
                // 從 context 中獲取完整的段落數據
                if let Some(full_paragraph) = paragraph_state.read().get_by_id(&paragraph.id) {
                    selected_paragraph.set(Some(full_paragraph.clone()));
                    
                    // 填充段落內容
                    if let Some(text) = full_paragraph.texts.iter().find(|t| t.lang == *paragraph_language.read()) {
                        paragraphs.set(text.paragraphs.clone());
                        
                        // 填充選項
                        let (new_choices, new_paragraphs) = process_paragraph_select(text, &full_paragraph, &paragraph_state, &paragraph_language);
                        let choices_len = new_choices.len();

                        choices.set(new_choices);
                        choice_paragraphs.set(new_paragraphs);
                        action_type_open.set(vec![false; choices_len]);
                        choice_chapters_open.set(vec![false; choices_len]);
                        choice_paragraphs_open.set(vec![false; choices_len]);
                        choice_paragraphs_search.set(vec![String::new(); choices_len]);
                    }
                }
            }
        }
    };

    let is_submitting = use_signal(|| false);

    let mut handle_submit = {
        let mut show_error_toast = show_error_toast.clone();
        let mut error_message = error_message.clone();
        let paragraph_state = paragraph_state.clone();
        let selected_paragraph = selected_paragraph.clone();
        let paragraph_language = paragraph_language.clone();
        let paragraphs = paragraphs.read().clone();
        let choices = choices.read().clone();
        let is_edit_mode = is_edit_mode.read().clone();
        let update_paragraph_previews = update_paragraph_previews.clone();
        let mut is_submitting = is_submitting.clone();
        move |_| {
            if *is_submitting.read() { return; }
            is_submitting.set(true);
            let text = ContextText {
                lang: paragraph_language.read().clone(),
                paragraphs: paragraphs.clone(),
                choices: choices.iter().map(|(choice_text, _, _, _, _, _, _, _)| {
                    choice_text.clone()
                }).collect(),
            };

            // 構建選項數據
            let paragraph_choices: Vec<ContextParagraphChoice> = choices.iter().map(|(_choice_text, to, type_, key, value, _target_chapter, same_page, time_limit)| {
                let mut complex = ContextParagraphChoice::Complex {
                    to: to.clone(),
                    type_: type_.clone(),
                    key: None,
                    value: None,
                    same_page: Some(*same_page),
                    time_limit: *time_limit,
                };
                if let Some(k) = key {
                    if !k.is_empty() {
                        if let ContextParagraphChoice::Complex { key, .. } = &mut complex {
                            *key = Some(k.to_string());
                        }
                    }
                }
                if let Some(v) = value {
                    if let ContextParagraphChoice::Complex { value, .. } = &mut complex {
                        *value = Some(v.clone());
                    }
                }
                complex
            }).collect();

            spawn_local({
                let update_paragraph_previews = update_paragraph_previews.clone();
                let mut is_submitting = is_submitting.clone();
                let mut paragraph_state = paragraph_state.clone();
                async move {
                    let client = reqwest::Client::new();
                    
                    // 建立新的段落資料
                    let chapter_id = selected_chapter.read().clone();
                    
                    // 建立新的段落資料
                    let new_paragraph = if chapter_id.is_empty() {
                        serde_json::json!({
                            "texts": if is_edit_mode {
                                // 在編輯模式下，保留所有現有的翻譯，只更新當前語言的翻譯
                                let mut existing_texts = selected_paragraph.read().as_ref().map(|p| p.texts.clone()).unwrap_or_default();
                                // 移除當前語言的舊翻譯（如果存在）
                                existing_texts.retain(|t| t.lang != *paragraph_language.read());
                                // 添加新的翻譯
                                existing_texts.push(text);
                                existing_texts
                            } else {
                                vec![text]
                            },
                            "choices": paragraph_choices
                        })
                    } else {
                        serde_json::json!({
                            "chapter_id": chapter_id,
                            "texts": if is_edit_mode {
                                // 在編輯模式下，保留所有現有的翻譯，只更新當前語言的翻譯
                                let mut existing_texts = selected_paragraph.read().as_ref().map(|p| p.texts.clone()).unwrap_or_default();
                                // 移除當前語言的舊翻譯（如果存在）
                                existing_texts.retain(|t| t.lang != *paragraph_language.read());
                                // 添加新的翻譯
                                existing_texts.push(text);
                                existing_texts
                            } else {
                                vec![text]
                            },
                            "choices": paragraph_choices
                        })
                    };
                    
                    // 發布到段落集合
                    let paragraphs_url = format!("{}{}", BASE_API_URL, PARAGRAPHS);
                    
                    let response = if is_edit_mode {
                        // 編輯模式：使用 PATCH 方法更新現有段落
                        if let Some(paragraph) = selected_paragraph.read().as_ref() {
                            let update_url = format!("{}{}/{}", BASE_API_URL, PARAGRAPHS, paragraph.id);
                            client.patch(&update_url)
                                .json(&new_paragraph)
                                .send()
                                .await
                        } else {
                            return;
                        }
                    } else {
                        // 新增模式：使用 POST 方法創建新段落
                        client.post(&paragraphs_url)
                            .json(&new_paragraph)
                            .send()
                            .await
                    };

                    match response {
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
                                                    // 更新 context 中的段落數據
                                                    paragraph_state.write().set_paragraphs(data.items.clone());
                                                    if let Ok(mut cb) = update_paragraph_previews.try_borrow_mut() {
                                                        (*cb)();
                                                    }
                                                    if let Ok(mut s) = show_toast.try_write() { *s = true; }
                                                    is_submitting.set(false);
                                                },
                                                Err(e) => {
                                                    if let Ok(mut s) = show_error_toast.try_write() { *s = true; }
                                                    if let Ok(mut s) = error_message.try_write() { *s = format!("解析段落數據失敗：{}", e); }
                                                    is_submitting.set(false);
                                                }
                                            }
                                        } else {
                                            if let Ok(mut s) = show_error_toast.try_write() { *s = true; }
                                            if let Ok(mut s) = error_message.try_write() { *s = format!("載入段落失敗，狀態碼：{}", response.status()); }
                                            is_submitting.set(false);
                                        }
                                    },
                                    Err(e) => {
                                        if let Ok(mut s) = show_error_toast.try_write() { *s = true; }
                                        if let Ok(mut s) = error_message.try_write() { *s = format!("載入段落請求失敗：{}", e); }
                                        is_submitting.set(false);
                                    }
                                }
                            } else {
                                if let Ok(mut s) = show_error_toast.try_write() { *s = true; }
                                if let Ok(mut s) = error_message.try_write() { *s = format!("保存段落失敗，狀態碼：{}", status); }
                                is_submitting.set(false);
                            }
                        },
                        Err(e) => {
                            if let Ok(mut s) = show_error_toast.try_write() { *s = true; }
                            if let Ok(mut s) = error_message.try_write() { *s = format!("保存段落請求失敗：{}", e); }
                            is_submitting.set(false);
                        }
                    }
                }
            });
        }
    };

    let handle_action_type_toggle = move |index: usize| {
        let mut current = action_type_open.read().clone();
        if let Some(is_open) = current.get_mut(index) {
            *is_open = !(*is_open as bool);
        }
        action_type_open.set(current);
    };

    let handle_add_choice = move || {
        let mut current_choices = choices.read().clone();
        current_choices.push((
            String::new(),
            String::new(),
            String::new(),
            None,
            None,
            String::new(),
            false,
            None,
        ));
        choices.set(current_choices);

        let mut current_action_types = action_type_open.read().clone();
        current_action_types.push(false);
        action_type_open.set(current_action_types);

        let mut current_chapters_open = choice_chapters_open.read().clone();
        current_chapters_open.push(false);
        choice_chapters_open.set(current_chapters_open);

        let mut current_chapters_search = choice_chapters_search.read().clone();
        current_chapters_search.push(String::new());
        choice_chapters_search.set(current_chapters_search);

        let mut current_paragraphs_open = choice_paragraphs_open.read().clone();
        current_paragraphs_open.push(false);
        choice_paragraphs_open.set(current_paragraphs_open);

        let mut current_paragraphs_search = choice_paragraphs_search.read().clone();
        current_paragraphs_search.push(String::new());
        choice_paragraphs_search.set(current_paragraphs_search);

        let mut current_paragraphs = choice_paragraphs.read().clone();
        current_paragraphs.push(Vec::new());
        choice_paragraphs.set(current_paragraphs);
    };

    let handle_remove_choice = move |index: usize| {
        let mut current_choices = choices.read().clone();
        current_choices.remove(index);
        choices.set(current_choices);

        let mut current_action_types = action_type_open.read().clone();
        current_action_types.remove(index);
        action_type_open.set(current_action_types);

        let mut current_chapters_open = choice_chapters_open.read().clone();
        current_chapters_open.remove(index);
        choice_chapters_open.set(current_chapters_open);

        let mut current_chapters_search = choice_chapters_search.read().clone();
        current_chapters_search.remove(index);
        choice_chapters_search.set(current_chapters_search);

        let mut current_paragraphs_open = choice_paragraphs_open.read().clone();
        current_paragraphs_open.remove(index);
        choice_paragraphs_open.set(current_paragraphs_open);

        let mut current_paragraphs_search = choice_paragraphs_search.read().clone();
        current_paragraphs_search.remove(index);
        choice_paragraphs_search.set(current_paragraphs_search);

        let mut current_paragraphs = choice_paragraphs.read().clone();
        current_paragraphs.remove(index);
        choice_paragraphs.set(current_paragraphs);
    };

    // 處理章節選擇器開關
    let handle_chapter_toggle = move |index: usize| {
        let mut current = choice_chapters_open.read().clone();
        if let Some(is_open) = current.get_mut(index) {
            *is_open = !*is_open;
        }
        choice_chapters_open.set(current);
    };

    // 處理章節搜索
    let handle_chapter_search = move |(index, query): (usize, String)| {
        let mut current = choice_chapters_search.read().clone();
        if let Some(search) = current.get_mut(index) {
            *search = query;
        }
        choice_chapters_search.set(current);
    };

    // 處理段落選擇器開關
    let handle_paragraph_toggle = move |index: usize| {
        let mut current = choice_paragraphs_open.read().clone();
        if let Some(is_open) = current.get_mut(index) {
            *is_open = !*is_open;
        }
        choice_paragraphs_open.set(current);
    };

    // 處理段落搜索
    let handle_paragraph_search = move |(index, query): (usize, String)| {
        let mut current = choice_paragraphs_search.read().clone();
        if let Some(search) = current.get_mut(index) {
            *search = query;
        }
        choice_paragraphs_search.set(current);
    };

    let mut reset_choices = move || {
        let mut choices_write = choices.write();
        choices_write.clear();
        choices_write.push((
            String::new(),
            String::new(),
            String::new(),
            None,
            None,
            String::new(),
            false,
            None,
        ));
        
        // 重置相關的選項狀態
        action_type_open.write().clear();
        action_type_open.write().push(false);
        
        choice_chapters_open.write().clear();
        choice_chapters_open.write().push(false);
        
        choice_chapters_search.write().clear();
        choice_chapters_search.write().push(String::new());
        
        choice_paragraphs_open.write().clear();
        choice_paragraphs_open.write().push(false);
        
        choice_paragraphs_search.write().clear();
        choice_paragraphs_search.write().push(String::new());
        
        choice_paragraphs.write().clear();
        choice_paragraphs.write().push(Vec::new());
    };

    rsx! {
        crate::pages::layout::Layout { 
            title: Some("Dashboard"),
            div { 
                class: "min-h-screen bg-gray-50 dark:bg-gray-900",
                // Toast 區域
                div {
                    class: "fixed bottom-4 right-4 z-50",
                    // 成功 Toast
                    if *toast_visible.read() {
                        div {
                            class: format!("bg-green-500 text-white px-6 py-3 rounded-lg shadow-lg {} {}",
                                if *toast_animating_out.read() {
                                    "toast-animate-out"
                                } else {
                                    "toast-animate-in"
                                },
                                "will-change-transform will-change-opacity"
                            ),
                            {t!("submit_success")}
                        }
                    }
                    // 錯誤 Toast
                    if *error_toast_visible.read() {
                        div {
                            class: format!("bg-red-500 text-white px-6 py-3 rounded-lg shadow-lg {} {}",
                                if *error_toast_animating_out.read() {
                                    "toast-animate-out"
                                } else {
                                    "toast-animate-in"
                                },
                                "will-change-transform will-change-opacity"
                            ),
                            {t!("submit_failed")}
                            div {
                                class: "mt-2 text-sm",
                                {error_message.read().clone()}
                            }
                        }
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
                                        label: t!("select_language"),
                                        value: current_language.read().to_string(),
                                        options: filtered_languages.read().clone(),
                                        is_open: *is_open.read(),
                                        search_query: search_query.read().to_string(),
                                        on_toggle: move |_| {
                                            let current = *is_open.read();
                                            is_open.set(!current);
                                        },
                                        on_search: move |query| search_query.set(query),
                                        on_select: {
                                            let update_paragraph_previews = update_paragraph_previews.clone();
                                            move |lang: &Language| {
                                                let current_lang = lang.code.to_string();
                                                paragraph_language.set(current_lang.clone());
                                                if let Ok(mut cb) = update_paragraph_previews.try_borrow_mut() {
                                                    (*cb)();
                                                }
                                                is_open.set(false);
                                                search_query.set(String::new());
                                                
                                                // 檢查是否有已存在的翻譯，使用精確匹配
                                                if let Some(paragraph) = selected_paragraph.read().as_ref() {
                                                    // 填充段落內容
                                                    if let Some(text) = paragraph.texts.iter().find(|text| text.lang == current_lang) {
                                                        paragraphs.set(text.paragraphs.clone());
                                                        
                                                        // 填充選項
                                                        let (new_choices, new_paragraphs) = process_paragraph_select(text, paragraph, &paragraph_state, &paragraph_language);
                                                        let choices_len = new_choices.len();

                                                        choices.set(new_choices);
                                                        choice_paragraphs.set(new_paragraphs);
                                                        action_type_open.set(vec![false; choices_len]);
                                                        choice_chapters_open.set(vec![false; choices_len]);
                                                        choice_paragraphs_open.set(vec![false; choices_len]);
                                                        choice_paragraphs_search.set(vec![String::new(); choices_len]);
                                                    } else {
                                                        // 如果找不到當前語言的翻譯，只清空段落內容和選項標題
                                                        paragraphs.set(String::new());
                                                        
                                                        // 保留目標章節和段落的選擇，只清空選項標題
                                                        let current_choices = choices.read().clone();
                                                        let new_choices = current_choices.iter().map(|(_, to, type_, key, value, _target_chapter, _same_page, _time_limit)| {
                                                            (String::new(), to.clone(), type_.clone(), key.clone(), value.clone(), String::new(), false, None)
                                                        }).collect();
                                                        choices.set(new_choices);
                                                    }
                                                }
                                            }
                                        },
                                        display_fn: display_language,
                                        has_error: false,
                                        search_placeholder: Box::leak(t!("search_language").into_boxed_str()),
                                        button_class: None,
                                        label_class: None,
                                    }
                                }

                                // 章節選擇器
                                div {
                                    class: "w-full",
                                    ChapterSelector {
                                        key: format!("chapter-dropdown-{}", paragraph_language.read()),
                                        label: Box::leak(t!("select_chapter").into_boxed_str()),
                                        value: selected_chapter.read().clone(),
                                        chapters: chapter_state.read().chapters.clone(),
                                        is_open: *is_chapter_open.read(),
                                        search_query: chapter_search_query.read().to_string(),
                                        on_toggle: move |_| {
                                            let current = *is_chapter_open.read();
                                            is_chapter_open.set(!current);
                                        },
                                        on_search: move |query| chapter_search_query.set(query),
                                        on_select: {
                                            let update_paragraph_previews = update_paragraph_previews.clone();
                                            move |chapter: Chapter| {
                                                selected_chapter.set(chapter.id.clone());
                                                is_chapter_open.set(false);
                                                chapter_search_query.set(String::new());
                                                validate_field(&chapter.id, &mut chapter_error);
                                                if let Ok(mut cb) = update_paragraph_previews.try_borrow_mut() {
                                                    (*cb)();
                                                }
                                            }
                                        },
                                        has_error: *chapter_error.read(),
                                        selected_language: paragraph_language.read().clone(),
                                    }
                                }
                            }

                            // 編輯/新增段落區域
                            div {
                                class: "pt-6 border-t border-gray-200 dark:border-gray-700",
                                div { 
                                    class: "w-full",
                                    div {
                                        class: "flex flex-col sm:flex-row items-start sm:items-end gap-2 sm:gap-4",
                                        div { 
                                            class: "w-full",
                                            crate::components::paragraph_list::ParagraphList {
                                                label: t!("select_paragraph"),
                                                value: selected_paragraph.read().as_ref().map(|p| p.id.clone()).unwrap_or(t!("select_paragraph").to_string()),
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
                                                on_select: EventHandler::new(move |id: String| {
                                                    if *is_edit_mode.read() {
                                                        // 找到選中段落的索引
                                                        let available_paragraphs = available_paragraphs.read();
                                                        if let Some(index) = available_paragraphs.iter().position(|p| p.id == id) {
                                                            handle_paragraph_select(index);
                                                        }
                                                        paragraph_search_query.set(String::new());
                                                    }
                                                }),
                                                has_error: false,
                                                disabled: !*is_edit_mode.read(),
                                                selected_language: paragraph_language.read().clone(),
                                            }
                                        }

                                        button {
                                            class: "w-full sm:w-10 h-10 inline-flex items-center justify-center rounded-lg text-white bg-blue-600 hover:bg-blue-700 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-500 dark:hover:bg-blue-600 dark:focus:ring-blue-800 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:bg-blue-600 dark:disabled:hover:bg-blue-500 flex-shrink-0",
                                            onclick: move |_| {
                                                let current_mode = *is_edit_mode.read();
                                                is_edit_mode.set(!current_mode);
                                                if current_mode {
                                                    // 退出編輯模式時清空所有欄位
                                                    paragraphs.set(String::new());
                                                    reset_choices();
                                                    selected_paragraph.set(None);
                                                }
                                            },
                                            disabled: selected_chapter.read().is_empty(),
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
                                    label: Box::leak(t!("paragraph_content").into_boxed_str()),
                                    placeholder: Box::leak(t!("paragraph_content").into_boxed_str()),
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
                                    choices: choices.read().clone(),
                                    on_choice_change: handle_choice_change,
                                    on_add_choice: handle_add_choice,
                                    on_remove_choice: handle_remove_choice,
                                    available_chapters: {
                                        let mut chapters = chapter_state.read().chapters.clone();
                                        // 在開頭添加 N/A 選項
                                        chapters.insert(0, Chapter {
                                            id: String::new(),
                                            titles: vec![crate::contexts::chapter_context::ChapterTitle {
                                                lang: paragraph_language.read().clone(),
                                                title: t!("not_applicable"),
                                            }],
                                            order: -1,
                                        });
                                        chapters
                                    },
                                    selected_language: paragraph_language.read().clone(),
                                    choice_chapters_open: choice_chapters_open.read().clone(),
                                    choice_chapters_search: choice_chapters_search.read().clone(),
                                    choice_paragraphs_open: choice_paragraphs_open.read().clone(),
                                    choice_paragraphs_search: choice_paragraphs_search.read().clone(),
                                    choice_paragraphs: choice_paragraphs.read().clone(),
                                    on_chapter_toggle: handle_chapter_toggle,
                                    on_chapter_search: handle_chapter_search,
                                    on_paragraph_toggle: handle_paragraph_toggle,
                                    on_paragraph_search: handle_paragraph_search,
                                    action_type_open: action_type_open.read().clone(),
                                    on_action_type_toggle: handle_action_type_toggle,
                                }
                            }
                        }

                        // 提交按鈕區域
                        div {
                            class: "px-4 sm:px-6 py-4 bg-gray-50 dark:bg-gray-700/50 border-t border-gray-200 dark:border-gray-700",
                            button {
                                class: "w-full inline-flex justify-center items-center px-4 sm:px-6 py-2.5 sm:py-3 bg-green-600 text-white rounded-lg hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 transition-transform duration-200 will-change-transform disabled:opacity-50 disabled:cursor-not-allowed font-medium text-base sm:text-lg shadow-sm",
                                disabled: {
                                    let edit_mode = *is_edit_mode.read();
                                    let selected_para = selected_paragraph.read().is_none();
                                    let has_changes = *has_changes.read();
                                    let is_valid = *is_form_valid.read();
                                    let submitting = *is_submitting.read();
                                    (edit_mode && selected_para) || !has_changes || !is_valid || submitting
                                },
                                onclick: move |_| {
                                    handle_submit(());
                                },
                                {t!("submit")}
                            }
                        }
                    }
                }
            }
        }
    }
}

fn process_paragraph_select(
    text: &ContextText,
    full_paragraph: &ContextParagraph,
    paragraph_state: &Signal<ParagraphState>,
    paragraph_language: &Signal<String>,
) -> (
    Vec<(String, String, String, Option<String>, Option<serde_json::Value>, String, bool, Option<u32>)>,
    Vec<Vec<ParagraphListParagraph>>,
) {
    let mut new_choices = Vec::new();
    let mut new_paragraphs = Vec::new();
    let text_choices = &text.choices;
    let paragraph_choices = &full_paragraph.choices;
    for (i, choice_text) in text_choices.iter().enumerate() {
        let (target_id, type_, key, value, same_page, time_limit) = if let Some(choice) = paragraph_choices.get(i) {
            match choice {
                ContextParagraphChoice::Simple(text) => (text.clone(), "goto".to_string(), None, None, false, None),
                ContextParagraphChoice::Complex { to, type_, key, value, same_page, time_limit, .. } => {
                    (to.clone(), type_.clone(), key.clone(), value.clone(), same_page.unwrap_or(false), *time_limit)
                },
            }
        } else {
            (String::new(), String::new(), None, None, false, None)
        };
        let target_chapter_id = if !target_id.is_empty() {
            if paragraph_state.read().get_by_id(&target_id).is_some() {
                paragraph_state.read().get_by_id(&target_id).map(|p| p.chapter_id.clone()).unwrap_or_default()
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        new_choices.push((
            choice_text.clone(),
            if target_chapter_id.is_empty() { String::new() } else { target_id.clone() },
            if target_chapter_id.is_empty() { String::new() } else { type_ },
            if target_chapter_id.is_empty() { None } else { key },
            if target_chapter_id.is_empty() { None } else { value },
            target_chapter_id.clone(),
            same_page,
            time_limit,
        ));
        if !target_chapter_id.is_empty() {
            let selected_lang = paragraph_language.read().clone();
            let filtered_paragraphs = paragraph_state.read().get_by_chapter(&target_chapter_id)
                .iter()
                .map(|item| {
                    let has_translation = item.texts.iter().any(|text| text.lang == selected_lang);
                    let preview = item.texts.iter()
                        .find(|t| t.lang == selected_lang)
                        .or_else(|| item.texts.iter().find(|t| t.lang == "en-US" || t.lang == "en-GB"))
                        .or_else(|| item.texts.first())
                        .map(|text| {
                            match text.paragraphs.lines().next() {
                                Some(line) => line.to_string(),
                                None => String::new(),
                            }
                        })
                        .unwrap_or_else(|| format!("[{}]", item.id));
                    ParagraphListParagraph {
                        id: item.id.clone(),
                        preview,
                        has_translation,
                    }
                })
                .collect::<Vec<_>>();
            new_paragraphs.push(filtered_paragraphs);
        } else {
            new_paragraphs.push(Vec::new());
        }
    }
    (new_choices, new_paragraphs)
}