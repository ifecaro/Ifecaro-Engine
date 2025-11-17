use crate::components::dropdown::Dropdown;
use crate::contexts::chapter_context::Chapter;
use crate::contexts::language_context::LanguageState;
use dioxus::hooks::use_context;
use dioxus::prelude::*;
use dioxus_i18n::t;
use std::cell::RefCell;
use std::thread_local;

thread_local! {
    static SELECTED_LANGUAGE: RefCell<String> = RefCell::new(String::new());
    static INTERFACE_LANGUAGE: RefCell<String> = RefCell::new(String::new());
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
    let language_state = use_context::<Signal<LanguageState>>();

    // Use passed language parameter or get from context
    let selected_lang = if props.selected_language.is_empty() {
        language_state.read().current_language.clone()
    } else {
        props.selected_language.clone()
    };

    // Get interface language as fallback option
    let interface_lang = language_state.read().current_language.clone();

    // Update thread_local variable
    SELECTED_LANGUAGE.with(|lang| {
        *lang.borrow_mut() = selected_lang.clone();
    });

    // Update interface language to thread_local
    INTERFACE_LANGUAGE.with(|lang| {
        *lang.borrow_mut() = interface_lang.clone();
    });

    // Filter chapters
    let filtered_chapters = props
        .chapters
        .iter()
        .filter(|chapter| {
            let query = props.search_query.to_lowercase();
            if query.is_empty() {
                return true;
            }

            let title = chapter
                .titles
                .iter()
                .find(|t| t.lang == selected_lang)
                .or_else(|| chapter.titles.iter().find(|t| t.lang == interface_lang))
                .or_else(|| {
                    chapter
                        .titles
                        .iter()
                        .find(|t| t.lang == "en-US" || t.lang == "en-GB")
                })
                .or_else(|| chapter.titles.first());

            if let Some(title) = title {
                title.title.to_lowercase().contains(&query)
            } else {
                false
            }
        })
        .cloned()
        .collect::<Vec<_>>();

    // Find currently selected chapter
    let selected_chapter_title = if props.value.is_empty() {
        props.label.clone()
    } else {
        props
            .chapters
            .iter()
            .find(|c| c.id == props.value)
            .map(|c| {
                // First look for translation in current language
                if let Some(title) = c.titles.iter().find(|t| t.lang == selected_lang) {
                    title.title.clone()
                } else {
                    // If current language translation not found, prioritize interface language, then English or first available translation, and add untranslated marker
                    let fallback_title = c
                        .titles
                        .iter()
                        .find(|t| t.lang == interface_lang)
                        .or_else(|| {
                            c.titles
                                .iter()
                                .find(|t| t.lang == "en-US" || t.lang == "en-GB")
                        })
                        .or_else(|| c.titles.first())
                        .map(|t| t.title.clone())
                        .unwrap_or_default();
                    format!("（{}）{}", t!("untranslated"), fallback_title)
                }
            })
            .unwrap_or_else(|| props.label.clone())
    };

    // Define display function, use thread_local variable to get current selected language
    fn display_chapter_title(chapter: &Chapter) -> String {
        let selected_lang = SELECTED_LANGUAGE.with(|lang| lang.borrow().clone());
        let interface_lang = INTERFACE_LANGUAGE.with(|lang| lang.borrow().clone());

        // First look for translation in current language
        if let Some(title) = chapter.titles.iter().find(|t| t.lang == selected_lang) {
            title.title.clone()
        } else {
            // If current language translation not found, prioritize interface language, then English or first available translation, and add untranslated marker
            let fallback_title = chapter
                .titles
                .iter()
                .find(|t| t.lang == interface_lang)
                .or_else(|| {
                    chapter
                        .titles
                        .iter()
                        .find(|t| t.lang == "en-US" || t.lang == "en-GB")
                })
                .or_else(|| chapter.titles.first())
                .map(|t| t.title.clone())
                .unwrap_or_default();
            format!("（{}）{}", dioxus_i18n::t!("untranslated"), fallback_title)
        }
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
            search_placeholder: t!("search_chapter"),
            button_class: None,
            label_class: None,
            dropdown_class: "",
            search_input_class: "",
            option_class: "",
        }
    }
}
