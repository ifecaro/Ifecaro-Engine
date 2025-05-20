use dioxus::prelude::*;
use dioxus_core::fc_to_builder;
use wasm_bindgen_futures::spawn_local;
use serde::Deserialize;
use crate::components::story_content::{StoryContent, Choice, Action};
use crate::contexts::story_context::use_story_context;
use crate::contexts::language_context::LanguageState;
use crate::constants::config::{BASE_API_URL, PARAGRAPHS, CHAPTERS};
use crate::services::indexeddb::set_setting_to_indexeddb;
use crate::contexts::settings_context::use_settings_context;
use crate::services::indexeddb::get_settings_from_indexeddb;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use crate::services::indexeddb::set_choice_to_indexeddb;
use crate::services::indexeddb::get_choice_from_indexeddb;
use rand::prelude::SliceRandom;
use std::rc::Rc;
use std::cell::RefCell;
use crate::contexts::story_merged_context::{use_story_merged_context, provide_story_merged_context};

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
struct Data {
    items: Vec<Paragraph>,
    page: i32,
    #[serde(rename = "perPage")]
    per_page: i32,
    #[serde(rename = "totalItems")]
    total_items: i32,
    #[serde(rename = "totalPages")]
    total_pages: i32,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct Paragraph {
    pub id: String,
    #[serde(default)]
    pub chapter_id: String,
    pub texts: Vec<Text>,
    pub choices: Vec<ComplexChoice>,
    #[serde(rename = "collectionId", default)]
    pub collection_id: String,
    #[serde(rename = "collectionName", default)]
    pub collection_name: String,
    #[serde(default)]
    pub created: String,
    #[serde(default)]
    pub updated: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Text {
    pub lang: String,
    pub paragraphs: String,
    pub choices: Vec<String>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct ComplexChoice {
    #[serde(default)]
    pub to: String,
    #[serde(rename = "type", default)]
    pub type_: String,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[serde(default)]
    pub same_page: Option<bool>,
    #[serde(default)]
    pub time_limit: Option<u32>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum StoryChoice {
    Complex(ComplexChoice),
    Simple(String),
}

impl From<StoryChoice> for Choice {
    fn from(choice: StoryChoice) -> Self {
        match choice {
            StoryChoice::Complex(complex) => Self {
                caption: String::new(),
                action: Action {
                    type_: complex.type_,
                    key: complex.key,
                    value: complex.value,
                    to: complex.to,
                },
            },
            StoryChoice::Simple(text) => Self {
                caption: text.clone(),
                action: Action {
                    type_: "goto".to_string(),
                    key: None,
                    value: None,
                    to: text,
                },
            },
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct StoryProps {
    pub lang: String,
}

#[derive(Deserialize, Clone, Debug)]
struct ChapterTitle {
    lang: String,
    title: String,
}

#[derive(Clone, Debug)]
pub struct Chapter {
    id: String,
    titles: Vec<ChapterTitle>,
    order: i32,
}

/// 合併多個段落的 paragraphs 欄位，根據語言與 reader_mode 規則
#[allow(dead_code)]
pub fn merge_paragraphs_for_lang(
    expanded: &[Paragraph],
    current_language: &str,
    reader_mode: bool,
    is_settings_chapter: bool,
    choice_ids: &[String],
) -> String {
    let mut merged_paragraph_str = String::new();
    if reader_mode && !is_settings_chapter {
        for (idx, paragraph) in expanded.iter().enumerate() {
            if idx == 0 || (choice_ids.contains(&paragraph.id) && paragraph.chapter_id != "settingschapter") {
                if let Some(text) = paragraph.texts.iter().find(|t| t.lang == current_language) {
                    if !merged_paragraph_str.is_empty() {
                        merged_paragraph_str.push_str("\n\n");
                    }
                    merged_paragraph_str.push_str(&text.paragraphs);
                }
            }
        }
    } else {
        for paragraph in expanded.iter() {
            if let Some(text) = paragraph.texts.iter().find(|t| t.lang == current_language) {
                if !merged_paragraph_str.is_empty() {
                    merged_paragraph_str.push_str("\n\n");
                }
                merged_paragraph_str.push_str(&text.paragraphs);
            }
        }
    }
    merged_paragraph_str
}

#[component]
pub fn Story(props: StoryProps) -> Element {
    provide_story_merged_context();
    let story_merged_context = use_story_merged_context();
    let state = use_context::<Signal<LanguageState>>();
    let story_context = use_story_context();
    let settings_context = use_settings_context();
    let current_paragraph = use_signal(|| None::<Paragraph>);
    let current_text = use_signal(|| None::<Text>);
    let current_choices = use_signal(|| Vec::<Choice>::new());
    let enabled_choices = use_signal(|| Vec::<String>::new());
    let paragraph_data = use_signal(|| story_context.read().paragraphs.read().clone());
    let mut _expanded_paragraphs = use_signal(|| {
        let ctx = story_context.read();
        let paragraphs = ctx.paragraphs.read();
        if !paragraphs.is_empty() {
            vec![paragraphs[0].clone()]
        } else {
            Vec::new()
        }
    });
    let last_paragraph_id = Rc::new(RefCell::new(String::new()));
    let _last_paragraph_id_effect = last_paragraph_id.clone();
    let story_context = story_context.clone();
    let _merged_paragraph = use_signal(|| String::new());
    let countdowns = use_signal(|| vec![]);
    let max_times = use_signal(|| vec![]);
    let progress_started = use_signal(|| vec![]);
    let disabled_by_countdown = use_signal(|| vec![]);
    let show_chapter_title = use_signal(|| true);
    
    // 只負責背景抓資料，抓到後再更新 context
    {
        let mut paragraph_data = paragraph_data.clone();
        let mut _expanded_paragraphs = _expanded_paragraphs.clone();
        let mut story_context = story_context.clone();
        let mut settings_context = settings_context.clone();
        use_effect(move || {
            spawn_local(async move {
                // 1. 先讀取 settings（indexedDB）
                let settings = wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |resolve, _reject| {
                    let cb = Closure::wrap(Box::new(move |js_value: wasm_bindgen::JsValue| {
                        resolve.call1(&JsValue::NULL, &js_value).unwrap_or_else(|e| {
                            // 日誌已清空
                            e
                        });
                    }) as Box<dyn FnMut(wasm_bindgen::JsValue)>);
                    get_settings_from_indexeddb(cb.as_ref().unchecked_ref());
                    cb.forget();
                }));
                let js_value = match settings.await {
                    Ok(val) => val,
                    Err(_e) => {
                        // 日誌已清空
                        return;
                    }
                };
                let mut map = std::collections::HashMap::new();
                if let Some(obj) = js_sys::Object::try_from(&js_value) {
                    let keys = js_sys::Object::keys(&obj);
                    for i in 0..keys.length() {
                        let key = keys.get(i);
                        let value = js_sys::Reflect::get(&obj, &key).unwrap_or(js_sys::JsString::from("").into());
                        map.insert(key.as_string().unwrap_or_default(), value.as_string().unwrap_or_default());
                    }
                }
                {
                    let mut ctx = settings_context.write();
                    ctx.settings = map;
                    ctx.loaded = true;
                }
                // 2. 再載入段落資料
                let paragraphs_url = format!("{}{}", BASE_API_URL, PARAGRAPHS);
                let client = reqwest::Client::new();
                match client.get(&paragraphs_url).send().await {
                    Ok(response) => {
                        if response.status().is_success() {
                            match response.text().await {
                                Ok(text) => {
                                    match serde_json::from_str::<Data>(&text) {
                                        Ok(data) => {
                                            let target_id = "storystartpoint";
                                            let skip_setting = settings_context.read().settings.get("settings_done").map(|v| v == "true").unwrap_or(false);
                                            let first_paragraph = if skip_setting {
                                                data.items.iter().find(|p| p.id.trim() == target_id)
                                                    .or_else(|| data.items.first())
                                            } else {
                                                data.items.first()
                                            };
                                            if let Some(first_paragraph) = first_paragraph {
                                                {
                                                    let mut ctx = story_context.write();
                                                    ctx.target_paragraph_id = Some(first_paragraph.id.clone());
                                                }
                                                _expanded_paragraphs.set(vec![first_paragraph.clone()]);
                                            }
                                            // 這裡先 set 到 paragraph_data（signal），再 set 到 context
                                            paragraph_data.set(data.items.clone());
                                            story_context.write().paragraphs.set(data.items.clone());
                                        },
                                        Err(_e) => {}
                                    }
                                },
                                Err(_e) => {}
                            }
                        }
                    },
                    Err(_e) => {}
                }
            });
            ()
        });
    }
    
    // 載入章節列表
    {
        let mut story_context = story_context.clone();
        use_effect(move || {
            spawn_local(async move {
                let chapters_url = format!("{}{}", BASE_API_URL, CHAPTERS);
                let client = reqwest::Client::new();
                if let Ok(response) = client.get(&chapters_url).send().await {
                    if response.status().is_success() {
                        if let Ok(text) = response.text().await {
                            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                                if let Some(items) = json.get("items").and_then(|i| i.as_array()) {
                                    let mut result = Vec::new();
                                    for item in items {
                                        let id = item.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        let order = item.get("order").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                                        let titles = item.get("titles").and_then(|v| v.as_array()).map(|arr| {
                                            arr.iter().filter_map(|title_obj| {
                                                let lang = title_obj.get("lang").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                                let title = title_obj.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                                if !lang.is_empty() && !title.is_empty() {
                                                    Some(ChapterTitle { lang, title })
                                                } else {
                                                    None
                                                }
                                            }).collect::<Vec<_>>()
                                        }).unwrap_or_default();
                                        result.push(Chapter { id, titles, order });
                                    }
                                    story_context.write().chapters.set(result);
                                }
                            }
                        }
                    }
                }
            });
            ()
        });
    }
    
    // 當目標段落ID改變時，更新當前段落
    {
        let mut current_paragraph = current_paragraph.clone();
        let mut current_text = current_text.clone();
        let mut current_choices = current_choices.clone();
        let mut enabled_choices = enabled_choices.clone();
        let paragraph_data = paragraph_data.clone();
        let state = state.clone();
        let story_context = story_context.clone();
        let mut _expanded_paragraphs = _expanded_paragraphs.clone();
        let settings_context = settings_context.clone();
        let last_target_id = Rc::new(RefCell::new(String::new()));
        let last_target_id = last_target_id.clone();
        use_effect(move || {
            let target_id = story_context.read().target_paragraph_id.clone();
            let target_id_str = target_id.clone().unwrap_or_default();
            if *last_target_id.borrow() != target_id_str {
                *last_target_id.borrow_mut() = target_id_str;
            }
            if target_id.is_none() {
                return;
            }
            if let Some(target_id) = target_id {
                if let Some(paragraph) = paragraph_data.read().iter().find(|p| &p.id == &target_id) {
                    current_paragraph.set(Some(paragraph.clone()));
                    if let Some(text) = paragraph.texts.iter().find(|t| t.lang == state().current_language) {
                        current_text.set(Some(text.clone()));
                        let choices: Vec<Choice> = paragraph.choices.iter().enumerate().map(|(index, c)| {
                            let choice: StoryChoice = StoryChoice::Complex(c.clone());
                            let mut choice_obj: Choice = choice.into();
                            if let Some(text) = paragraph.texts.iter().find(|t| t.lang == state().current_language) {
                                if let Some(caption) = text.choices.get(index) {
                                    choice_obj.caption = caption.clone();
                                }
                            }
                            choice_obj.action.to = c.to.clone();
                            choice_obj
                        }).collect();
                        current_choices.set(choices.clone());
                        // 檢查每個選項的目標段落是否有當前語言的翻譯
                        let mut enabled = Vec::new();
                        let _paragraph_data_read = paragraph_data.read();
                        for choice in &paragraph.choices {
                            let target_paragraph = _paragraph_data_read.iter().find(|p| p.id == choice.to);
                            if let Some(target_paragraph) = target_paragraph {
                                if target_paragraph.texts.iter().any(|t| t.lang == state().current_language) {
                                    enabled.push(choice.to.clone());
                                }
                            }
                        }
                        enabled_choices.set(enabled);
                        // ====== 新版 reader_mode 單線劇情自動展開邏輯（async） ======
                        // 先檢查 settings_context 是否 loaded
                        if !settings_context.read().loaded {
                            return;
                        }
                        let settings = settings_context.read().settings.clone();
                        let settings_done = settings.get("settings_done").map(|v| v == "true").unwrap_or(false);
                        let reader_mode = settings.get("reader_mode").map(|v| v == "true").unwrap_or(false);
                        let _chapter_id = paragraph.chapter_id.clone();
                        let choices = choices.clone();
                        if settings_done && reader_mode && paragraph.chapter_id != "settingschapter" && !choices.is_empty() {
                            let paragraph_data = paragraph_data.read().clone();
                            let state = state.clone();
                            let paragraph = paragraph.clone();
                            let mut story_context = story_context.clone();
                            spawn_local(async move {
                                let mut visited = vec![paragraph.id.clone()];
                                let mut path = vec![paragraph.clone()];
                                let mut current = paragraph.clone();
                                loop {
                                    let text = match current.texts.iter().find(|t| t.lang == state().current_language) {
                                        Some(t) => t,
                                        None => {
                                            // 日誌已清空
                                            break;
                                        }
                                    };
                                    if text.choices.is_empty() { break; }
                                    let mut choice_ids: Vec<String> = Vec::new();
                                    for (i, c) in text.choices.iter().enumerate() {
                                        if let Some(complex_choice) = current.choices.get(i) {
                                            choice_ids.push(complex_choice.to.clone());
                                        } else {
                                            choice_ids.push(c.clone());
                                        }
                                    }
                                    let valid_choice_ids: Vec<String> = choice_ids.iter().filter(|id| !id.is_empty()).cloned().collect();
                                    if valid_choice_ids.is_empty() {
                                        let mut story_context = story_context.clone();
                                        story_context.write().target_paragraph_id = None;
                                        _expanded_paragraphs.set(path.clone());
                                        break;
                                    }
                                    let context_choice_id: Option<String> = {
                                        let ctx = story_context.read();
                                        let ids = ctx.choice_ids.read();
                                        if !ids.is_empty() {
                                            ids.first().cloned()
                                        } else {
                                            None
                                        }
                                    };
                                    let next_id = if let Some(id) = context_choice_id {
                                        if !id.is_empty() {
                                            id
                                        } else {
                                            match valid_choice_ids.choose(&mut rand::thread_rng()).cloned() {
                                                Some(val) => val,
                                                None => {
                                                    // 日誌已清空
                                                    break;
                                                }
                                            }
                                        }
                                    } else {
                                        match valid_choice_ids.choose(&mut rand::thread_rng()).cloned() {
                                            Some(chosen) => {
                                                if !current.chapter_id.is_empty() {
                                                    set_choice_to_indexeddb(&current.chapter_id, &chosen);
                                                    story_context.write().choice_ids.set(vec![chosen.clone()]);
                                                }
                                                chosen
                                            },
                                            None => {
                                                // 日誌已清空
                                                break;
                                            }
                                        }
                                    };
                                    if let Some(next) = paragraph_data.iter().find(|p| p.id == next_id) {
                                        if visited.contains(&next.id) { break; }
                                        path.push(next.clone());
                                        visited.push(next.id.clone());
                                        current = next.clone();
                                    } else {
                                        break;
                                    }
                                }
                                _expanded_paragraphs.set(path.clone());
                                let mut story_context = story_context.clone();
                                story_context.write().target_paragraph_id = None;
                            });
                            return;
                        }
                        // ====== end reader_mode ======
                    }
                }
            }
            ()
        });
    }
    
    // 設置初始語言
    {
        let state = state.clone();
        use_effect(move || {
            state().set_language(&props.lang);
            ()
        });
    }
    
    // 初始化時同步所有章節的 choice_ids
    {
        use_effect(move || {
            let chapters = use_context::<Signal<crate::contexts::story_context::StoryContext>>().read().chapters.read().clone();
            if chapters.is_empty() {
                return;
            }
            let mut story_context = use_context::<Signal<crate::contexts::story_context::StoryContext>>();
            spawn_local(async move {
                let mut all_ids = Vec::new();
                for chapter in chapters {
                    let _chapter_id = chapter.id.clone();
                    let (tx, rx) = futures_channel::oneshot::channel();
                    let cb = Closure::once(Box::new(move |js_value: JsValue| {
                        let arr = js_sys::Array::from(&js_value);
                        let ids: Vec<String> = arr.iter().filter_map(|v| v.as_string()).collect();
                        let _ = tx.send(ids);
                    }) as Box<dyn FnOnce(JsValue)>);
                    get_choice_from_indexeddb(&chapter.id, cb.as_ref().unchecked_ref());
                    cb.forget();
                    if let Ok(ids) = rx.await {
                        all_ids.extend(ids);
                    }
                }
                let current_ids = story_context.read().choice_ids.read().clone();
                if current_ids != all_ids {
                    story_context.write().choice_ids.set(all_ids);
                }
            });
            ()
        });
    }
    
    // 自動跳轉到已選段落（只讀，寫入分離 async block，資料 clone）
    {
        let paragraph_data = paragraph_data.clone();
        let _expanded_paragraphs = _expanded_paragraphs.clone();
        let story_context = story_context.clone();
        use_effect(move || {
            let paragraph_data_vec = paragraph_data.read().clone();
            let ctx = story_context.read();
            let choice_ids_vec = ctx.choice_ids.read().clone();
            let mut expanded_paragraphs = _expanded_paragraphs.clone();
            let story_context = story_context.clone();
            if let Some(paragraph_id) = choice_ids_vec.first() {
                if let Some(target) = paragraph_data_vec.iter().find(|p| &p.id == paragraph_id) {
                    let target = target.clone();
                    let paragraph_id = paragraph_id.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let mut expanded = expanded_paragraphs.read().clone();
                        if !expanded.iter().any(|p| &p.id == &paragraph_id) {
                            expanded.push(target);
                            expanded_paragraphs.set(expanded);
                            let mut story_context = story_context.clone();
                            story_context.write().target_paragraph_id = Some(paragraph_id);
                        }
                    });
                }
            }
        });
    }
    
    let on_choice_click = {
        let paragraph_data = paragraph_data.clone();
        let mut _expanded_paragraphs = _expanded_paragraphs.clone();
        let story_context = story_context.clone();
        let mut show_chapter_title = show_chapter_title.clone();
        move |(goto, choice_index): (String, usize)| {
            let expanded_vec = _expanded_paragraphs.read().clone();
            let last_paragraph = expanded_vec.last().cloned();
            if let Some(ref last) = last_paragraph {
                if !last.chapter_id.is_empty() {
                    let order = story_context.read().chapters.read().iter().find(|c| c.id == last.chapter_id).map(|c| c.order).unwrap_or(0);
                    if order != 0 {
                        set_choice_to_indexeddb(&last.chapter_id, &goto);
                        let mut story_context = story_context.clone();
                        story_context.write().choice_ids.set(vec![goto.clone()]);
                    }
                }
            }
            let _paragraph_data_read = paragraph_data.read();
            let mut is_setting_action = false;
            let mut setting_key = None;
            let mut setting_value = None;
            if let Some(ref current_paragraph) = last_paragraph {
                if let Some(choice) = current_paragraph.choices.get(choice_index) {
                    if choice.type_ == "settings" || choice.type_ == "setting" {
                        if let (Some(key), Some(value)) = (choice.key.as_ref(), choice.value.as_ref()) {
                            let value_str = match value {
                                serde_json::Value::String(s) => s.clone(),
                                _ => value.to_string(),
                            };
                            is_setting_action = true;
                            setting_key = Some(key.clone());
                            setting_value = Some(value_str);
                        }
                    }
                }
            }
            if is_setting_action {
                // async: 設定寫入後馬上 get_settings 並更新 context，再跳轉
                let mut settings_context = settings_context.clone();
                let mut _expanded_paragraphs = _expanded_paragraphs.clone();
                let paragraphs = _paragraph_data_read.clone();
                let goto = goto.clone();
                let mut story_context = story_context.clone();
                let mut show_chapter_title = show_chapter_title.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    if let (Some(key), Some(value)) = (setting_key, setting_value) {
                        set_setting_to_indexeddb(&key, &value);
                        // 取得最新 settings
                        let settings = wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |resolve, _reject| {
                            let cb = Closure::wrap(Box::new(move |js_value: wasm_bindgen::JsValue| {
                                resolve.call1(&JsValue::NULL, &js_value).unwrap_or_else(|e| {
                                    // 日誌已清空
                                    e
                                });
                            }) as Box<dyn FnMut(wasm_bindgen::JsValue)>);
                            get_settings_from_indexeddb(cb.as_ref().unchecked_ref());
                            cb.forget();
                        }));
                        let js_value = match settings.await {
                            Ok(val) => val,
                            Err(_e) => {
                                // 日誌已清空
                                return;
                            }
                        };
                        let mut map = std::collections::HashMap::new();
                        if let Some(obj) = js_sys::Object::try_from(&js_value) {
                            let keys = js_sys::Object::keys(&obj);
                            for i in 0..keys.length() {
                                let key = keys.get(i);
                                let value = js_sys::Reflect::get(&obj, &key).unwrap_or(js_sys::JsString::from("").into());
                                map.insert(key.as_string().unwrap_or_default(), value.as_string().unwrap_or_default());
                            }
                        }
                        {
                            let mut ctx = settings_context.write();
                            ctx.settings = map;
                            ctx.loaded = true;
                        }
                    }
                    // 跳轉到第一章
                    if let Some(target_paragraph) = paragraphs.iter().find(|p| p.id == goto) {
                        _expanded_paragraphs.set(vec![target_paragraph.clone()]);
                        story_context.write().target_paragraph_id = Some(goto.clone());
                        show_chapter_title.set(true);
                    }
                });
                return;
            }
            if let Some(ref target_paragraph) = _paragraph_data_read.iter().find(|p| p.id == goto) {
                if let Some(ref last) = last_paragraph {
                    if !last.chapter_id.is_empty() {
                        let order = story_context.read().chapters.read().iter().find(|c| c.id == last.chapter_id).map(|c| c.order).unwrap_or(0);
                        if order != 0 {
                            set_choice_to_indexeddb(&last.chapter_id, &goto);
                            let mut story_context = story_context.clone();
                            story_context.write().choice_ids.set(vec![goto.clone()]);
                        }
                    }
                }
                let mut same_page = false;
                if let Some(ref last) = last_paragraph {
                    if let Some(idx) = last.choices.iter().position(|choice| choice.to == goto) {
                        if let Some(choice) = last.choices.get(idx) {
                            same_page = choice.same_page.unwrap_or(false);
                        }
                    }
                }
                if same_page {
                    let mut expanded = _expanded_paragraphs.read().clone();
                    expanded.push((*target_paragraph).clone());
                    let _ = _expanded_paragraphs;
                    _expanded_paragraphs = _expanded_paragraphs.clone();
                    _expanded_paragraphs.set(expanded);
                    show_chapter_title.set(true);
                } else {
                    // 切換新頁時自動捲動到頁首
                    if let Some(window) = web_sys::window() {
                        window.scroll_to_with_x_and_y(0.0, 0.0);
                    }
                    let _ = _expanded_paragraphs;
                    _expanded_paragraphs = _expanded_paragraphs.clone();
                    _expanded_paragraphs.set(vec![(*target_paragraph).clone()]);
                    show_chapter_title.set(false);
                    // 加回 target_paragraph_id 設定
                    let mut story_context = story_context.clone();
                    story_context.write().target_paragraph_id = Some(goto);
                }
            }
        }
    };
    
    // 合併段落內容寫入 merged context
    {
        let expanded = _expanded_paragraphs.clone();
        let paragraph_data = paragraph_data.clone();
        let state = state.clone();
        let story_context = story_context.clone();
        let settings_context = settings_context.clone();
        let story_merged_context = story_merged_context.clone();
        use_effect(move || {
            let expanded = expanded.read();
            let _paragraph_data_read = paragraph_data.read();
            let reader_mode = settings_context.read().settings.get("reader_mode").map(|v| v == "true").unwrap_or(false);
            let chapter_id = expanded.last().map(|p| p.chapter_id.clone()).unwrap_or_default();
            let is_settings_chapter = chapter_id == "settingschapter";
            let choice_ids = story_context.read().choice_ids.read().clone();
            let merged_paragraph_str = merge_paragraphs_for_lang(
                &expanded,
                &state.read().current_language,
                reader_mode,
                is_settings_chapter,
                &choice_ids,
            );
            let mut merged_paragraph_signal = story_merged_context.read().merged_paragraph.clone();
            merged_paragraph_signal.set(merged_paragraph_str);
            ()
        });
    }
    
    // 主要效果：設置settings、更新paragraph_id、初始化倒數計時
    use_effect(move || {
        let mut story_context = story_context.clone();
        let expanded = _expanded_paragraphs.clone();
        let state = state.clone();
        let last_paragraph_id = last_paragraph_id.clone();
        let expanded = expanded.read();
        if let Some(paragraph) = expanded.last() {
            let is_settings_chapter = paragraph.chapter_id == "settingschapter";
            story_context.write().is_settings_chapter.set(is_settings_chapter);
            if *last_paragraph_id.borrow() != paragraph.id {
                *last_paragraph_id.borrow_mut() = paragraph.id.clone();
                // 只在段落ID變動時初始化倒數計時
                if let Some(_text) = paragraph.texts.iter().find(|t| t.lang == state().current_language) {
                    let countdowns_vec = paragraph.choices.iter().map(|c| c.time_limit.unwrap_or(0)).collect::<Vec<u32>>();
                    // 日誌已清空
                    story_context.write().countdowns.set(countdowns_vec);
                }
            }
        }
        ()
    });
    
    // 監聽頁面可見性變化，失去焦點時顯示遮罩
    {
        use_effect(move || {
            use wasm_bindgen::JsCast;
            use web_sys::window;
            let window = window().unwrap();
            let document = std::rc::Rc::new(window.document().unwrap());
            let doc_cloned = std::rc::Rc::clone(&document);
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                let hidden = doc_cloned.hidden();
                if hidden {
                    if let Some(container) = doc_cloned.query_selector(".story-content-container").ok().flatten() {
                        let event = web_sys::CustomEvent::new("show_filter").unwrap();
                        let _ = container.dispatch_event(&event);
                    }
                }
            }) as Box<dyn FnMut()>);
            document.add_event_listener_with_callback("visibilitychange", closure.as_ref().unchecked_ref()).unwrap();
            closure.forget();
            (|| {})()
        });
    }
    
    let reader_mode = settings_context.read().settings.get("reader_mode").map(|v| v == "true").unwrap_or(false);
    // 取得目前章節標題
    let chapter_title = {
        if !*show_chapter_title.read() {
            String::new()
        } else {
            let expanded = _expanded_paragraphs.read();
            let ctx = story_context.read();
            let chapters = ctx.chapters.read();
            let state = state.read();
            let current_lang = &state.current_language;
            let chapter_id = expanded.last().map(|p| p.chapter_id.clone()).unwrap_or_default();
            if chapter_id.is_empty() {
                String::new()
            } else {
                chapters.iter()
                    .find(|c| c.id == chapter_id)
                    .and_then(|chapter| {
                        chapter.titles.iter()
                            .find(|t| &t.lang == current_lang)
                            .or_else(|| chapter.titles.iter().find(|t| t.lang == "en-US" || t.lang == "en-GB"))
                            .or_else(|| chapter.titles.first())
                            .map(|t| t.title.clone())
                    })
                    .unwrap_or_default()
            }
        }
    };
    rsx! {
        StoryContent {
            paragraph: story_merged_context.read().merged_paragraph.clone(),
            choices: current_choices.read().clone(),
            enabled_choices: enabled_choices.read().clone(),
            on_choice_click: on_choice_click.clone(),
            countdowns: countdowns.clone(),
            max_times: max_times.clone(),
            progress_started: progress_started.clone(),
            disabled_by_countdown: disabled_by_countdown.clone(),
            reader_mode: reader_mode,
            chapter_title: chapter_title,
        }
    }
}