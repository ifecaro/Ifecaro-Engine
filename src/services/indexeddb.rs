use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/services/indexeddb_js.js")]
extern "C" {
    #[wasm_bindgen(js_name = setSettingToIndexedDB)]
    pub fn set_setting_to_indexeddb(key: &str, value: &str);

    #[wasm_bindgen(js_name = getSettingsFromIndexedDB)]
    pub fn get_settings_from_indexeddb(callback: &js_sys::Function);

    #[wasm_bindgen(js_name = setChoiceToIndexedDB)]
    pub fn set_choice_to_indexeddb(chapter_id: &str, paragraph_id: &str);
} 