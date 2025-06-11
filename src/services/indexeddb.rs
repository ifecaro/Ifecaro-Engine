use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/services/indexeddb_js.js")]
extern "C" {
    #[wasm_bindgen(js_name = setSettingToIndexedDB)]
    pub fn set_setting_to_indexeddb(key: &str, value: &str);

    #[wasm_bindgen(js_name = getSettingsFromIndexedDB)]
    pub fn get_settings_from_indexeddb(callback: &js_sys::Function);

    #[wasm_bindgen(js_name = setChoiceToIndexedDB)]
    pub fn set_choice_to_indexeddb(chapter_id: &str, paragraph_id: &str);

    #[wasm_bindgen(js_name = getChoiceFromIndexedDB)]
    pub fn get_choice_from_indexeddb(chapter_id: &str, callback: &js_sys::Function);

    #[wasm_bindgen(js_name = setDisabledChoiceToIndexedDB)]
    pub fn set_disabled_choice_to_indexeddb(paragraph_id: &str, choice_index: u32);

    #[wasm_bindgen(js_name = getDisabledChoicesFromIndexedDB)]
    pub fn get_disabled_choices_from_indexeddb(paragraph_id: &str, callback: &js_sys::Function);

    #[wasm_bindgen(js_name = clearDisabledChoicesForParagraph)]
    pub fn clear_disabled_choices_for_paragraph(paragraph_id: &str);

    #[wasm_bindgen(js_name = setRandomChoiceToIndexedDB)]
    pub fn set_random_choice_to_indexeddb(paragraph_id: &str, choice_index: u32, original_choices: &js_sys::Array, selected_choice: &str);

    #[wasm_bindgen(js_name = getRandomChoiceFromIndexedDB)]
    pub fn get_random_choice_from_indexeddb(paragraph_id: &str, choice_index: u32, callback: &js_sys::Function);

    #[wasm_bindgen(js_name = setChoicesToIndexedDB)]
    pub fn set_choices_to_indexeddb(chapter_id: &str, ids_array: &js_sys::Array);

    #[wasm_bindgen(js_name = clearChoicesAndRandomChoices)]
    pub fn clear_choices_and_random_choices();

    #[wasm_bindgen(js_namespace = window)]
    pub fn clearAllDisabledChoices();
}

pub fn clear_all_disabled_choices_from_indexeddb() {
    clearAllDisabledChoices();
} 