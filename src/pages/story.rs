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
    perPage: i32,
    totalItems: i32,
    totalPages: i32,
}

#[derive(Deserialize, Clone, Debug)]
struct Paragraph {
    id: String,
    index: usize,
    #[serde(default)]
    chapter_id: String,
    texts: Vec<Text>,
    choices: Vec<ComplexChoice>,
    #[serde(default)]
    collectionId: String,
    #[serde(default)]
    collectionName: String,
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
                                    info!("收到的原始響應: {}", text);
                                    match serde_json::from_str::<Data>(&text) {
                                        Ok(data) => {
                                            if let Some(first_paragraph) = data.items.first() {
                                                story_context.write().target_paragraph_id = Some(first_paragraph.id.clone());
                                            }
                                            paragraph_data.set(data.items);
                                            has_loaded.set(true);
                                        },
                                        Err(e) => {
                                            error!("解析段落數據失敗：{}", e);
                                        }
                                    }
                                },
                                Err(e) => {
                                    error!("讀取響應文本失敗：{}", e);
                                }
                            }
                        } else {
                            error!("載入段落失敗，狀態碼：{}", response.status());
                        }
                    },
                    Err(e) => {
                        error!("載入段落請求失敗：{}", e);
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
        
        use_effect(move || {
            tracing::info!("=== use_effect 觸發 ===");
            let target_id = story_context.read().target_paragraph_id.clone();
            tracing::info!("story_context 中的 target_paragraph_id：{:?}", target_id);
            tracing::info!("當前段落數據數量：{}", paragraph_data.read().len());
            
            if let Some(target_id) = target_id {
                tracing::info!("嘗試查找段落 ID：{}", target_id);
                if let Some(paragraph) = paragraph_data.read().iter().find(|p| &p.id == &target_id) {
                    tracing::info!("找到目標段落：id = {}", paragraph.id);
                    current_paragraph.set(Some(paragraph.clone()));
                    
                    tracing::info!("當前語言：{}", state().current_language);
                    if let Some(text) = paragraph.texts.iter().find(|t| t.lang == state().current_language) {
                        tracing::info!("找到對應語言的文本：{}", text.paragraphs);
                        current_text.set(Some(text.clone()));
                        let choices: Vec<Choice> = text.choices.iter().enumerate().map(|(index, c)| {
                            let choice = if let Some(complex_choice) = paragraph.choices.get(index) {
                                StoryChoice::Complex(complex_choice.clone())
                            } else {
                                StoryChoice::Simple(c.clone())
                            };
                            let mut choice_obj: Choice = choice.into();
                            choice_obj.caption = c.clone();
                            choice_obj
                        }).collect();
                        tracing::info!("設置新的選項：{:?}", choices);
                        current_choices.set(choices);
                        enabled_choices.set(text.choices.iter().map(|c| c.clone()).collect());
                    } else {
                        tracing::error!("找不到對應語言的文本：lang = {}", state().current_language);
                    }
                } else {
                    tracing::error!("找不到目標段落：id = {}", target_id);
                }
            } else {
                tracing::info!("target_paragraph_id 為 None");
            }
            
            (move || {})()
        });
    }
    
    // 設置初始語言
    {
        let mut state = state.clone();
        use_effect(move || {
            tracing::info!("設置初始語言：{}", props.lang);
            state().set_language(&props.lang);
            (move || {})()
        });
    }
    
        rsx! {
            div {
            class: "max-w-3xl mx-auto p-8",
            if let Some(text) = current_text.read().as_ref() {
                {
                    debug!("渲染文本: {:?}", text);
                    debug!("當前選項: {:?}", current_choices.read());
                    debug!("啟用的選項: {:?}", enabled_choices.read());
                }
                StoryContent {
                    paragraph: text.paragraphs.clone(),
                    choices: current_choices.read().clone(),
                    enabled_choices: enabled_choices.read().clone(),
                    on_choice_click: move |goto: String| {
                        tracing::info!("=== on_choice_click 被調用 ===");
                        tracing::info!("goto = {}", goto);
                        let mut story_context = story_context.write();
                        tracing::info!("更新前的 target_paragraph_id = {:?}", story_context.target_paragraph_id);
                        story_context.target_paragraph_id = Some(goto.clone());
                        tracing::info!("更新後的 target_paragraph_id = {:?}", story_context.target_paragraph_id);
                        drop(story_context); // 確保寫鎖被釋放
                    }
                }
            } else {
                {
                    debug!("沒有找到當前文本");
                }
                div { "載入中..." }
            }
        }
    }
}