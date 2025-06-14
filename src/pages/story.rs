#![allow(unused_mut)]
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
use crate::services::indexeddb::get_choice_from_indexeddb;
use crate::services::indexeddb::{set_random_choice_to_indexeddb};
use rand::prelude::IteratorRandom;
use std::rc::Rc;
use std::cell::RefCell;
use crate::contexts::story_merged_context::{use_story_merged_context, provide_story_merged_context};
use js_sys;
use futures_util::future::join_all;

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

#[derive(Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct Text {
    pub lang: String,
    pub paragraphs: String,
    pub choices: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ComplexChoice {
    pub to: Vec<String>,
    pub type_: String,
    pub key: Option<String>,
    pub value: Option<serde_json::Value>,
    pub same_page: Option<bool>,
    pub time_limit: Option<u32>,
}

impl<'de> serde::Deserialize<'de> for ComplexChoice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum ToField {
            Multiple(Vec<String>),
            Single(String),
        }
        
        impl Default for ToField {
            fn default() -> Self {
                ToField::Multiple(Vec::new())
            }
        }
        
        #[derive(Deserialize)]
        struct Helper {
            #[serde(default)]
            to: ToField,
            #[serde(rename = "type", default)]
            type_: String,
            #[serde(default)]
            key: Option<String>,
            #[serde(default)]
            value: Option<serde_json::Value>,
            #[serde(default)]
            same_page: Option<bool>,
            #[serde(default)]
            time_limit: Option<u32>,
        }
        
        let helper = Helper::deserialize(deserializer)?;
        let to = match helper.to {
            ToField::Multiple(vec) => vec,
            ToField::Single(s) => if s.is_empty() { Vec::new() } else { vec![s] },
        };
        
        Ok(ComplexChoice {
            to,
            type_: helper.type_,
            key: helper.key,
            value: helper.value,
            same_page: helper.same_page,
            time_limit: helper.time_limit,
        })
    }
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
                    to: complex.to.first().cloned().unwrap_or_default(),
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

/// Merge multiple paragraphs' paragraphs field according to language and reader_mode rules
#[allow(dead_code)]
pub fn merge_paragraphs_for_lang(
    expanded: &[Paragraph],
    current_language: &str,
    reader_mode: bool,
    is_settings_chapter: bool,
    _choice_ids: &[String],
) -> String {
    let mut merged_paragraph_str = String::new();
    
    if reader_mode && !is_settings_chapter {
        // Reader mode: show the entire expanded path
        for paragraph in expanded.iter() {
            if let Some(text) = paragraph.texts.iter().find(|t| t.lang == current_language) {
                if !merged_paragraph_str.is_empty() {
                    merged_paragraph_str.push_str("\n\n");
                }
                merged_paragraph_str.push_str(&text.paragraphs);
            }
        }
    } else {
        // Game mode: concatenate the entire expanded path as well, ensuring the player can
        // review previously visited paragraphs after a page reload.  Real-time duplication is
        // avoided because in game mode `_expanded_paragraphs` normally contains only the last
        // paragraph when navigating to a new page (i.e. `same_page == false`).  Therefore this
        // concatenation will either produce the single latest paragraph or the complete stored
        // history when the page is refreshed.
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

// Add helper function after merge_paragraphs_for_lang
pub fn update_choice_history(mut current_history: Vec<String>, new_paragraph_id: &str) -> Vec<String> {
    if !current_history.contains(&new_paragraph_id.to_string()) {
        current_history.push(new_paragraph_id.to_string());
    }
    current_history
}

/// Compute the list of *enabled* target paragraph IDs for the given choices.
///
/// A choice is considered enabled **as long as it has a non-empty target id** –
/// we no longer require the paragraph to be present in the pre-loaded dataset
/// because it can be fetched on-demand when the user clicks.
#[allow(dead_code)]
pub fn compute_enabled_choices(choices: &[Choice]) -> Vec<String> {
    choices
        .iter()
        .filter_map(|c| {
            let trimmed = c.action.to.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
        .collect()
}

pub fn paragraph_has_translation(paragraphs: &[Paragraph], paragraph_id: &str, lang: &str) -> bool {
    paragraphs
        .iter()
        .find(|p| p.id.trim() == paragraph_id.trim())
        .map(|p| p.texts.iter().any(|t| t.lang == lang))
        .unwrap_or(false)
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
    let _story_context = story_context.clone();
    let _merged_paragraph = use_signal(|| String::new());
    let countdowns = use_signal(|| vec![]);
    let max_times = use_signal(|| vec![]);
    let progress_started = use_signal(|| vec![]);
    let disabled_by_countdown = use_signal(|| vec![]);
    let show_chapter_title = use_signal(|| true);
    let _settings_context = settings_context.clone();
    let _selected_targets: Vec<String> = Vec::new();
    let story_context = story_context.clone(); // 不需要 mut
    
    // Only responsible for background data fetching, update context after fetching
    {
        let mut _paragraph_data = paragraph_data.clone();
        let mut _expanded_paragraphs = _expanded_paragraphs.clone();
        let mut story_context = story_context.clone();
        let mut settings_context = settings_context.clone();
        use_effect(move || {
            spawn_local(async move {
                // 1. First read settings (indexedDB)
                let settings = wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |resolve, _reject| {
                    let cb = Closure::wrap(Box::new(move |js_value: wasm_bindgen::JsValue| {
                        resolve.call1(&JsValue::NULL, &js_value).unwrap_or_else(|e| {
                            tracing::error!("Failed to resolve JS callback: {:?}", e);
                            e
                        });
                    }) as Box<dyn FnMut(wasm_bindgen::JsValue)>);
                    get_settings_from_indexeddb(cb.as_ref().unchecked_ref());
                    cb.forget();
                }));
                let js_value = match settings.await {
                    Ok(val) => val,
                    Err(_e) => {
                        // Logs cleared
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
                // 2. Then load paragraph data
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
                                            // Here first set to paragraph_data (signal), then set to context
                                            _paragraph_data.set(data.items.clone());
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
    
    // Load chapter list
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
    
    // When target paragraph ID changes, update current paragraph
    {
        let mut current_paragraph = current_paragraph.clone();
        let mut current_text = current_text.clone();
        let mut current_choices = current_choices.clone();
        let mut enabled_choices = enabled_choices.clone();
        let _paragraph_data = paragraph_data.clone();
        let state = state.clone();
        let _story_context = story_context.clone();
        let mut _expanded_paragraphs = _expanded_paragraphs.clone();
        let _settings_context = settings_context.clone();
        let last_target_id = Rc::new(RefCell::new(String::new()));
        let _last_target_id = last_target_id.clone();
        use_effect(move || {
            // 取 expanded_paragraphs 的最後一個段落
            let expanded_vec = _expanded_paragraphs.read();
            let paragraph = expanded_vec.last();
            if let Some(paragraph) = paragraph {
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
                        // Check if there are multiple targets, if so perform random selection
                        if c.to.len() > 1 {
                            // Randomly select one target from multi-target choice, but avoid staying on the
                            // same paragraph (i.e. target id == current paragraph id). If after filtering there
                            // are no valid candidates, fall back to the original list.
                            let mut rng = rand::thread_rng();
                            let mut candidates: Vec<String> = c
                                .to
                                .iter()
                                .filter(|id| *id != &paragraph.id)
                                .cloned()
                                .collect();

                            if candidates.is_empty() {
                                candidates = c.to.clone();
                            }

                            let selected_target = candidates
                                .iter()
                                .choose(&mut rng)
                                .cloned()
                                .unwrap_or_default();
                            choice_obj.action.to = selected_target.clone();
                            
                            // Asynchronously record choice to IndexedDB
                            let paragraph_id = paragraph.id.clone();
                            let choice_index = index as u32;
                            let original_choices = c.to.clone();
                            let selected = selected_target.clone();
                            spawn_local(async move {
                                let js_array = js_sys::Array::new();
                                for choice in &original_choices {
                                    js_array.push(&JsValue::from_str(choice));
                                }
                                set_random_choice_to_indexeddb(&paragraph_id, choice_index, &js_array, &selected);
                            });
                        } else if !c.to.is_empty() {
                            choice_obj.action.to = c.to.first().cloned().unwrap_or_default();
                        }
                        choice_obj
                    }).collect();
                    current_choices.set(choices.clone());
                    // Compute enabled targets using the relaxed rule.
                    let enabled = compute_enabled_choices(&choices);
                    enabled_choices.set(enabled);
                }
            }
        });
    }
    
    // Separate effect for reader mode auto-expansion
    {
        let _paragraph_data = paragraph_data.clone();
        let mut _expanded_paragraphs = _expanded_paragraphs.clone();
        let mut story_context = story_context.clone();
        let settings_context = settings_context.clone();
        let state = state.clone();
        let last_expansion_id = Rc::new(RefCell::new(String::new()));
        use_effect(move || {
            let target_id = story_context.read().target_paragraph_id.clone();
            if target_id.is_none() {
                return;
            }
            let target_id_str = target_id.clone().unwrap_or_default();
            if *last_expansion_id.borrow() == target_id_str {
                return;
            }
            if !settings_context.read().loaded {
                return;
            }
            let settings = settings_context.read().settings.clone();
            let _reader_mode_enabled = settings.get("reader_mode").map(|v| v == "true").unwrap_or(false);
            let settings_done = settings.get("settings_done").map(|v| v == "true").unwrap_or(false);
            if let Some(target_id) = target_id {
                if let Ok(_paragraph_data_guard) = _paragraph_data.try_read() {
                    if let Some(paragraph) = _paragraph_data.iter().find(|p| &p.id == &target_id) {
                        if settings_done && _reader_mode_enabled && paragraph.chapter_id != "settingschapter" {
                            if let Some(text) = paragraph.texts.iter().find(|t| t.lang == state().current_language) {
                                if !text.choices.is_empty() {
                                    *last_expansion_id.borrow_mut() = target_id_str;
                                    if let Ok(_paragraph_data_clone) = _paragraph_data.try_read() {
                                        let _paragraph_data = _paragraph_data.clone();
                                        let state = state.clone();
                                        let paragraph = paragraph.clone();
                                        let story_context = story_context.clone();
                                        spawn_local(async move {
                                            let mut visited = vec![paragraph.id.clone()];
                                            let mut path = vec![paragraph.clone()];
                                            let mut current = paragraph.clone();
                                            let mut random_choice_futures = Vec::new();
                                            let mut random_choice_indices = Vec::new();
                                            let mut random_choice_paragraph_ids = Vec::new();
                                            let mut random_choice_originals = Vec::new();
                                            let mut random_choice_targets: Vec<(String, u32)> = Vec::new();
                                            // 預先組出所有要查詢 random_choices 的 (paragraph_id, choice_index, original_choices)
                                            loop {
                                                let text = match current.texts.iter().find(|t| t.lang == state().current_language) {
                                                    Some(t) => t,
                                                    None => { break; }
                                                };
                                                if text.choices.is_empty() { break; }
                                                
                                                let mut available_choices = Vec::new();
                                                for (i, _c) in text.choices.iter().enumerate() {
                                                    if let Some(complex_choice) = current.choices.get(i) {
                                                        if !complex_choice.to.is_empty() {
                                                            available_choices.push((i, complex_choice.clone()));
                                                        }
                                                    }
                                                }

                                                if available_choices.is_empty() { break; }

                                                let (choice_index, choice) = available_choices.into_iter().choose(&mut rand::thread_rng()).unwrap();
                                                
                                                if choice.to.len() > 1 {
                                                    random_choice_paragraph_ids.push(current.id.clone());
                                                    random_choice_indices.push(choice_index as u32);
                                                    random_choice_originals.push(choice.to.clone());
                                                    random_choice_targets.push((current.id.clone(), choice_index as u32));
                                                    break; // Only handle one multi-choice for now, then reconstruct path
                                                } else {
                                                    let next_id = &choice.to[0];
                                                    if let Some(next) = _paragraph_data.iter().find(|p| p.id == *next_id) {
                                                        if visited.contains(&next.id) { break; }
                                                        path.push(next.clone());
                                                        visited.push(next.id.clone());
                                                        current = next.clone();
                                                    } else {
                                                        break;
                                                    }
                                                }
                                            }
                                            // 查詢所有 random_choices
                                            let _selected_targets: Vec<String> = Vec::new();
                                            for (pid, idx) in random_choice_targets.iter() {
                                                let fut = wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |resolve, _reject| {
                                                    let cb = Closure::wrap(Box::new(move |js_value: wasm_bindgen::JsValue| {
                                                        resolve.call1(&JsValue::NULL, &js_value).unwrap_or_else(|e| {
                                                            tracing::error!("Failed to resolve JS callback: {:?}", e);
                                                            e
                                                        });
                                                    }) as Box<dyn FnMut(wasm_bindgen::JsValue)>);
                                                    crate::services::indexeddb::get_random_choice_from_indexeddb(pid, *idx as u32, cb.as_ref().unchecked_ref());
                                                    cb.forget();
                                                }));
                                                random_choice_futures.push(fut);
                                            }
                                            let results = join_all(random_choice_futures).await;
                                            // 用查詢結果組成完整路徑
                                            for (i, result) in results.into_iter().enumerate() {
                                                let original_choices = &random_choice_originals[i];
                                                let paragraph_id = &random_choice_paragraph_ids[i];
                                                let idx = random_choice_indices[i];
                                                let chosen_target = if let Ok(js_value) = result {
                                                    if let Some(s) = js_value.as_string() {
                                                        s
                                                    } else {
                                                        // 沒有紀錄才隨機
                                                        let chosen = original_choices.iter().choose(&mut rand::thread_rng()).cloned().unwrap_or_default();
                                                        // 寫入 random_choices
                                                        let js_array = js_sys::Array::new();
                                                        for choice in original_choices {
                                                            js_array.push(&JsValue::from_str(choice));
                                                        }
                                                        crate::services::indexeddb::set_random_choice_to_indexeddb(paragraph_id, idx, &js_array, &chosen);
                                                        chosen
                                                    }
                                                } else {
                                                    let chosen = original_choices.iter().choose(&mut rand::thread_rng()).cloned().unwrap_or_default();
                                                    let js_array = js_sys::Array::new();
                                                    for choice in original_choices {
                                                        js_array.push(&JsValue::from_str(choice));
                                                    }
                                                    crate::services::indexeddb::set_random_choice_to_indexeddb(paragraph_id, idx, &js_array, &chosen);
                                                    chosen
                                                };
                                                // 找到下一個段落
                                                if let Some(next) = _paragraph_data.iter().find(|p| p.id == chosen_target) {
                                                    if visited.contains(&next.id) { break; }
                                                    path.push(next.clone());
                                                    visited.push(next.id.clone());
                                                } else {
                                                    break;
                                                }
                                            }
                                            _expanded_paragraphs.set(path.clone());
                                            // 新增：讀者模式自動寫入 indexedDB（先比對再寫入）
                                            if _reader_mode_enabled {
                                                if let Some(first) = path.first() {
                                                    let chapter_id = &first.chapter_id;
                                                    let ids: Vec<String> = path.iter().map(|p| p.id.clone()).collect();
                                                    let js_array = js_sys::Array::new();
                                                    for id in &ids {
                                                        js_array.push(&JsValue::from_str(id));
                                                    }
                                                    // 先讀取 indexedDB，只有不同才寫入
                                                    let ids_clone = ids.clone();
                                                    let chapter_id_clone = chapter_id.clone();
                                                    wasm_bindgen_futures::spawn_local(async move {
                                                        if let Ok(js_value) = get_choice_from_indexeddb(&chapter_id_clone).await {
                                                            let arr = js_sys::Array::from(&js_value);
                                                            let existing: Vec<String> = arr.iter().filter_map(|v| v.as_string()).collect();

                                                            if existing.is_empty() {
                                                                crate::services::indexeddb::set_choices_to_indexeddb(&chapter_id_clone, &js_array).await.unwrap();
                                                                // 使用新的選擇
                                                                let mut story_context = story_context.clone();
                                                                let ids = ids_clone.clone();
                                                                story_context.write().choice_ids.set(ids.clone());
                                                                // 展開新的選擇
                                                                let mut expanded = Vec::new();
                                                                for id in &ids {
                                                                    if let Some(p) = _paragraph_data.iter().find(|p| p.id == *id) {
                                                                        expanded.push(p.clone());
                                                                    }
                                                                }
                                                                _expanded_paragraphs.set(expanded);
                                                            } else {
                                                                // 使用 IndexedDB 中的選擇
                                                                let mut story_context = story_context.clone();
                                                                story_context.write().choice_ids.set(existing.clone());
                                                                // 展開 IndexedDB 中的選擇
                                                                let mut expanded = Vec::new();
                                                                for id in &existing {
                                                                    if let Some(p) = _paragraph_data.iter().find(|p| p.id == *id) {
                                                                        expanded.push(p.clone());
                                                                    }
                                                                }
                                                                _expanded_paragraphs.set(expanded);
                                                            }
                                                        }
                                                    });
                                                }
                                            }
                                            let mut story_context = story_context.clone();
                                            story_context.write().target_paragraph_id = None;
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
            ()
        });
    }
    
    // Set initial language
    {
        let state = state.clone();
        use_effect(move || {
            state().set_language(&props.lang);
            ()
        });
    }
    
    // Initialize all chapters' choice_ids at the start
    {
        use_effect(move || {
            let chapters = use_context::<Signal<crate::contexts::story_context::StoryContext>>().read().chapters.read().clone();
            if chapters.is_empty() {
                return;
            }
            let mut story_context = use_context::<Signal<crate::contexts::story_context::StoryContext>>();
            spawn_local(async move {
                let mut all_ids = Vec::new();
                let mut chapters_sorted = chapters.clone();
                chapters_sorted.sort_by_key(|c| c.order);
                for chapter in chapters_sorted.iter() {
                    let _chapter_id = chapter.id.clone();
                    if let Ok(js_value) = get_choice_from_indexeddb(&_chapter_id).await {
                        let arr = js_sys::Array::from(&js_value);
                        let ids: Vec<String> = arr.iter().filter_map(|v| v.as_string()).collect();
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
    
    // Automatically jump to the stored paragraph when reloading the page.
    // In reader-mode we still expand the entire stored path, but in normal mode
    // we should *only* jump to the last paragraph to avoid the accidental
    // concatenation that the user reported.
    {
        let _paragraph_data = paragraph_data.clone();
        let settings_context = settings_context.clone();
        let mut expanded_paragraphs = _expanded_paragraphs.clone();
        let story_context = story_context.clone();
        let mut initialized = use_signal(|| false);
        use_effect(move || {
            if *initialized.read() {
                return;
            }

            // Read the flag at effect-run time (settings are already loaded in an earlier effect)
            let _reader_mode_enabled = settings_context
                .read()
                .settings
                .get("reader_mode")
                .map(|v| v == "true")
                .unwrap_or(false);

            if let Ok(_paragraph_data_vec) = _paragraph_data.try_read() {
                let _paragraph_data_vec = _paragraph_data.clone();
                let ctx = story_context.read();
                let choice_ids_vec = ctx.choice_ids.read().clone();
                if !choice_ids_vec.is_empty() {
                    wasm_bindgen_futures::spawn_local(async move {
                        let mut expanded: Vec<Paragraph> = Vec::new();
                        for paragraph_id in &choice_ids_vec {
                            if let Some(target) = _paragraph_data_vec.iter().find(|p| p.id == *paragraph_id) {
                                if !expanded.iter().any(|p: &Paragraph| p.id == *paragraph_id) {
                                    expanded.push(target.clone());
                                }
                            }
                        }

                        // In normal mode keep only the last paragraph.
                        // (Disabled – we now keep the entire expanded path for both reader and game modes)
                        /*
                        if !reader_mode_enabled {
                            if let Some(last) = expanded.last().cloned() {
                                expanded = vec![last];
                            }
                        }
                        */

                        if !expanded.is_empty() && expanded_paragraphs.read().as_slice() != expanded.as_slice() {
                            expanded_paragraphs.set(expanded.clone());
                            initialized.set(true);
                        }
                    });
                }
            }
        });
    }
    
    let on_choice_click = {
        let mut _expanded_paragraphs = _expanded_paragraphs.clone();
        let mut story_context = story_context.clone();
        let _paragraph_data = paragraph_data.clone();
        let mut show_chapter_title = show_chapter_title.clone();
        move |(goto, choice_index): (String, usize)| {
            let expanded_vec = _expanded_paragraphs.read().clone();
            let last_paragraph = expanded_vec.last().cloned();

            if let Some(ref last) = last_paragraph {
                if !last.chapter_id.is_empty() {
                    let order = story_context.read().chapters.read().iter().find(|c| c.id == last.chapter_id).map(|c| c.order).unwrap_or(0);
                    if order != 0 {
                        // 判斷是否多目標選項
                        let is_multi_target = last.choices.get(choice_index).map(|c| c.to.len() > 1).unwrap_or(false);
                        if is_multi_target {
                            // 多目標只寫入 random_choices，不寫入 choices
                            let paragraph_id = last.id.clone();
                            let original_choices = last.choices[choice_index].to.clone();
                            let selected = goto.clone();

                            let js_array = js_sys::Array::new();
                            for choice in &original_choices {
                                js_array.push(&JsValue::from_str(choice));
                            }

                            set_random_choice_to_indexeddb(&paragraph_id, choice_index as u32, &js_array, &selected);
                        } else {
                            // Always build history from existing stored choice_ids to preserve the full
                            // navigation path even after page reloads.  Fallback to currently expanded
                            // paragraphs if the history is still empty (e.g. very first choice).
                            let existing_history = story_context.read().choice_ids.read().clone();
                            let mut ids: Vec<String> = if existing_history.is_empty() {
                                expanded_vec.iter().map(|p| p.id.clone()).collect()
                            } else {
                                existing_history
                            };

                            ids = update_choice_history(ids, &goto);
                            // 更新 context 內的 choice_ids，避免其他 effect 將 expanded 還原
                            story_context.write().choice_ids.set(ids.clone());
                            let js_array = js_sys::Array::new();
                            for id in &ids {
                                js_array.push(&JsValue::from_str(id));
                            }
                            let chapter_id = last.chapter_id.clone();
                            spawn_local(async move {
                                let _ = crate::services::indexeddb::set_choices_to_indexeddb(&chapter_id, &js_array).await;
                            });
                        }
                    }
                }
            }

            if let Ok(_paragraph_data_read) = _paragraph_data.try_read() {
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
                    // async: Set after writing, immediately get_settings, and update context, then jump
                    let mut settings_context = settings_context.clone();
                    let mut _expanded_paragraphs = _expanded_paragraphs.clone();
                    let paragraphs = _paragraph_data.clone();
                    let goto = goto.clone();
                    let mut story_context = story_context.clone();
                    let mut show_chapter_title = show_chapter_title.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        if let (Some(key), Some(value)) = (setting_key, setting_value) {
                            set_setting_to_indexeddb(&key, &value);
                            // Get latest settings
                            let settings = wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |resolve, _reject| {
                                let cb = Closure::wrap(Box::new(move |js_value: wasm_bindgen::JsValue| {
                                    resolve.call1(&JsValue::NULL, &js_value).unwrap_or_else(|e| {
                                        tracing::error!("Failed to resolve JS callback: {:?}", e);
                                        e
                                    });
                                }) as Box<dyn FnMut(wasm_bindgen::JsValue)>);
                                get_settings_from_indexeddb(cb.as_ref().unchecked_ref());
                                cb.forget();
                            }));

                            let js_value = match settings.await {
                                Ok(val) => val,
                                Err(_e) => {
                                    // Logs cleared
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

                        // Jump to first chapter
                        if let Some(target_paragraph) = paragraphs.iter().find(|p| p.id == goto) {
                            _expanded_paragraphs.set(vec![target_paragraph.clone()]);
                            story_context.write().target_paragraph_id = Some(goto.clone());
                            show_chapter_title.set(true);
                        }
                    });
                    return;
                }

                if let Some(ref target_paragraph) = _paragraph_data.iter().find(|p| p.id == goto) {
                    if let Some(ref last) = last_paragraph {
                        if !last.chapter_id.is_empty() {
                            let order = story_context.read().chapters.read().iter().find(|c| c.id == last.chapter_id).map(|c| c.order).unwrap_or(0);
                            if order != 0 {
                                // ※ 已於上方 set_choices_to_indexeddb 儲存完整路徑，這裡不再覆寫為單一段落。
                            }
                        }
                    }
                    // Determine whether this choice should stay on the same page by directly
                    // checking the *clicked* choice (using choice_index) instead of searching
                    // by target paragraph id. This avoids accidentally matching another choice
                    // that happens to contain the same target but has a different `same_page` setting.
                    let mut same_page = false;
                    if let Some(ref last) = last_paragraph {
                        if let Some(choice) = last.choices.get(choice_index) {
                            same_page = choice.same_page.unwrap_or(false);
                        }
                    }

                    if same_page {
                        let mut expanded = _expanded_paragraphs.read().clone();
                        // Avoid pushing duplicate paragraph if it is already the last item
                        if expanded.last().map(|p| p.id != target_paragraph.id).unwrap_or(true) {
                            expanded.push((*target_paragraph).clone());
                        }
                        _expanded_paragraphs.set(expanded);
                        show_chapter_title.set(true);
                    } else {
                        // Auto scroll to top when switching new page
                        if let Some(window) = web_sys::window() {
                            window.scroll_to_with_x_and_y(0.0, 0.0);
                        }
                        _expanded_paragraphs.set(vec![(*target_paragraph).clone()]);
                        show_chapter_title.set(false);
                    }
                } else {
                    // Fallback: target paragraph is not yet loaded – fetch it from the API and proceed.
                    let goto_id = goto.clone();
                    let paragraph_data_signal = _paragraph_data.clone();
                    let mut _expanded_paragraphs = _expanded_paragraphs.clone();
                    let mut story_context = story_context.clone();
                    let mut show_chapter_title = show_chapter_title.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        // Construct the record URL: /collections/paragraphs/records/{id}
                        let fetch_url = format!("{}{}{}{}", BASE_API_URL, PARAGRAPHS, "/", goto_id);
                        let client = reqwest::Client::new();
                        if let Ok(response) = client.get(&fetch_url).send().await {
                            if response.status().is_success() {
                                if let Ok(text) = response.text().await {
                                    // Attempt to deserialize directly into a `Paragraph` record.
                                    if let Ok(paragraph) = serde_json::from_str::<Paragraph>(&text) {
                                        // Append to the global paragraph dataset.
                                        let mut pd_signal = paragraph_data_signal;
                                        let mut current_pd = pd_signal.read().clone();
                                        // Avoid duplicates.
                                        if !current_pd.iter().any(|p| p.id == paragraph.id) {
                                            current_pd.push(paragraph.clone());
                                            pd_signal.set(current_pd.clone());
                                            story_context.write().paragraphs.set(current_pd.clone());
                                        }

                                        // Update expanded paragraph path – mimic normal navigation (new page).
                                        _expanded_paragraphs.set(vec![paragraph.clone()]);
                                        story_context.write().target_paragraph_id = Some(paragraph.id.clone());
                                        show_chapter_title.set(false);
                                    }
                                }
                            }
                        }
                    });
                }
            }
        }
    };

    // Merge paragraph content into merged context
    {
        let expanded = _expanded_paragraphs.clone();
        let _paragraph_data = paragraph_data.clone();
        let state = state.clone();
        let story_context = story_context.clone();
        let settings_context = settings_context.clone();
        let story_merged_context = story_merged_context.clone();
        use_effect(move || {
            let expanded = expanded.read();
            if let Ok(_paragraph_data_read) = _paragraph_data.try_read() {
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
                merged_paragraph_signal.set(merged_paragraph_str.clone());
            }
            ()
        });
    }
    
    // Main effect: Set settings, update paragraph_id, initialize countdown
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
                // Initialize countdown only when paragraph ID changes
                if let Some(_text) = paragraph.texts.iter().find(|t| t.lang == state().current_language) {
                    let countdowns_vec = paragraph.choices.iter().map(|c| c.time_limit.unwrap_or(0)).collect::<Vec<u32>>();
                    // Logs cleared
                    story_context.write().countdowns.set(countdowns_vec);
                }
            }
        }
        ()
    });
    
    // Listen for page visibility change, hide overlay when losing focus
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
    // Get current chapter title
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
    let current_paragraph_id = use_signal(|| String::new());
    
    // Listen for _expanded_paragraphs changes and update current_paragraph_id
    {
        let mut current_paragraph_id = current_paragraph_id.clone();
        let _expanded_paragraphs = _expanded_paragraphs.clone();
        use_effect(move || {
            let expanded = _expanded_paragraphs.read();
            let new_id = expanded.last().map(|p| p.id.clone()).unwrap_or_default();
            current_paragraph_id.set(new_id);
        });
    }
    
    rsx! {
        StoryContent {
            paragraph: story_merged_context.read().merged_paragraph.clone(),
            choices: current_choices.read().clone(),
            enabled_choices: enabled_choices.read().clone(),
            on_choice_click: on_choice_click.clone(),
            on_toggle_reader_mode: EventHandler::new(|_| {}), // Empty handler since we moved it to navbar
            countdowns: countdowns.clone(),
            max_times: max_times.clone(),
            progress_started: progress_started.clone(),
            disabled_by_countdown: disabled_by_countdown.clone(),
            reader_mode: reader_mode,
            chapter_title: chapter_title,
            current_paragraph_id: current_paragraph_id,
        }
    }
}