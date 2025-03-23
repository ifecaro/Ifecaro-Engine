use crate::constants::config::config::{BASE_API_URL, SETTINGS};
use dioxus::{
    dioxus_core,
    hooks::{use_context, use_future, use_memo, use_signal},
    prelude::{component, dioxus_elements, fc_to_builder, rsx, Element, IntoDynNode, GlobalSignal, Readable},
    signals::{Signal, Writable},
};
// use dioxus_markdown::Markdown;
use serde::Deserialize;

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

#[component]
pub fn Story() -> Element {
    let data = use_signal(|| Data {
        // page: 0,
        // perPage: 0,
        totalItems: 0,
        // totalPages: 0,
        items: vec![],
    });
    let mut selected_paragraph_index: Signal<usize> = use_signal(|| 0);
    let lang = use_context::<Signal<&str>>();
    tracing::info!("{}", lang);

    let text_found = use_memo(move || {
        (*data.read())
            .items
            .iter()
            .find(|item| item.index == *selected_paragraph_index.read())
            .and_then(|item| item.texts.iter().find(|text| text.lang == lang()).cloned())
    });
    let paragraph = use_memo(move || {
        text_found
            .read()
            .as_ref()
            .and_then(|text| Some(text.paragraphs.clone()))
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

    // {
    //     {
    //         // let mut callback = callback.clone();
    //         let data = data.clone();
    //         let text_found = text_found.clone();
    //         let selected_paragraph_index = selected_paragraph_index.clone();
    //         let callback =
    //             Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move );

    //         window().and_then(|win| {
    //             win.add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref())
    //                 .unwrap();
    //             // callback.set(Some(callback_temp));

    //             callback.forget();

    //             // std::mem::forget(callback);s
    //             Some(())
    //         });
    //     }
    // }

    // {
    //     let callback = callback.clone();
    //     let selected_paragraph_index = selected_paragraph_index.clone();
    //     use_future(move || async move {
    //         if *selected_paragraph_index.read() > 0 {
    //             window().and_then(|win| {
    //                 (*callback.read()).as_ref().and_then(|cb| {
    //                     win.remove_event_listener_with_callback(
    //                         "keydown",
    //                         (*cb).as_ref().unchecked_ref(),
    //                     )
    //                     .unwrap();
    //                     Some(())
    //                 })
    //             });
    //         }
    //     });
    // }

    rsx! {
        crate::pages::layout::Layout { 
            if data.read().totalItems > 0 {
                { rsx!{
                    div {
                        class: "h-[calc(100%_-_48px)]",
                        tabindex: "0",
                        onkeydown: move |e| {
                            // let _ = callback.clone();
                            let data = data.clone();
                            let text_found = text_found.clone();
                            let mut selected_paragraph_index = selected_paragraph_index.clone();
                
                            // let key = e.key();
                            // let key_str = key.as_str();
                            // let re = Regex::new(r"[1-9]").unwrap();
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
                            text_found.read().clone().and_then(|text_found| {
                                Some(
                                    rsx!{
                                        article {
                                            class: "prose dark:prose-invert lg:prose-xl indent-10",
                                            div {
                                                class: "whitespace-pre-line",
                                                {
                                                    paragraph.read()
                                                        .as_ref()
                                                        .unwrap()
                                                        .split("\n")
                                                        .map(|p| rsx! {
                                                            p { class: "mb-6", { p } }
                                                        })
                                                }
                                            }
                                                    // Markdown {
                                                    //     content: &paragraph.as_ref().unwrap(),
                                                    // }
                                            ol {
                                                class: "mt-10 w-fit",
                                                {text_found.choices.iter().map(|choice| {
                                                    let index = (*data.read())
                                                        .items
                                                        .iter()
                                                        .position(|item| item.choice_id == choice.goto);
                
                                                    return rsx!{
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
                                )
                            }).unwrap()
                        }
                
                    }
                }
                }
            }
        }
    }
}
