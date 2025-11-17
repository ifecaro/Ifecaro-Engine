use crate::{
    components::paragraph_list::Paragraph as ParagraphListParagraph, models::impacts::Impact,
};
use dioxus::prelude::*;
use gloo_timers::callback::Timeout;
use serde_json::Value;
use std::rc::Rc;

/// 單一選項的資料結構（對應原本 tuple）
#[derive(Clone, PartialEq)]
pub struct Choice {
    pub caption: String,
    pub goto: Vec<String>,
    pub action_type: String,
    pub action_key: Option<String>,
    pub action_value: Option<Value>,
    pub target_chapter: String,
    pub same_page: bool,
    pub time_limit: Option<u32>,
    pub timeout_to: Option<String>,
    pub timeout_target_chapter: String,
    pub impacts: Vec<Impact>,
}

impl Default for Choice {
    fn default() -> Self {
        Self {
            caption: String::new(),
            goto: Vec::new(),
            action_type: String::new(),
            action_key: None,
            action_value: None,
            target_chapter: String::new(),
            same_page: false,
            time_limit: None,
            timeout_to: None,
            timeout_target_chapter: String::new(),
            impacts: Vec::new(),
        }
    }
}

impl Choice {
    /// Convert Choice into the tuple format originally used in Dashboard
    pub fn to_tuple(
        &self,
    ) -> (
        String,
        Vec<String>,
        String,
        Option<String>,
        Option<Value>,
        String,
        bool,
        Option<u32>,
        Option<String>,
        String,
        Vec<Impact>,
    ) {
        (
            self.caption.clone(),
            self.goto.clone(),
            self.action_type.clone(),
            self.action_key.clone(),
            self.action_value.clone(),
            self.target_chapter.clone(),
            self.same_page,
            self.time_limit,
            self.timeout_to.clone(),
            self.timeout_target_chapter.clone(),
            self.impacts.clone(),
        )
    }

    /// Build Choice from the original tuple format
    pub fn from_tuple(
        tup: (
            String,
            Vec<String>,
            String,
            Option<String>,
            Option<Value>,
            String,
            bool,
            Option<u32>,
            Option<String>,
            String,
            Vec<Impact>,
        ),
    ) -> Self {
        Self {
            caption: tup.0,
            goto: tup.1,
            action_type: tup.2,
            action_key: tup.3,
            action_value: tup.4,
            target_chapter: tup.5,
            same_page: tup.6,
            time_limit: tup.7,
            timeout_to: tup.8,
            timeout_target_chapter: tup.9,
            impacts: tup.10,
        }
    }
}

/// 所有與 choice 相關的 UI 狀態
#[derive(Clone, Default)]
pub struct ChoicesState {
    pub list: Vec<Choice>,
    pub action_type_open: Vec<bool>,
    pub chapter_open: Vec<bool>,
    pub chapter_search: Vec<String>,
    pub para_open: Vec<bool>,
    pub para_search: Vec<String>,
    pub para_cache: Vec<Vec<ParagraphListParagraph>>, // 每個選項對應可挑選段落清單
    pub timeout_chapter_open: Vec<bool>,
    pub timeout_chapter_search: Vec<String>,
    pub timeout_para_open: Vec<bool>,
    pub timeout_para_search: Vec<String>,
    pub timeout_para_cache: Vec<Vec<ParagraphListParagraph>>, // 每個選項 timeout 用的段落清單
}

/// reducer 的所有可能動作
pub enum Action {
    Add,
    Remove(usize),
    // 修改 Choice 欄位
    SetField {
        idx: usize,
        field: &'static str,
        value: String,
    },
    SetEffects {
        idx: usize,
        impacts: Vec<Impact>,
    },
    ToggleActionType(usize),
    ToggleChapter(usize),
    TogglePara(usize),
    SetChapterSearch {
        idx: usize,
        query: String,
    },
    SetParaSearch {
        idx: usize,
        query: String,
    },
    SetParaList {
        idx: usize,
        list: Vec<ParagraphListParagraph>,
    },
    ToggleTimeoutChapter(usize),
    ToggleTimeoutPara(usize),
    SetTimeoutChapterSearch {
        idx: usize,
        query: String,
    },
    SetTimeoutParaSearch {
        idx: usize,
        query: String,
    },
    SetTimeoutParaList {
        idx: usize,
        list: Vec<ParagraphListParagraph>,
    },
    SetList(Vec<Choice>),
}

/// Custom reducer-like hook: centralize all choice related state.
///
/// NOTE: We deliberately avoid calling another hook **inside** a hook initializer to
/// comply with Dioxus' hook-usage rules. Using `use_signal` directly at this level
/// guarantees that we're still in the primary hook execution sequence.
pub fn use_choices() -> (Signal<ChoicesState>, Rc<dyn Fn(Action)>) {
    // Centralised `ChoicesState` signal – stable across renders.
    let state: Signal<ChoicesState> = use_signal(|| ChoicesState::default());

    // Dispatcher: mutate the state signal in-place. We wrap it in an `Rc` so
    // that it can be freely cloned and moved into callbacks.
    let dispatch: Rc<dyn Fn(Action)> = {
        let state_signal = state.clone();
        Rc::new(move |action: Action| {
            // Clone current state (read-only borrow), modify the clone, then set back – avoids nested mutable borrows.
            let mut st = state_signal.read().clone();

            match action {
                Action::Add => {
                    st.list.push(Choice::default());
                    st.action_type_open.push(false);
                    st.chapter_open.push(false);
                    st.chapter_search.push(String::new());
                    st.para_open.push(false);
                    st.para_search.push(String::new());
                    st.para_cache.push(Vec::new());
                    st.timeout_chapter_open.push(false);
                    st.timeout_chapter_search.push(String::new());
                    st.timeout_para_open.push(false);
                    st.timeout_para_search.push(String::new());
                    st.timeout_para_cache.push(Vec::new());
                }
                Action::Remove(i) => {
                    fn remove_idx<T>(v: &mut Vec<T>, idx: usize) {
                        if idx < v.len() {
                            v.remove(idx);
                        }
                    }

                    remove_idx(&mut st.list, i);
                    remove_idx(&mut st.action_type_open, i);
                    remove_idx(&mut st.chapter_open, i);
                    remove_idx(&mut st.chapter_search, i);
                    remove_idx(&mut st.para_open, i);
                    remove_idx(&mut st.para_search, i);
                    remove_idx(&mut st.para_cache, i);
                    remove_idx(&mut st.timeout_chapter_open, i);
                    remove_idx(&mut st.timeout_chapter_search, i);
                    remove_idx(&mut st.timeout_para_open, i);
                    remove_idx(&mut st.timeout_para_search, i);
                    remove_idx(&mut st.timeout_para_cache, i);
                }
                Action::SetField { idx, field, value } => {
                    if let Some(choice) = st.list.get_mut(idx) {
                        match field {
                            "caption" => choice.caption = value,
                            "goto" => {
                                choice.goto =
                                    value.split(',').map(|s| s.trim().to_string()).collect();
                            }
                            "action_type" => choice.action_type = value,
                            "action_key" => choice.action_key = Some(value),
                            "action_value" => choice.action_value = Some(Value::String(value)),
                            "target_chapter" => choice.target_chapter = value,
                            "same_page" => choice.same_page = value == "true",
                            "time_limit" => choice.time_limit = value.parse().ok(),
                            "timeout_to" => {
                                if value.trim().is_empty() {
                                    choice.timeout_to = None;
                                } else {
                                    choice.timeout_to = Some(value);
                                }
                            }
                            "timeout_target_chapter" => choice.timeout_target_chapter = value,
                            _ => {}
                        }
                    }
                }
                Action::SetEffects { idx, impacts } => {
                    if let Some(choice) = st.list.get_mut(idx) {
                        choice.impacts = impacts;
                    }
                }

                Action::ToggleActionType(i) => {
                    if let Some(v) = st.action_type_open.get_mut(i) {
                        *v = !*v;
                    }
                }
                Action::ToggleChapter(i) => {
                    if let Some(v) = st.chapter_open.get_mut(i) {
                        *v = !*v;
                    }
                }
                Action::TogglePara(i) => {
                    if let Some(v) = st.para_open.get_mut(i) {
                        *v = !*v;
                    }
                }
                Action::SetChapterSearch { idx, query } => {
                    if let Some(v) = st.chapter_search.get_mut(idx) {
                        *v = query;
                    }
                }
                Action::SetParaSearch { idx, query } => {
                    if let Some(v) = st.para_search.get_mut(idx) {
                        *v = query;
                    }
                }
                Action::SetParaList { idx, list } => {
                    if idx < st.para_cache.len() {
                        st.para_cache[idx] = list;
                    }
                }
                Action::ToggleTimeoutChapter(i) => {
                    if let Some(v) = st.timeout_chapter_open.get_mut(i) {
                        *v = !*v;
                    }
                }
                Action::ToggleTimeoutPara(i) => {
                    if let Some(v) = st.timeout_para_open.get_mut(i) {
                        *v = !*v;
                    }
                }
                Action::SetTimeoutChapterSearch { idx, query } => {
                    if let Some(v) = st.timeout_chapter_search.get_mut(idx) {
                        *v = query;
                    }
                }
                Action::SetTimeoutParaSearch { idx, query } => {
                    if let Some(v) = st.timeout_para_search.get_mut(idx) {
                        *v = query;
                    }
                }
                Action::SetTimeoutParaList { idx, list } => {
                    if idx < st.timeout_para_cache.len() {
                        st.timeout_para_cache[idx] = list;
                    }
                }
                Action::SetList(new_list) => {
                    // Replace the whole list and sync the auxiliary vectors' lengths.
                    st.list = new_list;
                    let len = st.list.len();

                    fn sync_vec_len<T: Default + Clone>(vec: &mut Vec<T>, len: usize) {
                        if vec.len() < len {
                            vec.extend(std::iter::repeat_with(T::default).take(len - vec.len()));
                        } else {
                            vec.truncate(len);
                        }
                    }

                    sync_vec_len(&mut st.action_type_open, len);
                    sync_vec_len(&mut st.chapter_open, len);
                    sync_vec_len(&mut st.chapter_search, len);
                    sync_vec_len(&mut st.para_open, len);
                    sync_vec_len(&mut st.para_search, len);
                    sync_vec_len(&mut st.para_cache, len);
                    sync_vec_len(&mut st.timeout_chapter_open, len);
                    sync_vec_len(&mut st.timeout_chapter_search, len);
                    sync_vec_len(&mut st.timeout_para_open, len);
                    sync_vec_len(&mut st.timeout_para_search, len);
                    sync_vec_len(&mut st.timeout_para_cache, len);
                }
            }

            // After mutation, write back. If currently borrowed, defer to next tick to avoid panic.
            if let Ok(mut w) = state_signal.clone().try_write() {
                *w = st;
            } else {
                let mut sig2 = state_signal.clone();
                Timeout::new(0, move || {
                    *sig2.write() = st;
                })
                .forget();
            }
        })
    };

    (state.clone(), dispatch)
}
