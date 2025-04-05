use dioxus::prelude::*;
use crate::enums::translations::Translations;
use serde::{Serialize, Deserialize};

#[derive(Props, Clone, PartialEq)]
pub struct StoryContentProps {
    paragraph: String,
    choices: Vec<Choice>,
    on_choice_click: EventHandler<String>,
    t: Translations,
    enabled_choices: Vec<String>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Choice {
    pub caption: String,
    pub goto: String,
}

#[component]
pub fn StoryContent(props: StoryContentProps) -> Element {
    let paragraphs: Vec<String> = props.paragraph.split('\n').map(|s| s.to_string()).collect();
    
    let handle_choice_click = move |goto: String| {
        // println!("Clicked choice with goto: {}", goto);
        if let Some(on_choice_click) = props.on_choice_click {
            on_choice_click.call(goto);
        }
    };
    
    rsx! {
        article {
            class: "prose dark:prose-invert lg:prose-xl indent-10 mx-auto",
            div {
                class: "whitespace-pre-wrap",
                {paragraphs.iter().map(|p| {
                    rsx! {
                        p { class: "mb-6", {p.clone()} }
                    }
                })}
            }
            ol {
                class: "mt-10 w-fit",
                {props.choices.iter().map(|choice| {
                    let caption = choice.caption.clone();
                    let goto = choice.goto.clone();
                    let is_enabled = props.enabled_choices.contains(&goto);
                    rsx! {
                        li { 
                            class: if is_enabled { "" } else { "opacity-30" },
                            button {
                                class: if is_enabled {
                                    "text-left cursor-pointer"
                                } else {
                                    "text-left cursor-not-allowed"
                                },
                                disabled: !is_enabled,
                                onclick: move |_| {
                                    if is_enabled {
                                        handle_choice_click(goto.clone());
                                    }
                                },
                                {caption}
                            }
                        }
                    }
                })}
            }
        }
    }
} 