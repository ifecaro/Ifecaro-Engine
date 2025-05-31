use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use crate::constants::config::{BASE_API_URL, CHAPTERS};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Chapter {
    pub id: String,
    pub titles: Vec<ChapterTitle>,
    pub order: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChapterTitle {
    pub lang: String,
    pub title: String,
}

#[derive(Clone)]
pub struct ChapterState {
    pub chapters: Vec<Chapter>,
    pub loaded: bool,
}

impl ChapterState {
    pub fn new() -> Self {
        Self {
            chapters: Vec::new(),
            loaded: false,
        }
    }

    pub fn set_chapters(&mut self, chapters: Vec<Chapter>) {
        self.chapters = chapters;
        self.loaded = true;
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ChapterProviderProps {
    children: Element,
}

#[component]
pub fn ChapterProvider(props: ChapterProviderProps) -> Element {
    let state = use_signal(|| ChapterState::new());
    
    // 載入章節列表
    use_effect(move || {
        let mut state = state.clone();
        spawn_local(async move {
            if !state.read().loaded {
                let chapters_url = format!("{}{}", BASE_API_URL, CHAPTERS);
                let client = reqwest::Client::new();
                
                match client.get(&chapters_url)
                    .send()
                    .await 
                {
                    Ok(response) => {
                        if response.status().is_success() {
                            match response.json::<serde_json::Value>().await {
                                Ok(chapters_data) => {
                                    if let Some(items) = chapters_data.get("items").and_then(|i| i.as_array()) {
                                        let chapters: Vec<Chapter> = items.iter()
                                            .filter_map(|item| {
                                                let id = item.get("id")?.as_str()?.to_string();
                                                let titles = item.get("titles")?.as_array()?
                                                    .iter()
                                                    .filter_map(|title_obj| {
                                                        let lang = title_obj.get("lang")?.as_str()?.to_string();
                                                        let title = title_obj.get("title")?.as_str()?.to_string();
                                                        Some(ChapterTitle { lang, title })
                                                    })
                                                    .collect();
                                                let order = item.get("order")?.as_i64().unwrap_or(0) as i32;
                                                Some(Chapter { id, titles, order })
                                            })
                                            .collect();
                                        
                                        // 按 order 排序
                                        let mut sorted_chapters = chapters;
                                        sorted_chapters.sort_by(|a, b| a.order.cmp(&b.order));
                                        
                                        state.write().set_chapters(sorted_chapters);
                                    }
                                }
                                Err(_) => {
                                    // 忽略錯誤
                                }
                            }
                        }
                    }
                    Err(_) => {
                        // 忽略錯誤
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