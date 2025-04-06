use crate::constants::{BASE_API_URL, PARAGRAPHS, ACTIONS};
use crate::enums::translations::Translations;
use dioxus::{
    dioxus_core,
    hooks::{use_context, use_future, use_memo, use_signal},
    prelude::{component, dioxus_elements, fc_to_builder, rsx, Element, IntoDynNode, GlobalSignal, Readable, Props},
    signals::{Signal, Writable},
};
// use dioxus_markdown::Markdown;
use serde::Deserialize;
use crate::contexts::language_context::LanguageState;
use crate::components::story_content::{StoryContent, Choice};
use wasm_bindgen::prelude::*;
use web_sys::{
    window,
    IdbDatabase,
    IdbTransactionMode,
};
use wasm_bindgen_futures::JsFuture;

#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Debug)]
struct Data {
    // page: i32,
    // perPage: i32,
    totalItems: i32,
    // totalPages: i32,
    items: Vec<Paragraph>,
}

#[derive(Deserialize, Clone, Debug)]
struct Paragraph {
    id: String,
    index: usize,
    #[serde(default)]
    chapter_id: String,
    texts: Vec<Text>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
struct Action {
    #[serde(rename = "type")]
    type_: String,
    key: Option<String>,
    value: Option<serde_json::Value>,
    to: String,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
struct Text {
    lang: String,
    paragraphs: String,
    choices: Vec<Choice>,
    #[serde(default)]
    actions: Vec<Action>,
}

#[derive(Props, PartialEq, Clone)]
pub struct StoryProps {
    pub lang: String,
}

async fn sync_action_to_db(action: &Action, paragraph_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!("{}/api{}", BASE_API_URL, ACTIONS);
    
    let action_data = serde_json::json!({
        "paragraph_id": paragraph_id,
        "type": action.type_,
        "key": action.key,
        "value": action.value,
        "to": action.to
    });

    client.post(&url)
        .json(&action_data)
        .send()
        .await?;

    Ok(())
}

async fn save_setting(key: &str, value: &str, text: &mut Text, paragraph_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let action = Action {
        type_: "setting".to_string(),
        key: Some(key.to_string()),
        value: Some(serde_json::Value::String(value.to_string())),
        to: "".to_string(),
    };
    
    // 儲存到本地
    text.actions.push(action.clone());
    
    // 儲存到 localStorage
    if let Some(window) = web_sys::window() {
        if let Some(local_storage) = window.local_storage().ok().flatten() {
            let _ = local_storage.set_item(key, value);
        }
    }
    
    // 同步到資料庫
    sync_action_to_db(&action, paragraph_id).await?;
    
    Ok(())
}

async fn record_choice(choice: &Choice, text: &mut Text, paragraph_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let action = Action {
        type_: "choice".to_string(),
        key: Some(choice.action.to.clone()),
        value: Some(serde_json::Value::String(choice.caption.clone())),
        to: choice.action.to.clone(),
    };
    
    // 儲存到本地
    text.actions.push(action.clone());
    
    // 同步到資料庫
    sync_action_to_db(&action, paragraph_id).await?;
    
    Ok(())
}

// 修改 save_choice_to_indexeddb 函數
async fn save_choice_to_indexeddb(paragraph_id: &str, choice: &Choice) -> Result<(), JsValue> {
    let window = window().unwrap();
    let indexed_db = window.indexed_db()?.expect("Failed to get IndexedDB");
    
    // 修正 open 方法的使用
    let db_request = indexed_db.open("story_choices")?;
    
    // 設定資料庫升級事件
    let db_request_clone = db_request.clone();
    let onupgradeneeded = Closure::wrap(Box::new(move |_event: web_sys::Event| {
        let db = db_request_clone.result().unwrap();
        let db = db.dyn_into::<IdbDatabase>().unwrap();
        
        // 檢查 object store 是否存在
        let store_names = js_sys::Reflect::get(&db, &JsValue::from_str("objectStoreNames")).unwrap();
        let store_names = store_names.dyn_into::<js_sys::Array>().unwrap();
        
        if !store_names.includes(&JsValue::from_str("choices"), 0) {
            // 創建 object store，移除第二個參數
            let store = db.create_object_store("choices")?;
            // 使用正確的方法名稱
            store.create_index_with_str("paragraph_id", "paragraph_id")?;
        }
        Ok(())
    }) as Box<dyn FnMut(_) -> Result<(), JsValue>>);
    
    db_request.set_onupgradeneeded(Some(onupgradeneeded.as_ref().unchecked_ref()));
    onupgradeneeded.forget();
    
    // 等待資料庫開啟，添加明確的類型註解
    let db = JsFuture::from(db_request.dyn_into::<js_sys::Promise>()?).await?;
    let db = db.dyn_into::<IdbDatabase>()?;
    
    let transaction = db.transaction_with_str_and_mode("choices", IdbTransactionMode::Readwrite)?;
    let store = transaction.object_store("choices")?;
    
    let choice_data = serde_json::json!({
        "paragraph_id": paragraph_id,
        "type": choice.action.type_,
        "key": choice.action.key,
        "value": choice.action.value,
        "to": choice.action.to
    });
    
    // 將 serde_json::Value 轉換為 JsValue
    let choice_data_js = serde_wasm_bindgen::to_value(&choice_data)?;
    
    // 使用段落 ID 作為主鍵，確保每個段落只能有一個選擇
    store.put_with_key(&choice_data_js, &JsValue::from_str(paragraph_id))?;
    
    // 如果是設定類型的動作，也儲存到 localStorage
    if choice.action.type_ == "setting" {
        if let Some(storage) = window.local_storage().ok().flatten() {
            if let Some(key) = &choice.action.key {
                if let Some(value) = &choice.action.value {
                    let _ = storage.set_item(key, &value.to_string());
                }
            }
        }
    }
    
    Ok(())
}

#[component]
pub fn Story(props: StoryProps) -> Element {
    let data = use_signal(|| Data {
        totalItems: 0,
        items: vec![],
    });
    let mut selected_paragraph_index: Signal<usize> = use_signal(|| 0);
    let state = use_context::<Signal<LanguageState>>();
    let t = Translations::get(&state.read().current_language);

    let text_found = use_memo(move || {
        let current_data = data.read();
        let current_index = *selected_paragraph_index.read();
        let current_language = &state.read().current_language;
        
        let found_item = current_data.items.iter().find(|item| item.index == current_index);
        let found_text = found_item.and_then(|item| {
            item.texts.iter().find(|t| t.lang == *current_language).cloned()
        });
        
        found_text
    });

    let paragraph = use_memo(move || {
        let binding = text_found.read();
        let text = binding.as_ref();
        
        if text.is_none() {
            return None;
        }
        
        let paragraph_text = text.unwrap().paragraphs.clone();
        Some(paragraph_text)
    });

    let enabled_choices = use_memo(move || {
        let mut enabled = Vec::new();
        let current_data = data.read();
        let current_index = *selected_paragraph_index.read();
        
        if current_data.items.is_empty() {
            return enabled;
        }
        
        let found_item = current_data.items.iter().find(|item| item.index == current_index);
        if found_item.is_none() {
            return enabled;
        }
        
        let found_text = found_item.unwrap().texts.iter()
            .find(|t| t.lang == state.read().current_language);
            
        if found_text.is_none() {
            return enabled;
        }
        
        let choices = &found_text.unwrap().choices;
        
        for choice in choices {
            let target_id = &choice.action.to;
            let target_exists = current_data.items.iter().any(|item| item.id == *target_id);
            
            if target_exists {
                enabled.push(target_id.clone());
            }
        }
        
        enabled
    });

    {
        let mut data = data.clone();

        use_future(move || async move {
            let url = format!("{}/api{}", BASE_API_URL, PARAGRAPHS);
            
            match reqwest::get(&url).await {
                Ok(response) => {
                    match response.json::<Data>().await {
                        Ok(data2) => {
                            data.set(data2.clone());
                            Ok(data2)
                        },
                        Err(e) => Err(e)
                    }
                },
                Err(e) => Err(e)
            }
        });
    }

    let text_found_clone = text_found.clone();
    let paragraph_clone = paragraph.clone();

    let on_choice_click = move |goto: String| {
        if let Some(item) = data.read().items.iter().find(|item| item.id == goto) {
            let paragraph_id = item.id.clone();
            let choice = text_found.read().as_ref().and_then(|text| {
                text.choices.iter().find(|c| c.action.to == goto).cloned()
            });
            
            if let Some(choice) = choice {
                let paragraph_id_clone = paragraph_id.clone();
                let choice_clone = choice.clone();
                
                wasm_bindgen_futures::spawn_local(async move {
                    let _ = save_choice_to_indexeddb(&paragraph_id_clone, &choice_clone).await;
                });
            }
            
            selected_paragraph_index.set(item.index);
        }
    };

    let render_coming_soon = move || {
        rsx! {
            article {
                class: "prose dark:prose-invert lg:prose-xl indent-10 mx-auto",
                div {
                    class: "whitespace-pre-line",
                    p { class: "mb-6", { t.coming_soon } }
                }
                ol {
                    class: "mt-10 w-fit",
                    li { class: "opacity-30", { t.coming_soon } }
                }
            }
        }
    };

    let render_story_content = move || {
        if let Some(paragraph) = paragraph_clone.read().as_ref() {
            if let Some(text) = text_found_clone.read().as_ref() {
                rsx! {
                    StoryContent {
                        paragraph: paragraph.clone(),
                        choices: text.choices.clone(),
                        on_choice_click,
                        t: t.clone(),
                        enabled_choices: enabled_choices.read().clone()
                    }
                }
            } else {
                render_coming_soon()
            }
        } else {
            render_coming_soon()
        }
    };

    rsx! {
        div {
            class: "container mx-auto px-4 pt-16",
            div {
                class: "max-w-2xl mx-auto",
                {render_story_content()}
            }
        }
    }
}