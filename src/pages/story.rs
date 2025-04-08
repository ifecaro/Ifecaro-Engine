use crate::constants::{BASE_API_URL, PARAGRAPHS, ACTIONS};
use crate::enums::translations::Translations;
use dioxus::{
    dioxus_core,
    hooks::{use_context, use_future, use_memo, use_signal, use_effect},
    prelude::{dioxus_elements, fc_to_builder, rsx, Element, IntoDynNode, GlobalSignal, Readable, Props},
    signals::{Signal, Writable},
};
// use dioxus_markdown::Markdown;
use serde::Deserialize;
use crate::contexts::language_context::LanguageState;
use crate::components::story_content::{StoryContent, Choice};
use wasm_bindgen::prelude::*;
use web_sys::IdbDatabase;
use wasm_bindgen_futures::JsFuture;
use js_sys::Promise;
use wasm_bindgen::closure::Closure;
use crate::contexts::story_context::use_story_context;

#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Debug)]
struct Data {
    #[allow(dead_code)]
    totalItems: i32,
    items: Vec<Paragraph>,
}

#[derive(Deserialize, Clone, Debug)]
struct Paragraph {
    id: String,
    index: usize,
    #[allow(dead_code)]
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

#[allow(dead_code)]
async fn sync_action_to_db(action: &Action, paragraph_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/api{}", BASE_API_URL, ACTIONS);
    
    let action_data = serde_json::json!({
        "paragraph_id": paragraph_id,
        "type": action.type_,
        "key": action.key,
        "value": action.value,
        "to": action.to
    });

    reqwest::Client::new()
        .post(&url)
        .json(&action_data)
        .send()
        .await?;

    Ok(())
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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
async fn save_choice_to_indexeddb(paragraph_id: &str, _chapter_id: &str, choice: &Choice) -> Result<(), JsValue> {
    // 如果 action type 為空字串，不記錄選擇
    if choice.action.type_.is_empty() {
        return Ok(());
    }
    
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window found"))?;
    let indexed_db = window.indexed_db()
        .map_err(|e| JsValue::from_str(&format!("Failed to get indexed_db: {:?}", e)))?
        .ok_or_else(|| JsValue::from_str("No indexed_db found"))?;
    
    let db_request = indexed_db.open_with_u32("story_choices", 1)
        .map_err(|e| JsValue::from_str(&format!("Failed to open database: {:?}", e)))?;
    
    // 等待數據庫打開或升級完成
    let db_promise = js_sys::Promise::new(&mut |resolve, reject| {
        // 處理升級事件
        let db_request_upgrade = db_request.clone();
        let reject_upgrade = reject.clone();
        let onupgradeneeded_callback = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            match db_request_upgrade.result() {
                Ok(db_any) => {
                    match db_any.dyn_into::<IdbDatabase>() {
                        Ok(db) => {
                            let _ = db.create_object_store("choices");
                        },
                        Err(e) => {
                            reject_upgrade.call1(&JsValue::NULL, &e).unwrap();
                        }
                    }
                },
                Err(e) => {
                    reject_upgrade.call1(&JsValue::NULL, &e).unwrap();
                }
            }
        }) as Box<dyn FnMut(web_sys::Event)>);
        
        // 處理成功事件
        let db_request_success = db_request.clone();
        let resolve_success = resolve.clone();
        let reject_success = reject.clone();
        let onsuccess_callback = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            match db_request_success.result() {
                Ok(db_any) => {
                    match db_any.dyn_into::<IdbDatabase>() {
                        Ok(db) => {
                            resolve_success.call1(&JsValue::NULL, &db).unwrap();
                        },
                        Err(e) => {
                            reject_success.call1(&JsValue::NULL, &e).unwrap();
                        }
                    }
                },
                Err(e) => {
                    reject_success.call1(&JsValue::NULL, &e).unwrap();
                }
            }
        }) as Box<dyn FnMut(web_sys::Event)>);
        
        // 處理錯誤事件
        let reject_error = reject.clone();
        let onerror_callback = Closure::wrap(Box::new(move |event: web_sys::Event| {
            reject_error.call1(&JsValue::NULL, &event).unwrap();
        }) as Box<dyn FnMut(web_sys::Event)>);
        
        db_request.set_onupgradeneeded(Some(onupgradeneeded_callback.as_ref().unchecked_ref()));
        db_request.set_onsuccess(Some(onsuccess_callback.as_ref().unchecked_ref()));
        db_request.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        
        onupgradeneeded_callback.forget();
        onsuccess_callback.forget();
        onerror_callback.forget();
    });
    
    let db: IdbDatabase = JsFuture::from(db_promise)
        .await
        .map_err(|e| JsValue::from_str(&format!("等待數據庫打開失敗: {:?}", e)))?
        .dyn_into()
        .map_err(|e| JsValue::from_str(&format!("轉換為 IdbDatabase 失敗: {:?}", e)))?;
    
    let transaction = db.transaction_with_str_sequence_and_mode(
        &js_sys::Array::of1(&JsValue::from_str("choices")),
        web_sys::IdbTransactionMode::Readwrite,
    ).map_err(|e| JsValue::from_str(&format!("創建事務失敗: {:?}", e)))?;
    
    let store = transaction.object_store("choices")
        .map_err(|e| JsValue::from_str(&format!("獲取存儲對象失敗: {:?}", e)))?;
    
    // 根據 action type 處理不同的存儲邏輯
    match choice.action.type_.as_str() {
        "setting" => {
            if let Some(key) = &choice.action.key {
                if let Some(value) = &choice.action.value {
                    let data = serde_json::json!({
                        "type": "setting",
                        "key": key,
                        "value": value,
                        "paragraph_id": paragraph_id,
                        "timestamp": js_sys::Date::new_0().get_time()
                    });
                    let data_str = serde_json::to_string(&data)
                        .map_err(|e| JsValue::from_str(&format!("序列化數據失敗: {:?}", e)))?;
                    let request = store.put_with_key(&JsValue::from_str(&data_str), &JsValue::from_str(&format!("setting_{}", key)))
                        .map_err(|e| JsValue::from_str(&format!("保存數據失敗: {:?}", e)))?;
                    
                    // 等待請求完成
                    JsFuture::from(Promise::resolve(&request))
                        .await
                        .map_err(|e| JsValue::from_str(&format!("等待請求完成失敗: {:?}", e)))?;
                }
            }
        },
        "choice" => {
            let choices_key = "user_choices";
            let existing_request = store.get(&JsValue::from_str(choices_key))
                .map_err(|e| JsValue::from_str(&format!("獲取現有選擇失敗: {:?}", e)))?;
            
            let existing_choices = JsFuture::from(Promise::resolve(&existing_request))
                .await
                .map_err(|e| JsValue::from_str(&format!("等待獲取現有選擇失敗: {:?}", e)))?;
            
            let mut choices: Vec<String> = if !existing_choices.is_undefined() && !existing_choices.is_null() {
                let existing_str = existing_choices.as_string()
                    .ok_or_else(|| JsValue::from_str("轉換現有選擇為字符串失敗"))?;
                serde_json::from_str(&existing_str)
                    .map_err(|e| JsValue::from_str(&format!("解析現有選擇失敗: {:?}", e)))?
            } else {
                Vec::new()
            };
            
            choices.push(choice.action.to.clone());
            
            let data = serde_json::to_string(&choices)
                .map_err(|e| JsValue::from_str(&format!("序列化選擇失敗: {:?}", e)))?;
            let request = store.put_with_key(&JsValue::from_str(&data), &JsValue::from_str(choices_key))
                .map_err(|e| JsValue::from_str(&format!("保存選擇失敗: {:?}", e)))?;
            
            // 等待請求完成
            JsFuture::from(Promise::resolve(&request))
                .await
                .map_err(|e| JsValue::from_str(&format!("等待請求完成失敗: {:?}", e)))?;
        }
        _ => {}
    }
    
    // 等待事務完成
    let transaction_complete = js_sys::Promise::new(&mut |resolve, _reject| {
        let transaction_clone = transaction.clone();
        let resolve_clone = resolve.clone();
        let callback = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            resolve_clone.call1(&JsValue::NULL, &JsValue::NULL).unwrap();
        }) as Box<dyn FnMut(web_sys::Event)>);
        transaction_clone.set_oncomplete(Some(callback.as_ref().unchecked_ref()));
        callback.forget();
    });
    
    JsFuture::from(transaction_complete)
        .await
        .map_err(|e| JsValue::from_str(&format!("等待事務完成失敗: {:?}", e)))?;
    
    Ok(())
}

#[allow(dead_code)]
async fn read_indexeddb_data() -> Result<Vec<String>, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window found"))?;
    let indexed_db = window.indexed_db()
        .map_err(|e| JsValue::from_str(&format!("Failed to get indexed_db: {:?}", e)))?
        .ok_or_else(|| JsValue::from_str("No indexed_db found"))?;
    
    let db_request = indexed_db.open_with_u32("story_choices", 1)
        .map_err(|e| JsValue::from_str(&format!("Failed to open database: {:?}", e)))?;
    
    let db: IdbDatabase = JsFuture::from(Promise::resolve(&db_request))
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to open database: {:?}", e)))?
        .dyn_into()
        .map_err(|e| JsValue::from_str(&format!("Failed to convert to IdbDatabase: {:?}", e)))?;
    
    let transaction = db.transaction_with_str_sequence_and_mode(
        &js_sys::Array::of1(&JsValue::from_str("choices")),
        web_sys::IdbTransactionMode::Readonly,
    ).map_err(|e| JsValue::from_str(&format!("Failed to create transaction: {:?}", e)))?;
    
    let store = transaction.object_store("choices")
        .map_err(|e| JsValue::from_str(&format!("Failed to get store: {:?}", e)))?;
    
    let choices_request = store.get(&JsValue::from_str("user_choices"))
        .map_err(|e| JsValue::from_str(&format!("Failed to get user_choices: {:?}", e)))?;
    
    let choices_result = JsFuture::from(Promise::resolve(&choices_request))
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to get choices result: {:?}", e)))?;
    
    let choices: Vec<String> = if !choices_result.is_undefined() && !choices_result.is_null() {
        let choices_str = choices_result.as_string()
            .ok_or_else(|| JsValue::from_str("Failed to convert choices to string"))?;
        serde_json::from_str(&choices_str)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse choices: {:?}", e)))?
    } else {
        Vec::new()
    };
    
    Ok(choices)
}

#[allow(non_snake_case)]
pub fn Story(_props: StoryProps) -> Element {
    let data = use_signal(|| Data {
        totalItems: 0,
        items: vec![],
    });
    let mut selected_paragraph_index: Signal<usize> = use_signal(|| 0);
    let state = use_context::<Signal<LanguageState>>();
    let mut t = use_signal(|| Translations::get(&state.read().current_language));

    // 監聽語言變化
    {
        let state = state.clone();
        let mut t = t.clone();
        use_effect(move || {
            t.set(Translations::get(&state.read().current_language));
            (|| ())()
        });
    }

    let mut story_context = use_story_context();

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

    // 更新 context 中的選項
    {
        let text_found = text_found.clone();
        let mut story_context = story_context.clone();
        use_effect(move || {
            if let Some(text) = text_found.read().as_ref() {
                story_context.write().current_choices = text.choices.clone();
            }
            (|| ())()
        });
    }

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

    // 在組件加載時讀取 IndexedDB 數據
    {
        let data = data.clone();
        let mut selected_paragraph_index = selected_paragraph_index.clone();

        use_future(move || async move {
            match read_indexeddb_data().await {
                Ok(choices) => {
                    if !choices.is_empty() {
                        // 找到最後一個選擇對應的段落
                        if let Some(last_choice) = choices.last() {
                            if let Some(item) = data.read().items.iter().find(|item| item.id == *last_choice) {
                                selected_paragraph_index.set(item.index);
                            }
                        }
                    }
                    Ok(())
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
            let chapter_id = item.chapter_id.clone();
            
            // 更新 context 中的目標段落 ID
            story_context.write().target_paragraph_id = Some(goto.clone());
            
            // 從當前段落的 text_found 中查找選項
            let choice = text_found.read().as_ref().and_then(|text| {
                text.choices.iter().find(|c| c.action.to == goto).cloned()
            });
            
            if let Some(choice) = choice {
                let paragraph_id_clone = paragraph_id.clone();
                let chapter_id_clone = chapter_id.clone();
                let choice_clone = choice.clone();
                
                // 先保存選擇
                wasm_bindgen_futures::spawn_local(async move {
                    let _ = save_choice_to_indexeddb(&paragraph_id_clone, &chapter_id_clone, &choice_clone).await;
                });
                
                // 然後更新段落索引
                selected_paragraph_index.set(item.index);
            }
        }
    };

    let render_coming_soon = move || {
        rsx! {
            div {
                class: "prose dark:prose-invert lg:prose-xl mx-auto",
                div {
                    class: "whitespace-pre-wrap mt-16 space-y-8",
                    p {
                        class: "indent-10",
                        {t.read().coming_soon}
                    }
                }
                StoryContent {
                    paragraph: "".to_string(),
                    choices: vec![],
                    on_choice_click: move |_| {},
                    t: t.read().clone(),
                    enabled_choices: vec![]
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
                        t: t.read().clone(),
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