use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::prelude::*;
use crate::constants::config::config::{BASE_API_URL, SETTINGS, COLLECTIONS, AUTH_TOKEN};
use crate::enums::translations::Translations;
use crate::components::toast::Toast;
use crate::components::form::{InputField, TextareaField, ChoiceOptions};
use crate::components::story_content::Choice;
use crate::components::dropdown::Dropdown;
use crate::components::paragraph_list::ParagraphList;
use dioxus::events::{FormEvent, FocusEvent};

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
    #[serde(default)]
    pub chapter_id: String,
    pub texts: Vec<Text>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Text {
    pub lang: String,
    pub paragraphs: String,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Chapter {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub titles: std::collections::HashMap<String, String>,
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

#[component]
pub fn Dashboard(props: DashboardProps) -> Element {
    let mut choices = use_signal(|| Vec::<Choice>::new());
    let mut paragraphs = use_signal(|| String::new());
    let mut new_caption = use_signal(|| String::new());
    let mut new_goto = use_signal(|| String::new());
    let mut extra_captions = use_signal(|| Vec::<String>::new());
    let mut extra_gotos = use_signal(|| Vec::<String>::new());
    let mut show_extra_options = use_signal(|| Vec::<()>::new());
    let mut show_toast = use_signal(|| false);
    let mut toast_visible = use_signal(|| false);
    let mut selected_lang = use_signal(|| props.lang.clone());
    let mut is_open = use_signal(|| false);
    let mut search_query = use_signal(|| String::new());
    let mut is_goto_open = use_signal(|| false);
    let mut goto_search_query = use_signal(|| String::new());
    let mut available_choices = use_signal(|| Vec::<ChoiceOption>::new());
    let mut available_paragraphs = use_signal(|| Vec::<crate::components::paragraph_list::Paragraph>::new());
    let mut available_chapters = use_signal(|| Vec::<Chapter>::new());
    let mut selected_chapter = use_signal(|| String::new());
    let mut is_chapter_open = use_signal(|| false);
    let mut chapter_search_query = use_signal(|| String::new());
    let mut selected_paragraph = use_signal(|| None::<Paragraph>);
    let mut is_edit_mode = use_signal(|| false);
    let mut is_paragraph_open = use_signal(|| false);
    let mut paragraph_search_query = use_signal(|| String::new());
    let mut paragraph_data = use_signal(|| Vec::<Paragraph>::new());
    let t = Translations::get(&props.lang);

    let mut choice_id_error = use_signal(|| false);
    let mut paragraphs_error = use_signal(|| false);
    let mut new_caption_error = use_signal(|| false);
    let mut new_goto_error = use_signal(|| false);
    let mut chapter_error = use_signal(|| false);
    let has_loaded = use_signal(|| false);

    use_effect(move || {
        let chapters_url = format!("{}{}", BASE_API_URL, COLLECTIONS);
        let settings_url = format!("{}{}", BASE_API_URL, SETTINGS);
        let client = reqwest::Client::new();
        let mut has_loaded = has_loaded.clone();
        let current_lang = selected_lang.read().to_string();
        let mut paragraph_data = paragraph_data.clone();

        wasm_bindgen_futures::spawn_local(async move {
            // 載入 chapters
            match client.get(&chapters_url)
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
                            // 將完整的段落資料載入到 paragraph_data 中
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
                        Err(e) => {}
                    }
                }
                Err(_) => {}
            }
        });
        
        // 返回一個清理函數
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

    let dropdown_class = use_memo(move || {
        if *is_open.read() {
            "translate-y-0 opacity-100"
        } else {
            "-translate-y-2 opacity-0 pointer-events-none"
        }
    });

    let current_language = use_memo(move || {
        AVAILABLE_LANGUAGES.iter()
            .find(|l| l.code == *selected_lang.read())
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

    // 檢查翻譯是否有變化
    let has_changes = use_memo(move || {
        if let Some(paragraph) = selected_paragraph.read().as_ref() {
            // 檢查當前語言的翻譯是否存在
            if let Some(existing_text) = paragraph.texts.iter().find(|text| text.lang == *selected_lang.read()) {
                // 比較段落內容
                let paragraphs_changed = existing_text.paragraphs != *paragraphs.read();
                
                // 比較選項
                let choices_changed = if !existing_text.choices.is_empty() {
                    // 檢查第一個選項
                    let first_choice_changed = existing_text.choices[0].caption != *new_caption.read() ||
                                            existing_text.choices[0].goto != *new_goto.read();
                    
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
                                    existing.caption != *current_caption || existing.goto != *current_goto
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

        let mut all_choices = Vec::new();
        
        if !new_caption.read().trim().is_empty() && !new_goto.read().trim().is_empty() {
            all_choices.push(Choice {
                caption: new_caption.read().clone(),
                goto: new_goto.read().clone(),
            });
        }

        for i in 0..extra_captions.read().len() {
            let caption = &extra_captions.read()[i];
            let goto = &extra_gotos.read()[i];
            if !caption.trim().is_empty() && !goto.trim().is_empty() {
                all_choices.push(Choice {
                    caption: caption.clone(),
                    goto: goto.clone(),
                });
            }
        }

        let text = Text {
            lang: selected_lang.read().clone(),
            paragraphs: paragraphs.read().clone(),
            choices: all_choices,
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
                "chapter_id": selected_chapter.read().clone(),
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
            updated_texts.push(Text {
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
        // 從完整的段落資料中尋找選中的段落
        if let Some(paragraph) = paragraph_data.read().iter().find(|p| p.id == id) {
            // 將 dashboard::Paragraph 轉換為 translation_form::Paragraph
            let dashboard_paragraph = crate::components::translation_form::Paragraph {
                id: paragraph.id.clone(),
                index: paragraph.index,
                chapter_id: paragraph.chapter_id.clone(),
                texts: paragraph.texts.iter().map(|text| crate::components::translation_form::Text {
                    lang: text.lang.clone(),
                    paragraphs: text.paragraphs.clone(),
                    choices: text.choices.iter().map(|choice| crate::components::translation_form::Choice {
                        caption: choice.caption.clone(),
                        goto: choice.goto.clone(),
                    }).collect(),
                }).collect(),
            };
            selected_paragraph.set(Some(paragraph.clone()));

            // 檢查是否有已存在的翻譯
            let current_lang = selected_lang.read().to_string();
            if let Some(existing_text) = paragraph.texts.iter().find(|text| text.lang == current_lang) {
                // 填充段落內容
                paragraphs.set(existing_text.paragraphs.clone());
                
                // 填充選項
                if !existing_text.choices.is_empty() {
                    // 設置第一個選項
                    new_caption.set(existing_text.choices[0].caption.clone());
                    new_goto.set(existing_text.choices[0].goto.clone());
                    
                    // 設置額外選項
                    let mut captions = Vec::new();
                    let mut gotos = Vec::new();
                    let mut options = Vec::new();
                    
                    for choice in existing_text.choices.iter().skip(1) {
                        captions.push(choice.caption.clone());
                        gotos.push(choice.goto.clone());
                        options.push(());
                    }
                    
                    // 確保所有向量都有相同的長度
                    let len = captions.len();
                    if len > 0 {
                        extra_captions.set(captions);
                        extra_gotos.set(gotos);
                        show_extra_options.set(options);
                    } else {
                        // 如果沒有額外選項，清空所有向量
                        extra_captions.set(Vec::new());
                        extra_gotos.set(Vec::new());
                        show_extra_options.set(Vec::new());
                    }
                } else {
                    // 如果沒有選項，清空所有向量
                    extra_captions.set(Vec::new());
                    extra_gotos.set(Vec::new());
                    show_extra_options.set(Vec::new());
                }
            }
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
                    Dropdown {
                        label: t.select_language.to_string(),
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
                            selected_lang.set(lang.code.to_string());
                            is_open.set(false);
                            search_query.set(String::new());
                        },
                        display_fn: display_language
                    }

                    // 章節選擇器
                    Dropdown {
                        label: t.select_chapter.to_string(),
                        value: if selected_chapter.read().is_empty() {
                            t.select_chapter.to_string()
                        } else {
                            available_chapters.read().iter()
                                .find(|c| c.id == *selected_chapter.read())
                                .map(|c| c.title.clone())
                                .unwrap_or_else(|| t.select_chapter.to_string())
                        },
                        options: available_chapters.read().iter()
                            .filter(|chapter| {
                                let query = chapter_search_query.read().to_lowercase();
                                chapter.title.to_lowercase().contains(&query)
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
                        display_fn: |chapter: &Chapter| chapter.title.clone()
                    }

                    // 根據模式顯示不同的表單
                    if !*is_edit_mode.read() {
                        // 新增段落表單
                        div { class: "space-y-2",
                            div { class: "flex items-end space-x-4",
                                div { class: "flex-1",
                                    InputField {
                                        label: t.option_text.clone(),
                                        placeholder: t.option_text.clone(),
                                        value: new_caption.read().to_string(),
                                        required: true,
                                        has_error: *new_caption_error.read(),
                                        on_input: move |value: String| {
                                            new_caption.set(value.clone());
                                            validate_field(&value, &mut new_caption_error);
                                        },
                                        on_blur: move |_| validate_field(&new_caption.read(), &mut new_caption_error),
                                        children: rsx! {
                                            button {
                                                class: "inline-flex items-center justify-center w-10 h-10 text-sm font-medium text-white bg-gray-700 rounded-lg hover:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-600 transition-colors duration-200 shadow-sm mb-[2px]",
                                                onclick: move |_| {
                                                    let current_mode = *is_edit_mode.read();
                                                    is_edit_mode.set(!current_mode);
                                                },
                                                // 使用鉛筆圖標表示「新增段落」
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
                        }

                        TextareaField {
                            label: t.paragraph.clone(),
                            placeholder: t.paragraph.clone(),
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

                        ChoiceOptions {
                            t: t.clone(),
                            new_caption: new_caption.read().to_string(),
                            new_goto: new_goto.read().to_string(),
                            extra_captions: extra_captions.read().clone(),
                            extra_gotos: extra_gotos.read().clone(),
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
                            on_extra_caption_change: move |(i, value): (usize, String)| {
                                let mut captions = extra_captions.write();
                                captions[i] = value;
                            },
                            on_extra_goto_change: move |(i, value): (usize, String)| {
                                let mut gotos = extra_gotos.write();
                                gotos[i] = value;
                            },
                            on_add_choice: move |_| {
                                show_extra_options.write().push(());
                                extra_captions.write().push(String::new());
                                extra_gotos.write().push(String::new());
                            }
                        }

                        button {
                            class: "w-full px-6 py-3 bg-green-500 text-white rounded-md hover:bg-green-600 transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed font-medium text-lg",
                            disabled: !*is_form_valid.read() || selected_paragraph.read().is_none() || !*has_changes.read(),
                            onclick: handle_submit,
                            "{t.submit}"
                        }
                    } else {
                        // 新增翻譯表單
                        // 段落選擇器和取消按鈕
                        div { class: "flex items-end space-x-4",
                            div { class: "flex-1",
                                ParagraphList {
                                    label: "選擇段落".to_string(),
                                    value: selected_paragraph.read().as_ref().map(|p| p.id.clone()).unwrap_or("選擇段落".to_string()),
                                    paragraphs: available_paragraphs.read().clone(),
                                    is_open: *is_paragraph_open.read(),
                                    search_query: paragraph_search_query.read().to_string(),
                                    on_toggle: move |_| {
                                        let current = *is_paragraph_open.read();
                                        is_paragraph_open.set(!current);
                                    },
                                    on_search: move |query| paragraph_search_query.set(query),
                                    on_select: handle_paragraph_select,
                                }
                            }
                            button {
                                class: "inline-flex items-center justify-center w-10 h-10 text-sm font-medium text-white bg-gray-700 rounded-lg hover:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-600 transition-colors duration-200 shadow-sm mb-[2px]",
                                onclick: move |_| {
                                    let current_mode = *is_edit_mode.read();
                                    is_edit_mode.set(!current_mode);
                                },
                                // 使用加號圖標表示「取消新增段落」
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

                        TextareaField {
                            label: t.paragraph,
                            placeholder: t.paragraph,
                            value: paragraphs.read().to_string(),
                            required: true,
                            has_error: *paragraphs_error.read(),
                            rows: 5,
                            on_input: move |event: FormEvent| {
                                let value = event.value().to_string();
                                validate_field(&value, &mut paragraphs_error);
                                paragraphs.set(value);
                            },
                            on_blur: move |_| {}
                        }

                        ChoiceOptions {
                            t: t.clone(),
                            new_caption: new_caption.read().to_string(),
                            new_goto: new_goto.read().to_string(),
                            extra_captions: extra_captions.read().clone(),
                            extra_gotos: extra_gotos.read().clone(),
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
                            on_extra_caption_change: move |(i, value): (usize, String)| {
                                let mut captions = extra_captions.write();
                                captions[i] = value;
                            },
                            on_extra_goto_change: move |(i, value): (usize, String)| {
                                let mut gotos = extra_gotos.write();
                                gotos[i] = value;
                            },
                            on_add_choice: move |_| {
                                show_extra_options.write().push(());
                                extra_captions.write().push(String::new());
                                extra_gotos.write().push(String::new());
                        }
                    }

                    button {
                            class: "w-full px-6 py-3 bg-green-500 text-white rounded-md hover:bg-green-600 transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed font-medium text-lg",
                            disabled: !*is_form_valid.read() || selected_paragraph.read().is_none() || !*has_changes.read(),
                            onclick: handle_add_translation,
                            "{t.submit}"
                        }
                    }
                }
            }
        }
    }
}