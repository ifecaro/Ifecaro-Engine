use dioxus::prelude::*;
use dioxus_core::IntoDynNode;
use dioxus_core::fc_to_builder;
use wasm_bindgen_futures::spawn_local;
use tracing::{error, info, debug};
use serde::Deserialize;
use crate::components::story_content::{StoryContent, Choice, Action};
use crate::contexts::story_context::use_story_context;
use crate::contexts::language_context::LanguageState;
use crate::constants::config::{BASE_API_URL, PARAGRAPHS};

#[derive(Deserialize, Clone, Debug)]
struct Data {
    items: Vec<Paragraph>,
    page: i32,
    per_page: i32,
    total_items: i32,
    total_pages: i32,
}

#[derive(Deserialize, Clone, Debug)]
struct Paragraph {
    id: String,
    index: usize,
    #[serde(default)]
    chapter_id: String,
    text: Vec<Text>,
    choices: Vec<ComplexChoice>,
    collection_id: String,
    #[serde(default)]
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
    pub to: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub value: Option<serde_json::Value>,
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
    
    // 載入段落數據
    {
        let mut has_loaded = has_loaded.clone();
        let mut paragraph_data = paragraph_data.clone();
        let mut story_context = story_context.clone();
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
                                            }
                                            paragraph_data.set(data.items);
                                            has_loaded.set(true);
                                        },
                                        Err(_) => {}
                                    }
                                },
                                Err(_) => {}
                            }
                        }
                    },
                    Err(_) => {}
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
        
        use_effect(move || {
            let target_id = story_context.read().target_paragraph_id.clone();
            
            if let Some(target_id) = target_id {
                if let Some(paragraph) = paragraph_data.read().iter().find(|p| &p.id == &target_id) {
                    current_paragraph.set(Some(paragraph.clone()));
                    
                    if let Some(text) = paragraph.text.iter().find(|t| t.lang == state().current_language) {
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
                        enabled_choices.set(text.choices.clone());
                    }
                }
            }
            
            (move || {})()
        });
    }
    
    // 設置初始語言
    {
        let mut state = state.clone();
        use_effect(move || {
            state().set_language(&props.lang);
            (move || {})()
        });
    }
    
    rsx! {
        div {
            class: "max-w-3xl mx-auto p-8",
            if let Some(text) = current_text.read().as_ref() {
                StoryContent {
                    paragraph: text.paragraphs.clone(),
                    choices: current_choices.read().clone(),
                    enabled_choices: enabled_choices.read().clone(),
                    on_choice_click: move |goto: String| {
                        let mut story_context = story_context.write();
                        story_context.target_paragraph_id = Some(goto.clone());
                        drop(story_context);
                    }
                }
            } else {
                div { "載入中..." }
            }
        }
    }
}