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
use crate::services::indexeddb::{set_random_choice_to_indexeddb};
use rand::prelude::SliceRandom;
use rand::prelude::IteratorRandom;
use std::rc::Rc;
use std::cell::RefCell;
use crate::contexts::story_merged_context::{use_story_merged_context, provide_story_merged_context};
use js_sys;

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
        // In reader mode, show all paragraphs in the expanded path
        for paragraph in expanded.iter() {
            if let Some(text) = paragraph.texts.iter().find(|t| t.lang == current_language) {
                if !merged_paragraph_str.is_empty() {
                    merged_paragraph_str.push_str("\n\n");
                }
                merged_paragraph_str.push_str(&text.paragraphs);
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
    
    // Only responsible for background data fetching, update context after fetching
    {
        let mut paragraph_data = paragraph_data.clone();
        let mut _expanded_paragraphs = _expanded_paragraphs.clone();
        let mut story_context = story_context.clone();
        let mut settings_context = settings_context.clone();
        use_effect(move || {
            spawn_local(async move {
                // 1. First read settings (indexedDB)
                let settings = wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |resolve, _reject| {
                    let cb = Closure::wrap(Box::new(move |js_value: wasm_bindgen::JsValue| {
                        resolve.call1(&JsValue::NULL, &js_value).unwrap_or_else(|e| {
                            // Logs cleared
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
                if let Ok(paragraph_data_guard) = paragraph_data.try_read() {
                    if let Some(paragraph) = paragraph_data_guard.iter().find(|p| &p.id == &target_id) {
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
                                    // Randomly select one target from multi-target paragraph
                                    let selected_target = c.to.iter()
                                        .choose(&mut rand::thread_rng())
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
                            // Check if each option's target paragraph has translation in current language
                            let mut enabled = Vec::new();
                            if let Ok(_paragraph_data_read) = paragraph_data.try_read() {
                                // Use already randomly selected choices for checking
                                for choice in &choices {
                                    let target_id = &choice.action.to;
                                    if !target_id.is_empty() {
                                        if let Some(target_paragraph) = _paragraph_data_read.iter().find(|p| p.id == *target_id) {
                                            if target_paragraph.texts.iter().any(|t| t.lang == state().current_language) {
                                                enabled.push(target_id.clone());
                                            }
                                        }
                                    }
                                }
                            }
                            enabled_choices.set(enabled);
                        }
                    }
                }
            }
            ()
        });
    }
    
    // Separate effect for reader mode auto-expansion
    {
        let paragraph_data = paragraph_data.clone();
        let mut _expanded_paragraphs = _expanded_paragraphs.clone();
        let story_context = story_context.clone();
        let settings_context = settings_context.clone();
        let state = state.clone();
        let last_expansion_id = Rc::new(RefCell::new(String::new()));
        
        use_effect(move || {
            let target_id = story_context.read().target_paragraph_id.clone();
            if target_id.is_none() {
                return;
            }
            
            let target_id_str = target_id.clone().unwrap_or_default();
            
            // Prevent duplicate expansions for the same paragraph
            if *last_expansion_id.borrow() == target_id_str {
                return;
            }
            
            if !settings_context.read().loaded {
                return;
            }
            
            let settings = settings_context.read().settings.clone();
            let settings_done = settings.get("settings_done").map(|v| v == "true").unwrap_or(false);
            let reader_mode = settings.get("reader_mode").map(|v| v == "true").unwrap_or(false);
            
            if let Some(target_id) = target_id {
                if let Ok(paragraph_data_guard) = paragraph_data.try_read() {
                    if let Some(paragraph) = paragraph_data_guard.iter().find(|p| &p.id == &target_id) {
                        if settings_done && reader_mode && paragraph.chapter_id != "settingschapter" {
                            if let Some(text) = paragraph.texts.iter().find(|t| t.lang == state().current_language) {
                                if !text.choices.is_empty() {
                                    // Mark this paragraph as processed
                                    *last_expansion_id.borrow_mut() = target_id_str;
                                    
                                    if let Ok(paragraph_data_clone) = paragraph_data.try_read() {
                                        let paragraph_data = paragraph_data_clone.clone();
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
                                                        break;
                                                    }
                                                };
                                                
                                                if text.choices.is_empty() { 
                                                    break; 
                                                }
                                                
                                                let mut choice_ids: Vec<String> = Vec::new();
                                                for (i, c) in text.choices.iter().enumerate() {
                                                    if let Some(complex_choice) = current.choices.get(i) {
                                                        // Randomly select one target from multi-target paragraph
                                                        if !complex_choice.to.is_empty() {
                                                            let chosen_target = complex_choice.to.iter()
                                                                .choose(&mut rand::thread_rng())
                                                                .cloned()
                                                                .unwrap_or_default();
                                                            choice_ids.push(chosen_target);
                                                        }
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
                                                            break;
                                                        }
                                                    }
                                                };
                                                
                                                if let Some(next) = paragraph_data.iter().find(|p| p.id == next_id) {
                                                    if visited.contains(&next.id) { 
                                                        break; 
                                                    }
                                                    path.push(next.clone());
                                                    visited.push(next.id.clone());
                                                    current = next.clone();
                                                    
                                                    // Clear context choice_ids to allow fresh random selection for next iteration
                                                    story_context.write().choice_ids.set(vec![]);
                                                } else {
                                                    break;
                                                }
                                            }
                                            
                                            _expanded_paragraphs.set(path.clone());
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
    
    // Automatically jump to selected paragraph (read-only, separate async block for data cloning)
    {
        let paragraph_data = paragraph_data.clone();
        let _expanded_paragraphs = _expanded_paragraphs.clone();
        let story_context = story_context.clone();
        use_effect(move || {
            if let Ok(paragraph_data_vec) = paragraph_data.try_read() {
                let paragraph_data_vec = paragraph_data_vec.clone();
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
            if let Ok(paragraph_data_read) = paragraph_data.try_read() {
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
                    let paragraphs = paragraph_data_read.clone();
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
                                        // Logs cleared
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
                if let Some(ref target_paragraph) = paragraph_data_read.iter().find(|p| p.id == goto) {
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
                        // Find matching target in multiple target paragraph
                        if let Some(choice) = last.choices.iter().find(|choice| choice.to.contains(&goto)) {
                            same_page = choice.same_page.unwrap_or(false);
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
                        // Auto scroll to top when switching new page
                        if let Some(window) = web_sys::window() {
                            window.scroll_to_with_x_and_y(0.0, 0.0);
                        }
                        let _ = _expanded_paragraphs;
                        _expanded_paragraphs = _expanded_paragraphs.clone();
                        _expanded_paragraphs.set(vec![(*target_paragraph).clone()]);
                        show_chapter_title.set(false);
                        // Add back target_paragraph_id setting
                        let mut story_context = story_context.clone();
                        story_context.write().target_paragraph_id = Some(goto);
                    }
                }
            }
        }
    };
    
    // Merge paragraph content into merged context
    {
        let expanded = _expanded_paragraphs.clone();
        let paragraph_data = paragraph_data.clone();
        let state = state.clone();
        let story_context = story_context.clone();
        let settings_context = settings_context.clone();
        let story_merged_context = story_merged_context.clone();
        use_effect(move || {
            let expanded = expanded.read();
            if let Ok(_paragraph_data_read) = paragraph_data.try_read() {
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