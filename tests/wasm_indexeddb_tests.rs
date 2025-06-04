#![cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test_configure;
use wasm_bindgen_test::wasm_bindgen_test;
use js_sys::Array;
use wasm_bindgen::JsValue;
use ifecaro::services::indexeddb::{set_choices_to_indexeddb, get_choice_from_indexeddb};
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