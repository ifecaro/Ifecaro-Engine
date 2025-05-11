use dioxus::prelude::*;
use dioxus_core::IntoDynNode;
use dioxus_core::fc_to_builder;
use wasm_bindgen_futures::spawn_local;
use serde::Deserialize;
use dioxus_i18n::t;
use crate::components::story_content::{StoryContent, Choice, Action};
use crate::contexts::story_context::use_story_context;
use crate::contexts::language_context::LanguageState;
use crate::constants::config::{BASE_API_URL, PARAGRAPHS};

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

#[component]
pub fn Story(props: StoryProps) -> Element {
    let state = use_context::<Signal<LanguageState>>();
    let mut story_context = use_story_context();
    let current_paragraph = use_signal(|| None::<Paragraph>);
    let current_text = use_signal(|| None::<Text>);
    let current_choices = use_signal(|| Vec::<Choice>::new());
    let enabled_choices = use_signal(|| Vec::<String>::new());
    let has_loaded = use_signal(|| false);
    let paragraph_data = use_signal(|| Vec::<Paragraph>::new());
    let expanded_paragraphs = use_signal(|| Vec::<Paragraph>::new());
    
    // 載入段落數據
    {
        let mut has_loaded = has_loaded.clone();
        let mut paragraph_data = paragraph_data.clone();
        let mut story_context = story_context.clone();
        let mut expanded_paragraphs = expanded_paragraphs.clone();
        use_effect(move || {
            let paragraphs_url = format!("{}{}", BASE_API_URL, PARAGRAPHS);
            let client = reqwest::Client::new();
            
            spawn_local(async move {
                match client.get(&paragraphs_url)
                    .send()
                    .await {
                    Ok(response) => {
                        if response.status().is_success() {
                            match response.text().await {
                                Ok(text) => {
                                    match serde_json::from_str::<Data>(&text) {
                                        Ok(data) => {
                                            if let Some(first_paragraph) = data.items.first() {
                                                story_context.write().target_paragraph_id = Some(first_paragraph.id.clone());
                                                expanded_paragraphs.set(vec![first_paragraph.clone()]);
                                            }
                                            paragraph_data.set(data.items);
                                            has_loaded.set(true);
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
    
    // 當目標段落ID改變時，更新當前段落
    {
        let mut current_paragraph = current_paragraph.clone();
        let mut current_text = current_text.clone();
        let mut current_choices = current_choices.clone();
        let mut enabled_choices = enabled_choices.clone();
        let paragraph_data = paragraph_data.clone();
        let state = state.clone();
        let story_context = story_context.clone();
        let expanded_paragraphs = expanded_paragraphs.clone();
        
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
    
    let on_choice_click = {
        let paragraph_data = paragraph_data.clone();
        let mut expanded_paragraphs = expanded_paragraphs.clone();
        let mut story_context = story_context.clone();
        move |goto: String| {
            let paragraphs = paragraph_data.read();
            let last_paragraph = {
                let expanded = expanded_paragraphs.read();
                expanded.last().cloned()
            };
            if let Some(target_paragraph) = paragraphs.iter().find(|p| p.id == goto) {
                let mut same_page = false;
                if let Some(last) = last_paragraph {
                    if let Some(idx) = last.choices.iter().position(|choice| choice.to == goto) {
                        if let Some(choice) = last.choices.get(idx) {
                            same_page = choice.same_page.unwrap_or(false);
                        }
                    }
                }
                if same_page {
                    let mut expanded = expanded_paragraphs.read().clone();
                    expanded.push(target_paragraph.clone());
                    let _ = expanded_paragraphs;
                    expanded_paragraphs = expanded_paragraphs.clone();
                    expanded_paragraphs.set(expanded);
                } else {
                    let _ = expanded_paragraphs;
                    expanded_paragraphs = expanded_paragraphs.clone();
                    expanded_paragraphs.set(vec![target_paragraph.clone()]);
                }
                story_context.write().target_paragraph_id = Some(goto);
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
            if !*has_loaded.read() {
                div { class: "text-white", "{t!(\"loading\")}" }
            }
        }
    }
}