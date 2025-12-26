#![allow(unused_mut)]
use crate::components::story_content::{Action, Choice, StoryContent};
use crate::constants::config::{BASE_API_URL, CHAPTERS, PARAGRAPHS};
use crate::contexts::language_context::LanguageState;
use crate::contexts::settings_context::use_settings_context;
use crate::contexts::story_context::use_story_context;
use crate::contexts::story_merged_context::{
    provide_story_merged_context, use_story_merged_context,
};
use crate::models::impacts::{CharacterStateSnapshot, Impact};
use crate::services::indexeddb::get_choice_from_indexeddb;
use crate::services::indexeddb::get_settings_from_indexeddb;
use crate::services::indexeddb::set_setting_to_indexeddb;
use crate::services::indexeddb::{
    get_latest_character_state_from_indexeddb, set_latest_character_state_to_indexeddb,
    set_random_choice_to_indexeddb,
};
use crate::utils::theme::{apply_theme_class, ThemeMode};
use dioxus::prelude::*;
use dioxus_core::fc_to_builder;
use futures_util::future::join_all;
use gloo_timers::callback::Timeout;
use js_sys;
use rand::prelude::IteratorRandom;
use serde::Deserialize;
use serde_json;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;

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
    pub timeout_to: Option<String>,
    pub impacts: Option<Vec<Impact>>,
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
            #[serde(default, rename = "timeout_to")]
            timeout_to: Option<String>,
            #[serde(default)]
            impacts: Option<Vec<Impact>>,
        }

        let helper = Helper::deserialize(deserializer)?;
        let to = match helper.to {
            ToField::Multiple(vec) => vec,
            ToField::Single(s) => {
                if s.is_empty() {
                    Vec::new()
                } else {
                    vec![s]
                }
            }
        };

        Ok(ComplexChoice {
            to,
            type_: helper.type_,
            key: helper.key,
            value: helper.value,
            same_page: helper.same_page,
            time_limit: helper.time_limit,
            timeout_to: helper.timeout_to,
            impacts: helper.impacts,
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
                caption: String::new().into(),
                action: Action {
                    type_: complex.type_.into(),
                    key: complex.key,
                    value: complex.value,
                    to: Cow::Owned(complex.to.first().cloned().unwrap_or_default()),
                },
            },
            StoryChoice::Simple(text) => Self {
                caption: Cow::Owned(text.clone()),
                action: Action {
                    type_: "goto".into(),
                    key: None,
                    value: None,
                    to: Cow::Owned(text),
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
    _is_settings_chapter: bool,
    _choice_ids: &[String],
) -> String {
    // 遊戲模式 (reader_mode == false):
    //   • 一般點擊（expanded.len()==1）→ 只顯示最後一段。
    //   • 重新整理還原（expanded.len()>1）→ 顯示完整歷程。
    // 讀者模式 (reader_mode == true) → 永遠顯示完整歷程。

    let paragraphs_to_process: Vec<&Paragraph> = if reader_mode || expanded.len() > 1 {
        expanded.iter().collect()
    } else {
        expanded.last().into_iter().collect()
    };

    let estimated_capacity: usize = paragraphs_to_process
        .iter()
        .filter_map(|p| {
            p.texts
                .iter()
                .find(|t| t.lang == current_language)
                .map(|t| t.paragraphs.len() + 2)
        })
        .sum();
    let mut merged_paragraph_str = String::with_capacity(estimated_capacity);

    // If it's a settings chapter in reader mode, we still want to merge paragraphs just like normal chapters.

    for paragraph in paragraphs_to_process {
        if let Some(text) = paragraph.texts.iter().find(|t| t.lang == current_language) {
            if !merged_paragraph_str.is_empty() {
                merged_paragraph_str.push_str("\n\n");
            }
            merged_paragraph_str.push_str(&text.paragraphs);
        }
    }

    merged_paragraph_str
}

// Add helper function after merge_paragraphs_for_lang
pub fn update_choice_history(
    mut current_history: Vec<String>,
    new_paragraph_id: &str,
) -> Vec<String> {
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
pub fn compute_enabled_choices(choices: &[Choice]) -> HashSet<String> {
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
    let enabled_choices = use_signal(|| HashSet::<String>::new());
    let paragraph_data = use_signal(|| story_context.read().paragraphs.read().clone());
    let mut _expanded_paragraphs = use_signal(|| {
        let ctx = story_context.read();
        let paragraphs = ctx.paragraphs.read();
        let choice_ids = ctx.choice_ids.read();

        // If we already have a stored history, rebuild the full path immediately so that
        // the UI is consistent even before the async auto-restore impact runs.
        if !choice_ids.is_empty() {
            let mut path = Vec::new();
            for id in choice_ids.iter() {
                if let Some(p) = paragraphs.iter().find(|p| p.id == *id) {
                    path.push(p.clone());
                }
            }
            if !path.is_empty() {
                return path;
            }
        }

        // Fallback to the very first paragraph when no history is available yet.
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
    let auto_restored = use_signal(|| false);
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
        let mut fetch_initialized = use_signal(|| false);
        use_effect(move || {
            if *fetch_initialized.peek() {
                return;
            }
            fetch_initialized.set(true);
            spawn_local(async move {
                // 1. First read settings (indexedDB)
                let settings = wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(
                    &mut |resolve, _reject| {
                        let cb = Closure::wrap(Box::new(move |js_value: wasm_bindgen::JsValue| {
                            resolve
                                .call1(&JsValue::NULL, &js_value)
                                .unwrap_or_else(|e| {
                                    tracing::error!("Failed to resolve JS callback: {:?}", e);
                                    e
                                });
                        })
                            as Box<dyn FnMut(wasm_bindgen::JsValue)>);
                        get_settings_from_indexeddb(cb.as_ref().unchecked_ref());
                        cb.forget();
                    },
                ));
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
                        let value = js_sys::Reflect::get(&obj, &key)
                            .unwrap_or(js_sys::JsString::from("").into());
                        map.insert(
                            key.as_string().unwrap_or_default(),
                            value.as_string().unwrap_or_default(),
                        );
                    }
                }
                let theme_mode = map
                    .get("theme_mode")
                    .cloned()
                    .unwrap_or_else(|| "auto".to_string());
                {
                    let mut ctx = settings_context.write();
                    ctx.settings = map;
                    ctx.loaded = true;
                }
                apply_theme_class(ThemeMode::from_value(&theme_mode));
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
                                            let skip_setting = settings_context
                                                .read()
                                                .settings
                                                .get("settings_done")
                                                .map(|v| v == "true")
                                                .unwrap_or(false);
                                            let first_paragraph = if skip_setting {
                                                data.items
                                                    .iter()
                                                    .find(|p| p.id.trim() == target_id)
                                                    .or_else(|| data.items.first())
                                            } else {
                                                data.items.first()
                                            };
                                            if let Some(first_paragraph) = first_paragraph {
                                                // Only reset the expanded path when we don't already have a
                                                // reconstructed history (i.e. no stored choice_ids).
                                                let has_history = !story_context
                                                    .read()
                                                    .choice_ids
                                                    .read()
                                                    .is_empty();
                                                if !has_history {
                                                    if let Ok(mut ctx) = story_context.try_write() {
                                                        ctx.target_paragraph_id =
                                                            Some(first_paragraph.id.clone());
                                                    }
                                                    if let Ok(mut expanded) =
                                                        _expanded_paragraphs.try_write()
                                                    {
                                                        *expanded = vec![first_paragraph.clone()];
                                                    }
                                                }
                                            }
                                            // Here first set to paragraph_data (signal), then set to context
                                            if let Ok(mut paragraph_guard) =
                                                _paragraph_data.try_write()
                                            {
                                                *paragraph_guard = data.items.clone();
                                            }
                                            if let Ok(mut ctx) = story_context.try_write() {
                                                if let Ok(mut paragraphs) =
                                                    ctx.paragraphs.try_write()
                                                {
                                                    *paragraphs = data.items.clone();
                                                }
                                            }
                                        }
                                        Err(_e) => {}
                                    }
                                }
                                Err(_e) => {}
                            }
                        }
                    }
                    Err(_e) => {}
                }
            });
            ()
        });
    }

    // Load chapter list
    {
        let mut story_context = story_context.clone();
        let mut chapters_loaded = use_signal(|| false);
        use_effect(move || {
            if *chapters_loaded.peek() {
                return;
            }
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
                                        let id = item
                                            .get("id")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("")
                                            .to_string();
                                        let order =
                                            item.get("order").and_then(|v| v.as_i64()).unwrap_or(0)
                                                as i32;
                                        let titles = item
                                            .get("titles")
                                            .and_then(|v| v.as_array())
                                            .map(|arr| {
                                                arr.iter()
                                                    .filter_map(|title_obj| {
                                                        let lang = title_obj
                                                            .get("lang")
                                                            .and_then(|v| v.as_str())
                                                            .unwrap_or("")
                                                            .to_string();
                                                        let title = title_obj
                                                            .get("title")
                                                            .and_then(|v| v.as_str())
                                                            .unwrap_or("")
                                                            .to_string();
                                                        if !lang.is_empty() && !title.is_empty() {
                                                            Some(ChapterTitle { lang, title })
                                                        } else {
                                                            None
                                                        }
                                                    })
                                                    .collect::<Vec<_>>()
                                            })
                                            .unwrap_or_default();
                                        result.push(Chapter { id, titles, order });
                                    }
                                    if let Ok(mut ctx) = story_context.try_write() {
                                        if let Ok(mut chapters) = ctx.chapters.try_write() {
                                            *chapters = result;
                                        }
                                    }
                                    if let Ok(mut loaded) = chapters_loaded.try_write() {
                                        *loaded = true;
                                    }
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
            let expanded_vec = _expanded_paragraphs.read().clone();
            let paragraph = expanded_vec.last();
            if let Some(paragraph) = paragraph {
                current_paragraph.set(Some(paragraph.clone()));
                if let Some(text) = paragraph
                    .texts
                    .iter()
                    .find(|t| t.lang == state().current_language)
                {
                    current_text.set(Some(text.clone()));
                    let choices: Vec<Choice> = paragraph
                        .choices
                        .iter()
                        .enumerate()
                        .map(|(index, c)| {
                            let choice: StoryChoice = StoryChoice::Complex(c.clone());
                            let mut choice_obj: Choice = choice.into();
                            if let Some(text) = paragraph
                                .texts
                                .iter()
                                .find(|t| t.lang == state().current_language)
                            {
                                if let Some(caption) = text.choices.get(index) {
                                    choice_obj.caption = caption.clone().into();
                                }
                            }
                            // Check if there are multiple targets, if so perform random selection
                            if c.to.len() > 1 {
                                // Randomly select one target from multi-target choice, but avoid staying on the
                                // same paragraph (i.e. target id == current paragraph id). If after filtering there
                                // are no valid candidates, fall back to the original list.
                                let mut rng = rand::thread_rng();
                                let mut candidates: Vec<String> =
                                    c.to.iter()
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
                                choice_obj.action.to = Cow::Owned(selected_target.clone());

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
                                    set_random_choice_to_indexeddb(
                                        &paragraph_id,
                                        choice_index,
                                        &js_array,
                                        &selected,
                                    );
                                });
                            } else if !c.to.is_empty() {
                                choice_obj.action.to =
                                    Cow::Owned(c.to.first().cloned().unwrap_or_default());
                            }
                            choice_obj
                        })
                        .collect();
                    current_choices.set(choices.clone());
                    // Compute enabled targets using the relaxed rule.
                    let enabled = compute_enabled_choices(&choices);
                    enabled_choices.set(enabled);
                }
            }
        });
    }

    // Separate impact for reader mode auto-expansion
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
            let _reader_mode_enabled = settings
                .get("reader_mode")
                .map(|v| v == "true")
                .unwrap_or(false);
            let settings_done = settings
                .get("settings_done")
                .map(|v| v == "true")
                .unwrap_or(false);
            if let Some(target_id) = target_id {
                if let Ok(_paragraph_data_guard) = _paragraph_data.try_read() {
                    if let Some(paragraph) = _paragraph_data.iter().find(|p| &p.id == &target_id) {
                        if settings_done
                            && _reader_mode_enabled
                            && paragraph.chapter_id != "settingschapter"
                        {
                            if let Some(text) = paragraph
                                .texts
                                .iter()
                                .find(|t| t.lang == state().current_language)
                            {
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
                                            let mut random_choice_targets: Vec<(String, u32)> =
                                                Vec::new();
                                            // 預先組出所有要查詢 random_choices 的 (paragraph_id, choice_index, original_choices)
                                            loop {
                                                let text =
                                                    match current.texts.iter().find(|t| {
                                                        t.lang == state().current_language
                                                    }) {
                                                        Some(t) => t,
                                                        None => {
                                                            break;
                                                        }
                                                    };
                                                if text.choices.is_empty() {
                                                    break;
                                                }

                                                let mut available_choices = Vec::new();
                                                for (i, _c) in text.choices.iter().enumerate() {
                                                    if let Some(complex_choice) =
                                                        current.choices.get(i)
                                                    {
                                                        if !complex_choice.to.is_empty() {
                                                            available_choices
                                                                .push((i, complex_choice.clone()));
                                                        }
                                                    }
                                                }

                                                if available_choices.is_empty() {
                                                    break;
                                                }

                                                let (choice_index, choice) = available_choices
                                                    .into_iter()
                                                    .choose(&mut rand::thread_rng())
                                                    .unwrap();

                                                if choice.to.len() > 1 {
                                                    random_choice_paragraph_ids
                                                        .push(current.id.clone());
                                                    random_choice_indices.push(choice_index as u32);
                                                    random_choice_originals.push(choice.to.clone());
                                                    random_choice_targets.push((
                                                        current.id.clone(),
                                                        choice_index as u32,
                                                    ));
                                                    break; // Only handle one multi-choice for now, then reconstruct path
                                                } else {
                                                    let next_id = &choice.to[0];
                                                    if let Some(next) = _paragraph_data
                                                        .iter()
                                                        .find(|p| p.id == *next_id)
                                                    {
                                                        if visited.contains(&next.id) {
                                                            break;
                                                        }
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
                                                let fut = wasm_bindgen_futures::JsFuture::from(
                                                    js_sys::Promise::new(
                                                        &mut |resolve, _reject| {
                                                            let cb = Closure::wrap(Box::new(move |js_value: wasm_bindgen::JsValue| {
                                                        resolve.call1(&JsValue::NULL, &js_value).unwrap_or_else(|e| {
                                                            tracing::error!("Failed to resolve JS callback: {:?}", e);
                                                            e
                                                        });
                                                    }) as Box<dyn FnMut(wasm_bindgen::JsValue)>);
                                                            crate::services::indexeddb::get_random_choice_from_indexeddb(pid, *idx as u32, cb.as_ref().unchecked_ref());
                                                            cb.forget();
                                                        },
                                                    ),
                                                );
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
                                                        let chosen = original_choices
                                                            .iter()
                                                            .choose(&mut rand::thread_rng())
                                                            .cloned()
                                                            .unwrap_or_default();
                                                        // 寫入 random_choices
                                                        let js_array = js_sys::Array::new();
                                                        for choice in original_choices {
                                                            js_array
                                                                .push(&JsValue::from_str(choice));
                                                        }
                                                        crate::services::indexeddb::set_random_choice_to_indexeddb(paragraph_id, idx, &js_array, &chosen);
                                                        chosen
                                                    }
                                                } else {
                                                    let chosen = original_choices
                                                        .iter()
                                                        .choose(&mut rand::thread_rng())
                                                        .cloned()
                                                        .unwrap_or_default();
                                                    let js_array = js_sys::Array::new();
                                                    for choice in original_choices {
                                                        js_array.push(&JsValue::from_str(choice));
                                                    }
                                                    crate::services::indexeddb::set_random_choice_to_indexeddb(paragraph_id, idx, &js_array, &chosen);
                                                    chosen
                                                };
                                                // 找到下一個段落
                                                if let Some(next) = _paragraph_data
                                                    .iter()
                                                    .find(|p| p.id == chosen_target)
                                                {
                                                    if visited.contains(&next.id) {
                                                        break;
                                                    }
                                                    path.push(next.clone());
                                                    visited.push(next.id.clone());
                                                } else {
                                                    break;
                                                }
                                            }
                                            {
                                                let mut ap = _expanded_paragraphs.clone();
                                                let path_clone = path.clone();
                                                Timeout::new(0, move || {
                                                    ap.set(path_clone);
                                                })
                                                .forget();
                                            }
                                            // 新增：讀者模式自動寫入 indexedDB（先比對再寫入）
                                            if _reader_mode_enabled {
                                                if let Some(first) = path.first() {
                                                    let chapter_id = &first.chapter_id;
                                                    let ids: Vec<String> =
                                                        path.iter().map(|p| p.id.clone()).collect();
                                                    let js_array = js_sys::Array::new();
                                                    for id in &ids {
                                                        js_array.push(&JsValue::from_str(id));
                                                    }
                                                    // 先讀取 indexedDB，只有不同才寫入
                                                    let ids_clone = ids.clone();
                                                    let chapter_id_clone = chapter_id.clone();
                                                    wasm_bindgen_futures::spawn_local(async move {
                                                        if let Ok(js_value) =
                                                            get_choice_from_indexeddb(
                                                                &chapter_id_clone,
                                                            )
                                                            .await
                                                        {
                                                            let arr =
                                                                js_sys::Array::from(&js_value);
                                                            let existing: Vec<String> = arr
                                                                .iter()
                                                                .filter_map(|v| v.as_string())
                                                                .collect();

                                                            if existing.is_empty() {
                                                                // Skip persisting for settings chapter
                                                                if chapter_id_clone
                                                                    != "settingschapter"
                                                                {
                                                                    crate::services::indexeddb::set_choices_to_indexeddb(&chapter_id_clone, &js_array).await.unwrap();
                                                                }
                                                                // 使用新的選擇
                                                                let mut story_context =
                                                                    story_context.clone();
                                                                let ids = ids_clone.clone();
                                                                story_context
                                                                    .write()
                                                                    .choice_ids
                                                                    .set(ids.clone());
                                                                // 展開新的選擇
                                                                let mut expanded = Vec::new();
                                                                for id in &ids {
                                                                    if let Some(p) = _paragraph_data
                                                                        .iter()
                                                                        .find(|p| p.id == *id)
                                                                    {
                                                                        expanded.push(p.clone());
                                                                    }
                                                                }
                                                                {
                                                                    let mut ap =
                                                                        _expanded_paragraphs
                                                                            .clone();
                                                                    let expanded_clone =
                                                                        expanded.clone();
                                                                    Timeout::new(0, move || {
                                                                        ap.set(expanded_clone);
                                                                    })
                                                                    .forget();
                                                                }
                                                            } else {
                                                                // 使用 IndexedDB 中的選擇
                                                                let mut story_context =
                                                                    story_context.clone();
                                                                story_context
                                                                    .write()
                                                                    .choice_ids
                                                                    .set(existing.clone());
                                                                // 展開 IndexedDB 中的選擇
                                                                let mut expanded = Vec::new();
                                                                for id in &existing {
                                                                    if let Some(p) = _paragraph_data
                                                                        .iter()
                                                                        .find(|p| p.id == *id)
                                                                    {
                                                                        expanded.push(p.clone());
                                                                    }
                                                                }
                                                                {
                                                                    let mut ap =
                                                                        _expanded_paragraphs
                                                                            .clone();
                                                                    let expanded_clone =
                                                                        expanded.clone();
                                                                    Timeout::new(0, move || {
                                                                        ap.set(expanded_clone);
                                                                    })
                                                                    .forget();
                                                                }
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
        let lang_prop = props.lang.clone();
        use_effect(move || {
            if state().current_language != lang_prop {
                state().set_language(&lang_prop);
            }
        });
    }

    // Initialize all chapters' choice_ids at the start
    {
        let mut choice_ids_initialized = use_signal(|| false);
        // Capture context signal once (hook call allowed here)
        let story_ctx_signal = use_story_context();
        use_effect(move || {
            if *choice_ids_initialized.peek() {
                return;
            }
            let chapters = story_ctx_signal.read().chapters.read().clone();
            if chapters.is_empty() {
                return;
            }
            choice_ids_initialized.set(true);
            let mut story_context = story_ctx_signal.clone();
            spawn_local(async move {
                let mut all_ids = Vec::new();
                let mut chapters_sorted = chapters.clone();
                chapters_sorted.sort_by_key(|c| c.order);
                for chapter in chapters_sorted.iter() {
                    let _chapter_id = chapter.id.clone();
                    if _chapter_id == "settingschapter" {
                        continue;
                    }
                    if let Ok(js_value) = get_choice_from_indexeddb(&_chapter_id).await {
                        let arr = js_sys::Array::from(&js_value);
                        let ids: Vec<String> = arr.iter().filter_map(|v| v.as_string()).collect();
                        all_ids.extend(ids);
                    }
                }
                let current_ids = story_context.read().choice_ids.read().clone();
                // Avoid accidentally clearing an existing history – only overwrite when we
                // have a **non-empty** result from IndexedDB that actually differs.
                if !all_ids.is_empty() && current_ids != all_ids {
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
        let mut auto_restored_sig = auto_restored.clone();
        use_effect(move || {
            // 若已執行還原，直接退出
            if *auto_restored_sig.peek() {
                return;
            }

            if *initialized.peek() {
                return;
            }

            // 讀取 reader_mode 設定（僅在 impact 執行時）
            let _reader_mode_enabled = settings_context
                .read()
                .settings
                .get("reader_mode")
                .map(|v| v == "true")
                .unwrap_or(false);

            if let Ok(paragraph_guard) = _paragraph_data.try_read() {
                // 若 paragraph_data 尚未載入，先返回
                if paragraph_guard.is_empty() {
                    return;
                }

                let _paragraph_data_vec = _paragraph_data.clone();
                let ctx = story_context.read();
                let choice_ids_vec = ctx.choice_ids.read().clone();

                // 若 choice_ids 尚未載入，先返回，等待下次 impact 觸發
                if choice_ids_vec.is_empty() {
                    return;
                }

                // choice_ids 已存在，開始重建段落路徑
                let mut initialized_clone = initialized.clone();
                let mut auto_restored_inner = auto_restored_sig.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let mut expanded: Vec<Paragraph> = Vec::new();
                    for paragraph_id in &choice_ids_vec {
                        if let Some(target) =
                            _paragraph_data_vec.iter().find(|p| p.id == *paragraph_id)
                        {
                            if !expanded.iter().any(|p: &Paragraph| p.id == *paragraph_id) {
                                expanded.push(target.clone());
                            }
                        }
                    }

                    if !expanded.is_empty() {
                        if expanded_paragraphs.peek().as_slice() != expanded.as_slice() {
                            let mut ap = expanded_paragraphs.clone();
                            let vec_clone = expanded.clone();
                            Timeout::new(0, move || {
                                ap.set(vec_clone);
                            })
                            .forget();
                        }
                        initialized_clone.set(true);
                        auto_restored_inner.set(true);
                    }
                });
            }
        });
    }

    // ===== New impact: Append timeout_to paragraph when countdown expires =====
    {
        let disabled_by_countdown = disabled_by_countdown.clone();
        let mut _expanded_paragraphs = _expanded_paragraphs.clone();
        let paragraph_data = paragraph_data.clone();
        let story_context = story_context.clone();
        use_effect(move || {
            let disabled_vec = disabled_by_countdown.read().clone();
            if disabled_vec.is_empty() {
                return;
            }

            let mut expanded_vec = _expanded_paragraphs.read().clone();
            let current_para_opt = expanded_vec.last().cloned();
            let current_para = match current_para_opt {
                Some(p) => p,
                None => return,
            };

            for (idx, is_disabled) in disabled_vec.iter().enumerate() {
                if *is_disabled {
                    if let Some(choice) = current_para.choices.get(idx) {
                        if let Some(timeout_raw) = choice.timeout_to.clone() {
                            // Allow comma-separated list of IDs for multi-select support.
                            let timeout_ids: Vec<String> = timeout_raw
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect();

                            // Find the first candidate that has not been loaded.
                            let candidate = timeout_ids
                                .into_iter()
                                .find(|id| !expanded_vec.iter().any(|p| &p.id == id));

                            if let Some(timeout_id) = candidate {
                                // First, try cached paragraphs
                                if let Some(target_para) = paragraph_data
                                    .read()
                                    .iter()
                                    .find(|p| p.id == timeout_id)
                                    .cloned()
                                {
                                    let mut expanded_vec = expanded_vec.clone();
                                    expanded_vec.push(target_para.clone());
                                    let mut ap = _expanded_paragraphs.clone();
                                    Timeout::new(0, move || {
                                        ap.set(expanded_vec);
                                    })
                                    .forget();
                                    continue;
                                }

                                // Otherwise, fetch from API asynchronously
                                let timeout_id_clone = timeout_id.clone();
                                let mut paragraph_data_sig = paragraph_data.clone();
                                let mut expanded_sig = _expanded_paragraphs.clone();
                                let mut story_ctx_clone = story_context.clone();

                                spawn_local(async move {
                                    let fetch_url = format!(
                                        "{}{}{}{}",
                                        BASE_API_URL, PARAGRAPHS, "/", timeout_id_clone
                                    );
                                    let client = reqwest::Client::new();
                                    if let Ok(resp) = client.get(&fetch_url).send().await {
                                        if resp.status().is_success() {
                                            if let Ok(txt) = resp.text().await {
                                                if let Ok(paragraph) =
                                                    serde_json::from_str::<Paragraph>(&txt)
                                                {
                                                    // Update global dataset
                                                    let mut data_vec =
                                                        paragraph_data_sig.read().clone();
                                                    if !data_vec
                                                        .iter()
                                                        .any(|p| p.id == paragraph.id)
                                                    {
                                                        data_vec.push(paragraph.clone());
                                                        {
                                                            let mut pd = paragraph_data_sig.clone();
                                                            let data_clone = data_vec.clone();
                                                            Timeout::new(0, move || {
                                                                pd.set(data_clone);
                                                            })
                                                            .forget();
                                                        }
                                                        story_ctx_clone
                                                            .write()
                                                            .paragraphs
                                                            .set(data_vec);
                                                    }

                                                    // Append to expanded path
                                                    let mut expanded_now =
                                                        expanded_sig.read().clone();
                                                    if !expanded_now
                                                        .iter()
                                                        .any(|p| p.id == paragraph.id)
                                                    {
                                                        expanded_now.push(paragraph);
                                                        let mut es = expanded_sig.clone();
                                                        Timeout::new(0, move || {
                                                            es.set(expanded_now);
                                                        })
                                                        .forget();
                                                    }
                                                }
                                            }
                                        }
                                    }
                                });
                            }
                        }
                    }
                }
            }

            // Persist any in-place changes only if different to avoid reactive loop
            if _expanded_paragraphs.read().as_slice() != expanded_vec.as_slice() {
                let mut ap = _expanded_paragraphs.clone();
                let vec_clone = expanded_vec.clone();
                Timeout::new(0, move || {
                    ap.set(vec_clone);
                })
                .forget();
            }
            ()
        });
    }

    let on_choice_click = {
        let mut _expanded_paragraphs = _expanded_paragraphs.clone();
        let mut story_context = story_context.clone();
        let _paragraph_data = paragraph_data.clone();
        let mut show_chapter_title = show_chapter_title.clone();
        let mut auto_restored_click = auto_restored.clone();
        move |(goto, choice_index): (String, usize)| {
            // 一旦使用者點擊，就禁止自動還原 impact 再次跑
            auto_restored_click.set(true);

            let expanded_vec = _expanded_paragraphs.read().clone();
            let last_paragraph = expanded_vec.last().cloned();

            if let Some(ref last) = last_paragraph {
                if let Some(choice) = last.choices.get(choice_index) {
                    if let Some(impacts) = &choice.impacts {
                        let relevant: Vec<Impact> = impacts
                            .iter()
                            .filter(|impact| {
                                matches!(
                                    impact,
                                    Impact::CharacterAttribute { .. } | Impact::Relationship { .. }
                                )
                            })
                            .cloned()
                            .collect();

                        if !relevant.is_empty() {
                            spawn_local(async move {
                                let base_state: CharacterStateSnapshot =
                                    get_latest_character_state_from_indexeddb()
                                        .await
                                        .ok()
                                        .and_then(|val| val.as_string())
                                        .and_then(|raw| serde_json::from_str(&raw).ok())
                                        .unwrap_or_default();

                                let updated = base_state.apply_impacts(&relevant);

                                if let Ok(serialized) = serde_json::to_string(&updated) {
                                    let _ =
                                        set_latest_character_state_to_indexeddb(&serialized).await;
                                }
                            });
                        }
                    }
                }
            }

            if let Some(ref last) = last_paragraph {
                if !last.chapter_id.is_empty() {
                    // 判斷是否多目標選項
                    let is_multi_target = last
                        .choices
                        .get(choice_index)
                        .map(|c| c.to.len() > 1)
                        .unwrap_or(false);
                    if is_multi_target {
                        // 多目標只寫入 random_choices，不寫入 choices
                        let paragraph_id = last.id.clone();
                        let original_choices = last.choices[choice_index].to.clone();
                        let selected = goto.clone();

                        let js_array = js_sys::Array::new();
                        for choice in &original_choices {
                            js_array.push(&JsValue::from_str(choice));
                        }

                        set_random_choice_to_indexeddb(
                            &paragraph_id,
                            choice_index as u32,
                            &js_array,
                            &selected,
                        );
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
                        // 更新 context 內的 choice_ids，避免其他 impact 將 expanded 還原
                        story_context.write().choice_ids.set(ids.clone());
                        let js_array = js_sys::Array::new();
                        for id in &ids {
                            js_array.push(&JsValue::from_str(id));
                        }
                        let chapter_id = last.chapter_id.clone();
                        if chapter_id != "settingschapter" {
                            spawn_local(async move {
                                let _ = crate::services::indexeddb::set_choices_to_indexeddb(
                                    &chapter_id,
                                    &js_array,
                                )
                                .await;
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
                            if let (Some(key), Some(value)) =
                                (choice.key.as_ref(), choice.value.as_ref())
                            {
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
                            let settings = wasm_bindgen_futures::JsFuture::from(
                                js_sys::Promise::new(&mut |resolve, _reject| {
                                    let cb = Closure::wrap(Box::new(
                                        move |js_value: wasm_bindgen::JsValue| {
                                            resolve
                                                .call1(&JsValue::NULL, &js_value)
                                                .unwrap_or_else(|e| {
                                                    tracing::error!(
                                                        "Failed to resolve JS callback: {:?}",
                                                        e
                                                    );
                                                    e
                                                });
                                        },
                                    )
                                        as Box<dyn FnMut(wasm_bindgen::JsValue)>);
                                    get_settings_from_indexeddb(cb.as_ref().unchecked_ref());
                                    cb.forget();
                                }),
                            );

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
                                    let value = js_sys::Reflect::get(&obj, &key)
                                        .unwrap_or(js_sys::JsString::from("").into());
                                    map.insert(
                                        key.as_string().unwrap_or_default(),
                                        value.as_string().unwrap_or_default(),
                                    );
                                }
                            }

                            let theme_mode = map
                                .get("theme_mode")
                                .cloned()
                                .unwrap_or_else(|| "auto".to_string());

                            {
                                let mut ctx = settings_context.write();
                                ctx.settings = map;
                                ctx.loaded = true;
                            }

                            apply_theme_class(ThemeMode::from_value(&theme_mode));
                        }

                        // Jump to first chapter
                        if let Some(target_paragraph) = paragraphs.iter().find(|p| p.id == goto) {
                            _expanded_paragraphs.set(vec![target_paragraph.clone()]);
                            story_context.write().target_paragraph_id = Some(goto.clone());
                            show_chapter_title.set(true);
                            // Reset choice history when navigating away from the settings chapter
                            story_context.write().choice_ids.set(Vec::new());
                        }
                    });
                    return;
                }

                if let Some(ref target_paragraph) = _paragraph_data.iter().find(|p| p.id == goto) {
                    if let Some(ref last) = last_paragraph {
                        if !last.chapter_id.is_empty() {
                            // ※ 已於上方 set_choices_to_indexeddb 儲存完整路徑，這裡不再覆寫為單一段落。
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
                        if expanded
                            .last()
                            .map(|p| p.id != target_paragraph.id)
                            .unwrap_or(true)
                        {
                            expanded.push((*target_paragraph).clone());
                        }
                        let mut ap = _expanded_paragraphs.clone();
                        let expanded_clone = expanded.clone();
                        Timeout::new(0, move || {
                            ap.set(expanded_clone);
                        })
                        .forget();
                        show_chapter_title.set(true);
                    } else {
                        // Auto scroll to top when switching new page
                        if let Some(window) = web_sys::window() {
                            window.scroll_to_with_x_and_y(0.0, 0.0);
                        }
                        {
                            let mut ap = _expanded_paragraphs.clone();
                            let target_clone = (*target_paragraph).clone();
                            Timeout::new(0, move || {
                                ap.set(vec![target_clone]);
                            })
                            .forget();
                        }
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
                                    if let Ok(paragraph) = serde_json::from_str::<Paragraph>(&text)
                                    {
                                        // Append to the global paragraph dataset.
                                        let mut pd_signal = paragraph_data_signal;
                                        let mut current_pd = pd_signal.read().clone();
                                        // Avoid duplicates.
                                        if !current_pd.iter().any(|p| p.id == paragraph.id) {
                                            current_pd.push(paragraph.clone());
                                            pd_signal.set(current_pd.clone());
                                            story_context
                                                .write()
                                                .paragraphs
                                                .set(current_pd.clone());
                                        }

                                        // Update expanded paragraph path – mimic normal navigation (new page).
                                        _expanded_paragraphs.set(vec![paragraph.clone()]);
                                        story_context.write().target_paragraph_id =
                                            Some(paragraph.id.clone());
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
        let _expanded_paragraphs = _expanded_paragraphs.clone();
        let _paragraph_data = paragraph_data.clone();
        let state = state.clone();
        let story_context = story_context.clone();
        let settings_context = settings_context.clone();
        let story_merged_context = story_merged_context.clone();
        use_effect(move || {
            let expanded_vec = _expanded_paragraphs.read();
            let expanded = expanded_vec.as_slice();
            if let Ok(_paragraph_data_read) = _paragraph_data.try_read() {
                let reader_mode = settings_context
                    .read()
                    .settings
                    .get("reader_mode")
                    .map(|v| v == "true")
                    .unwrap_or(false);
                let chapter_id = expanded
                    .last()
                    .map(|p| p.chapter_id.clone())
                    .unwrap_or_default();
                let _is_settings_chapter = chapter_id == "settingschapter";
                let choice_ids = story_context.read().choice_ids.read().clone();
                let merged_paragraph_str = merge_paragraphs_for_lang(
                    &expanded,
                    &state.read().current_language,
                    reader_mode,
                    _is_settings_chapter,
                    &choice_ids,
                );
                let mut merged_paragraph_signal =
                    story_merged_context.read().merged_paragraph.clone();
                merged_paragraph_signal.set(merged_paragraph_str.clone());
            }
            ()
        });
    }

    // Main impact: Set settings, update paragraph_id, initialize countdown
    use_effect(move || {
        let mut story_context = story_context.clone();
        let expanded = _expanded_paragraphs.read();
        let state = state.clone();
        let last_paragraph_id = last_paragraph_id.clone();
        let expanded = expanded.as_slice();
        if let Some(paragraph) = expanded.last() {
            let _is_settings_chapter = paragraph.chapter_id == "settingschapter";
            story_context
                .write()
                .is_settings_chapter
                .set(_is_settings_chapter);
            if *last_paragraph_id.borrow() != paragraph.id {
                *last_paragraph_id.borrow_mut() = paragraph.id.clone();
                // Initialize countdown only when paragraph ID changes
                if let Some(_text) = paragraph
                    .texts
                    .iter()
                    .find(|t| t.lang == state().current_language)
                {
                    let countdowns_vec = paragraph
                        .choices
                        .iter()
                        .map(|c| c.time_limit.unwrap_or(0))
                        .collect::<Vec<u32>>();
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
                    if let Some(container) = doc_cloned
                        .query_selector(".story-content-container")
                        .ok()
                        .flatten()
                    {
                        let event = web_sys::CustomEvent::new("show_filter").unwrap();
                        let _ = container.dispatch_event(&event);
                    }
                }
            }) as Box<dyn FnMut()>);
            document
                .add_event_listener_with_callback(
                    "visibilitychange",
                    closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            closure.forget();
            (|| {})()
        });
    }

    let reader_mode = settings_context
        .read()
        .settings
        .get("reader_mode")
        .map(|v| v == "true")
        .unwrap_or(false);
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
            let chapter_id = expanded
                .last()
                .map(|p| p.chapter_id.clone())
                .unwrap_or_default();
            if chapter_id.is_empty() {
                String::new()
            } else {
                chapters
                    .iter()
                    .find(|c| c.id == chapter_id)
                    .and_then(|chapter| {
                        chapter
                            .titles
                            .iter()
                            .find(|t| &t.lang == current_lang)
                            .or_else(|| {
                                chapter
                                    .titles
                                    .iter()
                                    .find(|t| t.lang == "en-US" || t.lang == "en-GB")
                            })
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
