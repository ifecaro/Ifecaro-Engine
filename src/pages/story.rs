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
use web_sys::IdbDatabase;
use wasm_bindgen_futures::JsFuture;
use js_sys::Promise;
use wasm_bindgen::closure::Closure;

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
async fn save_choice_to_indexeddb(paragraph_id: &str, chapter_id: &str, choice: &Choice) -> Result<(), JsValue> {
    web_sys::console::log_1(&JsValue::from_str(&format!("開始保存選擇: {:?}", choice)));
    
    // 如果 action type 為空字串，不記錄選擇
    if choice.action.type_.is_empty() {
        web_sys::console::log_1(&JsValue::from_str("action type 為空字串，不記錄選擇"));
        return Ok(());
    }
    
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window found"))?;
    let indexed_db = window.indexed_db()
        .map_err(|e| {
            web_sys::console::error_1(&JsValue::from_str(&format!("Failed to get indexed_db: {:?}", e)));
            JsValue::from_str(&format!("Failed to get indexed_db: {:?}", e))
        })?
        .ok_or_else(|| {
            web_sys::console::error_1(&JsValue::from_str("No indexed_db found"));
            JsValue::from_str("No indexed_db found")
        })?;
    
    web_sys::console::log_1(&JsValue::from_str("成功獲取 indexed_db"));
    
    let db_request = indexed_db.open_with_u32("story_choices", 1)
        .map_err(|e| {
            web_sys::console::error_1(&JsValue::from_str(&format!("Failed to open database: {:?}", e)));
            JsValue::from_str(&format!("Failed to open database: {:?}", e))
        })?;
    
    // 等待數據庫打開或升級完成
    let db_promise = js_sys::Promise::new(&mut |resolve, reject| {
        // 處理升級事件
        let db_request_upgrade = db_request.clone();
        let onupgradeneeded_callback = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            web_sys::console::log_1(&JsValue::from_str("數據庫升級事件觸發"));
            match db_request_upgrade.result() {
                Ok(db_any) => {
                    match db_any.dyn_into::<IdbDatabase>() {
                        Ok(db) => {
                            match db.create_object_store("choices") {
                                Ok(_) => {
                                    web_sys::console::log_1(&JsValue::from_str("成功創建 choices 存儲對象"));
                                },
                                Err(e) => {
                                    web_sys::console::error_1(&JsValue::from_str(&format!("創建存儲對象失敗: {:?}", e)));
                                }
                            }
                        },
                        Err(e) => {
                            web_sys::console::error_1(&JsValue::from_str(&format!("轉換數據庫對象失敗: {:?}", e)));
                        }
                    }
                },
                Err(e) => {
                    web_sys::console::error_1(&JsValue::from_str(&format!("獲取數據庫結果失敗: {:?}", e)));
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
                            web_sys::console::log_1(&JsValue::from_str("數據庫打開成功"));
                            resolve_success.call1(&JsValue::NULL, &db).unwrap();
                        },
                        Err(e) => {
                            web_sys::console::error_1(&JsValue::from_str(&format!("轉換數據庫對象失敗: {:?}", e)));
                            reject_success.call1(&JsValue::NULL, &e).unwrap();
                        }
                    }
                },
                Err(e) => {
                    web_sys::console::error_1(&JsValue::from_str(&format!("獲取數據庫結果失敗: {:?}", e)));
                    reject_success.call1(&JsValue::NULL, &e).unwrap();
                }
            }
        }) as Box<dyn FnMut(web_sys::Event)>);
        
        // 處理錯誤事件
        let reject_error = reject.clone();
        let onerror_callback = Closure::wrap(Box::new(move |event: web_sys::Event| {
            web_sys::console::error_1(&JsValue::from_str("數據庫打開失敗"));
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
        .map_err(|e| {
            web_sys::console::error_1(&JsValue::from_str(&format!("等待數據庫打開失敗: {:?}", e)));
            JsValue::from_str(&format!("等待數據庫打開失敗: {:?}", e))
        })?
        .dyn_into()
        .map_err(|e| {
            web_sys::console::error_1(&JsValue::from_str(&format!("轉換為 IdbDatabase 失敗: {:?}", e)));
            JsValue::from_str(&format!("轉換為 IdbDatabase 失敗: {:?}", e))
        })?;
    
    web_sys::console::log_1(&JsValue::from_str("成功獲取數據庫"));
    
    let transaction = db.transaction_with_str_sequence_and_mode(
        &js_sys::Array::of1(&JsValue::from_str("choices")),
        web_sys::IdbTransactionMode::Readwrite,
    ).map_err(|e| {
        web_sys::console::error_1(&JsValue::from_str(&format!("創建事務失敗: {:?}", e)));
        JsValue::from_str(&format!("創建事務失敗: {:?}", e))
    })?;
    
    let store = transaction.object_store("choices")
        .map_err(|e| {
            web_sys::console::error_1(&JsValue::from_str(&format!("獲取存儲對象失敗: {:?}", e)));
            JsValue::from_str(&format!("獲取存儲對象失敗: {:?}", e))
        })?;
    
    web_sys::console::log_1(&JsValue::from_str("成功獲取存儲對象"));
    
    // 根據 action type 處理不同的存儲邏輯
    match choice.action.type_.as_str() {
        "setting" => {
            web_sys::console::log_1(&JsValue::from_str("處理 setting 類型"));
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
                        .map_err(|e| {
                            web_sys::console::error_1(&JsValue::from_str(&format!("序列化數據失敗: {:?}", e)));
                            JsValue::from_str(&format!("序列化數據失敗: {:?}", e))
                        })?;
                    web_sys::console::log_1(&JsValue::from_str(&format!("保存 setting 數據: {}", data_str)));
                    let request = store.put_with_key(&JsValue::from_str(&data_str), &JsValue::from_str(&format!("setting_{}", key)))
                        .map_err(|e| {
                            web_sys::console::error_1(&JsValue::from_str(&format!("保存數據失敗: {:?}", e)));
                            JsValue::from_str(&format!("保存數據失敗: {:?}", e))
                        })?;
                    
                    // 等待請求完成
                    JsFuture::from(Promise::resolve(&request))
                        .await
                        .map_err(|e| {
                            web_sys::console::error_1(&JsValue::from_str(&format!("等待請求完成失敗: {:?}", e)));
                            JsValue::from_str(&format!("等待請求完成失敗: {:?}", e))
                        })?;
                }
            }
        },
        _ => {
            web_sys::console::log_1(&JsValue::from_str("處理選擇類型"));
            let choices_key = "user_choices";
            
            // 創建一個 Promise 來等待請求完成
            let request_promise = js_sys::Promise::new(&mut |resolve, reject| {
                let request_result = store.get(&JsValue::from_str(choices_key));
                match request_result {
                    Ok(request) => {
                        let resolve_success = resolve.clone();
                        let reject_success = reject.clone();
                        let request_success = request.clone();
                        
                        let onsuccess = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                            match request_success.result() {
                                Ok(result) => {
                                    web_sys::console::log_1(&JsValue::from_str(&format!("獲取現有選擇成功: {:?}", result)));
                                    resolve_success.call1(&JsValue::NULL, &result).unwrap();
                                },
                                Err(e) => {
                                    web_sys::console::error_1(&JsValue::from_str(&format!("獲取現有選擇結果失敗: {:?}", e)));
                                    reject_success.call1(&JsValue::NULL, &JsValue::from_str(&format!("獲取現有選擇結果失敗: {:?}", e))).unwrap();
                                }
                            }
                        }) as Box<dyn FnMut(web_sys::Event)>);
                        
                        let reject_error = reject.clone();
                        let onerror = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                            web_sys::console::error_1(&JsValue::from_str("獲取現有選擇請求失敗"));
                            reject_error.call1(&JsValue::NULL, &JsValue::from_str("獲取現有選擇請求失敗")).unwrap();
                        }) as Box<dyn FnMut(web_sys::Event)>);
                        
                        request.set_onsuccess(Some(onsuccess.as_ref().unchecked_ref()));
                        request.set_onerror(Some(onerror.as_ref().unchecked_ref()));
                        
                        onsuccess.forget();
                        onerror.forget();
                    },
                    Err(e) => {
                        web_sys::console::error_1(&JsValue::from_str(&format!("獲取現有選擇失敗: {:?}", e)));
                        reject.call1(&JsValue::NULL, &JsValue::from_str(&format!("獲取現有選擇失敗: {:?}", e))).unwrap();
                    }
                }
            });
            
            // 等待請求完成
            let existing_choices = JsFuture::from(request_promise)
                .await
                .map_err(|e| {
                    web_sys::console::error_1(&JsValue::from_str(&format!("等待現有選擇失敗: {:?}", e)));
                    JsValue::from_str(&format!("等待現有選擇失敗: {:?}", e))
                })?;
            
            web_sys::console::log_1(&JsValue::from_str(&format!("現有選擇: {:?}", existing_choices)));
            
            let mut choices: Vec<String> = if !existing_choices.is_undefined() && !existing_choices.is_null() {
                let existing_str = existing_choices.as_string()
                    .ok_or_else(|| {
                        web_sys::console::error_1(&JsValue::from_str("轉換現有選擇為字符串失敗"));
                        JsValue::from_str("轉換現有選擇為字符串失敗")
                    })?;
                web_sys::console::log_1(&JsValue::from_str(&format!("現有選擇字符串: {}", existing_str)));
                serde_json::from_str(&existing_str)
                    .map_err(|e| {
                        web_sys::console::error_1(&JsValue::from_str(&format!("解析現有選擇失敗: {:?}", e)));
                        JsValue::from_str(&format!("解析現有選擇失敗: {:?}", e))
                    })?
            } else {
                web_sys::console::log_1(&JsValue::from_str("沒有現有選擇，創建新數組"));
                Vec::new()
            };
            
            choices.push(choice.action.to.clone());
            web_sys::console::log_1(&JsValue::from_str(&format!("更新後的選擇: {:?}", choices)));
            
            let data = serde_json::to_string(&choices)
                .map_err(|e| {
                    web_sys::console::error_1(&JsValue::from_str(&format!("序列化選擇失敗: {:?}", e)));
                    JsValue::from_str(&format!("序列化選擇失敗: {:?}", e))
                })?;
            web_sys::console::log_1(&JsValue::from_str(&format!("保存選擇數據: {}", data)));
            let request = store.put_with_key(&JsValue::from_str(&data), &JsValue::from_str(choices_key))
                .map_err(|e| {
                    web_sys::console::error_1(&JsValue::from_str(&format!("保存選擇失敗: {:?}", e)));
                    JsValue::from_str(&format!("保存選擇失敗: {:?}", e))
                })?;
            
            // 等待請求完成
            JsFuture::from(Promise::resolve(&request))
                .await
                .map_err(|e| {
                    web_sys::console::error_1(&JsValue::from_str(&format!("等待請求完成失敗: {:?}", e)));
                    JsValue::from_str(&format!("等待請求完成失敗: {:?}", e))
                })?;
        }
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
        .map_err(|e| {
            web_sys::console::error_1(&JsValue::from_str(&format!("等待事務完成失敗: {:?}", e)));
            JsValue::from_str(&format!("等待事務完成失敗: {:?}", e))
        })?;
    
    web_sys::console::log_1(&JsValue::from_str("成功保存選擇"));
    Ok(())
}

async fn read_indexeddb_data() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window found"))?;
    let indexed_db = window.indexed_db()
        .map_err(|e| {
            web_sys::console::error_1(&JsValue::from_str(&format!("Failed to get indexed_db: {:?}", e)));
            JsValue::from_str(&format!("Failed to get indexed_db: {:?}", e))
        })?
        .ok_or_else(|| {
            web_sys::console::error_1(&JsValue::from_str("No indexed_db found"));
            JsValue::from_str("No indexed_db found")
        })?;
    
    let db_request = indexed_db.open_with_u32("story_choices", 1)
        .map_err(|e| {
            web_sys::console::error_1(&JsValue::from_str(&format!("Failed to open database: {:?}", e)));
            JsValue::from_str(&format!("Failed to open database: {:?}", e))
        })?;
    
    // 等待數據庫打開或升級完成
    let db_promise = js_sys::Promise::new(&mut |resolve, reject| {
        // 處理升級事件
        let db_request_upgrade = db_request.clone();
        let onupgradeneeded_callback = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            web_sys::console::log_1(&JsValue::from_str("數據庫升級事件觸發"));
            match db_request_upgrade.result() {
                Ok(db_any) => {
                    match db_any.dyn_into::<IdbDatabase>() {
                        Ok(db) => {
                            match db.create_object_store("choices") {
                                Ok(_) => {
                                    web_sys::console::log_1(&JsValue::from_str("成功創建 choices 存儲對象"));
                                },
                                Err(e) => {
                                    web_sys::console::error_1(&JsValue::from_str(&format!("創建存儲對象失敗: {:?}", e)));
                                }
                            }
                        },
                        Err(e) => {
                            web_sys::console::error_1(&JsValue::from_str(&format!("轉換數據庫對象失敗: {:?}", e)));
                        }
                    }
                },
                Err(e) => {
                    web_sys::console::error_1(&JsValue::from_str(&format!("獲取數據庫結果失敗: {:?}", e)));
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
                            web_sys::console::log_1(&JsValue::from_str("數據庫打開成功"));
                            resolve_success.call1(&JsValue::NULL, &db).unwrap();
                        },
                        Err(e) => {
                            web_sys::console::error_1(&JsValue::from_str(&format!("轉換數據庫對象失敗: {:?}", e)));
                            reject_success.call1(&JsValue::NULL, &e).unwrap();
                        }
                    }
                },
                Err(e) => {
                    web_sys::console::error_1(&JsValue::from_str(&format!("獲取數據庫結果失敗: {:?}", e)));
                    reject_success.call1(&JsValue::NULL, &e).unwrap();
                }
            }
        }) as Box<dyn FnMut(web_sys::Event)>);
        
        // 處理錯誤事件
        let reject_error = reject.clone();
        let onerror_callback = Closure::wrap(Box::new(move |event: web_sys::Event| {
            web_sys::console::error_1(&JsValue::from_str("數據庫打開失敗"));
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
        .map_err(|e| {
            web_sys::console::error_1(&JsValue::from_str(&format!("等待數據庫打開失敗: {:?}", e)));
            JsValue::from_str(&format!("等待數據庫打開失敗: {:?}", e))
        })?
        .dyn_into()
        .map_err(|e| {
            web_sys::console::error_1(&JsValue::from_str(&format!("轉換為 IdbDatabase 失敗: {:?}", e)));
            JsValue::from_str(&format!("轉換為 IdbDatabase 失敗: {:?}", e))
        })?;
    
    web_sys::console::log_1(&JsValue::from_str("成功獲取數據庫"));
    
    let transaction = db.transaction_with_str_sequence_and_mode(
        &js_sys::Array::of1(&JsValue::from_str("choices")),
        web_sys::IdbTransactionMode::Readonly,
    ).map_err(|e| {
        web_sys::console::error_1(&JsValue::from_str(&format!("創建事務失敗: {:?}", e)));
        JsValue::from_str(&format!("創建事務失敗: {:?}", e))
    })?;
    
    let store = transaction.object_store("choices")
        .map_err(|e| {
            web_sys::console::error_1(&JsValue::from_str(&format!("獲取存儲對象失敗: {:?}", e)));
            JsValue::from_str(&format!("獲取存儲對象失敗: {:?}", e))
        })?;
    
    // 獲取所有數據
    let request = store.get_all()
        .map_err(|e| {
            web_sys::console::error_1(&JsValue::from_str(&format!("獲取所有數據失敗: {:?}", e)));
            JsValue::from_str(&format!("獲取所有數據失敗: {:?}", e))
        })?;
    
    let result = JsFuture::from(Promise::resolve(&request))
        .await
        .map_err(|e| {
            web_sys::console::error_1(&JsValue::from_str(&format!("獲取結果失敗: {:?}", e)));
            JsValue::from_str(&format!("獲取結果失敗: {:?}", e))
        })?;
    
    web_sys::console::log_1(&JsValue::from_str("IndexedDB 中的所有數據:"));
    web_sys::console::log_1(&result);
    
    // 特別獲取 user_choices
    let choices_request = store.get(&JsValue::from_str("user_choices"))
        .map_err(|e| {
            web_sys::console::error_1(&JsValue::from_str(&format!("獲取 user_choices 失敗: {:?}", e)));
            JsValue::from_str(&format!("獲取 user_choices 失敗: {:?}", e))
        })?;
    
    let choices_result = JsFuture::from(Promise::resolve(&choices_request))
        .await
        .map_err(|e| {
            web_sys::console::error_1(&JsValue::from_str(&format!("獲取 user_choices 結果失敗: {:?}", e)));
            JsValue::from_str(&format!("獲取 user_choices 結果失敗: {:?}", e))
        })?;
    
    web_sys::console::log_1(&JsValue::from_str("user_choices 的內容:"));
    web_sys::console::log_1(&choices_result);
    
    Ok(())
}

pub fn Story(_props: StoryProps) -> Element {
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

    // 在組件加載時讀取 IndexedDB 數據
    {
        use_future(move || async move {
            match read_indexeddb_data().await {
                Ok(_) => web_sys::console::log_1(&JsValue::from_str("成功讀取 IndexedDB 數據")),
                Err(e) => web_sys::console::error_1(&JsValue::from_str(&format!("讀取 IndexedDB 數據失敗: {:?}", e)))
            }
        });
    }

    let text_found_clone = text_found.clone();
    let paragraph_clone = paragraph.clone();

    let on_choice_click = move |goto: String| {
        web_sys::console::log_1(&JsValue::from_str(&format!("選擇點擊: {}", goto)));
        
        if let Some(item) = data.read().items.iter().find(|item| item.id == goto) {
            web_sys::console::log_1(&JsValue::from_str(&format!("找到段落: {:?}", item)));
            let paragraph_id = item.id.clone();
            let chapter_id = item.chapter_id.clone();
            let choice = text_found.read().as_ref().and_then(|text| {
                text.choices.iter().find(|c| c.action.to == goto).cloned()
            });
            
            if let Some(choice) = choice {
                web_sys::console::log_1(&JsValue::from_str(&format!("找到選擇: {:?}", choice)));
                let paragraph_id_clone = paragraph_id.clone();
                let chapter_id_clone = chapter_id.clone();
                let choice_clone = choice.clone();
                
                wasm_bindgen_futures::spawn_local(async move {
                    web_sys::console::log_1(&JsValue::from_str("開始保存選擇到 IndexedDB"));
                    match save_choice_to_indexeddb(&paragraph_id_clone, &chapter_id_clone, &choice_clone).await {
                        Ok(_) => web_sys::console::log_1(&JsValue::from_str("成功保存選擇到 IndexedDB")),
                        Err(e) => web_sys::console::error_1(&JsValue::from_str(&format!("保存選擇到 IndexedDB 失敗: {:?}", e)))
                    }
                });
            } else {
                web_sys::console::error_1(&JsValue::from_str(&format!("未找到選擇: {}", goto)));
            }
            
            selected_paragraph_index.set(item.index);
        } else {
            web_sys::console::error_1(&JsValue::from_str(&format!("未找到段落: {}", goto)));
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