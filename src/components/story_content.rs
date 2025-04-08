use dioxus::prelude::*;
use crate::enums::translations::Translations;
use serde::{Serialize, Deserialize};
use serde_json;
use crate::KeyboardState;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::FocusEvent;
use crate::contexts::story_context::use_story_context;

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
    let mut keyboard_state = use_context::<Signal<KeyboardState>>();
    let mut story_context = use_story_context();
    
    let mut show_filter = use_signal(|| true);
    let mut is_focused = use_signal(|| false);
    let mut is_mobile = use_signal(|| false);
    
    {
        use_effect(move || {
            let window = web_sys::window().unwrap();
            let width = window.inner_width().unwrap().as_f64().unwrap();
            is_mobile.set(width < 768.0);
            (|| ())()
        });
    }
    
    {
        let mut show_filter = show_filter.clone();
        let mut is_focused = is_focused.clone();
        use_effect(move || {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            
            let focus_handler = Closure::wrap(Box::new(move |_: FocusEvent| {
                is_focused.set(true);
                show_filter.set(false);
            }) as Box<dyn FnMut(FocusEvent)>);
            
            let blur_handler = Closure::wrap(Box::new(move |_: FocusEvent| {
                is_focused.set(false);
                show_filter.set(true);
            }) as Box<dyn FnMut(FocusEvent)>);
            
            document.add_event_listener_with_callback("focusin", focus_handler.as_ref().unchecked_ref()).unwrap();
            document.add_event_listener_with_callback("focusout", blur_handler.as_ref().unchecked_ref()).unwrap();
            
            focus_handler.forget();
            blur_handler.forget();
            
            (|| ())()
        });
    }
    
    {
        let choices = choices.clone();
        let enabled_choices = enabled_choices.clone();
        let on_choice_click = on_choice_click.clone();
        let story_context = story_context.clone();
        use_effect(move || {
            keyboard_state.write().selected_index = 0;
            keyboard_state.write().choices = choices.clone();
            keyboard_state.write().enabled_choices = enabled_choices.clone();
            keyboard_state.write().on_choice_click = Some(on_choice_click);
            
            if let Some(target_id) = story_context.read().target_paragraph_id.clone() {
                if let Some(_choice) = choices.iter().find(|c| c.action.to == target_id) {
                    let idx = choices.iter().position(|c| c.action.to == target_id).unwrap();
                    keyboard_state.write().selected_index = idx as i32;
                }
            }
            
            (|| ())()
        });
    }
    
    rsx! {
        div {
            class: "relative",
            div {
                class: {
                    let mut classes = vec!["fixed inset-0 bg-black bg-opacity-50 z-10 flex items-center justify-center transition-opacity duration-500"];
                    if !*show_filter.read() || *is_mobile.read() {
                        classes.push("opacity-0 pointer-events-none");
                    } else {
                        classes.push("opacity-100");
                    }
                    classes.join(" ")
                },
                button {
                    class: "bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline",
                    onclick: move |_| {
                        show_filter.set(false);
                        is_focused.set(true);
                        if let Some(window) = web_sys::window() {
                            if let Some(document) = window.document() {
                                if let Some(main) = document.query_selector("main").unwrap() {
                                    let _ = main.unchecked_into::<web_sys::HtmlElement>().focus();
                                }
                            }
                        }
                    },
                    { if *is_focused.read() { "點擊這裡繼續" } else { "點擊這裡開始" } }
                }
            }
            article {
                class: "prose dark:prose-invert lg:prose-xl mx-auto",
                onkeydown: move |event: Event<KeyboardData>| {
                    match event.data.key() {
                        Key::Enter => {
                            let idx = keyboard_state.read().selected_index as usize;
                            if idx < choices.len() {
                                let choice = &choices[idx];
                                let goto = choice.action.to.clone();
                                if enabled_choices.contains(&goto) {
                                    on_choice_click.call(goto.clone());
                                    story_context.write().target_paragraph_id = Some(goto.clone());
                                }
                            }
                            event.stop_propagation();
                        }
                        _ => {}
                    }
                },
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
                    class: "mt-10 w-fit list-decimal",
                    {choices.iter().enumerate().map(|(index, choice)| {
                        let caption = choice.caption.clone();
                        let goto = choice.action.to.clone();
                        let is_enabled = enabled_choices.contains(&goto);
                        let is_selected = keyboard_state.read().selected_index == index as i32;
                        rsx! {
                            li {
                                class: {
                                    let mut classes = vec![];
                                    if is_enabled {
                                        classes.push("cursor-pointer hover:text-blue-500");
                                    } else {
                                        classes.push("opacity-30 cursor-not-allowed");
                                    }
                                    if is_selected {
                                        classes.push("text-blue-500 font-bold");
                                    }
                                    classes.join(" ")
                                },
                                onclick: move |_| {
                                    if is_enabled {
                                        keyboard_state.write().selected_index = index as i32;
                                        on_choice_click.call(goto.clone());
                                        story_context.write().target_paragraph_id = Some(goto.clone());
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