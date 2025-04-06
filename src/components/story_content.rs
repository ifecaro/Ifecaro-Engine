use dioxus::prelude::*;
use crate::enums::translations::Translations;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Props, Clone, PartialEq)]
pub struct StoryContentProps {
    pub paragraph: String,
    pub choices: Vec<Choice>,
    pub on_choice_click: EventHandler<String>,
    pub t: Translations,
    pub enabled_choices: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    pub caption: String,
    pub action: Action,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    pub to: String,
}

#[component]
pub fn StoryContent(props: StoryContentProps) -> Element {
    let paragraph = props.paragraph.clone();
    let choices = props.choices.clone();
    let enabled_choices = props.enabled_choices.clone();
    let on_choice_click = props.on_choice_click.clone();
    let _t = props.t.clone();
    
    rsx! {
        article {
            class: "prose dark:prose-invert lg:prose-xl mx-auto",
            div {
                class: "whitespace-pre-wrap mt-16 space-y-8",
                {paragraph.split('\n').map(|p| {
                    if p.trim().is_empty() {
                        rsx! { br {} }
                    } else {
                        rsx! {
                            p { 
                                class: "indent-10",
                                {p}
                            }
                        }
                    }
                })}
            }
            ol {
                class: "mt-10 w-fit",
                {choices.iter().map(|choice| {
                    let caption = choice.caption.clone();
                    let goto = choice.action.to.clone();
                    let is_enabled = enabled_choices.contains(&goto);
                    rsx! {
                        li {
                            class: if is_enabled { "cursor-pointer hover:text-blue-500" } else { "opacity-30 cursor-not-allowed" },
                            onclick: move |_| {
                                if is_enabled {
                                    on_choice_click.call(goto.clone());
                                }
                            },
                            { caption }
                        }
                    }
                })}
            }
        }
    }
} 