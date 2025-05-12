use dioxus::prelude::*;
use dioxus_core::IntoDynNode;
use dioxus_core::fc_to_builder;
use wasm_bindgen_futures::spawn_local;
use serde::Deserialize;
use dioxus_i18n::t;
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
struct Paragraph {
    id: String,
    #[serde(default)]
    chapter_id: String,
    texts: Vec<Text>,
    choices: Vec<ComplexChoice>,
    #[serde(rename = "collectionId", default)]
    collection_id: String,
    #[serde(rename = "collectionName", default)]
    collection_name: String,
    #[serde(default)]
    created: String,
    #[serde(default)]
    updated: String,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
struct Text {
    lang: String,
    paragraphs: String,
    choices: Vec<String>,
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
struct Chapter {
    id: String,
    order: i32,
}

#[component]
pub fn Story(props: StoryProps) -> Element {
    let state = use_context::<Signal<LanguageState>>();
    let story_context = use_story_context();
    let settings_context = use_settings_context();
    let current_paragraph = use_signal(|| None::<Paragraph>);
    let current_text = use_signal(|| None::<Text>);
    let current_choices = use_signal(|| Vec::<Choice>::new());
    let enabled_choices = use_signal(|| Vec::<String>::new());
    let paragraph_data = use_signal(|| Vec::<Paragraph>::new());
    let expanded_paragraphs = use_signal(|| Vec::<Paragraph>::new());
    let chapters = use_signal(|| Vec::<Chapter>::new());
    
    // 載入設定與段落數據（合併為一個 async 流程）
    {
        let mut paragraph_data = paragraph_data.clone();
        let mut expanded_paragraphs = expanded_paragraphs.clone();
        let mut story_context = story_context.clone();
        let mut settings_context = settings_context.clone();
        use_effect(move || {
            spawn_local(async move {
                // 1. 先讀取 settings（indexedDB）
                let settings = wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |resolve, _reject| {
                    let cb = Closure::wrap(Box::new(move |js_value: wasm_bindgen::JsValue| {
                        resolve.call1(&JsValue::NULL, &js_value).unwrap();
                    }) as Box<dyn FnMut(wasm_bindgen::JsValue)>);
                    get_settings_from_indexeddb(cb.as_ref().unchecked_ref());
                    cb.forget();
                }));
                let js_value = settings.await.unwrap();
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
                                            let skip_setting = settings_context.read().settings.get("settings_done").map(|v| v == "true").unwrap_or(false);
                                            let first_paragraph = if skip_setting {
                                                data.items.iter().find(|p| p.id == "gmld01c8s9982iy")
                                            } else {
                                                data.items.first()
                                            };
                                            if let Some(first_paragraph) = first_paragraph {
                                                {
                                                    let mut ctx = story_context.write();
                                                    ctx.target_paragraph_id = Some(first_paragraph.id.clone());
                                                }
                                                expanded_paragraphs.set(vec![first_paragraph.clone()]);
                                            }
                                            paragraph_data.set(data.items);
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
            (move || {})()
        });
    }
    
    // 載入章節列表
    {
        let mut chapters = chapters.clone();
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
                                        result.push(Chapter { id, order });
                                    }
                                    chapters.set(result);
                                }
                            }
                        }
                    }
                }
            });
            (move || {})()
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
        let mut expanded_paragraphs = expanded_paragraphs.clone();
        
        use_effect(move || {
            let target_id = story_context.read().target_paragraph_id.clone();
            
            if let Some(target_id) = target_id {
                if let Some(paragraph) = paragraph_data.read().iter().find(|p| &p.id == &target_id) {
                    current_paragraph.set(Some(paragraph.clone()));
                    
                    if let Some(text) = paragraph.texts.iter().find(|t| t.lang == state().current_language) {
                        current_text.set(Some(text.clone()));
                        let choices: Vec<Choice> = text.choices.iter().enumerate().map(|(index, c)| {
                            let choice: StoryChoice = if let Some(complex_choice) = paragraph.choices.get(index) {
                                StoryChoice::Complex(complex_choice.clone())
                            } else {
                                StoryChoice::Simple(c.clone())
                            };
                            let mut choice_obj: Choice = choice.into();
                            choice_obj.caption = c.clone();
                            choice_obj
                        }).collect();
                        current_choices.set(choices);
                        
                        // 檢查每個選項的目標段落是否有當前語言的翻譯
                        let mut enabled = Vec::new();
                        let paragraph_data_read = paragraph_data.read();
                        for choice in &text.choices {
                            let target_paragraph = paragraph_data_read.iter().find(|p| {
                                if let Some(complex_choice) = paragraph.choices.get(text.choices.iter().position(|c| c == choice).unwrap_or(0)) {
                                    p.id == complex_choice.to
                                } else {
                                    p.id == *choice
                                }
                            });
                            
                            if let Some(target_paragraph) = target_paragraph {
                                if target_paragraph.texts.iter().any(|t| t.lang == state().current_language) {
                                    enabled.push(choice.clone());
                                }
                            }
                        }
                        enabled_choices.set(enabled);
                    }
                }
            }
            
            (move || {})()
        });
    }
    
    // 設置初始語言
    {
        let state = state.clone();
        use_effect(move || {
            state().set_language(&props.lang);
            (move || {})()
        });
    }
    
    // 自動跳轉到已選段落
    {
        let paragraph_data = paragraph_data.clone();
        let mut expanded_paragraphs = expanded_paragraphs.clone();
        let story_context = story_context.clone();
        let chapters = chapters.clone();
        use_effect(move || {
            let paragraph_data = paragraph_data.read();
            let chapters = chapters.read();
            // 只在段落與章節都載入後執行
            if paragraph_data.is_empty() || chapters.is_empty() {
                return;
            }
            // 取得目前章節 id
            let current_paragraph = expanded_paragraphs.read().last().cloned();
            let chapter_id = current_paragraph.as_ref().map(|p| p.chapter_id.clone()).unwrap_or_default();
            if chapter_id.is_empty() {
                return;
            }
            // 查詢 indexedDB
            let paragraph_data = paragraph_data.clone();
            let mut expanded_paragraphs = expanded_paragraphs.clone();
            let mut story_context = story_context.clone();
            let cb = Closure::wrap(Box::new(move |js_value: JsValue| {
                let arr = js_sys::Array::from(&js_value);
                if let Some(paragraph_id) = arr.get(0).as_string() {
                    // 找到該段落
                    if let Some(target) = paragraph_data.iter().find(|p| p.id == paragraph_id) {
                        let mut expanded = expanded_paragraphs.read().clone();
                        // 避免重複 push
                        if !expanded.iter().any(|p| p.id == paragraph_id) {
                            expanded.push(target.clone());
                            expanded_paragraphs.set(expanded);
                            let mut ctx = story_context.write();
                            ctx.target_paragraph_id = Some(paragraph_id);
                        }
                    }
                }
            }) as Box<dyn FnMut(JsValue)>);
            get_choice_from_indexeddb(&chapter_id, cb.as_ref().unchecked_ref());
            cb.forget();
            (move || {})()
        });
    }
    
    let on_choice_click = {
        let paragraph_data = paragraph_data.clone();
        let mut expanded_paragraphs = expanded_paragraphs.clone();
        let mut story_context = story_context.clone();
        let chapters = chapters.clone();
        move |(goto, choice_index): (String, usize)| {
            let paragraphs = paragraph_data.read();
            let last_paragraph = {
                let expanded = expanded_paragraphs.read();
                expanded.last().cloned()
            };
            if let Some(ref current_paragraph) = last_paragraph {
                if let Some(choice) = current_paragraph.choices.get(choice_index) {
                    if choice.type_ == "settings" || choice.type_ == "setting" {
                        if let (Some(key), Some(value)) = (choice.key.as_ref(), choice.value.as_ref()) {
                            let value_str = match value {
                                serde_json::Value::String(s) => s.clone(),
                                _ => value.to_string(),
                            };
                            set_setting_to_indexeddb(key, &value_str);
                        }
                    }
                }
            }
            if let Some(ref target_paragraph) = paragraphs.iter().find(|p| p.id == goto) {
                // 第一章 id 通常是 "gmld01c8s9982iy"，其後才記錄
                if let Some(ref last) = last_paragraph {
                    if !last.chapter_id.is_empty() && last.chapter_id != "gmld01c8s9982iy" {
                        // 查找章節 order
                        let order = chapters.read().iter().find(|c| c.id == last.chapter_id).map(|c| c.order).unwrap_or(0);
                        if order != 0 {
                            set_choice_to_indexeddb(&last.chapter_id, &goto);
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
                    let mut expanded = expanded_paragraphs.read().clone();
                    expanded.push((*target_paragraph).clone());
                    let _ = expanded_paragraphs;
                    expanded_paragraphs = expanded_paragraphs.clone();
                    expanded_paragraphs.set(expanded);
                } else {
                    // 切換新頁時自動捲動到頁首
                    if let Some(window) = web_sys::window() {
                        window.scroll_to_with_x_and_y(0.0, 0.0);
                    }
                    let _ = expanded_paragraphs;
                    expanded_paragraphs = expanded_paragraphs.clone();
                    expanded_paragraphs.set(vec![(*target_paragraph).clone()]);
                }
                {
                    let mut ctx = story_context.write();
                    ctx.target_paragraph_id = Some(goto);
                }
            }
        }
    };
    
    rsx! {
        div {
            class: "max-w-3xl mx-auto py-8 md:p-8",
            {
                // 合併所有段落內容
                let mut merged_paragraph = String::new();
                let mut merged_choices = Vec::new();
                let mut merged_enabled_choices = Vec::new();
                let expanded = expanded_paragraphs.read();
                let paragraph_data_read = paragraph_data.read();
                for (i, paragraph) in expanded.iter().enumerate() {
                    if let Some(text) = paragraph.texts.iter().find(|t| t.lang == state().current_language) {
                        if !merged_paragraph.is_empty() {
                            merged_paragraph.push_str("\n\n");
                        }
                        merged_paragraph.push_str(&text.paragraphs);
                        // 只合併最後一個段落的選項
                        if i == expanded.len() - 1 {
                            merged_choices = text.choices.iter().enumerate().map(|(index, c)| {
                                let choice: StoryChoice = if let Some(complex_choice) = paragraph.choices.get(index) {
                                    StoryChoice::Complex(complex_choice.clone())
                                } else {
                                    StoryChoice::Simple(c.clone())
                                };
                                let mut choice_obj: Choice = choice.into();
                                choice_obj.caption = c.clone();
                                choice_obj
                            }).collect();
                            merged_enabled_choices = {
                                let mut enabled = Vec::new();
                                for choice in &text.choices {
                                    let target_paragraph = paragraph_data_read.iter().find(|p| {
                                        if let Some(complex_choice) = paragraph.choices.get(text.choices.iter().position(|c| c == choice).unwrap_or(0)) {
                                            p.id == complex_choice.to
                                        } else {
                                            p.id == *choice
                                        }
                                    });
                                    if let Some(target_paragraph) = target_paragraph {
                                        if target_paragraph.texts.iter().any(|t| t.lang == state().current_language) {
                                            enabled.push(choice.clone());
                                        }
                                    }
                                }
                                enabled
                            };
                        }
                    }
                }
                rsx! {
                    StoryContent {
                        paragraph: merged_paragraph,
                        choices: merged_choices,
                        enabled_choices: merged_enabled_choices,
                        on_choice_click: on_choice_click.clone(),
                    }
                }
            }
            if paragraph_data.read().is_empty() {
                div { class: "text-white", "{t!(\"loading\")}" }
            }
        }
    }
}