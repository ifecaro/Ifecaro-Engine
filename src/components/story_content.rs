use std::sync::Arc;
use dioxus::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json;
use crate::KeyboardState;
use wasm_bindgen::JsCast;
use web_sys::{Window, Document};
use crate::contexts::story_context::use_story_context;
use dioxus_i18n::t;

#[derive(Props, Clone, PartialEq)]
pub struct StoryContentProps {
    pub paragraph: String,
    pub choices: Vec<Choice>,
    pub on_choice_click: EventHandler<String>,
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

fn get_window_document() -> Option<(Window, Document)> {
    let window = web_sys::window()?;
    let document = window.document()?;
    Some((window, document))
}

#[component]
pub fn StoryContent(props: StoryContentProps) -> Element {
    let paragraph = Arc::new(props.paragraph.clone());
    let choices = Arc::new(props.choices.clone());
    let enabled_choices = Arc::new(props.enabled_choices.clone());
    let on_choice_click = props.on_choice_click.clone();
    let mut keyboard_state = use_context::<Signal<KeyboardState>>();
    let mut story_context = use_story_context();
    
    let mut show_filter = use_signal(|| true);
    let mut is_focused = use_signal(|| false);
    let mut is_mobile = use_signal(|| false);
    
    use_effect(move || show_filter.set(true));
    
    use_effect(move || {
        if let Some((window, _)) = get_window_document() {
            if let Ok(width) = window.inner_width() {
                if let Some(width) = width.as_f64() {
                    is_mobile.set(width < 768.0);
                }
            }
        }
    });
    
    let mut handle_choice_click = move |goto: String| {
        on_choice_click.call(goto.clone());
        story_context.write().target_paragraph_id = Some(goto);
    };
    
    rsx! {
        div {
            class: "relative story-content-container",
            tabindex: "0",
            onkeydown: move |event: Event<KeyboardData>| {
                match event.data.key() {
                    Key::Enter => {
                        let idx = keyboard_state.read().selected_index as usize;
                        if idx < choices.len() {
                            let choice = &choices[idx];
                            let goto = choice.action.to.clone();
                            if enabled_choices.contains(&goto) {
                                handle_choice_click(goto);
                            }
                        }
                        event.stop_propagation();
                    }
                    Key::ArrowUp => {
                        let current_index = keyboard_state.read().selected_index;
                        if current_index > 0 {
                            keyboard_state.write().selected_index = current_index - 1;
                        }
                        event.stop_propagation();
                    }
                    Key::ArrowDown => {
                        let current_index = keyboard_state.read().selected_index;
                        if (current_index as usize) < choices.len() - 1 {
                            keyboard_state.write().selected_index = current_index + 1;
                        }
                        event.stop_propagation();
                    }
                    key => {
                        // 處理數字鍵 1-9
                        if let Some(num) = key.to_string().parse::<usize>().ok() {
                            if num > 0 && num <= choices.len() {
                                let idx = num - 1;
                                let choice = &choices[idx];
                                let goto = choice.action.to.clone();
                                if enabled_choices.contains(&goto) {
                                    keyboard_state.write().selected_index = idx as i32;
                                    handle_choice_click(goto);
                                }
                            }
                            event.stop_propagation();
                        }
                    }
                }
            },
            div {
                class: {
                    format!("fixed inset-0 bg-[rgba(0,0,0,0.7)] backdrop-blur-sm z-10 flex items-center justify-center transition-opacity duration-500 cursor-pointer {}",
                        if !*show_filter.read() || *is_mobile.read() { "opacity-0 pointer-events-none" } else { "opacity-100" }
                    )
                },
                onclick: move |_| {
                    show_filter.set(false);
                    is_focused.set(true);
                    if let Some((_, document)) = get_window_document() {
                        if let Ok(Some(container)) = document.query_selector(".story-content-container") {
                            let _ = container.unchecked_into::<web_sys::HtmlElement>().focus();
                        }
                    }
                },
                div {
                    class: "text-white text-xl font-bold",
                    { if *is_focused.read() { t!("continue-reading") } else { t!("start-reading") } }
                }
            }
            article {
                class: "prose dark:prose-invert lg:prose-xl mx-auto",
                div {
                    class: "whitespace-pre-wrap lg:mt-16 space-y-8",
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
                    class: "mt-10 w-full md:w-fit list-decimal",
                    {choices.iter().enumerate().map(|(index, choice)| {
                        let caption = choice.caption.clone();
                        let goto = choice.action.to.clone();
                        let is_enabled = enabled_choices.contains(&goto);
                        let is_selected = keyboard_state.read().selected_index == index as i32;
                        rsx! {
                            li {
                                class: {
                                    format!("!ml-0 md:!ml-20 {} {}",
                                        if is_enabled { "cursor-pointer hover:text-blue-500" } else { "opacity-30 cursor-not-allowed" },
                                        if is_selected { "text-blue-500 font-bold" } else { "" }
                                    )
                                },
                                onclick: move |_| {
                                    if is_enabled {
                                        keyboard_state.write().selected_index = index as i32;
                                        handle_choice_click(goto.clone());
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
} 