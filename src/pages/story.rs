
use crate::{constants::config::config::{BASE_API_URL, SETTINGS}, Language};
use dioxus::{
    hooks::{ use_callback, use_effect, use_future, use_memo, use_shared_state, use_state, UseState }, html::GlobalAttributes, prelude::{dioxus_elements, fc_to_builder, rsx, Element, IntoDynNode, Scope }
};
use dioxus_markdown::Markdown;
use serde::Deserialize;
use web_sys::window;
use wasm_bindgen::{ JsCast, prelude::Closure };
use regex::Regex;
// use futures::future::join_all;

#[allow(non_snake_case)]
#[derive(Deserialize, Clone,Debug)]
struct Data {
    // page: i32,
    // perPage: i32,
    totalItems: i32,
    // totalPages: i32,
    items: Vec<Paragraph>,
}

#[derive(Deserialize, Clone,Debug)]
struct Paragraph {
    index: usize,
    choice_id: String,
    texts: Vec<Text>,
    // actions: Vec<Action>,
}

#[derive(Deserialize, Clone,Debug)]
struct Action {
    // action: String,
    // name: String,
    // method: String,
    // key: String,
    // value: bool,
}

#[derive(Deserialize, Clone,Debug,PartialEq)]
struct Text {
    lang: String,
    paragraphs: String,
    choices: Vec<Choice>,
}

#[derive(Deserialize, Clone,Debug,PartialEq)]
struct Choice {
    caption: String,
    goto: String,
}

#[allow(non_snake_case)]
pub fn Story(cx: Scope) -> Element {
    let data = use_state(cx, || Data {
        // page: 0,
        // perPage: 0,
        totalItems: 0,
        // totalPages: 0,
        items: vec![],
    });
    let selected_paragraph_index: &UseState<usize> = use_state(cx, || 0);
    let lang = use_shared_state::<Language>(cx).unwrap();
    let text_found =  use_memo(cx, (data, selected_paragraph_index, lang), |_| (*data).items.iter()
            .find(|item| item.index == **selected_paragraph_index)
            .and_then(|item| item.texts.iter().find(|text| text.lang == lang.read().0).cloned()));
    let paragraph = use_memo(cx, text_found, |_| text_found.as_ref().and_then(|text| Some(text.paragraphs.clone())));


    let callback: &UseState<Option<Closure<dyn Fn(web_sys::KeyboardEvent)>>> = use_state(cx, || None);

    {
        let data = data.clone();

        use_future(cx, (), |()| async move {
            let url = format!("{}{}", BASE_API_URL, SETTINGS);
            let resp = reqwest::get(&url)
                .await?
                .json::<Data>()
                .await
                .inspect_err(|err| {
                    log::error!("{}", err);
                })
                .and_then(|data2| {
                    data.set(data2.clone());
                    return Ok(data2);
                });

            return resp;
        });
    }


    {




        {
            let callback = callback.clone();
            let data = data.clone();
            let text_found = text_found.clone();
            let selected_paragraph_index = selected_paragraph_index.clone();
            
            use_effect(cx, &text_found.clone(), |_| async move {
                window().and_then(|win| {
                    let callback_temp = {
                        let callback = callback.clone();

                        Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
                            let _ = callback.clone();
                            let data = data.clone();
                            let text_found = text_found.clone();
                            let selected_paragraph_index = selected_paragraph_index.clone();

                            let key = e.key();
                            let key_str = key.as_str();
                            let re = Regex::new(r"[1-9]").unwrap();
                            if re.is_match(key_str) {
                                key_str.parse::<usize>().and_then(|option_index| {
                                    let option_index = option_index - 1;
                                    text_found
                                        .and_then(|text| if option_index < text.choices.len() {Some(text.choices[option_index].clone())} else {None})
                                        .and_then(move |choice| {
                                            let index = (*data)
                                                .items
                                                .iter()
                                                .position(|item| item.choice_id == choice.goto);

                                            if index.is_some() {selected_paragraph_index.set(index.unwrap())};
                                            Some(())
                                        }
                                    );
                                    Ok(())
                                }).err();
                                
                            };
                        })
                    };
                    win.add_event_listener_with_callback(
                        "keydown",
                        callback_temp.as_ref().unchecked_ref()
                    ).unwrap();
                    callback.set(Some(callback_temp));

                    // callback_temp.forget();

                    // std::mem::forget(callback);s
                    Some(())
                });
            });
        }
    }

    {
            let callback = callback.clone();
            let selected_paragraph_index = selected_paragraph_index.clone();
            use_effect(cx, &selected_paragraph_index.clone(), |_| async move {
                if *selected_paragraph_index > 0 {
                    window().and_then(|win| {
                        (*callback).as_ref().and_then(|cb| {
                            win.remove_event_listener_with_callback(
                                "keydown",
                                (*cb).as_ref().unchecked_ref()
                            ).unwrap();
                            Some(())
                        })
                    });
                }
            })
                            
    }


    cx.render(rsx! {
        crate::pages::layout::Layout { 
            if data.totalItems > 0 {
                rsx!{
                    div {
                        {
                            text_found.clone().and_then(|text_found| {
                                Some(
                                    rsx!{
                                        article {
                                            class: "prose dark:prose-invert lg:prose-xl indent-10",

                                                    Markdown {
                                                        content: &paragraph.as_ref().unwrap(),
                                                    }
                                            ol {
                                                class: "mt-10 w-fit",
                                                {text_found.choices.iter().map(|choice| {
                                                    let index = (*data)
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
    })
}
