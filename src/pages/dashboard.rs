use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use dioxus::events::FormEvent;
use crate::components::dropdown::Dropdown;
use crate::components::chapter_selector::ChapterSelector;
use dioxus::hooks::use_context;
use crate::contexts::language_context::LanguageState;
use crate::contexts::chapter_context::ChapterState;
use crate::contexts::paragraph_context::{ParagraphState, Paragraph as ContextParagraph, Text as ContextText, ParagraphChoice as ContextParagraphChoice};
use crate::components::language_selector::{Language, AVAILABLE_LANGUAGES};
use crate::constants::config::{BASE_API_URL, PARAGRAPHS};
use crate::components::paragraph_list::Paragraph as ParagraphListParagraph;
use dioxus_i18n::t;
use crate::components::form::{TextareaField, ChoiceOptions};
use gloo_timers::callback::Timeout;
use crate::contexts::chapter_context::Chapter;
use crate::hooks::choices_reducer::{use_choices, Action as CAct, Choice as ChoiceStruct};
use crate::contexts::toast_context::use_toast;
use crate::components::toast::ToastType;

#[derive(Props, Clone, PartialEq)]
pub struct DashboardProps {
    pub lang: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {
    pub items: Vec<ContextParagraph>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Collection {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub collection_type: String,
    pub system: bool,
    pub schema: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollectionsData {
    pub items: Vec<Collection>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChapterData {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub chapter_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChaptersData {
    pub items: Vec<ChapterData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChapterInfo {
    pub id: String,
    pub titles: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemData {
    pub id: String,
    pub key: String,
    #[serde(rename = "value")]
    pub value_raw: serde_json::Value,
    #[serde(rename = "collectionId")]
    pub collection_id: String,
    #[serde(rename = "collectionName")]
    pub collection_name: String,
    pub created: String,
    pub updated: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemDataResponse {
    pub items: Vec<SystemData>,
    pub page: i32,
    #[serde(rename = "perPage")]
    pub per_page: i32,
    #[serde(rename = "totalItems")]
    pub total_items: i32,
    #[serde(rename = "totalPages")]
    pub total_pages: i32,
}

#[allow(dead_code)]
struct ChoiceOption {
    id: String,
    preview: String,
}

fn display_language(lang: &&Language) -> String {
    lang.name.to_string()
}

#[allow(non_snake_case)]
pub fn Dashboard(_props: DashboardProps) -> Element {
    let language_state = use_context::<Signal<LanguageState>>();
    let chapter_state = use_context::<Signal<ChapterState>>();
    let paragraph_state = use_context::<Signal<ParagraphState>>();
    let current_lang = language_state.read().current_language.clone();
    
    // Initialize paragraph_language to current interface language
    let mut paragraph_language = use_signal(|| current_lang.clone());

    // Update thread_local variable and paragraph_language when language changes
    use_effect(move || {
        let _current_lang = language_state.read().current_language.clone();

        let paragraph_lang_ref = paragraph_language.clone();
        let mut paragraph_lang_ref2 = paragraph_lang_ref.clone();
        Timeout::new(0, move || {
            paragraph_lang_ref2.set(_current_lang);
        }).forget();

        // No cleanup necessary
    });

    let (choices_state, dispatch_choice) = use_choices();

    let mut paragraphs = use_signal(|| String::new());
    let mut choices = use_signal(|| Vec::new());
    let _init_done = use_signal(|| false);
    let mut is_open = use_signal(|| false);
    let mut search_query = use_signal(|| String::new());
    let mut is_paragraph_open = use_signal(|| false);
    let mut paragraph_search_query = use_signal(|| String::new());
    let available_paragraphs = use_signal(|| Vec::<crate::components::paragraph_list::Paragraph>::new());
    let _target_chapter_paragraphs = use_signal(|| Vec::<crate::components::paragraph_list::Paragraph>::new());
    let mut selected_chapter = use_signal(|| String::new());
    let mut is_chapter_open = use_signal(|| false);
    let mut chapter_search_query = use_signal(|| String::new());
    let mut selected_paragraph = use_signal(|| None::<ContextParagraph>);
    let mut is_edit_mode = use_signal(|| false);
    let _has_loaded = use_signal(|| paragraph_state.read().loaded);
    let mut _should_scroll = use_signal(|| false);
    let _target_chapter = use_signal(|| String::new());

    // Add three independent chapter selector states
    let _header_chapter = use_signal(|| String::new());
    let _header_chapter_open = use_signal(|| false);
    let _header_chapter_search = use_signal(|| String::new());
    
    let _first_choice_chapter = use_signal(|| String::new());
    let _first_choice_chapter_open = use_signal(|| false);
    let _first_choice_chapter_search = use_signal(|| String::new());
    
    let _extra_choice_chapter = use_signal(|| String::new());
    let _extra_choice_chapter_open = use_signal(|| false);
    let _extra_choice_chapter_search = use_signal(|| String::new());

    let mut paragraphs_error = use_signal(|| false);
    let mut chapter_error = use_signal(|| false);

    let _new_action_type = use_signal(|| String::new());
    let _new_action_key = use_signal(|| None::<String>);
    let _new_action_value = use_signal(|| None::<serde_json::Value>);
    let _extra_action_types = use_signal(|| Vec::<String>::new());
    let _extra_action_keys = use_signal(|| Vec::<Option<String>>::new());
    let _extra_action_values = use_signal(|| Vec::<Option<serde_json::Value>>::new());

    let _paragraph_previews = use_signal(|| Vec::<crate::components::paragraph_list::Paragraph>::new());

    // Signal to indicate async submit in progress
    let is_submitting = use_signal(|| false);

    // Recalculate paragraph previews whenever selected chapter, paragraph language, or paragraph data changes
    {
        let paragraph_state = paragraph_state.clone();
        let paragraph_language = paragraph_language.clone();
        let selected_chapter = selected_chapter.clone();
        let mut available_paragraphs = available_paragraphs.clone();
        let language_state = language_state.clone();

        use_effect(move || {
            let selected_language = paragraph_language.read().clone();
            let selected_chapter_id = selected_chapter.read().clone();
            let interface_language = language_state.read().current_language.clone();

            // 讀取 paragraph_state signal，以便 effect 於資料變動時重新觸發
            let chapter_paragraphs_snapshot = paragraph_state.read().get_by_chapter(&selected_chapter_id).clone();

            if selected_language.is_empty() || selected_chapter_id.is_empty() {
                available_paragraphs.write().clear();
                return;
            }

            let (translated, untranslated): (Vec<_>, Vec<_>) = chapter_paragraphs_snapshot
                .iter()
                .map(|p| {
                    let has_translation = p.texts.iter().any(|text| text.lang == selected_language);
                    let preview = p.texts
                        .iter()
                        .find(|text| text.lang == selected_language)
                        .or_else(|| p.texts.iter().find(|text| text.lang == interface_language))
                        .or_else(|| p.texts.iter().find(|text| text.lang == "en-US" || text.lang == "en-GB"))
                        .or_else(|| p.texts.first())
                        .map(|text| text.paragraphs.lines().next().unwrap_or("").to_string())
                        .unwrap_or_default();
                    (crate::components::paragraph_list::Paragraph { id: p.id.clone(), preview, has_translation }, has_translation)
                })
                .partition(|(_, has_tr)| *has_tr);

            let mut merged = translated.into_iter().map(|(p, _)| p).collect::<Vec<_>>();
            merged.extend(untranslated.into_iter().map(|(p, _)| p));
            *available_paragraphs.write() = merged;

            // No cleanup necessary
        });
    }

    let filtered_languages = use_memo(move || {
        let query = search_query.read().to_lowercase();
        AVAILABLE_LANGUAGES
            .iter()
            .filter(|l| {
                l.name.to_lowercase().contains(&query)
                    || l.code.to_lowercase().contains(&query)
            })
            .collect::<Vec<_>>()
    });

    let _dropdown_class = use_memo(move || {
        if *is_open.read() {
            "translate-y-0 opacity-100"
        } else {
            "-translate-y-2 opacity-0 pointer-events-none"
        }
    });

    let current_language = use_memo(move || {
        if let Ok(lang_code) = paragraph_language.try_read() {
            AVAILABLE_LANGUAGES
                .iter()
                .find(|l| l.code == *lang_code)
                .map(|l| l.name)
                .unwrap_or("繁體中文")
        } else {
            "繁體中文"
        }
    });

    let is_form_valid = use_memo(move || {
        let main_fields_valid = if let (Ok(p), Ok(c)) = (paragraphs.try_read(), selected_chapter.try_read()) {
            !p.trim().is_empty() && !c.is_empty()
        } else {
            false
        };
        let has_any_choices = if let Ok(choices) = choices.try_read() {
            !choices.is_empty()
        } else {
            false
        };
        let choices_valid = if let Ok(choices) = choices.try_read() {
            choices.iter().all(|(choice_text, _, _, _, _, _, _, _, _, _): &(String, Vec<String>, String, Option<String>, Option<serde_json::Value>, String, bool, Option<u32>, Option<String>, String)| {
                let has_content = !choice_text.is_empty();
                if has_content {
                    !choice_text.is_empty()
                } else {
                    true
                }
            })
        } else {
            false
        };
        main_fields_valid && (!has_any_choices || choices_valid)
    });

    let has_changes = use_memo(move || {
        if let Ok(edit_mode) = is_edit_mode.try_read() {
            if *edit_mode {
                let paragraphs_changed = if let (Ok(p), Ok(sel_para), Ok(lang)) = (paragraphs.try_read(), selected_paragraph.try_read(), paragraph_language.try_read()) {
                    p.to_string() != sel_para.as_ref()
                        .map(|p| p.texts.iter().find(|t| t.lang == *lang)
                            .map(|t| t.paragraphs.clone())
                            .unwrap_or_default())
                        .unwrap_or_default()
                } else {
                    false
                };
                let has_option_changes = if let (Ok(sel_para), Ok(lang), Ok(new_choices)) = (selected_paragraph.try_read(), paragraph_language.try_read(), choices.try_read()) {
                    if let Some(paragraph) = sel_para.as_ref() {
                        let current_text_choices = &paragraph.texts.iter().find(|t| t.lang == *lang)
                            .map(|t| t.choices.clone())
                            .unwrap_or_default();
                        let current_paragraph_choices = &paragraph.choices;
                        
                        // Check if option quantity changes
                        if current_text_choices.len() != new_choices.len() || current_paragraph_choices.len() != new_choices.len() {
                            return true;
                        }
                        
                        // Check each option's detailed changes
                        for (i, (new_choice_text, new_to, new_type, new_key, new_value, _new_target_chapter, new_same_page, new_time_limit, new_timeout_to, _new_timeout_target_chapter)) in new_choices.iter().enumerate() {
                            // Check if option text changes
                            if let Some(old_choice_text) = current_text_choices.get(i) {
                                if old_choice_text != new_choice_text {
                                    return true;
                                }
                            }
                            
                            // Check if option data changes
                            if let Some(old_choice) = current_paragraph_choices.get(i) {
                                let (old_to, old_type, old_key, old_value, old_same_page, old_time_limit, old_choice_timeout) = match old_choice {
                                    ContextParagraphChoice::Simple(texts) => (texts.clone(), "goto".to_string(), None, None, false, None, None),
                                    ContextParagraphChoice::SimpleOld(text) => (vec![text.clone()], "goto".to_string(), None, None, false, None, None),
                                    ContextParagraphChoice::Complex { to, type_, key, value, same_page, time_limit, timeout_to, .. } => {
                                        (to.clone(), type_.clone(), key.clone(), value.clone(), same_page.unwrap_or(false), time_limit.clone(), timeout_to.clone())
                                    },
                                    ContextParagraphChoice::ComplexOld { to, type_, key, value, same_page, time_limit, timeout_to, .. } => {
                                        (vec![to.clone()], type_.clone(), key.clone(), value.clone(), same_page.unwrap_or(false), time_limit.clone(), timeout_to.clone())
                                    },
                                };
                                
                                // Compare all attributes
                                if old_to != *new_to || 
                                   old_type != *new_type || 
                                   old_key != *new_key || 
                                   old_value != *new_value || 
                                   old_same_page != *new_same_page || 
                                   old_time_limit != *new_time_limit ||
                                   old_choice_timeout != *new_timeout_to {
                                    return true;
                                }
                            }
                        }
                        false
                    } else {
                        false
                    }
                } else {
                    false
                };
                paragraphs_changed || has_option_changes
            } else {
                let has_paragraph = if let (Ok(p), Ok(c)) = (paragraphs.try_read(), selected_chapter.try_read()) {
                    !p.trim().is_empty() && !c.is_empty()
                } else {
                    false
                };
                let has_valid_choices = if let Ok(choices) = choices.try_read() {
                    choices.iter().any(|(choice_text, _, _, _, _, _, _, _, _, _)| {
                        let has_content = !choice_text.is_empty();
                        if has_content {
                            !choice_text.is_empty()
                        } else {
                            false
                        }
                    })
                } else {
                    false
                };
                has_paragraph || has_valid_choices
            }
        } else {
            false
        }
    });

    let validate_field = |value: &str, error_signal: &mut Signal<bool>| {
        if value.trim().is_empty() {
            error_signal.set(true);
        } else {
            error_signal.set(false);
        }
    };

    let paragraph_language_cl = paragraph_language.clone();
    let paragraph_state_cl = paragraph_state.clone();
    let language_state_cl = language_state.clone();

    let dispatch_choice_handle = dispatch_choice.clone();
    let handle_choice_change = move |(index, field, value): (usize, String, String)| {
        // 1. send field update into reducer as the single source-of-truth
        let dispatch = dispatch_choice_handle.clone();
        match field.as_str() {
            "caption" => (dispatch.clone())(CAct::SetField { idx: index, field: "caption", value: value.clone() }),
            "goto" => (dispatch.clone())(CAct::SetField { idx: index, field: "goto", value: value.clone() }),
            "action_type" => (dispatch.clone())(CAct::SetField { idx: index, field: "action_type", value: value.clone() }),
            "action_key" => (dispatch.clone())(CAct::SetField { idx: index, field: "action_key", value: value.clone() }),
            "action_value" => (dispatch.clone())(CAct::SetField { idx: index, field: "action_value", value: value.clone() }),
            "target_chapter" => (dispatch.clone())(CAct::SetField { idx: index, field: "target_chapter", value: value.clone() }),
            "same_page" => (dispatch.clone())(CAct::SetField { idx: index, field: "same_page", value: value.clone() }),
            "time_limit" => (dispatch.clone())(CAct::SetField { idx: index, field: "time_limit", value: value.clone() }),
            "timeout_to" => (dispatch.clone())(CAct::SetField { idx: index, field: "timeout_to", value: value.clone() }),
            "timeout_target_chapter" => {
                // Update chosen timeout chapter
                (dispatch.clone())(CAct::SetField { idx: index, field: "timeout_target_chapter", value: value.clone() });

                // Ensure dropdown closed
                (dispatch.clone())(CAct::ToggleTimeoutChapter(index));

                // Regenerate paragraph list for timeout selector
                if !value.is_empty() {
                    let selected_lang = paragraph_language_cl.read().clone();
                    let interface_language = language_state_cl.read().current_language.clone();

                    let chapter_paragraphs = paragraph_state_cl.read().get_by_chapter(&value);
                    let filtered = chapter_paragraphs
                        .iter()
                        .map(|item| {
                            let has_translation = item.texts.iter().any(|t| t.lang == selected_lang);
                            let preview = item.texts
                                .iter()
                                .find(|t| t.lang == selected_lang)
                                .or_else(|| item.texts.iter().find(|t| t.lang == interface_language))
                                .or_else(|| item.texts.iter().find(|t| t.lang == "en-US" || t.lang == "en-GB"))
                                .or_else(|| item.texts.first())
                                .map(|t| t.paragraphs.lines().next().unwrap_or("").to_string())
                                .unwrap_or_default();
                            crate::components::paragraph_list::Paragraph { id: item.id.clone(), preview, has_translation }
                        })
                        .collect::<Vec<_>>();
                    (dispatch.clone())(CAct::SetTimeoutParaList { idx: index, list: filtered });
                } else {
                    (dispatch.clone())(CAct::SetTimeoutParaList { idx: index, list: Vec::new() });
                }
            },
            _ => {}
        }

        // 2. extra handling when target_chapter changed
        if field == "target_chapter" {
            // ensure chapter dropdown closed via reducer toggle (only if open)
            (dispatch.clone())(CAct::ToggleChapter(index));

            // regenerate paragraph preview list cache in reducer
            if !value.is_empty() {
                let selected_lang = paragraph_language_cl.read().clone();
                let interface_language = language_state_cl.read().current_language.clone();

                let chapter_paragraphs = paragraph_state_cl.read().get_by_chapter(&value);
                let filtered = chapter_paragraphs
                    .iter()
                    .map(|item| {
                        let has_translation = item.texts.iter().any(|t| t.lang == selected_lang);
                        let preview = item.texts
                            .iter()
                            .find(|t| t.lang == selected_lang)
                            .or_else(|| item.texts.iter().find(|t| t.lang == interface_language))
                            .or_else(|| item.texts.iter().find(|t| t.lang == "en-US" || t.lang == "en-GB"))
                            .or_else(|| item.texts.first())
                            .map(|t| t.paragraphs.lines().next().unwrap_or("").to_string())
                            .unwrap_or_default();
                        crate::components::paragraph_list::Paragraph { id: item.id.clone(), preview, has_translation }
                    })
                    .collect::<Vec<_>>();
                (dispatch.clone())(CAct::SetParaList { idx: index, list: filtered });
            } else {
                (dispatch.clone())(CAct::SetParaList { idx: index, list: Vec::new() });
            }
        }
    };

    let handle_choice_add_paragraph = {
        let dispatch = dispatch_choice.clone();
        let choices_state = choices_state.clone();
        move |(index, paragraph_id): (usize, String)| {
            if let Some(choice) = choices_state.read().list.get(index) {
                let mut new_goto = choice.goto.clone();
                if !new_goto.contains(&paragraph_id) {
                    new_goto.push(paragraph_id);
                    let joined = new_goto.join(",");
                    (dispatch.clone())(CAct::SetField { idx: index, field: "goto", value: joined });
                }
            }
        }
    };

    let handle_choice_remove_paragraph = {
        let dispatch = dispatch_choice.clone();
        let choices_state = choices_state.clone();
        move |(index, paragraph_id): (usize, String)| {
            if let Some(choice) = choices_state.read().list.get(index) {
                let new_goto: Vec<String> = choice.goto.iter().cloned().filter(|id| id != &paragraph_id).collect();
                let joined = new_goto.join(",");
                (dispatch.clone())(CAct::SetField { idx: index, field: "goto", value: joined });
            }
        }
    };

    let handle_add_choice = {
        let dispatch = dispatch_choice.clone();
        move || {
            // reducer add
            (dispatch.clone())(CAct::Add);
        }
    };

    let handle_remove_choice = {
        let dispatch = dispatch_choice.clone();
        move |index: usize| {
            // reducer remove
            (dispatch.clone())(CAct::Remove(index));
        }
    };

    // Handle chapter selector toggle
    let handle_chapter_toggle = {
        let dispatch = dispatch_choice.clone();
        move |index: usize| {
            (dispatch.clone())(CAct::ToggleChapter(index));
        }
    };

    // Handle chapter search
    let handle_chapter_search = {
        let dispatch = dispatch_choice.clone();
        move |(index, query): (usize, String)| {
            (dispatch.clone())(CAct::SetChapterSearch { idx: index, query });
        }
    };

    // Handle paragraph selector toggle
    let handle_paragraph_toggle = {
        let dispatch = dispatch_choice.clone();
        move |index: usize| {
            (dispatch.clone())(CAct::TogglePara(index));
        }
    };

    // Handle paragraph search
    let handle_paragraph_search = {
        let dispatch = dispatch_choice.clone();
        move |(index, query): (usize, String)| {
            (dispatch.clone())(CAct::SetParaSearch { idx: index, query });
        }
    };

    // Handle timeout chapter selector toggle
    let handle_timeout_chapter_toggle = {
        let dispatch = dispatch_choice.clone();
        move |index: usize| {
            (dispatch.clone())(CAct::ToggleTimeoutChapter(index));
        }
    };

    // Handle timeout chapter search
    let handle_timeout_chapter_search = {
        let dispatch = dispatch_choice.clone();
        move |(index, query): (usize, String)| {
            (dispatch.clone())(CAct::SetTimeoutChapterSearch { idx: index, query });
        }
    };

    // Handle timeout paragraph selector toggle
    let handle_timeout_paragraph_toggle = {
        let dispatch = dispatch_choice.clone();
        move |index: usize| {
            (dispatch.clone())(CAct::ToggleTimeoutPara(index));
        }
    };

    // Handle timeout paragraph search
    let handle_timeout_paragraph_search = {
        let dispatch = dispatch_choice.clone();
        move |(index, query): (usize, String)| {
            (dispatch.clone())(CAct::SetTimeoutParaSearch { idx: index, query });
        }
    };

    let dispatch_reset = dispatch_choice.clone();
    let mut reset_choices = move || {
        choices.write().clear();
        (dispatch_reset.clone())(CAct::SetList(Vec::new()));
    };

    let toast = use_toast();

    let mut handle_submit = {
        let paragraph_state = paragraph_state.clone();
        let selected_paragraph = selected_paragraph.clone();
        let paragraph_language = paragraph_language.clone();
        let paragraphs = paragraphs.read().clone();
        let choices = choices.read().clone();
        let is_edit_mode = is_edit_mode.read().clone();
        let mut is_submitting = is_submitting.clone();
        let toast = toast.clone();
        move |_| {
            if *is_submitting.read() { return; }
            is_submitting.set(true);
            let text = ContextText {
                lang: paragraph_language.read().clone(),
                paragraphs: paragraphs.clone(),
                choices: choices.iter().map(|(choice_text, _, _, _, _, _, _, _, _, _)| {
                    choice_text.clone()
                }).collect(),
            };

            // Build option data
            let paragraph_choices: Vec<ContextParagraphChoice> = choices.iter().map(|(choice_text, to_list, type_, key, value, _target_chapter, same_page, time_limit, timeout_to, _timeout_target_chapter)| {
                let mut complex = ContextParagraphChoice::Complex {
                    to: to_list.clone(),
                    type_: type_.clone(),
                    key: None,
                    value: None,
                    same_page: Some(*same_page),
                    time_limit: *time_limit,
                    timeout_to: timeout_to.clone(),
                };
                if let Some(k) = key {
                    if !k.is_empty() {
                        if let ContextParagraphChoice::Complex { key, .. } = &mut complex {
                            *key = Some(k.to_string());
                        }
                    }
                }
                if let Some(v) = value {
                    if let ContextParagraphChoice::Complex { value, .. } = &mut complex {
                        *value = Some(v.clone());
                    }
                }
                complex
            }).collect();

            // Pre-fetch translation strings inside component scope where `I18n` context is available
            let submit_success_text = t!("submit_success").to_string();
            let submit_failed_text  = t!("submit_failed").to_string();

            spawn_local({
                let mut is_submitting = is_submitting.clone();
                let mut paragraph_state = paragraph_state.clone();
                let mut toast = toast.clone();
                let submit_success_text = submit_success_text.clone();
                let submit_failed_text  = submit_failed_text.clone();
                async move {
                    let client = reqwest::Client::new();
                    
                    // Build new paragraph data
                    let chapter_id = selected_chapter.read().clone();
                    
                    // Build new paragraph data
                    let new_paragraph = if chapter_id.is_empty() {
                        serde_json::json!({
                            "texts": if is_edit_mode {
                                // In edit mode, keep all existing translations, only update translations for current language
                                let mut existing_texts = selected_paragraph.read().as_ref().map(|p| p.texts.clone()).unwrap_or_default();
                                // Remove old translations for current language (if exists)
                                existing_texts.retain(|t| t.lang != *paragraph_language.read());
                                // Add new translations
                                existing_texts.push(text);
                                existing_texts
                            } else {
                                vec![text]
                            },
                            "choices": paragraph_choices
                        })
                    } else {
                        serde_json::json!({
                            "chapter_id": chapter_id,
                            "texts": if is_edit_mode {
                                // In edit mode, keep all existing translations, only update translations for current language
                                let mut existing_texts = selected_paragraph.read().as_ref().map(|p| p.texts.clone()).unwrap_or_default();
                                // Remove old translations for current language (if exists)
                                existing_texts.retain(|t| t.lang != *paragraph_language.read());
                                // Add new translations
                                existing_texts.push(text);
                                existing_texts
                            } else {
                                vec![text]
                            },
                            "choices": paragraph_choices
                        })
                    };
                    
                    // Publish to paragraph collection
                    let paragraphs_url = format!("{}{}", BASE_API_URL, PARAGRAPHS);
                    
                    let response = if is_edit_mode {
                        // Edit mode: Use PATCH method to update existing paragraph
                        if let Some(paragraph) = selected_paragraph.read().as_ref() {
                            let update_url = format!("{}{}/{}", BASE_API_URL, PARAGRAPHS, paragraph.id);
                            client.patch(&update_url)
                                .json(&new_paragraph)
                                .send()
                                .await
                        } else {
                            return;
                        }
                    } else {
                        // New mode: Use POST method to create new paragraph
                        client.post(&paragraphs_url)
                            .json(&new_paragraph)
                            .send()
                            .await
                    };

                    match response {
                        Ok(response) => {
                            let status = response.status();
                            if status.is_success() {
                                // Reload paragraph data
                                let paragraphs_url = format!("{}{}", BASE_API_URL, PARAGRAPHS);
                                match client.get(&paragraphs_url)
                                    .send()
                                    .await {
                                    Ok(response) => {
                                        if response.status().is_success() {
                                            match response.json::<Data>().await {
                                                Ok(data) => {
                                                    // Update paragraph data in context
                                                    paragraph_state.write().set_paragraphs(data.items.clone());
                                                    toast.write().show(submit_success_text.clone(), ToastType::Success, 3000);
                                                    is_submitting.set(false);
                                                },
                                                Err(e) => {
                                                    toast.write().show(format!("{}: {}", submit_failed_text.clone(), e), ToastType::Error, 3000);
                                                    is_submitting.set(false);
                                                }
                                            }
                                        } else {
                                            toast.write().show(format!("{}: {}", submit_failed_text.clone(), response.status()), ToastType::Error, 3000);
                                            is_submitting.set(false);
                                        }
                                    },
                                    Err(e) => {
                                        toast.write().show(format!("{}: {}", submit_failed_text.clone(), e), ToastType::Error, 3000);
                                        is_submitting.set(false);
                                    }
                                }
                            } else {
                                toast.write().show(format!("{}: {}", submit_failed_text.clone(), status), ToastType::Error, 3000);
                                is_submitting.set(false);
                            }
                        },
                        Err(e) => {
                            toast.write().show(format!("{}: {}", submit_failed_text.clone(), e), ToastType::Error, 3000);
                            is_submitting.set(false);
                        }
                    }
                }
            });
        }
    };

    let handle_action_type_toggle = {
        let dispatch = dispatch_choice.clone();
        move |index: usize| {
            (dispatch.clone())(CAct::ToggleActionType(index));
        }
    };

    // Restore handle_paragraph_select (used by paragraph picker)
    let mut handle_paragraph_select = {
        let mut selected_paragraph = selected_paragraph.clone();
        let paragraph_state = paragraph_state.clone();
        let paragraph_language = paragraph_language.clone();
        let mut paragraphs = paragraphs.clone();
        let mut choices = choices.clone();
        let available_paragraphs = available_paragraphs.clone();
        let language_state = language_state.clone();
        // Clone dispatch_choice ahead of time to avoid moving the original Rc into the closure
        let dispatch_ps = dispatch_choice.clone();
        move |index: usize| {
            let available_paragraphs = available_paragraphs.read();
            if let Some(paragraph) = available_paragraphs.get(index) {
                // Get full paragraph data from context
                if let Some(full_paragraph) = paragraph_state.read().get_by_id(&paragraph.id) {
                    selected_paragraph.set(Some(full_paragraph.clone()));

                    // Fill paragraph content
                    if let Some(text) = full_paragraph.texts.iter().find(|t| t.lang == *paragraph_language.read()) {
                        paragraphs.set(text.paragraphs.clone());

                        // Fill options
                        let (new_choices, _new_paragraphs) = process_paragraph_select(
                            text,
                            &full_paragraph,
                            &paragraph_state,
                            &paragraph_language,
                            &language_state.read().current_language.clone(),
                        );
                        let _choices_len = new_choices.len();

                        choices.set(new_choices.clone());

                        // sync to reducer
                        let dispatch = dispatch_ps.clone();
                        let converted = new_choices.iter().cloned().map(ChoiceStruct::from_tuple).collect::<Vec<_>>();
                        (dispatch.clone())(CAct::SetList(converted));

                        // Set paragraph cache for each option
                        for (idx, list) in _new_paragraphs.into_iter().enumerate() {
                            (dispatch.clone())(CAct::SetParaList { idx, list });
                        }

                        // === New: set timeout paragraph cache as well ===
                        for (idx, choice) in new_choices.iter().enumerate() {
                            let timeout_target_chapter_id = &choice.9;
                            if !timeout_target_chapter_id.is_empty() {
                                let selected_lang = paragraph_language.read().clone();
                                let interface_lang = language_state.read().current_language.clone();

                                let timeout_paragraphs_list = paragraph_state.read().get_by_chapter(timeout_target_chapter_id)
                                    .iter()
                                    .map(|item| {
                                        let has_translation = item.texts.iter().any(|t| t.lang == selected_lang);
                                        let preview = item.texts
                                            .iter()
                                            .find(|t| t.lang == selected_lang)
                                            .or_else(|| item.texts.iter().find(|t| t.lang == interface_lang))
                                            .or_else(|| item.texts.iter().find(|t| t.lang == "en-US" || t.lang == "en-GB"))
                                            .or_else(|| item.texts.first())
                                            .map(|t| t.paragraphs.lines().next().unwrap_or("").to_string())
                                            .unwrap_or_default();
                                        crate::components::paragraph_list::Paragraph { id: item.id.clone(), preview, has_translation }
                                    })
                                    .collect::<Vec<_>>();

                                (dispatch.clone())(CAct::SetTimeoutParaList { idx, list: timeout_paragraphs_list });
                            }
                        }
                    } else {
                        // If no translation found for current language, clear all content
                        paragraphs.set(String::new());
                        choices.set(Vec::new());
                        let dispatch = dispatch_ps.clone();
                        (dispatch.clone())(CAct::SetList(Vec::new()));
                    }
                }
            }
        }
    };

    // Keep local `choices` Signal in sync with reducer (read-only sync)
    {
        let choices = choices.clone();
        let choices_state = choices_state.clone();
        use_effect(move || {
            let list = choices_state.read().list.clone();
            let tuple_list = list.iter().map(ChoiceStruct::to_tuple).collect::<Vec<_>>();
            if tuple_list != *choices.read() {
                // 同樣排程至下一 tick 再 set，避免同步 render 期間修改 hook list
                let mut choices_ref = choices.clone();
                Timeout::new(0, move || {
                    choices_ref.set(tuple_list);
                }).forget();
            }
            // No cleanup necessary
        });
    }

    // Add: Listen for interface language change, reload paragraph content in edit mode
    let dispatch_effect = dispatch_choice.clone();
    use_effect(move || {
        let _interface_lang = language_state.read().current_language.clone();
        let is_edit = *is_edit_mode.read();

        if is_edit {
            if let Some(paragraph) = selected_paragraph.read().clone() {
                let current_lang = paragraph_language.read().clone();

                let mut paragraphs_ref = paragraphs.clone();
                let mut choices_ref = choices.clone();
                let paragraph_state_ref = paragraph_state.clone();
                let paragraph_lang_ref = paragraph_language.clone();
                let language_state_ref = language_state.clone();
                let dispatch_ref = dispatch_effect.clone();

                Timeout::new(0, move || {
                    if let Some(text) = paragraph.texts.iter().find(|t| t.lang == current_lang) {
                        paragraphs_ref.set(text.paragraphs.clone());

                        let interface_lang = language_state_ref.read().current_language.clone();
                        let (new_choices, _new_paragraphs) = process_paragraph_select(text, &paragraph, &paragraph_state_ref, &paragraph_lang_ref, &interface_lang);
                        choices_ref.set(new_choices.clone());

                        let converted = new_choices.iter().cloned().map(ChoiceStruct::from_tuple).collect::<Vec<_>>();
                        (dispatch_ref.clone())(CAct::SetList(converted));
                    } else {
                        paragraphs_ref.set(String::new());

                        let current_choices = choices_ref.read().clone();
                        let new_choices = current_choices.iter().map(|(_, to, type_, key, value, target_chapter, same_page, time_limit, timeout_to, timeout_target_chapter)| {
                            (String::new(), to.clone(), type_.clone(), key.clone(), value.clone(), target_chapter.clone(), *same_page, *time_limit, timeout_to.clone(), timeout_target_chapter.clone())
                        }).collect();
                        choices_ref.set(new_choices);
                    }
                }).forget();
            }
        }

        // No cleanup necessary
    });

    rsx! {
        crate::pages::layout::Layout { 
            title: Some("Dashboard"),
            div { 
                class: "min-h-screen bg-gray-50 dark:bg-gray-900",
                div {
                    class: "w-full max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 sm:py-6 lg:py-8",
                    // Main content area
                    div { 
                        class: "bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700",
                        // Form area
                        div {
                            class: "p-4 sm:p-6 lg:p-8",
                            // Language and chapter selector area
                            div {
                                class: "flex flex-col lg:flex-row lg:items-end gap-4 lg:gap-6 mb-6",
                                // Selector grid area
                                div {
                                    class: if *is_edit_mode.read() {
                                        "grid grid-cols-1 lg:grid-cols-3 gap-4 lg:gap-6 flex-1"
                                    } else {
                                        "grid grid-cols-1 lg:grid-cols-2 gap-4 lg:gap-6 flex-1"
                                    },
                                    // Language selector
                                    div {
                                        class: "w-full",
                                        Dropdown {
                                            label: t!("select_language"),
                                            value: current_language.read().to_string(),
                                            options: filtered_languages.read().clone(),
                                            is_open: *is_open.read(),
                                            search_query: search_query.read().to_string(),
                                            on_toggle: move |_| {
                                                let current = *is_open.read();
                                                is_open.set(!current);
                                            },
                                            on_search: move |query| search_query.set(query),
                                            on_select: {
                                                let dispatch_dropdown = dispatch_choice.clone();
                                                move |lang: &Language| {
                                                    let current_lang = lang.code.to_string();
                                                    paragraph_language.set(current_lang.clone());
                                                    is_open.set(false);
                                                    search_query.set(String::new());
                                                    
                                                    // Check if there is already existing translation, use exact match
                                                    if let Some(paragraph) = selected_paragraph.read().as_ref() {
                                                        // Fill paragraph content
                                                        if let Some(text) = paragraph.texts.iter().find(|text| text.lang == current_lang) {
                                                            paragraphs.set(text.paragraphs.clone());
                                                            
                                                            // Fill options
                                                            let (new_choices, _new_paragraphs) = process_paragraph_select(text, paragraph, &paragraph_state, &paragraph_language, &language_state.read().current_language.clone());
                                                            let _choices_len = new_choices.len();

                                                            choices.set(new_choices.clone());
                                                        } else {
                                                            // If no translation found for current language, clear all content
                                                            paragraphs.set(String::new());
                                                            choices.set(Vec::new());
                                                            let dispatch = dispatch_dropdown.clone();
                                                            (dispatch.clone())(CAct::SetList(Vec::new()));
                                                        }
                                                    }
                                                }
                                            },
                                            display_fn: display_language,
                                            has_error: false,
                                            search_placeholder: Box::leak(t!("search_language").into_boxed_str()),
                                            button_class: None,
                                            label_class: None,
                                        }
                                    }

                                    // Chapter selector
                                    div {
                                        class: "w-full",
                                        ChapterSelector {
                                            key: format!("chapter-dropdown-{}", paragraph_language.read()),
                                            label: Box::leak(t!("select_chapter").into_boxed_str()),
                                            value: selected_chapter.read().clone(),
                                            chapters: chapter_state.read().chapters.clone(),
                                            is_open: *is_chapter_open.read(),
                                            search_query: chapter_search_query.read().to_string(),
                                            on_toggle: move |_| {
                                                let current = *is_chapter_open.read();
                                                is_chapter_open.set(!current);
                                            },
                                            on_search: move |query| chapter_search_query.set(query),
                                            on_select: {
                                                move |chapter: Chapter| {
                                                    selected_chapter.set(chapter.id.clone());
                                                    is_chapter_open.set(false);
                                                    chapter_search_query.set(String::new());
                                                    validate_field(&chapter.id, &mut chapter_error);
                                                }
                                            },
                                            has_error: *chapter_error.read(),
                                            selected_language: paragraph_language.read().clone(),
                                        }
                                    }

                                    // Paragraph selector (only show in edit mode)
                                    if *is_edit_mode.read() {
                                        div { 
                                            class: "w-full",
                                            crate::components::paragraph_list::ParagraphList {
                                                label: t!("select_paragraph"),
                                                value: selected_paragraph.read().as_ref().map(|p| p.id.clone()).unwrap_or(t!("select_paragraph").to_string()),
                                                paragraphs: available_paragraphs.read().clone(),
                                                is_open: *is_paragraph_open.read(),
                                                search_query: paragraph_search_query.read().to_string(),
                                                on_toggle: move |_| {
                                                    if *is_edit_mode.read() {
                                                        let current = *is_paragraph_open.read();
                                                        is_paragraph_open.set(!current);
                                                    }
                                                },
                                                on_search: move |query| {
                                                    if *is_edit_mode.read() {
                                                        paragraph_search_query.set(query);
                                                    }
                                                },
                                                on_select: EventHandler::new(move |id: String| {
                                                    if *is_edit_mode.read() {
                                                        // Find selected paragraph index
                                                        let available_paragraphs = available_paragraphs.read();
                                                        if let Some(index) = available_paragraphs.iter().position(|p| p.id == id) {
                                                            handle_paragraph_select(index);
                                                        }
                                                        paragraph_search_query.set(String::new());
                                                    }
                                                }),
                                                has_error: false,
                                                disabled: !*is_edit_mode.read(),
                                                selected_language: paragraph_language.read().clone(),
                                            }
                                        }
                                    }
                                }

                                // Edit mode control button (right)
                                if !selected_chapter.read().is_empty() {
                                    div {
                                        class: "flex-shrink-0",
                                        button {
                                            class: "w-full lg:w-auto h-10 px-4 inline-flex items-center justify-center rounded-lg text-white bg-blue-600 hover:bg-blue-700 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-500 dark:hover:bg-blue-600 dark:focus:ring-blue-800 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:bg-blue-600 dark:disabled:hover:bg-blue-500",
                                            onclick: move |_| {
                                                let current_mode = *is_edit_mode.read();
                                                is_edit_mode.set(!current_mode);
                                                if current_mode {
                                                    // Exit edit mode and clear all fields
                                                    paragraphs.set(String::new());
                                                    reset_choices();
                                                    selected_paragraph.set(None);
                                                }
                                            },
                                            disabled: selected_chapter.read().is_empty(),
                                            svg { 
                                                xmlns: "http://www.w3.org/2000/svg",
                                                class: "h-5 w-5 mr-2",
                                                fill: "none",
                                                view_box: "0 0 24 24",
                                                stroke: "currentColor",
                                                stroke_width: "2",
                                                path { 
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                    d: if *is_edit_mode.read() {
                                                        "M12 4v16m8-8H4"
                                                    } else {
                                                        "M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"
                                                    }
                                                }
                                            }
                                            if *is_edit_mode.read() {
                                                {t!("new_paragraph")}
                                            } else {
                                                {t!("edit_mode")}
                                            }
                                        }
                                    }
                                }
                            }

                            // Paragraph selection and content editing area
                            if !selected_chapter.read().is_empty() {
                                div {
                                    class: "space-y-6",
                                    // Paragraph content area (add title)
                                    div {
                                        class: "w-full",
                                        // Paragraph content title
                                        h3 {
                                            class: "text-lg font-semibold text-gray-900 dark:text-gray-100 mb-4",
                                            {t!("paragraph_content")}
                                        }
                                        TextareaField {
                                            label: Box::leak("".to_string().into_boxed_str()), // Remove label, because title already exists
                                            placeholder: Box::leak(t!("paragraph_content").into_boxed_str()),
                                            value: paragraphs.read().to_string(),
                                            required: true,
                                            has_error: *paragraphs_error.read(),
                                            rows: 8,
                                            auto_resize: Some(true),
                                            on_input: move |event: FormEvent| {
                                                let value = event.value().clone();
                                                paragraphs.set(value.clone());
                                                validate_field(&value, &mut paragraphs_error);
                                            },
                                            on_blur: move |_| validate_field(&paragraphs.read(), &mut paragraphs_error)
                                        }
                                    }

                                    // Option area (keep existing title)
                                    div {
                                        class: "w-full mt-6 pt-6 border-t border-gray-200 dark:border-gray-700",
                                        ChoiceOptions {
                                            choices: choices.read().clone(),
                                            on_choice_change: handle_choice_change,
                                            on_choice_add_paragraph: handle_choice_add_paragraph,
                                            on_choice_remove_paragraph: handle_choice_remove_paragraph,
                                            on_add_choice: handle_add_choice,
                                            on_remove_choice: handle_remove_choice,
                                            available_chapters: {
                                                let mut chapters = chapter_state.read().chapters.clone();
                                                // Add N/A option at the beginning
                                                chapters.insert(0, Chapter {
                                                    id: String::new(),
                                                    titles: vec![crate::contexts::chapter_context::ChapterTitle {
                                                        lang: paragraph_language.read().clone(),
                                                        title: t!("not_applicable"),
                                                    }],
                                                    order: -1,
                                                });
                                                chapters
                                            },
                                            selected_language: paragraph_language.read().clone(),
                                            choice_chapters_open: choices_state.read().chapter_open.clone(),
                                            choice_chapters_search: choices_state.read().chapter_search.clone(),
                                            choice_paragraphs_open: choices_state.read().para_open.clone(),
                                            choice_paragraphs_search: choices_state.read().para_search.clone(),
                                            choice_paragraphs: choices_state.read().para_cache.clone(),
                                            on_chapter_toggle: handle_chapter_toggle,
                                            on_chapter_search: handle_chapter_search,
                                            on_paragraph_toggle: handle_paragraph_toggle,
                                            on_paragraph_search: handle_paragraph_search,
                                            timeout_chapter_open: choices_state.read().timeout_chapter_open.clone(),
                                            timeout_chapter_search: choices_state.read().timeout_chapter_search.clone(),
                                            timeout_paragraphs_open: choices_state.read().timeout_para_open.clone(),
                                            timeout_paragraphs_search: choices_state.read().timeout_para_search.clone(),
                                            timeout_paragraphs: choices_state.read().timeout_para_cache.clone(),
                                            on_timeout_chapter_toggle: handle_timeout_chapter_toggle,
                                            on_timeout_chapter_search: handle_timeout_chapter_search,
                                            on_timeout_paragraph_toggle: handle_timeout_paragraph_toggle,
                                            on_timeout_paragraph_search: handle_timeout_paragraph_search,
                                            action_type_open: choices_state.read().action_type_open.clone(),
                                            on_action_type_toggle: handle_action_type_toggle,
                                        }
                                    }
                                }
                            }
                        }

                        // Submit button area
                        if !selected_chapter.read().is_empty() {
                            div {
                                class: "px-4 sm:px-6 lg:px-8 py-4 bg-gray-50 dark:bg-gray-700/50 border-t border-gray-200 dark:border-gray-700",
                                div {
                                    class: "max-w-md mx-auto",
                                    button {
                                        class: "w-full inline-flex justify-center items-center px-6 py-3 bg-green-600 text-white rounded-lg hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 transition-transform duration-200 will-change-transform disabled:opacity-50 disabled:cursor-not-allowed font-medium text-lg shadow-sm",
                                        disabled: {
                                            let edit_mode = *is_edit_mode.read();
                                            let selected_para = selected_paragraph.read().is_none();
                                            let has_changes = *has_changes.read();
                                            let is_valid = *is_form_valid.read();
                                            let submitting = *is_submitting.read();
                                            (edit_mode && selected_para) || !has_changes || !is_valid || submitting
                                        },
                                        onclick: move |_| {
                                            handle_submit(());
                                        },
                                        {t!("submit")}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn process_paragraph_select(
    text: &ContextText,
    full_paragraph: &ContextParagraph,
    paragraph_state: &Signal<ParagraphState>,
    paragraph_language: &Signal<String>,
    interface_language: &str,
) -> (
    Vec<(String, Vec<String>, String, Option<String>, Option<serde_json::Value>, String, bool, Option<u32>, Option<String>, String)>,
    Vec<Vec<ParagraphListParagraph>>,
) {
    let mut new_choices = Vec::new();
    let mut new_paragraphs = Vec::new();
    let text_choices = &text.choices;
    let paragraph_choices = &full_paragraph.choices;
    for (i, choice_text) in text_choices.iter().enumerate() {
        let (target_ids, type_, key, value, same_page, time_limit, timeout_to_opt) = if let Some(choice) = paragraph_choices.get(i) {
            match choice {
                ContextParagraphChoice::Simple(texts) => (texts.clone(), "goto".to_string(), None, None, false, None, None),
                ContextParagraphChoice::SimpleOld(text) => (vec![text.clone()], "goto".to_string(), None, None, false, None, None),
                ContextParagraphChoice::Complex { to, type_, key, value, same_page, time_limit, timeout_to, .. } => {
                    (to.clone(), type_.clone(), key.clone(), value.clone(), same_page.unwrap_or(false), *time_limit, timeout_to.clone())
                },
                ContextParagraphChoice::ComplexOld { to, type_, key, value, same_page, time_limit, timeout_to, .. } => {
                    (vec![to.clone()], type_.clone(), key.clone(), value.clone(), same_page.unwrap_or(false), *time_limit, timeout_to.clone())
                },
            }
        } else {
            (Vec::new(), String::new(), None, None, false, None, None)
        };
        
        // Get all target paragraph chapter IDs (should be in the same chapter)
        let target_chapter_id = if !target_ids.is_empty() {
            if let Some(first_paragraph) = paragraph_state.read().get_by_id(&target_ids[0]) {
                first_paragraph.chapter_id
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        
        // Determine timeout target chapter (if any)
        let timeout_target_chapter_id = if let Some(ref timeout_para_id) = timeout_to_opt {
            if let Some(timeout_para) = paragraph_state.read().get_by_id(timeout_para_id) {
                timeout_para.chapter_id.clone()
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        
        new_choices.push((
            choice_text.clone(),
            if target_chapter_id.is_empty() { Vec::new() } else { target_ids.clone() },
            if target_chapter_id.is_empty() { String::new() } else { type_ },
            if target_chapter_id.is_empty() { None } else { key },
            if target_chapter_id.is_empty() { None } else { value },
            target_chapter_id.clone(),
            same_page,
            time_limit,
            timeout_to_opt.clone(),
            timeout_target_chapter_id.clone(),
        ));
        
        if !target_chapter_id.is_empty() {
            let selected_lang = paragraph_language.read().clone();
            let filtered_paragraphs = paragraph_state.read().get_by_chapter(&target_chapter_id)
                .iter()
                .map(|item| {
                    let has_translation = item.texts.iter().any(|text| text.lang == selected_lang);
                    let preview = item.texts
                        .iter()
                        .find(|t| t.lang == selected_lang)
                        .or_else(|| item.texts.iter().find(|t| t.lang == interface_language))
                        .or_else(|| item.texts.iter().find(|t| t.lang == "en-US" || t.lang == "en-GB"))
                        .or_else(|| item.texts.first())
                        .map(|text| {
                            match text.paragraphs.lines().next() {
                                Some(line) => line.to_string(),
                                None => String::new(),
                            }
                        })
                        .unwrap_or_else(|| format!("[{}]", item.id));
                    ParagraphListParagraph {
                        id: item.id.clone(),
                        preview,
                        has_translation,
                    }
                })
                .collect::<Vec<_>>();
            new_paragraphs.push(filtered_paragraphs);
        } else {
            new_paragraphs.push(Vec::new());
        }
    }
    (new_choices, new_paragraphs)
}