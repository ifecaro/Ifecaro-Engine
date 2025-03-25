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

#[derive(Deserialize, Clone, Debug, PartialEq)]
struct Choice {
    caption: String,
    goto: String,
}

#[derive(Props, PartialEq, Clone)]
pub struct StoryProps {
    pub lang: String,
}

#[component]
pub fn Story(props: StoryProps) -> Element {
    tracing::info!("Story component rendering with lang: {}", props.lang);
    
    let data = use_signal(|| Data {
        totalItems: 0,
        items: vec![],
    });
    let mut selected_paragraph_index: Signal<usize> = use_signal(|| 0);
    let t = Translations::get(&props.lang);

    let text_found = use_memo(move || {
        (*data.read())
            .items
            .iter()
            .find(|item| item.index == *selected_paragraph_index.read())
            .and_then(|item| item.texts.iter().find(|text| text.lang == props.lang).cloned())
    });
    let paragraph = use_memo(move || {
        text_found
            .read()
            .as_ref()
            .map(|text| text.paragraphs.clone())
    });

    {
        let mut data = data.clone();

        use_future(move || async move {
            let url = format!("{}{}", BASE_API_URL, SETTINGS);
            let resp = reqwest::get(&url)
                .await?
                .json::<Data>()
                .await
                .inspect_err(|err| {
                    tracing::error!("{}", err);
                })
                .and_then(|data2| {
                    data.set(data2.clone());
                    return Ok(data2);
                });

            return resp;
        });
    }

    rsx! {
        crate::pages::layout::Layout { 
            if data.read().totalItems > 0 {
                div {
                    class: "h-[calc(100%_-_48px)]",
                    tabindex: "0",
                    onkeydown: move |e| {
                        let data = data.clone();
                        let text_found = text_found.clone();
                        let mut selected_paragraph_index = selected_paragraph_index.clone();
            
                        if let dioxus::events::Key::Character(key_char) = e.key() {
                            if let Some(digit) = key_char.chars().next().and_then(|c| c.to_digit(10)) {
                                let option_index = digit as usize - 1;
                                text_found
                                    .read()
                                    .as_ref()
                                    .and_then(|text| {
                                        if option_index < text.choices.len() {
                                            Some(text.choices[option_index].clone())
                                        } else {
                                            None
                                        }
                                    })
                                    .and_then(move |choice| {
                                        let index = (*data.read())
                                            .items
                                            .iter()
                                            .position(|item| item.choice_id == choice.goto);
            
                                        if let Some(idx) = index {
                                            selected_paragraph_index.set(idx);
                                        }
                                        Some(())
                                    });
                            }
                        }
                    },
                    {
                        text_found.read().clone().map(|text_found| {
                            rsx! {
                                article {
                                    class: "prose dark:prose-invert lg:prose-xl indent-10 mx-auto",
                                    div {
                                        class: "whitespace-pre-line",
                                        {
                                            paragraph.read()
                                                .as_ref()
                                                .unwrap_or(&"".to_string())
                                                .split("\n")
                                                .map(|p| rsx! {
                                                    p { class: "mb-6", { p } }
                                                })
                                        }
                                    }
                                    ol {
                                        class: "mt-10 w-fit",
                                        {text_found.choices.iter().map(|choice| {
                                            let index = (*data.read())
                                                .items
                                                .iter()
                                                .position(|item| item.choice_id == choice.goto);
        
                                            return rsx! {
                                                li {
                                                    class: if index.is_some() {"cursor-pointer"} else {"opacity-30"},
                                                    onclick: move |_| if index.is_some() {
                                                        selected_paragraph_index.set(index.unwrap());
                                                    },
                                                    {choice.caption.clone()}
                                                }
                                            }
                                        })}
                                    }
                                }
                            }
                        }).unwrap_or_else(|| {
                            rsx! {
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
                        })
                    }
                }
            } else {
                div { 
                    class: "container mx-auto px-4 pt-16",
                    div { 
                        class: "max-w-2xl mx-auto",
                        p { 
                            class: "text-lg leading-relaxed text-center text-gray-500 dark:text-gray-400", 
                            "{t.coming_soon}" 
                        }
                    }
                }
            }
        }
    }
}