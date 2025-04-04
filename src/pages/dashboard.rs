use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::prelude::*;
use crate::constants::config::config::{BASE_API_URL, SETTINGS, COLLECTIONS, AUTH_TOKEN};
use crate::enums::translations::Translations;
use crate::components::toast::Toast;
use crate::components::language_selector::LanguageSelector;
use crate::components::paragraph_form::ParagraphForm;
use crate::components::translation_form::{TranslationForm, Paragraph as TranslationParagraph, Text as TranslationText, Choice as TranslationChoice};
use dioxus::events::{FormEvent, FocusEvent};
use crate::components::dropdown::Dropdown;

// 從 chapter_selector.rs 移動過來的結構和組件
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chapter {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub titles: std::collections::HashMap<String, String>,
}

fn display_chapter(chapter: &Chapter) -> String {
    chapter.title.clone()
}

#[derive(Props, Clone, PartialEq)]
pub struct DashboardProps {
    lang: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {
    pub items: Vec<Paragraph>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Paragraph {
    pub id: String,
    pub index: usize,
    pub choice_id: String,
    #[serde(default)]
    pub chapter_id: String,
    pub texts: Vec<Text>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Text {
    pub lang: String,
    pub paragraphs: String,
    pub choices: Vec<TranslationChoice>,
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

#[derive(Debug, Clone, PartialEq)]
struct ChoiceOption {
    id: String,
    preview: String,
}

#[component]
pub fn Dashboard(props: DashboardProps) -> Element {
    let mut choices = use_signal(|| Vec::<TranslationChoice>::new());
    let mut choice_id = use_signal(|| String::new());
    let mut paragraphs = use_signal(|| String::new());
    let mut new_caption = use_signal(|| String::new());
    let mut new_goto = use_signal(|| String::new());
    let mut extra_captions = use_signal(|| Vec::<String>::new());
    let mut extra_gotos = use_signal(|| Vec::<String>::new());
    let mut show_extra_options = use_signal(|| Vec::<()>::new());
    let mut show_toast = use_signal(|| false);
    let mut toast_visible = use_signal(|| false);
    let mut selected_lang = use_signal(|| props.lang.clone());
    let mut available_choices = use_signal(|| Vec::<ChoiceOption>::new());
    let mut available_paragraphs = use_signal(|| Vec::<crate::components::paragraph_list::Paragraph>::new());
    let mut available_chapters = use_signal(|| Vec::<Chapter>::new());
    let mut selected_chapter = use_signal(|| String::new());
    let mut selected_paragraph = use_signal(|| None::<TranslationParagraph>);
    let mut is_edit_mode = use_signal(|| false);
    let t = Translations::get(&props.lang);

    let mut choice_id_error = use_signal(|| false);
    let mut paragraphs_error = use_signal(|| false);
    let mut new_caption_error = use_signal(|| false);
    let mut new_goto_error = use_signal(|| false);
    let mut chapter_error = use_signal(|| false);
    let has_loaded = use_signal(|| false);

    // 章節選擇器的狀態
    let mut chapter_is_open = use_signal(|| false);
    let mut chapter_search_query = use_signal(|| String::new());

    let filtered_chapters = {
        let chapters = available_chapters.read().clone();
        let query = chapter_search_query.read().to_lowercase();
        chapters.iter()
            .filter(|chapter| {
                chapter.title.to_lowercase().contains(&query)
            })
            .cloned()
            .collect::<Vec<_>>()
    };

    let current_chapter = {
        let chapters = available_chapters.read().clone();
        let selected_id = selected_chapter.read().clone();
        if selected_id.is_empty() {
            t.select_chapter.to_string()
        } else {
            chapters.iter()
                .find(|c| c.id == selected_id)
                .map(|c| c.title.clone())
                .unwrap_or_else(|| t.select_chapter.to_string())
        }
    };

    let chapter_dropdown_class = if *chapter_is_open.read() {
        "translate-y-0 opacity-100"
    } else {
        "-translate-y-2 opacity-0 pointer-events-none"
    };

    use_effect(move || {
        let chapters_url = format!("{}{}", BASE_API_URL, COLLECTIONS);
        let settings_url = format!("{}{}", BASE_API_URL, SETTINGS);
        let client = reqwest::Client::new();
        let mut has_loaded = has_loaded.clone();
        let current_lang = selected_lang.read().to_string();
        let mut available_chapters = available_chapters.clone();

        wasm_bindgen_futures::spawn_local(async move {
            // 載入 chapters
            match client.get(&chapters_url)
                .header("Authorization", format!("Bearer {}", AUTH_TOKEN))
                .send()
                .await {
                Ok(response) => {
                    match response.json::<SystemDataResponse>().await {
                        Ok(data) => {
                            // 尋找 key 為 "chapters" 的記錄
                            if let Some(chapters_data) = data.items.iter().find(|item| item.key == "chapters") {
                                // 直接從 JSON 值中獲取陣列
                                if let Some(chapter_infos) = chapters_data.value_raw.as_array() {
                                    // 將每個章節資訊轉換為 Chapter 結構
                                    let chapters: Vec<Chapter> = chapter_infos.iter()
                                        .filter_map(|info| {
                                            match serde_json::from_value::<ChapterInfo>(info.clone()) {
                                                Ok(chapter_info) => {
                                                    // 獲取當前語言的標題，如果沒有則使用其他可用的語言
                                                    let title = chapter_info.titles.get(&current_lang)
                                                        .cloned()
                                                        .unwrap_or_else(|| {
                                                            // 如果找不到當前語言的標題，嘗試使用其他語言的標題
                                                            // 優先使用英文，然後是中文，最後是任何可用的語言
                                                            chapter_info.titles.get("en-US")
                                                                .or_else(|| chapter_info.titles.get("zh-TW"))
                                                                .or_else(|| chapter_info.titles.get("es-ES"))
                                                                .or_else(|| chapter_info.titles.get("es-CL"))
                                                                .or_else(|| {
                                                                    // 如果以上都沒有，則使用任何可用的語言
                                                                    chapter_info.titles.values().next()
                                                                })
                                                                .cloned()
                                                                .unwrap_or_else(|| chapter_info.id.clone())
                                                        });
                                                    
                                                    Some(Chapter {
                                                        id: chapter_info.id,
                                                        title,
                                                        titles: chapter_info.titles,
                                                    })
                                                },
                                                Err(_) => None
                                            }
                                        })
                                        .collect();
                                    available_chapters.set(chapters);
                                }
                            }
                        }
                        Err(_) => {}
                    }
                }
                Err(_) => {}
            }

            // 載入設定
            match client.get(&settings_url)
                .header("Authorization", format!("Bearer {}", AUTH_TOKEN))
                .send()
                .await {
                Ok(response) => {
                    match response.json::<Data>().await {
                        Ok(data) => {
                        let choices = data.items.iter()
                            .map(|item| ChoiceOption {
                                id: item.choice_id.clone(),
                                preview: item.texts.first()
                                    .map(|t| t.paragraphs.lines().next().unwrap_or("").to_string())
                                    .unwrap_or_default(),
                            })
                            .collect();
                        let paragraphs = data.items.iter()
                            .map(|item| crate::components::paragraph_list::Paragraph {
                                id: item.choice_id.clone(),
                                preview: item.texts.first()
                                    .map(|t| t.paragraphs.lines().next().unwrap_or("").to_string())
                                    .unwrap_or_default(),
                            })
                            .collect();
                        available_choices.set(choices);
                        available_paragraphs.set(paragraphs);
                        has_loaded.set(true);
                        }
                        Err(_) => {}
                    }
                }
                Err(_) => {}
        }
        });
        
        // 返回一個清理函數
        (move || {})()
    });

    let validate_field = |value: &str, error_signal: &mut Signal<bool>| {
        if value.trim().is_empty() {
            error_signal.set(true);
        } else {
            error_signal.set(false);
        }
    };

    let handle_submit = move |_| {
        if choice_id.read().trim().is_empty() || 
           paragraphs.read().trim().is_empty() || 
           new_caption.read().trim().is_empty() || 
           new_goto.read().trim().is_empty() {
            return;
        }

        let mut all_choices = Vec::new();
        
        if !new_caption.read().trim().is_empty() && !new_goto.read().trim().is_empty() {
            all_choices.push(TranslationChoice {
                caption: new_caption.read().clone(),
                goto: new_goto.read().clone(),
            });
        }

        for i in 0..extra_captions.read().len() {
            let caption = &extra_captions.read()[i];
            let goto = &extra_gotos.read()[i];
            if !caption.trim().is_empty() && !goto.trim().is_empty() {
                all_choices.push(TranslationChoice {
                    caption: caption.clone(),
                    goto: goto.clone(),
                });
            }
        }

        let text = TranslationText {
            lang: selected_lang.read().clone(),
            paragraphs: paragraphs.read().clone(),
            choices: all_choices,
        };

        spawn_local(async move {
            let client = reqwest::Client::new();
            
            // 檢查是否選擇了章節
            let chapter_id = selected_chapter.read().clone();
            if chapter_id.is_empty() {
                return;
            }
            
            // 建立一個足夠長的 id
            let choice_id_value = choice_id.read().clone();
            let timestamp = js_sys::Date::new_0().get_time();
            let unique_id = format!("{}_{}_{}", chapter_id, choice_id_value, timestamp);
            
            // 建立新的段落資料
            let new_paragraph = serde_json::json!({
                "id": unique_id,
                "choice_id": choice_id.read().clone(),
                "chapter_id": chapter_id,
                "texts": [
                    {
                        "lang": selected_lang.read().clone(),
                        "paragraphs": paragraphs.read().clone(),
                        "choices": choices.read().clone()
                    }
                ]
            });
            
            // 發布到段落集合
            let paragraphs_url = format!("{}/api/collections/paragraphs/records", BASE_API_URL);
            
            match client.post(&paragraphs_url)
                .json(&new_paragraph)
                .send()
                .await {
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
                                    });
                    }
                }
                Err(_) => {}
            }
        });
    };

    let handle_add_translation = move |_| {
        if let Some(paragraph) = selected_paragraph.read().as_ref() {
            let mut updated_texts = paragraph.texts.clone();
            updated_texts.push(TranslationText {
                lang: selected_lang.read().clone(),
                paragraphs: paragraphs.read().clone(),
                choices: choices.read().clone(),
            });

            let updated_paragraph = serde_json::json!({
                "texts": updated_texts
            });

            let client = reqwest::Client::new();
            let paragraphs_url = format!("{}/api/collections/paragraphs/records/{}", BASE_API_URL, paragraph.id);

            spawn_local(async move {
                match client.patch(&paragraphs_url)
                    .json(&updated_paragraph)
                    .send()
                    .await {
                    Ok(response) => {
                        if response.status().is_success() {
                            paragraphs.set(String::new());
                            choices.write().clear();
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
                                }
                            }
                            Err(_) => {}
                        }
            });
            }
    };

    let handle_paragraph_select = move |id: String| {
        if let Some(paragraph) = available_paragraphs.read().iter().find(|p| p.id == id) {
            // 將 paragraph_list::Paragraph 轉換為 translation_form::Paragraph
            let dashboard_paragraph = TranslationParagraph {
                id: paragraph.id.clone(),
                index: 0, // 這裡需要從 API 獲取正確的索引
                choice_id: paragraph.id.clone(),
                chapter_id: String::new(),
                texts: Vec::new(),
            };
            selected_paragraph.set(Some(dashboard_paragraph));
        }
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
            div { 
                class: "max-w-3xl mx-auto p-8 bg-white dark:bg-gray-800 rounded-lg shadow-lg border border-gray-100 dark:border-gray-700",
                div { class: "space-y-8",
                    // 語言選擇器
                    LanguageSelector {
                        t: t.clone(),
                        selected_lang: selected_lang.read().to_string(),
                        on_language_change: move |lang: String| selected_lang.set(lang)
                    }

                    // 章節選擇器
                    crate::components::dropdown::Dropdown {
                        label: t.select_chapter.to_string(),
                        value: current_chapter,
                        options: filtered_chapters,
                        is_open: *chapter_is_open.read(),
                        search_query: chapter_search_query.read().to_string(),
                        on_toggle: move |_| {
                            let current = *chapter_is_open.read();
                            chapter_is_open.set(!current);
                        },
                        on_search: move |query| chapter_search_query.set(query),
                        on_select: move |chapter: Chapter| {
                            let chapter_id = chapter.id.clone();
                            selected_chapter.set(chapter_id.clone());
                            validate_field(&chapter_id, &mut chapter_error);
                            chapter_is_open.set(false);
                            chapter_search_query.set(String::new());
                        },
                        display_fn: display_chapter
                    }

                    // 根據模式顯示不同的表單
                    if !*is_edit_mode.read() {
                        // 新增段落表單
                        ParagraphForm {
                            t: t.clone(),
                            choice_id: choice_id.read().to_string(),
                            paragraphs: paragraphs.read().to_string(),
                            new_caption: new_caption.read().to_string(),
                            new_goto: new_goto.read().to_string(),
                            extra_captions: extra_captions.read().clone(),
                            extra_gotos: extra_gotos.read().clone(),
                            show_extra_options: show_extra_options.read().clone(),
                            choice_id_error: *choice_id_error.read(),
                            paragraphs_error: *paragraphs_error.read(),
                            new_caption_error: *new_caption_error.read(),
                            new_goto_error: *new_goto_error.read(),
                            available_paragraphs: available_paragraphs.read().clone(),
                            on_choice_id_change: move |value: String| {
                                choice_id.set(value.clone());
                                validate_field(&value, &mut choice_id_error);
                            },
                            on_paragraphs_change: move |value: String| {
                                paragraphs.set(value.clone());
                                validate_field(&value, &mut paragraphs_error);
                            },
                            on_new_caption_change: move |value: String| {
                                validate_field(&value, &mut new_caption_error);
                                new_caption.set(value);
                            },
                            on_new_goto_change: move |value: String| {
                                validate_field(&value, &mut new_goto_error);
                                new_goto.set(value);
                            },
                            on_extra_caption_change: move |(i, value)| {
                                let mut captions = extra_captions.write();
                                captions[i] = value;
                            },
                            on_extra_goto_change: move |(i, value)| {
                                let mut gotos = extra_gotos.write();
                                gotos[i] = value;
                            },
                            on_add_choice: move |_| {
                                show_extra_options.write().push(());
                                extra_captions.write().push(String::new());
                                extra_gotos.write().push(String::new());
                            },
                            on_submit: handle_submit
                        }
                    } else {
                        // 新增翻譯表單
                        TranslationForm {
                            t: t.clone(),
                            paragraphs: paragraphs.read().to_string(),
                            new_caption: new_caption.read().to_string(),
                            new_goto: new_goto.read().to_string(),
                            extra_captions: extra_captions.read().clone(),
                            extra_gotos: extra_gotos.read().clone(),
                            show_extra_options: show_extra_options.read().clone(),
                            paragraphs_error: *paragraphs_error.read(),
                            new_caption_error: *new_caption_error.read(),
                            new_goto_error: *new_goto_error.read(),
                            available_paragraphs: available_paragraphs.read().clone(),
                            selected_paragraph: selected_paragraph.read().clone(),
                            on_paragraphs_change: move |value: String| {
                                paragraphs.set(value.clone());
                                validate_field(&value, &mut paragraphs_error);
                            },
                            on_new_caption_change: move |value: String| {
                                validate_field(&value, &mut new_caption_error);
                                new_caption.set(value);
                            },
                            on_new_goto_change: move |value: String| {
                                validate_field(&value, &mut new_goto_error);
                                new_goto.set(value);
                            },
                            on_extra_caption_change: move |(i, value)| {
                                let mut captions = extra_captions.write();
                                captions[i] = value;
                            },
                            on_extra_goto_change: move |(i, value)| {
                                let mut gotos = extra_gotos.write();
                                gotos[i] = value;
                            },
                            on_add_choice: move |_| {
                                show_extra_options.write().push(());
                                extra_captions.write().push(String::new());
                                extra_gotos.write().push(String::new());
                            },
                            on_submit: handle_add_translation,
                            on_paragraph_select: handle_paragraph_select
                        }
                    }

                    // 切換模式按鈕
                    button {
                        class: "w-full px-4 py-2 text-sm font-medium text-white bg-gray-600 rounded-md hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500",
                        onclick: move |_| {
                            let current_mode = *is_edit_mode.read();
                            is_edit_mode.set(!current_mode);
                        },
                        if *is_edit_mode.read() {
                            "切換到新增段落模式"
                        } else {
                            "切換到新增翻譯模式"
                        }
                    }
                }
            }
        }
    }
}