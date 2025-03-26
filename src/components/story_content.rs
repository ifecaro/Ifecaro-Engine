use dioxus::prelude::*;
use crate::enums::translations::Translations;
use serde::{Serialize, Deserialize};

#[derive(Props, Clone, PartialEq)]
pub struct StoryContentProps {
    paragraph: String,
    choices: Vec<Choice>,
    on_choice_click: EventHandler<String>,
    t: Translations,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Choice {
    pub caption: String,
    pub goto: String,
}

#[component]
pub fn StoryContent(props: StoryContentProps) -> Element {
    rsx! {
        article {
            class: "prose dark:prose-invert lg:prose-xl indent-10 mx-auto",
            div {
                class: "whitespace-pre-line",
                p { class: "mb-6", {props.paragraph} }
            }
            ol {
                class: "mt-10 w-fit",
                {props.choices.iter().map(|choice| {
                    let caption = choice.caption.clone();
                    let goto = choice.goto.clone();
                    rsx! {
                        li { 
                            class: "opacity-30",
                            button {
                                class: "text-left hover:opacity-100 transition-opacity duration-200",
                                onclick: move |_| {
                                    props.on_choice_click.call(goto.clone());
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