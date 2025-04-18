use dioxus::prelude::*;
use crate::components::dropdown::Dropdown;
use crate::pages::dashboard::Chapter;
use crate::contexts::language_context::LanguageState;
use dioxus::hooks::use_context;
use std::cell::RefCell;
use std::thread_local;

thread_local! {
    static SELECTED_LANGUAGE: RefCell<String> = RefCell::new(String::new());
}

#[derive(Props, Clone, PartialEq)]
pub struct ChapterSelectorProps {
    pub label: String,
    pub value: String,
    pub chapters: Vec<Chapter>,
    pub is_open: bool,
    pub search_query: String,
    pub on_toggle: EventHandler<()>,
    pub on_search: EventHandler<String>,
    pub on_select: EventHandler<Chapter>,
    #[props(default = false)]
    pub has_error: bool,
    #[props(default = String::new())]
    pub class: String,
    #[props(default = String::new())]
    pub selected_language: String,
}

#[component]
pub fn ChapterSelector(props: ChapterSelectorProps) -> Element {
    // 使用傳入的語言參數
    let selected_lang = if props.selected_language.is_empty() {
        // 如果沒有傳入語言，則從 context 中獲取
        let language_state = use_context::<Signal<LanguageState>>();
        let current_lang = language_state.read().current_language.clone();
        current_lang
    } else {
        props.selected_language.clone()
    };
    
    // 更新 thread_local 變量
    SELECTED_LANGUAGE.with(|lang| {
        *lang.borrow_mut() = selected_lang.clone();
    });
    
    // 過濾章節
    let filtered_chapters = props.chapters.iter()
        .filter(|chapter| {
            let query = props.search_query.to_lowercase();
            chapter.titles.iter()
                .find(|t| t.lang == selected_lang)
                .map(|t| t.title.to_lowercase().contains(&query))
                .unwrap_or(false)
        })
        .cloned()
        .collect::<Vec<_>>();
    
    // 找到當前選中的章節
    let selected_chapter_title = if props.value.is_empty() {
        props.label.clone()
    } else {
        props.chapters.iter()
            .find(|c| c.id == props.value)
            .map(|c| {
                c.titles.iter()
                    .find(|t| t.lang == selected_lang)
                    .or_else(|| c.titles.first())
                    .map(|t| t.title.clone())
                    .unwrap_or_default()
            })
            .unwrap_or_else(|| props.label.clone())
    };

    // 定義顯示函數，使用 thread_local 變量獲取當前選擇的語言
    fn display_chapter_title(chapter: &Chapter) -> String {
        let selected_lang = SELECTED_LANGUAGE.with(|lang| lang.borrow().clone());
        chapter.titles.iter()
            .find(|t| t.lang == selected_lang)
            .or_else(|| chapter.titles.first())
            .map(|t| t.title.clone())
            .unwrap_or_default()
    }

    rsx! {
        Dropdown {
            label: props.label,
            value: selected_chapter_title,
            options: filtered_chapters,
            is_open: props.is_open,
            search_query: props.search_query,
            on_toggle: props.on_toggle,
            on_search: props.on_search,
            on_select: props.on_select,
            display_fn: display_chapter_title,
            has_error: props.has_error,
            class: props.class,
            search_placeholder: "搜尋章節...",
            button_class: "",
            dropdown_class: "",
            search_input_class: "",
            option_class: "",
        }
    }
} 