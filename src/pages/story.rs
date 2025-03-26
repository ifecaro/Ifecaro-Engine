use crate::constants::config::config::{BASE_API_URL, SETTINGS};
use crate::enums::translations::Translations;
use dioxus::{
    dioxus_core,
    hooks::{use_context, use_future, use_memo, use_signal},
    prelude::{component, dioxus_elements, fc_to_builder, rsx, Element, IntoDynNode, GlobalSignal, Readable, Props},
    signals::{Signal, Writable},
};
// use dioxus_markdown::Markdown;
use serde::Deserialize;
use crate::enums::route::Route;
use crate::contexts::language_context::LanguageState;
use crate::components::story_content::{StoryContent, Choice};

#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Debug)]
struct Data {
    // page: i32,
    // perPage: i32,
    totalItems: i32,
    // totalPages: i32,
    items: Vec<Paragraph>,
}

#[derive(Deserialize, Clone, Debug)]
struct Paragraph {
    index: usize,
    choice_id: String,
    texts: Vec<Text>,
    // actions: Vec<Action>,
}

#[derive(Deserialize, Clone, Debug)]
struct Action {
    // action: String,
    // name: String,
    // method: String,
    // key: String,
    // value: bool,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
struct Text {
    lang: String,
    paragraphs: String,
    choices: Vec<Choice>,
}

#[derive(Props, PartialEq, Clone)]
pub struct StoryProps {
    pub lang: String,
}

#[component]
pub fn Story(props: StoryProps) -> Element {
    let data = use_signal(|| Data {
        totalItems: 0,
        items: vec![],
    });
    let mut selected_paragraph_index: Signal<usize> = use_signal(|| 0);
    let state = use_context::<Signal<LanguageState>>();
    let t = Translations::get(&state.read().current_language);

    let text_found = use_memo(move || {
        (*data.read())
            .items
            .iter()
            .find(|item| item.index == *selected_paragraph_index.read())
            .and_then(|item| item.texts.iter().find(|text| text.lang == state.read().current_language).cloned())
    });

    let paragraph = use_memo(move || {
        text_found
            .read()
            .as_ref()
            .map(|text| text.paragraphs.clone())
    });

    let enabled_choices = use_memo(move || {
        let mut enabled = Vec::new();
        let current_data = data.read();
        if let Some(data) = current_data.items.iter().find(|item| item.index == *selected_paragraph_index.read()) {
            if let Some(text) = data.texts.iter().find(|t| t.lang == state.read().current_language) {
                for choice in &text.choices {
                    if current_data.items.iter().any(|item| item.choice_id == choice.goto) {
                        enabled.push(choice.goto.clone());
                    }
                }
            }
        }
        enabled
    });

    {
        let mut data = data.clone();

        use_future(move || async move {
            let url = format!("{}{}", BASE_API_URL, SETTINGS);
            let resp = reqwest::get(&url)
                .await?
                .json::<Data>()
                .await
                .and_then(|data2| {
                    data.set(data2.clone());
                    return Ok(data2);
                });

            return resp;
        });
    }

    let text_found_clone = text_found.clone();
    let paragraph_clone = paragraph.clone();

    rsx! {
        div { 
            class: "container mx-auto px-4 pt-16",
            div { 
                class: "max-w-2xl mx-auto",
                if let Some(paragraph) = paragraph_clone.read().as_ref() {
                    if let Some(text) = text_found_clone.read().as_ref() {
                        StoryContent {
                            paragraph: paragraph.clone(),
                            choices: text.choices.clone(),
                            on_choice_click: move |goto: String| {
                                if let Some(item) = data.read().items.iter().find(|item| item.choice_id == goto) {
                                    selected_paragraph_index.set(item.index);
                                }
                            },
                            t: t.clone(),
                            enabled_choices: enabled_choices.read().clone()
                        }
                    }
                } else {
                    article {
                        class: "prose dark:prose-invert lg:prose-xl indent-10 mx-auto",
                        div {
                            class: "whitespace-pre-line",
                            p { class: "mb-6", { t.coming_soon } }
                        }
                        ol {
                            class: "mt-10 w-fit",
                            li { class: "opacity-30", { t.coming_soon } }
                        }
                    }
                }
            }
        }
    }
}