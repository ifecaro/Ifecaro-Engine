use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use crate::constants::{BASE_API_URL, PARAGRAPHS, AUTH_TOKEN, CHAPTERS};
use crate::enums::translations::Translations;
use crate::components::toast::Toast;
use crate::components::form::{InputField, TextareaField, ChoiceOptions};
use crate::components::story_content::{Choice, Action};
use crate::components::dropdown::Dropdown;
use crate::components::translation_form::{Paragraph, Text};
use crate::components::paragraph_list::ParagraphList;
use dioxus::events::FormEvent;
use dioxus::hooks::use_context;
use crate::contexts::language_context::LanguageState;
use std::cell::RefCell;
use std::thread_local;

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

#[derive(Clone, PartialEq)]
struct Language {
    code: &'static str,
    name: &'static str,
}

const AVAILABLE_LANGUAGES: &[Language] = &[
    Language { code: "zh-TW", name: "繁體中文" },
    Language { code: "zh-CN", name: "簡體中文" },
    Language { code: "en", name: "English" },
    Language { code: "ja", name: "日本語" },
    Language { code: "ko", name: "한국어" },
    Language { code: "es", name: "Español" },
    Language { code: "fr", name: "Français" },
    Language { code: "de", name: "Deutsch" },
    Language { code: "it", name: "Italiano" },
    Language { code: "pt", name: "Português" },
    Language { code: "ru", name: "Русский" },
    Language { code: "ar", name: "العربية" },
    Language { code: "hi", name: "हिंदी" },
    Language { code: "bn", name: "বাংলা" },
    Language { code: "id", name: "Bahasa Indonesia" },
    Language { code: "ms", name: "Bahasa Melayu" },
    Language { code: "th", name: "ไทย" },
    Language { code: "vi", name: "Tiếng Việt" },
    Language { code: "nl", name: "Nederlands" },
    Language { code: "pl", name: "Polski" },
    Language { code: "uk", name: "Українська" },
    Language { code: "el", name: "Ελληνικά" },
    Language { code: "he", name: "עברית" },
    Language { code: "tr", name: "Türkçe" },
    Language { code: "sv", name: "Svenska" },
    Language { code: "da", name: "Dansk" },
    Language { code: "no", name: "Norsk" },
    Language { code: "cs", name: "Čeština" },
    Language { code: "ro", name: "Română" },
    Language { code: "hu", name: "Magyar" },
    Language { code: "sk", name: "Slovenčina" },
    Language { code: "hr", name: "Hrvatski" },
    Language { code: "ca", name: "Català" },
    Language { code: "fil", name: "Filipino" },
    Language { code: "fa", name: "فارسی" },
    Language { code: "lv", name: "Latviešu" },
    Language { code: "af", name: "Afrikaans" },
    Language { code: "sw", name: "Kiswahili" },
    Language { code: "ga", name: "Gaeilge" },
    Language { code: "et", name: "Eesti" },
    Language { code: "eu", name: "Euskara" },
    Language { code: "is", name: "Íslenska" },
    Language { code: "mk", name: "Македонски" },
    Language { code: "hy", name: "Հայերեն" },
    Language { code: "ne", name: "नेपाली" },
    Language { code: "lb", name: "Lëtzebuergesch" },
    Language { code: "my", name: "မြန်မာဘာသာ" },
    Language { code: "gl", name: "Galego" },
    Language { code: "mr", name: "मराठी" },
    Language { code: "ka", name: "ქართული" },
    Language { code: "mn", name: "Монгол" },
    Language { code: "si", name: "සිංහල" },
    Language { code: "km", name: "ខ្មែរ" },
    Language { code: "sn", name: "chiShona" },
    Language { code: "yo", name: "Yorùbá" },
    Language { code: "so", name: "Soomaali" },
    Language { code: "ha", name: "Hausa" },
    Language { code: "zu", name: "isiZulu" },
    Language { code: "xh", name: "isiXhosa" },
    Language { code: "am", name: "አማርኛ" },
    Language { code: "be", name: "Беларуская" },
    Language { code: "az", name: "Azərbaycan" },
    Language { code: "uz", name: "O'zbek" },
    Language { code: "kk", name: "Қазақ" },
    Language { code: "ky", name: "Кыргызча" },
    Language { code: "tg", name: "Тоҷикӣ" },
    Language { code: "tk", name: "Türkmen" },
    Language { code: "ur", name: "اردو" },
    Language { code: "pa", name: "ਪੰਜਾਬੀ" },
    Language { code: "gu", name: "ગુજરાતી" },
    Language { code: "or", name: "ଓଡ଼ିଆ" },
    Language { code: "ta", name: "தமிழ்" },
    Language { code: "te", name: "తెలుగు" },
    Language { code: "kn", name: "ಕನ್ನಡ" },
    Language { code: "ml", name: "മലയാളം" },
    Language { code: "as", name: "অসমীয়া" },
    Language { code: "mai", name: "मैथिली" },
    Language { code: "mni", name: "মৈতৈলোন্" },
    Language { code: "doi", name: "डोगरी" },
    Language { code: "bho", name: "भोजपुरी" },
    Language { code: "sat", name: "ᱥᱟᱱᱛᱟᱲᱤ" },
    Language { code: "ks", name: "کٲشُر" },
    Language { code: "sa", name: "संस्कृतम्" },
    Language { code: "sd", name: "سنڌي" },
    Language { code: "kok", name: "कोंकणी" },
    Language { code: "gom", name: "कोंकणी" },
];

#[derive(Debug, Clone, PartialEq)]
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

    let mut choices = use_signal(|| Vec::<Choice>::new());
    let mut paragraphs = use_signal(|| String::new());
    let mut new_caption = use_signal(|| String::new());
    let mut new_goto = use_signal(|| String::new());
    let mut extra_captions = use_signal(|| Vec::<String>::new());
    let mut extra_gotos = use_signal(|| Vec::<String>::new());
    let mut show_extra_options = use_signal(|| Vec::<()>::new());
    let mut show_toast = use_signal(|| false);
    let toast_visible = use_signal(|| false);
    let mut is_open = use_signal(|| false);
    let mut search_query = use_signal(|| String::new());
    let _is_goto_open = use_signal(|| false);
    let _goto_search_query = use_signal(|| String::new());
    let _available_choices = use_signal(|| Vec::<ChoiceOption>::new());
    let mut available_paragraphs = use_signal(|| Vec::<crate::components::paragraph_list::Paragraph>::new());
    let available_chapters = use_signal(|| Vec::<Chapter>::new());
    let mut selected_chapter = use_signal(|| String::new());
    let mut is_chapter_open = use_signal(|| false);
    let mut chapter_search_query = use_signal(|| String::new());
    let mut selected_paragraph = use_signal(|| None::<Paragraph>);
    let mut is_edit_mode = use_signal(|| false);
    let mut _is_paragraph_open = use_signal(|| false);
    let mut _paragraph_search_query = use_signal(|| String::new());
    let paragraph_data = use_signal(|| Vec::<Paragraph>::new());
    let t = Translations::get(&current_lang);
    let mut should_scroll = use_signal(|| false);

    let mut paragraphs_error = use_signal(|| false);
    let mut new_caption_error = use_signal(|| false);
    let mut new_goto_error = use_signal(|| false);
    let mut chapter_error = use_signal(|| false);
    let has_loaded = use_signal(|| false);

    // 在 DashboardProps 結構體後添加新的狀態
    let mut new_action_type = use_signal(|| String::new());
    let mut new_action_key = use_signal(|| None::<String>);
    let mut new_action_value = use_signal(|| None::<serde_json::Value>);
    let mut extra_action_types = use_signal(|| Vec::<String>::new());
    let mut extra_action_keys = use_signal(|| Vec::<Option<String>>::new());
    let mut extra_action_values = use_signal(|| Vec::<Option<serde_json::Value>>::new());

    // 載入章節列表
    use_effect(move || {
        let chapters_url = format!("{}/api{}", BASE_API_URL, CHAPTERS);
        let client = reqwest::Client::new();
        let mut available_chapters = available_chapters.clone();
        
        wasm_bindgen_futures::spawn_local(async move {
            match client.get(&chapters_url)
                .header("Authorization", format!("Bearer {}", AUTH_TOKEN))
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

    use_effect(move || {
        let paragraphs_url = format!("{}/api{}", BASE_API_URL, PARAGRAPHS);
        let client = reqwest::Client::new();
        let mut has_loaded = has_loaded.clone();
        let language_state = use_context::<Signal<LanguageState>>();
        let current_lang = language_state.read().current_language.clone();
        let mut paragraph_data = paragraph_data.clone();

        wasm_bindgen_futures::spawn_local(async move {
            // 載入段落
            match client.get(&paragraphs_url)
                .header("Authorization", format!("Bearer {}", AUTH_TOKEN))
                .send()
                .await {
                Ok(response) => {
                    match response.json::<Data>().await {
                        Ok(data) => {
                            paragraph_data.set(data.items.clone());
                            
                            let paragraphs: Vec<crate::components::paragraph_list::Paragraph> = data.items.iter()
                                .map(|item| {
                                    let preview = item.texts.iter()
                                        .find(|t| t.lang == current_lang)
                                        .or_else(|| item.texts.first())
                                        .map(|t| t.paragraphs.lines().next().unwrap_or("").to_string())
                                        .unwrap_or_default();
                                    
                                    crate::components::paragraph_list::Paragraph {
                                        id: item.id.clone(),
                                        preview,
                                    }
                                })
                                .collect();
                            
                            available_paragraphs.set(paragraphs);
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
            .find(|l| l.code == language_state.read().current_language)
            .map(|l| l.name)
            .unwrap_or("繁體中文")
    });

    let is_form_valid = use_memo(move || {
        // 檢查主要欄位
        let main_fields_valid = !paragraphs.read().trim().is_empty() &&
            !new_caption.read().trim().is_empty() &&
            !new_goto.read().trim().is_empty();

        // 檢查額外選項
        let extra_choices_valid = extra_captions.read().iter().zip(extra_gotos.read().iter())
            .all(|(caption, goto)| !caption.trim().is_empty() && !goto.trim().is_empty());

        main_fields_valid && extra_choices_valid
    });

    let has_changes = use_memo(move || {
        if let Some(paragraph) = selected_paragraph.read().as_ref() {
            let language_state = use_context::<Signal<LanguageState>>();
            let current_lang = language_state.read().current_language.clone();
            // 檢查當前語言的翻譯是否存在
            if let Some(existing_text) = paragraph.texts.iter().find(|text| text.lang == current_lang) {
                // 比較段落內容
                let paragraphs_changed = existing_text.paragraphs != *paragraphs.read();
                
                // 比較選項
                let choices_changed = if !existing_text.choices.is_empty() {
                    // 檢查第一個選項
                    let first_choice_changed = existing_text.choices[0].caption != *new_caption.read() ||
                                            existing_text.choices[0].action.to != *new_goto.read();
                    
                    // 檢查額外選項
                    let extra_choices_changed = if existing_text.choices.len() > 1 {
                        let existing_extra = &existing_text.choices[1..];
                        let current_extra_captions = &extra_captions.read();
                        let current_extra_gotos = &extra_gotos.read();
                        
                        if existing_extra.len() != current_extra_captions.len() {
                            true
                        } else {
                            existing_extra.iter().zip(current_extra_captions.iter().zip(current_extra_gotos.iter()))
                                .any(|(existing, (current_caption, current_goto))| {
                                    existing.caption != *current_caption || existing.action.to != *current_goto
                                })
                        }
                    } else {
                        !extra_captions.read().is_empty() || !extra_gotos.read().is_empty()
                    };
                    
                    first_choice_changed || extra_choices_changed
                } else {
                    !new_caption.read().is_empty() || !new_goto.read().is_empty() ||
                    !extra_captions.read().is_empty() || !extra_gotos.read().is_empty()
                };
                
                paragraphs_changed || choices_changed
            } else {
                // 如果是新翻譯，只要有任何內容就表示有變化
                !paragraphs.read().trim().is_empty() ||
                !new_caption.read().trim().is_empty() ||
                !new_goto.read().trim().is_empty() ||
                !extra_captions.read().is_empty() ||
                !extra_gotos.read().is_empty()
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
            lang: current_lang.clone(),
            paragraphs: paragraphs.read().clone(),
            choices: choices.clone(),
        };

        spawn_local(async move {
            let client = reqwest::Client::new();
            
            // 檢查是否選擇了章節
            if selected_chapter.read().is_empty() {
                return;
            }
            
            // 建立一個足夠長的 id
            let chapter_id = selected_chapter.read().clone();
            let timestamp = js_sys::Date::new_0().get_time();
            let unique_id = format!("{}_{}", chapter_id, timestamp);
            
            // 建立新的段落資料
            let new_paragraph = serde_json::json!({
                "id": unique_id,
                "chapter_id": chapter_id,
                "texts": [text]
            });
            
            // 發布到段落集合
            let paragraphs_url = format!("{}/api{}", BASE_API_URL, PARAGRAPHS);
            
            match client.post(&paragraphs_url)
                .json(&new_paragraph)
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
            let language_state = use_context::<Signal<LanguageState>>();
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
                lang: current_lang,
                paragraphs: paragraphs.read().clone(),
                choices: choices.clone(),
            });

            let updated_paragraph = serde_json::json!({
                "texts": updated_texts
            });

            let client = reqwest::Client::new();
            let paragraphs_url = format!("{}/api{}/{}", BASE_API_URL, PARAGRAPHS, paragraph.id);

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
                            });
                        }
                    }
                    Err(_) => {}
                }
            });
        }
    };

    let handle_paragraph_select = move |id: String| {
        // 從完整的段落資料中尋找選中的段落
        if let Some(paragraph) = paragraph_data.read().iter().find(|p| p.id == id) {
            let language_state = use_context::<Signal<LanguageState>>();
            let current_lang = language_state.read().current_language.clone();
            
            selected_paragraph.set(Some(paragraph.clone()));

            // 檢查是否有已存在的翻譯
            if let Some(existing_text) = paragraph.texts.iter().find(|text| text.lang == current_lang) {
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

    // 定義章節標題顯示函數
    let display_chapter_title = move |chapter: &Chapter| {
        let language_state = use_context::<Signal<LanguageState>>();
        let current_lang = language_state.read().current_language.clone();
        chapter.titles.iter()
            .find(|t| t.lang == current_lang)
            .or_else(|| chapter.titles.first())
            .map(|t| t.title.clone())
            .unwrap_or_default()
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
                class: "max-w-3xl md:max-w-5xl mx-auto p-8 bg-white dark:bg-gray-800 rounded-lg shadow-lg border border-gray-100 dark:border-gray-700 space-y-6",
                div { 
                    class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    // 語言選擇
                    div {
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
                                let mut language_state = use_context::<Signal<LanguageState>>();
                                language_state.write().current_language = lang.code.to_string();
                                is_open.set(false);
                                search_query.set(String::new());
                            },
                            display_fn: display_language
                        }
                    }

                    // 章節選擇器
                    div {
                        Dropdown {
                            key: format!("chapter-dropdown-{}", current_lang),
                            label: t.select_chapter,
                            value: {
                                if selected_chapter.read().is_empty() {
                                    t.select_chapter.to_string()
                                } else {
                                    let chapter = available_chapters.read().iter()
                                        .find(|c| c.id == *selected_chapter.read())
                                        .cloned()
                                        .unwrap_or_else(|| Chapter { id: String::new(), titles: Vec::new(), order: 0 });
                                    
                                    display_chapter_title(&chapter)
                                }
                            },
                            options: available_chapters.read().iter()
                                .filter(|chapter| {
                                    let query = chapter_search_query.read().to_lowercase();
                                    chapter.titles.iter()
                                        .find(|t| t.lang == current_lang)
                                        .map(|t| t.title.to_lowercase().contains(&query))
                                        .unwrap_or(false)
                                })
                                .cloned()
                                .collect::<Vec<_>>(),
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
                            display_fn: display_chapter_title
                        }
                    }

                    // 根據模式顯示不同的表單
                    if *is_edit_mode.read() {
                        // 編輯段落表單
                        div { 
                            class: "md:col-span-2",
                            div {
                                class: "flex items-center space-x-4",
                                div { class: "flex-1",
                                    ParagraphList {
                                        label: "選擇段落",
                                        value: selected_paragraph.read().as_ref().map(|p| p.id.clone()).unwrap_or("選擇段落".to_string()),
                                        paragraphs: available_paragraphs.read().clone(),
                                        is_open: *_is_paragraph_open.read(),
                                        search_query: _paragraph_search_query.read().to_string(),
                                        on_toggle: move |_| {
                                            let current = *_is_paragraph_open.read();
                                            _is_paragraph_open.set(!current);
                                        },
                                        on_search: move |query| _paragraph_search_query.set(query),
                                        on_select: handle_paragraph_select,
                                    }
                                }

                                button {
                                    class: "inline-flex items-center justify-center w-10 h-10 text-sm font-medium text-white bg-gray-700 rounded-lg hover:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-600 transition-colors duration-200 shadow-sm",
                                    onclick: move |_| {
                                        let current_mode = *is_edit_mode.read();
                                        is_edit_mode.set(!current_mode);
                                        // 清空表單
                                        if !current_mode {
                                            paragraphs.set(String::new());
                                            new_caption.set(String::new());
                                            new_goto.set(String::new());
                                            extra_captions.write().clear();
                                            extra_gotos.write().clear();
                                            show_extra_options.write().clear();
                                            selected_paragraph.set(None);
                                        }
                                    },
                                    // 根據模式使用不同圖標
                                    if *is_edit_mode.read() {
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
                                    } else {
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
                                                d: "M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        // 新增段落表單
                        div { 
                            class: "md:col-span-2",
                            div {
                                class: "flex items-center space-x-4",
                                div { class: "flex-1",
                                    InputField {
                                        label: t.paragraph_title,
                                        placeholder: t.paragraph_title,
                                        value: new_caption.read().to_string(),
                                        required: true,
                                        has_error: *new_caption_error.read(),
                                        on_input: move |value: String| {
                                            new_caption.set(value.clone());
                                            validate_field(&value, &mut new_caption_error);
                                        },
                                        on_blur: move |_| validate_field(&new_caption.read(), &mut new_caption_error)
                                    }
                                }

                                button {
                                    class: "inline-flex items-center justify-center w-10 h-10 text-sm font-medium text-white bg-gray-700 rounded-lg hover:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-600 transition-colors duration-200 shadow-sm",
                                    onclick: move |_| {
                                        let current_mode = *is_edit_mode.read();
                                        is_edit_mode.set(!current_mode);
                                        // 清空表單
                                        if !current_mode {
                                            paragraphs.set(String::new());
                                            new_caption.set(String::new());
                                            new_goto.set(String::new());
                                            extra_captions.write().clear();
                                            extra_gotos.write().clear();
                                            show_extra_options.write().clear();
                                            selected_paragraph.set(None);
                                        }
                                    },
                                    // 根據模式使用不同圖標
                                    if *is_edit_mode.read() {
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
                                    } else {
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
                                                d: "M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 段落內容欄位
                    div { 
                        class: "md:col-span-2",
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

                    ChoiceOptions {
                        t: t.clone(),
                        new_caption: new_caption.read().to_string(),
                        new_goto: new_goto.read().to_string(),
                        new_action_type: new_action_type.read().to_string(),
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
                            validate_field(&value, &mut new_caption_error);
                            new_caption.set(value);
                        },
                        on_new_goto_change: move |value: String| {
                            validate_field(&value, &mut new_goto_error);
                            new_goto.set(value);
                        },
                        on_new_action_type_change: move |value: String| {
                            new_action_type.set(value);
                        },
                        on_new_action_key_change: move |value: Option<String>| {
                            new_action_key.set(value);
                        },
                        on_new_action_value_change: move |value: Option<serde_json::Value>| {
                            new_action_value.set(value);
                        },
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
                        on_add_choice: handle_add_choice
                    }
                }

                // 提交按鈕（移到網格外）
                button {
                    class: "w-full px-6 py-3 bg-green-500 text-white rounded-md hover:bg-green-600 transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed font-medium text-lg",
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