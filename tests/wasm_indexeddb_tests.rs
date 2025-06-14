#![cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test_configure;
use wasm_bindgen_test::wasm_bindgen_test;
use js_sys::Array;
use wasm_bindgen::JsValue;
use ifecaro::services::indexeddb::{
    set_choices_to_indexeddb, 
    get_choice_from_indexeddb, 
    set_disabled_choice_to_indexeddb, 
    get_disabled_choices_from_indexeddb,
    clear_all_disabled_choices_from_indexeddb
};
use web_sys::console;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_indexeddb_choices_full_path_real() {
    console::log_1(&"Starting test_indexeddb_choices_full_path_real".into());
    
    let chapter_id = "chapter1";
    
    // 清空 chapter1 的 choices
    let empty_arr = Array::new();
    set_choices_to_indexeddb(chapter_id, &empty_arr).await.unwrap();
    
    // 寫入完整路徑
    let ids = vec!["start", "cave", "treasure"];
    let arr = ids.iter().map(|s| JsValue::from_str(s)).collect::<Array>();
    set_choices_to_indexeddb(chapter_id, &arr).await.unwrap();
    
    // 讀出驗證
    console::log_1(&"Getting choices from IndexedDB".into());
    let result_js = get_choice_from_indexeddb(chapter_id).await.unwrap();
    let result_arr = Array::from(&result_js);
    let result: Vec<String> = result_arr.iter().filter_map(|v| v.as_string()).collect();

    console::log_1(&format!("Final result: {:?}", result).into());
    assert_eq!(result, ids);
}

#[wasm_bindgen_test]
async fn test_disabled_choices_storage() {
    console::log_1(&"Starting test_disabled_choices_storage".into());
    
    // 清空所有停用選項
    clear_all_disabled_choices_from_indexeddb().await.unwrap();
    
    // 設定一個停用選項
    let paragraph_id = "test_paragraph_storage";
    let choice_index = 1u32;
    set_disabled_choice_to_indexeddb(paragraph_id, choice_index).await.unwrap();
    
    // 讀出並驗證
    console::log_1(&"Getting disabled choices from IndexedDB".into());
    let result_js = get_disabled_choices_from_indexeddb(paragraph_id).await.unwrap();
    let result_arr = Array::from(&result_js);
    let result: Vec<u32> = result_arr.iter()
        .filter_map(|v| v.as_f64().map(|n| n as u32))
        .collect();

    console::log_1(&format!("Final result: {:?}", result).into());
    assert!(result.contains(&choice_index), "Disabled choice should be stored in IndexedDB");
    assert_eq!(result.len(), 1, "Should only have one disabled choice");
}

#[wasm_bindgen_test]
async fn test_disabled_choices_persistence() {
    console::log_1(&"Starting test_disabled_choices_persistence".into());
    
    let paragraph_id = "test_paragraph_persistence";
    
    // 清空停用選項
    clear_all_disabled_choices_from_indexeddb().await.unwrap();

    // 設定多個停用選項
    let choice_indices = vec![1u32, 3u32, 5u32];
    for &index in &choice_indices {
        console::log_1(&format!("Setting disabled choice: {}", index).into());
        set_disabled_choice_to_indexeddb(paragraph_id, index).await.unwrap();
    }
    
    // 讀出並驗證
    console::log_1(&"Getting disabled choices from IndexedDB".into());
    let result_js = get_disabled_choices_from_indexeddb(paragraph_id).await.unwrap();
    let result_arr = Array::from(&result_js);
    let mut result: Vec<u32> = result_arr.iter()
        .filter_map(|v| v.as_f64().map(|n| n as u32))
        .collect();
        
    console::log_1(&format!("Final result: {:?}", result).into());
    
    // 排序以進行確定性比較
    result.sort();
    let mut expected = choice_indices.clone();
    expected.sort();

    // 驗證所有停用選項都存在
    assert_eq!(result, expected, "All disabled choices should persist");
}

#[wasm_bindgen_test]
async fn test_clear_all_disabled_choices_functionality() {
    console::log_1(&"Starting test_clear_all_disabled_choices_functionality".into());

    // Insert a disabled choice
    let paragraph_id = "test_paragraph_clear";
    let choice_index = 2u32;
    set_disabled_choice_to_indexeddb(paragraph_id, choice_index).await.unwrap();

    // Verify it exists
    let pre_clear_js = get_disabled_choices_from_indexeddb(paragraph_id).await.unwrap();
    let pre_clear_arr = Array::from(&pre_clear_js);
    assert!(pre_clear_arr.length() > 0, "Disabled choice should exist before clearing");

    // Clear all disabled choices
    clear_all_disabled_choices_from_indexeddb().await.unwrap();

    // Retrieve again to ensure it's cleared
    let post_clear_js = get_disabled_choices_from_indexeddb(paragraph_id).await.unwrap();
    let post_clear_arr = Array::from(&post_clear_js);
    assert_eq!(post_clear_arr.length(), 0, "Disabled choices should be cleared from IndexedDB");
} 