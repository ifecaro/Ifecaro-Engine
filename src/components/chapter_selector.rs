use dioxus::prelude::*;
use crate::components::dropdown::Dropdown;
use crate::pages::dashboard::Chapter;
use crate::contexts::language_context::LanguageState;
use dioxus::hooks::use_context;

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
}

#[component]
pub fn ChapterSelector(props: ChapterSelectorProps) -> Element {
    let language_state = use_context::<Signal<LanguageState>>();
    let current_lang = language_state.read().current_language.clone();
    
    // 過濾章節
    let filtered_chapters = props.chapters.iter()
        .filter(|chapter| {
            let query = props.search_query.to_lowercase();
            chapter.titles.iter()
                .find(|t| t.lang == current_lang)
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
                    .find(|t| t.lang == current_lang)
                    .or_else(|| c.titles.first())
                    .map(|t| t.title.clone())
                    .unwrap_or_default()
            })
            .unwrap_or_else(|| props.label.clone())
    };

    // 定義顯示函數，不捕獲任何變量
    fn display_chapter_title(chapter: &Chapter) -> String {
        chapter.titles.first()
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