use std::sync::Arc;
use dioxus::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json;
use crate::KeyboardState;
use wasm_bindgen::JsCast;
use web_sys::{Window, Document};
use dioxus_i18n::t;
use crate::contexts::story_context::use_story_context;

#[derive(Props, Clone, PartialEq)]
pub struct StoryContentProps {
    pub paragraph: Signal<String>,
    pub choices: Vec<Choice>,
    pub on_choice_click: EventHandler<(String, usize)>,
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

impl From<crate::components::translation_form::ParagraphChoice> for Choice {
    fn from(choice: crate::components::translation_form::ParagraphChoice) -> Self {
        Self {
            caption: String::new(), // caption 現在由前端生成
            action: Action {
                type_: choice.get_type(),
                key: choice.get_key(),
                value: choice.get_value(),
                to: choice.get_to(),
            },
        }
    }
}

impl From<crate::models::story::Choice> for Choice {
    fn from(choice: crate::models::story::Choice) -> Self {
        Self {
            caption: choice.caption,
            action: Action {
                type_: choice.action.type_,
                key: choice.action.key,
                value: choice.action.value,
                to: choice.action.to,
            },
        }
    }
}

fn get_window_document() -> Option<(Window, Document)> {
    let window = web_sys::window()?;
    let document = window.document()?;
    Some((window, document))
}

#[component]
pub fn StoryContent(props: StoryContentProps) -> Element {
    let paragraph = props.paragraph.read().clone();
    let choices = Arc::new(props.choices.clone());
    let enabled_choices = Arc::new(props.enabled_choices.clone());
    let on_choice_click = props.on_choice_click.clone();
    let mut keyboard_state = use_context::<Signal<KeyboardState>>();
    let story_ctx = use_story_context();
    let target_paragraph_id = story_ctx.read().target_paragraph_id.clone();
    let countdowns = use_signal(|| vec![]);
    let max_times = use_signal(|| vec![]);
    let story_ctx = use_story_context();
    let target_paragraph_id = story_ctx.read().target_paragraph_id.clone();
    // 新增一個 signal 控制動畫啟動
    let progress_started = use_signal(|| vec![]);
    // 段落 id 變動時重設倒數
    {
        let mut countdowns = countdowns.clone();
        let mut max_times = max_times.clone();
        let mut progress_started = progress_started.clone();
        let story_ctx = story_ctx.clone();
        use_effect(move || {
            let time_limits = story_ctx.read().countdowns.read().clone();
            countdowns.set(time_limits.clone());
            max_times.set(time_limits.clone());
            // 初始化動畫啟動狀態
            progress_started.set(vec![false; time_limits.len()]);
            // 下一個 tick 啟動動畫
            gloo_timers::callback::Timeout::new(10, move || {
                let mut arr = progress_started.write();
                for v in arr.iter_mut() {
                    *v = true;
                }
            }).forget();
        });
    }
    
    let mut show_filter = use_signal(|| true);
    let mut is_focused = use_signal(|| false);
    let mut is_mobile = use_signal(|| false);
    
    let is_mobile_memo = use_memo(move || {
        if let Some((window, _)) = get_window_document() {
            if let Ok(width) = window.inner_width() {
                if let Some(width) = width.as_f64() {
                    return width < 768.0;
                }
            }
        }
        false
    });
    
    use_effect(move || {
        is_mobile.set(*is_mobile_memo.read());
    });
    
    use_effect(move || show_filter.set(true));
    
    rsx! {
        div {
            class: "relative story-content-container",
            tabindex: "0",
            onkeydown: move |event: Event<KeyboardData>| {
                match event.data.key() {
                    key => {
                        // 處理數字鍵 1-9
                        if let Some(num) = key.to_string().parse::<usize>().ok() {
                            if num > 0 && num <= choices.len() {
                                let idx = num - 1;
                                let choice = &choices[idx];
                                let goto = choice.action.to.clone();
                                if enabled_choices.contains(&choice.caption) {
                                    keyboard_state.write().selected_index = idx as i32;
                                    on_choice_click.call((goto.clone(), idx));
                                }
                            }
                            event.stop_propagation();
                        }
                    }
                }
            },
            div {
                class: {
                    format!("fixed inset-0 backdrop-blur-sm z-10 flex items-center justify-center transition-opacity duration-500 cursor-pointer {}",
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
                    {
                        if *is_focused.read() {
                            t!("continue_reading")
                        } else {
                            t!("start_reading")
                        }
                    }
                }
            }
            article {
                class: "prose-sm dark:prose-invert lg:prose-base mx-auto max-w-3xl p-8 text-gray-900 dark:text-white bg-white dark:bg-transparent",
                div {
                    class: "whitespace-pre-wrap lg:mt-16 space-y-8",
                    {props.paragraph.read().split('\n')
                        .filter(|p| !p.trim().is_empty())
                        .map(|p| rsx! {
                            p {
                                class: "indent-10 tracking-wide leading-relaxed text-justify",
                                {p}
                            }
                        })
                    }
                }
                ol {
                    class: "mt-10 w-full md:w-fit list-decimal",
                    {choices.iter().enumerate().map(|(index, choice)| {
                        let caption = choice.caption.clone();
                        let goto = choice.action.to.clone();
                        let is_enabled = enabled_choices.contains(&caption);
                        let is_selected = keyboard_state.read().selected_index == index as i32;
                        let on_click = {
                            let goto = goto.clone();
                            let on_choice_click = on_choice_click.clone();
                            let mut keyboard_state = keyboard_state.clone();
                            move |evt: Event<MouseData>| {
                                evt.stop_propagation();
                                if is_enabled {
                                    keyboard_state.write().selected_index = index as i32;
                                    on_choice_click.call((goto.clone(), index));
                                }
                            }
                        };
                        let countdown = countdowns.read().get(index).copied().unwrap_or(0);
                        let max_time = max_times.read().get(index).copied().unwrap_or(0);
                        let animation_name = format!("progress-bar-{}", index);
                        let keyframes = format!(
                            "@keyframes {} {{ from {{ width: 100%; }} to {{ width: 0%; }} }}",
                            animation_name
                        );
                        let duration = format!("{}s", max_time);
                        rsx! {
                            li {
                                class: {{
                                    format!(
                                        "p-4 rounded-lg transition-colors duration-200 relative {} {}",
                                        if is_enabled {
                                            "cursor-pointer text-gray-900 hover:text-gray-700 dark:text-white dark:hover:text-gray-300"
                                        } else {
                                            "opacity-50 cursor-not-allowed text-gray-400 dark:text-gray-400"
                                        },
                                        if is_selected {
                                            "text-gray-100 dark:text-gray-300"
                                        } else {
                                            ""
                                        }
                                    )
                                }},
                                onclick: on_click,
                                span { class: "mr-2", {caption} }
                                { (countdown > 0).then(|| rsx! {
                                    style { "{keyframes}" }
                                    div {
                                        class: "w-full h-px bg-current mt-2",
                                        style: format!(
                                            "animation: {} linear {} forwards;",
                                            animation_name, duration
                                        ),
                                    }
                                }) }
                            }
                        }
                    })}
                }
            }
        }
    }
} 