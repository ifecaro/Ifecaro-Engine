
use crate::{constants::config::config::{BASE_API_URL, SETTINGS}, Language};
use dioxus::{
    hooks::{ use_callback, use_future, use_shared_state, use_state, UseState }, html::GlobalAttributes, prelude::{dioxus_elements, fc_to_builder, rsx, Element, IntoDynNode, Scope }
};
use serde::Deserialize;
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

#[derive(Deserialize, Clone,Debug)]
struct Text {
    lang: String,
    paragraphs: Vec<String>,
    choices: Vec<Choice>,
}

#[derive(Deserialize, Clone,Debug)]
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
        crate::pages::layout::Layout { 
            if data.totalItems > 0 {
                {(*data).items.iter().find(|item| item.index == **selected_paragraph_index).and_then(|item| {
                    Some(
                        rsx!{
                            div {
                                {
                                    item.texts.iter().find(|text| text.lang == lang.read().0).and_then(|text_found| {
                                        Some(
                                            rsx!{
                                                {text_found.paragraphs.iter().map(|paragraph| 
                                                    rsx!{
                                                        div {
                                                            {paragraph}
                                                        }
                                                    }
                                                )},
                                                div {
                                                    class: "mt-8 ml-8 w-fit grid gap-y-4",
                                                    {text_found.choices.iter().enumerate().map(|(i,choice)| 
                                                        rsx!{
                                                            div {
                                                                class: "cursor-pointer",
                                                                onclick: |_| {
                                                                            (*data)
                                                                                .items
                                                                                .iter()
                                                                                .position(|item| item.choice_id == choice.goto)
                                                                                .and_then(
                                                                                    |index| {
                                                                                        selected_paragraph_index.set(index);
                                                                                        return Some(());
                                                                                    }
                                                                                ).expect("Paragraph not found.")
                                                                        },
                                                                {format!("{}. {}",(i + 1).to_string(),&choice.caption)}
                                                            }
                                                        }
                                                    )}
                                                }
                                            }
                                        )
                                    }).unwrap()
                                }
                            }
                        }
                    )
                })}
            }
        }
    })
}
