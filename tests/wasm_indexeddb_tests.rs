#![cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test_configure;
use wasm_bindgen_test::wasm_bindgen_test;
use js_sys::Array;
use wasm_bindgen::JsValue;
use ifecaro::services::indexeddb::{set_choices_to_indexeddb, get_choice_from_indexeddb, set_disabled_choice_to_indexeddb, get_disabled_choices_from_indexeddb};
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_indexeddb_choices_full_path_real() {
    use wasm_bindgen::JsCast;
    // 清空 chapter1 的 choices
    let chapter_id = "chapter1";
    let arr = Array::new();
    set_choices_to_indexeddb(chapter_id, &arr);
    // 寫入完整路徑
    let ids = vec!["start", "cave", "treasure"];
    let arr = Array::new();
    for id in &ids {
        arr.push(&JsValue::from_str(id));
    }
    set_choices_to_indexeddb(chapter_id, &arr);
    // 讀出驗證
    let (tx, rx) = futures_channel::oneshot::channel();
    let cb = wasm_bindgen::closure::Closure::once(Box::new(move |js_value: JsValue| {
        let arr = js_sys::Array::from(&js_value);
        let result: Vec<String> = arr.iter().filter_map(|v| v.as_string()).collect();
        let _ = tx.send(result);
    }) as Box<dyn FnOnce(JsValue)>);
    get_choice_from_indexeddb(chapter_id, cb.as_ref().unchecked_ref());
    cb.forget();
    let result = rx.await.unwrap();
    assert_eq!(result, vec!["start", "cave", "treasure"]);
}

#[wasm_bindgen_test]
async fn test_disabled_choices_storage() {
    use wasm_bindgen::JsCast;
    // 清空所有停用選項
    ifecaro::services::indexeddb::clear_all_disabled_choices_from_indexeddb();
    
    // 設定一個停用選項
    let paragraph_id = "test_paragraph";
    let choice_index = 1;
    set_disabled_choice_to_indexeddb(paragraph_id, choice_index);
    
    // 讀出並驗證
    let (tx, rx) = futures_channel::oneshot::channel();
    let cb = wasm_bindgen::closure::Closure::once(Box::new(move |js_value: JsValue| {
        let arr = js_sys::Array::from(&js_value);
        let result: Vec<usize> = arr.iter()
            .filter_map(|v| v.as_f64().map(|n| n as usize))
            .collect();
        let _ = tx.send(result);
    }) as Box<dyn FnOnce(JsValue)>);
    
    get_disabled_choices_from_indexeddb(paragraph_id, cb.as_ref().unchecked_ref());
    cb.forget();
    
    let result = rx.await.unwrap();
    assert!(result.contains(&choice_index), "Disabled choice should be stored in IndexedDB");
}

#[wasm_bindgen_test]
async fn test_disabled_choices_persistence() {
    use wasm_bindgen::JsCast;
    use ifecaro::services::indexeddb::{set_disabled_choice_to_indexeddb, get_disabled_choices_from_indexeddb};
    
    // 清空所有停用選項
    ifecaro::services::indexeddb::clear_all_disabled_choices_from_indexeddb();
    
    // 設定多個停用選項
    let paragraph_id = "test_paragraph";
    let choice_indices = vec![1, 3, 5];
    for &index in &choice_indices {
        set_disabled_choice_to_indexeddb(paragraph_id, index as u32);
    }
    
    // 模擬重新載入：讀出並驗證
    let (tx, rx) = futures_channel::oneshot::channel();
    let cb = wasm_bindgen::closure::Closure::once(Box::new(move |js_value: JsValue| {
        let arr = js_sys::Array::from(&js_value);
        let result: Vec<usize> = arr.iter()
            .filter_map(|v| v.as_f64().map(|n| n as usize))
            .collect();
        let _ = tx.send(result);
    }) as Box<dyn FnOnce(JsValue)>);
    
    get_disabled_choices_from_indexeddb(paragraph_id, cb.as_ref().unchecked_ref());
    cb.forget();
    
    let result = rx.await.unwrap();
    
    // 驗證所有停用選項都存在
    for &index in &choice_indices {
        assert!(result.contains(&index), "Disabled choice {} should persist after reload", index);
    }
    
    // 驗證沒有多餘的停用選項
    assert_eq!(result.len(), choice_indices.len(), "Should have exactly the same number of disabled choices");
} 