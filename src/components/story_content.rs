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
    pub countdowns: Signal<Vec<u32>>,
    pub max_times: Signal<Vec<u32>>,
    pub progress_started: Signal<Vec<bool>>,
    pub disabled_by_countdown: Signal<Vec<bool>>,
    pub reader_mode: bool,
    pub chapter_title: String,
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

#[derive(Props, Clone, PartialEq)]
pub struct StoryContentUIProps {
    pub paragraph: String,
    pub choices: Vec<Choice>,
    pub enabled_choices: Vec<String>,
    pub disabled_by_countdown: Vec<bool>,
    pub chapter_title: String,
}

#[component]
pub fn StoryContentUI(props: StoryContentUIProps) -> Element {
    rsx! {
        div {
            class: "w-full flex items-center justify-center min-h-[calc(100vh-56px)]",
            div {
                class: "text-3xl md:text-4xl text-gray-900 dark:text-white text-center w-full select-none flex items-center justify-center",
                style: "letter-spacing: 0.1em;",
                {props.chapter_title.clone()}
            }
        }
        article {
            class: "prose-sm dark:prose-invert lg:prose-base mx-auto max-w-3xl p-8 text-gray-900 dark:text-white bg-white dark:bg-transparent",
            div {
                class: "whitespace-pre-wrap space-y-8",
                {props.paragraph.split('\n')
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
                {props.choices.iter().enumerate().map(|(index, choice)| {
                    let caption = &choice.caption;
                    let is_enabled = props.enabled_choices.contains(caption)
                        && !props.disabled_by_countdown.get(index).copied().unwrap_or(false);
                    rsx! {
                        li {
                            class: {{
                                format!(
                                    "p-4 rounded-lg transition duration-200 relative {}",
                                    if is_enabled {
                                        "cursor-pointer text-gray-900 hover:text-gray-700 dark:text-white dark:hover:text-gray-300 transition-opacity transition-transform"
                                    } else {
                                        "opacity-50 cursor-not-allowed text-gray-400 dark:text-gray-400"
                                    }
                                )
                            }},
                            span { class: "mr-2", {caption.clone()} }
                        }
                    }
                })}
            }
        }
    }
}

#[component]
pub fn StoryContent(props: StoryContentProps) -> Element {
    let choices = Arc::new(props.choices.clone());
    let enabled_choices = Arc::new(props.enabled_choices.clone());
    let on_choice_click = props.on_choice_click.clone();
    let mut keyboard_state = use_context::<Signal<KeyboardState>>();
    let story_ctx = use_story_context();
    let countdowns = props.countdowns.clone();
    let max_times = props.max_times.clone();
    let progress_started = props.progress_started.clone();
    let mut disabled_by_countdown = props.disabled_by_countdown.clone();
    {
        let mut countdowns = countdowns.clone();
        let mut max_times = max_times.clone();
        let story_ctx = story_ctx.clone();
        use_effect(move || {
            let time_limits = story_ctx.read().countdowns.read().clone();
            countdowns.set(time_limits.clone());
            max_times.set(time_limits.clone());
        });
    }
    {
        let mut progress_started = progress_started.clone();
        let story_ctx = story_ctx.clone();
        use_effect(move || {
            let time_limits = story_ctx.read().countdowns.read().clone();
            progress_started.set(vec![false; time_limits.len()]);
            gloo_timers::callback::Timeout::new(10, move || {
                let mut arr = progress_started.write();
                for v in arr.iter_mut() {
                    *v = true;
                }
            }).forget();
        });
    }
    {
        let mut disabled_by_countdown = disabled_by_countdown.clone();
        let story_ctx = story_ctx.clone();
        use_effect(move || {
            let time_limits = story_ctx.read().countdowns.read().clone();
            disabled_by_countdown.set(vec![false; time_limits.len()]);
        });
    }
    
    let mut show_filter = use_signal(|| true);
    let mut is_focused = use_signal(|| false);
    let mut is_mobile = use_signal(|| false);
    let is_countdown_paused = use_signal(|| true);
    {
        let show_filter = show_filter.clone();
        let mut is_countdown_paused = is_countdown_paused.clone();
        use_effect(move || {
            is_countdown_paused.set(*show_filter.read());
        });
    }
    
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
    
    // 監聽自訂事件 show_filter，收到時顯示遮罩
    {
        let show_filter = show_filter.clone();
        use_effect(move || {
            if let Some((_, document)) = get_window_document() {
                let mut show_filter = show_filter.clone();
                let handler = wasm_bindgen::closure::Closure::wrap(Box::new(move |_event: web_sys::CustomEvent| {
                    show_filter.set(true);
                }) as Box<dyn FnMut(web_sys::CustomEvent)>);
                document.add_event_listener_with_callback("show_filter", handler.as_ref().unchecked_ref()).unwrap();
                handler.forget();
            }
            (|| {})()
        });
    }
    
    let is_settings_chapter = story_ctx.read().is_settings_chapter();
    
    rsx! {
        div {
            class: "relative story-content-container",
            tabindex: "0",
            onkeydown: move |event: Event<KeyboardData>| {
                match event.data.key() {
                    key => {
                        if let Some(num) = key.to_string().parse::<usize>().ok() {
                            if num > 0 && num <= choices.len() {
                                let idx = num - 1;
                                let choice = &choices[idx];
                                let goto = choice.action.to.clone();
                                let is_disabled = disabled_by_countdown.read().get(idx).copied().unwrap_or(false);
                                if enabled_choices.contains(&choice.action.to) && !is_disabled {
                                    keyboard_state.write().selected_index = idx as i32;
                                    on_choice_click.call((goto.clone(), idx));
                                }
                            }
                            event.stop_propagation();
                        }
                    }
                }
            },
            onblur: move |_| {
                show_filter.set(true);
            },
            div {
                class: {
                    format!("fixed inset-0 backdrop-blur-sm z-10 flex items-center justify-center transition duration-500 cursor-pointer will-change-transform will-change-opacity {}",
                        if !*show_filter.read() || *is_mobile.read() { "opacity-0 pointer-events-none transform translate-y-2" } else { "opacity-100 transform translate-y-0" }
                    )
                },
                onclick: move |_| {
                    show_filter.set(false);
                    if let Some((_, document)) = get_window_document() {
                        if let Ok(Some(container)) = document.query_selector(".story-content-container") {
                            let _ = container.unchecked_into::<web_sys::HtmlElement>().focus();
                        }
                    }
                },
                ontransitionend: move |_| {
                    if !*show_filter.read() {
                        is_focused.set(true);
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
            if !is_settings_chapter && !props.chapter_title.is_empty() {
                div {
                    class: "w-full flex items-center justify-center min-h-[calc(100vh_-_56px)]",
                    div {
                        class: "text-3xl md:text-4xl text-gray-900 dark:text-white text-center w-full select-none flex items-center justify-center",
                        style: "letter-spacing: 0.1em;",
                        {props.chapter_title.clone()}
                    }
                }
            }
            article {
                class: "prose-sm dark:prose-invert lg:prose-base mx-auto max-w-3xl p-8 text-gray-900 dark:text-white bg-white dark:bg-transparent",
                div {
                    class: "whitespace-pre-wrap space-y-8",
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
                if is_settings_chapter || !props.reader_mode {
                    ol {
                        class: "mt-10 w-full md:w-fit list-decimal",
                        {choices.iter().enumerate().map(|(index, choice)| {
                            let caption = choice.caption.clone();
                            let goto = choice.action.to.clone();
                            let is_enabled = enabled_choices.contains(&choice.action.to)
                                && !disabled_by_countdown.read().get(index).copied().unwrap_or(false);
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
                                "@keyframes {} {{ from {{ transform: scaleX(1); }} to {{ transform: scaleX(0); }} }}",
                                animation_name
                            );
                            let duration = format!("{}s", max_time);
                            let animation_play_state = if *is_countdown_paused.read() { "paused" } else { "running" };
                            rsx! {
                                li {
                                    class: {{
                                        format!(
                                            "p-4 rounded-lg transition duration-200 relative {} {}",
                                            if is_enabled {
                                                "cursor-pointer text-gray-900 hover:text-gray-700 dark:text-white dark:hover:text-gray-300 transition-opacity transition-transform"
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
                                    span { class: "mr-2", {caption.clone()} }
                                    { (countdown > 0).then(|| rsx! {
                                        style { "{keyframes}" }
                                        div {
                                            class: "w-full h-px bg-current mt-2 origin-left will-change-transform",
                                            style: format!(
                                                "animation: {} linear {} forwards;animation-play-state:{};",
                                                animation_name, duration, animation_play_state
                                            ),
                                            onanimationend: move |_| {
                                                if countdown > 0 {
                                                    let mut arr = disabled_by_countdown.write();
                                                    if !arr.get(index).copied().unwrap_or(false) {
                                                        arr[index] = true;
                                                    }
                                                }
                                            },
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
} 