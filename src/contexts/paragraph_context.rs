use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use crate::{
    constants::config::{BASE_API_URL, PARAGRAPHS},
    models::effects::Effect,
};

// Reuse paragraph and text structures from translation_form
// pub use crate::components::translation_form::{Paragraph as ContextParagraph, Text as ContextText, ParagraphChoice as ContextParagraphChoice};
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ParagraphChoice {
    Complex {
        to: Vec<String>,
        #[serde(rename = "type")]
        type_: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        same_page: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        time_limit: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timeout_to: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        effects: Option<Vec<Effect>>,
    },
    ComplexOld {
        to: String,
        #[serde(rename = "type")]
        type_: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        same_page: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        time_limit: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timeout_to: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        effects: Option<Vec<Effect>>,
    },
    Simple(Vec<String>),
    SimpleOld(String),
}

#[allow(dead_code)]
impl ParagraphChoice {
    pub fn get_to(&self) -> Vec<String> {
        match self {
            ParagraphChoice::Complex { to, .. } => to.clone(),
            ParagraphChoice::ComplexOld { to, .. } => vec![to.clone()],
            ParagraphChoice::Simple(texts) => texts.clone(),
            ParagraphChoice::SimpleOld(text) => vec![text.clone()],
        }
    }

    pub fn get_type(&self) -> String {
        match self {
            ParagraphChoice::Complex { type_, .. } => type_.clone(),
            ParagraphChoice::ComplexOld { type_, .. } => type_.clone(),
            ParagraphChoice::Simple(_) => "goto".to_string(),
            ParagraphChoice::SimpleOld(_) => "goto".to_string(),
        }
    }

    pub fn get_key(&self) -> Option<String> {
        match self {
            ParagraphChoice::Complex { key, .. } => key.clone(),
            ParagraphChoice::ComplexOld { key, .. } => key.clone(),
            ParagraphChoice::Simple(_) => None,
            ParagraphChoice::SimpleOld(_) => None,
        }
    }

    pub fn get_value(&self) -> Option<serde_json::Value> {
        match self {
            ParagraphChoice::Complex { value, .. } => value.clone(),
            ParagraphChoice::ComplexOld { value, .. } => value.clone(),
            ParagraphChoice::Simple(_) => None,
            ParagraphChoice::SimpleOld(_) => None,
        }
    }

    pub fn get_same_page(&self) -> Option<bool> {
        match self {
            ParagraphChoice::Complex { same_page, .. } => *same_page,
            ParagraphChoice::ComplexOld { same_page, .. } => *same_page,
            ParagraphChoice::Simple(_) => None,
            ParagraphChoice::SimpleOld(_) => None,
        }
    }

    pub fn get_time_limit(&self) -> Option<u32> {
        match self {
            ParagraphChoice::Complex { time_limit, .. } => *time_limit,
            ParagraphChoice::ComplexOld { time_limit, .. } => *time_limit,
            ParagraphChoice::Simple(_) => None,
            ParagraphChoice::SimpleOld(_) => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Paragraph {
    pub id: String,
    #[serde(default)]
    pub chapter_id: String,
    pub texts: Vec<Text>,
    pub choices: Vec<ParagraphChoice>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Text {
    pub lang: String,
    pub paragraphs: String,
    pub choices: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParagraphData {
    pub items: Vec<Paragraph>,
}

#[derive(Clone)]
pub struct ParagraphState {
    pub paragraphs: Vec<Paragraph>,
    pub loaded: bool,
}

impl ParagraphState {
    pub fn new() -> Self {
        Self {
            paragraphs: Vec::new(),
            loaded: false,
        }
    }

    pub fn set_paragraphs(&mut self, paragraphs: Vec<Paragraph>) {
        self.paragraphs = paragraphs;
        self.loaded = true;
    }
    
    pub fn get_by_chapter(&self, chapter_id: &str) -> Vec<Paragraph> {
        self.paragraphs.iter()
            .filter(|p| p.chapter_id == chapter_id)
            .cloned()
            .collect()
    }
    
    pub fn get_by_id(&self, id: &str) -> Option<Paragraph> {
        self.paragraphs.iter()
            .find(|p| p.id == id)
            .cloned()
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ParagraphProviderProps {
    children: Element,
}

#[component]
pub fn ParagraphProvider(props: ParagraphProviderProps) -> Element {
    let state = use_signal(|| ParagraphState::new());
    
    // Load paragraph list
    use_effect(move || {
        let mut state = state.clone();
        spawn_local(async move {
            if !state.read().loaded {
                let paragraphs_url = format!("{}{}", BASE_API_URL, PARAGRAPHS);
                let client = reqwest::Client::new();
                
                match client.get(&paragraphs_url)
                    .send()
                    .await 
                {
                    Ok(response) => {
                        if response.status().is_success() {
                            match response.json::<ParagraphData>().await {
                                Ok(data) => {
                                    state.write().set_paragraphs(data.items);
                                }
                                Err(_) => {
                                    // Ignore errors
                                }
                            }
                        }
                    }
                    Err(_) => {
                        // Ignore errors
                    }
                }
            }
        });
        
        (move || {})()
    });
    
    provide_context(state);
    
    rsx! {
        {props.children}
    }
} 