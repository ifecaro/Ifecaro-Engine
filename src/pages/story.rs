use std::borrow::Borrow;

use crate::constants::config::config::{BASE_API_URL, SETTINGS};
use dioxus::{
    hooks::{use_callback, use_effect, use_future, use_ref, use_state, UseEffectReturn},
    prelude::{dioxus_elements, fc_to_builder, rsx, Element, IntoDynNode, Scope},
};
use dioxus_std::i18n;
use serde::Deserialize;
// use futures::future::join_all;

#[allow(non_snake_case)]
#[derive(Deserialize, Clone)]
struct Data {
    page: i32,
    perPage: i32,
    totalItems: i32,
    totalPages: i32,
    items: Vec<Setting>,
}

#[derive(Deserialize, Clone)]
struct Setting {
    choice_id: String,
    texts: Vec<Text>,
    actions: Vec<Action>,
}

#[derive(Deserialize, Clone)]
struct Action {
    action: String,
    target: String,
    name: String,
    method: String,
    key: String,
    value: bool,
}

#[derive(Deserialize, Clone)]
struct Text {
    lang: String,
    paragraphs: Vec<String>,
    choices: Vec<Choice>,
}

#[derive(Deserialize, Clone)]
struct Choice {
    caption: String,
    goto: String,
}

#[allow(non_snake_case)]
pub fn Story(cx: Scope) -> Element {
    let data = use_state(cx, || Data {
        page: 0,
        perPage: 0,
        totalItems: 0,
        totalPages: 0,
        items: vec![],
    });
    let lang = use_ref(cx, || "zh-TW");

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

    cx.render(rsx! {
        crate::pages::layout::Layout { title: "Story",
            if data.totalItems > 0 {
                {(*data).items.iter().map(|item| {
                    rsx!{
                        div {
                            div {
                                div {
                                    {item.texts.iter().find(|text|  text.lang == "en-US" ).and_then(
                                        |text_found| {
                                            Some(
                                                text_found.paragraphs.iter().map(|paragraph| 
                                                    rsx!{
                                                        div {
                                                            {paragraph}
                                                        }
                                                    }
                                                )
                                            )
                                        })
                                    }.unwrap()
                                }
                            }
                        }
                    }

                })}
            }
        }
    })
}
