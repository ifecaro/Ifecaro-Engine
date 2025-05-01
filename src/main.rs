mod components;
mod enums;
mod i18n;
mod layout;
mod pages;
mod contexts;
mod constants;
mod models;

use dioxus::{
    prelude::*,
    document::{Stylesheet, Script},
};
use crate::{
    enums::route::Route,
    components::navbar::Navbar,
    contexts::language_context::{LanguageProvider, LanguageState},
    components::story_content::Choice,
    contexts::story_context::{use_story_context, provide_story_context, StoryContext},
};
use std::sync::Arc;
use wasm_bindgen::closure::Closure;
use web_sys::Event as WebEvent;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        head {
            link { rel: "icon", r#type: "image/x-icon", href: "/img/icons/favicon.ico" }
            link { rel: "shortcut icon", r#type: "image/x-icon", href: "/img/icons/favicon.ico" }
            link { rel: "apple-touch-icon", sizes: "180x180", href: "/img/icons/apple-touch-icon.png" }
            link { rel: "icon", r#type: "image/png", sizes: "32x32", href: "/img/icons/favicon-32x32.png" }
            link { rel: "icon", r#type: "image/png", sizes: "16x16", href: "/img/icons/favicon-16x16.png" }
            link { rel: "manifest", href: "/manifest.json" }
            Stylesheet { href: asset!("public/tailwind.css") }
            Script { src: asset!("public/sw.js") }
            link { rel: "modulepreload", href: asset!("public/sw.js") }
            meta { name: "theme-color", content: "#000000" }
            meta { name: "apple-mobile-web-app-capable", content: "yes" }
            meta { name: "apple-mobile-web-app-status-bar-style", content: "black" }
            meta { name: "apple-mobile-web-app-title", content: "Ifecaro" }
        }
        LanguageProvider {
            StoryProvider {
                Router::<Route> {}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct StoryProviderProps {
    children: Element,
}

#[component]
fn StoryProvider(props: StoryProviderProps) -> Element {
    provide_story_context();
    rsx! {
        {props.children}
    }
}

#[derive(Debug, Clone)]
pub struct KeyboardState {
    pub selected_index: i32,
    pub choices: Arc<Vec<Choice>>,
    pub enabled_choices: Arc<Vec<String>>,
    pub on_choice_click: Option<Arc<EventHandler<String>>>,
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self {
            selected_index: 0,
            choices: Arc::new(Vec::new()),
            enabled_choices: Arc::new(Vec::new()),
            on_choice_click: None,
        }
    }
}

#[component]
pub fn Layout() -> Element {
    let route = use_route::<Route>();
    let mut state = use_context::<Signal<LanguageState>>();
    let mut keyboard_state = use_signal(KeyboardState::default);
    let mut story_context = use_story_context();
    let closure_signal = use_signal(|| None::<Closure<dyn FnMut(WebEvent)>>);
    
    use_effect(move || {
        let lang = match &route {
            Route::Home {} => "zh-TW",
            Route::Story { lang } | Route::Dashboard { lang } => lang,
            Route::PageNotFound { .. } => "zh-TW",
        };
        state.write().set_language(lang);
    });
    
    provide_context(keyboard_state);
    
    let _handle_choice = {
        let mut _story_context = story_context.clone();
        move |_choice: String| {
            // ... existing code ...
        }
    };

    let _handle_keyboard_choice = {
        let mut story_context = story_context.clone();
        let mut keyboard_state = keyboard_state.clone();
        let enabled_choices = keyboard_state.read().enabled_choices.clone();
        
        move |choice: &Choice, on_choice_click: EventHandler<String>| {
            let goto = choice.action.to.clone();
            if enabled_choices.contains(&goto) {
                keyboard_state.write().selected_index = -1;
                story_context.write().target_paragraph_id = Some(goto.clone());
                on_choice_click.call(goto);
            }
        }
    };
    
    let handle_key_press = move |event: Event<KeyboardData>| {
        let mut state = keyboard_state.write();
        match event.data.key() {
            Key::ArrowUp => {
                if state.selected_index > 0 {
                    state.selected_index -= 1;
                }
                event.stop_propagation();
            }
            Key::ArrowDown => {
                if state.selected_index < state.choices.len() as i32 - 1 {
                    state.selected_index += 1;
                }
                event.stop_propagation();
            }
            Key::Character(key) => {
                if let Ok(num) = key.parse::<usize>() {
                    if num > 0 && num <= state.choices.len() {
                        let idx = num - 1;
                        let choice = &state.choices[idx];
                        let goto = choice.action.to.clone();
                        if state.enabled_choices.contains(&goto) {
                            state.selected_index = idx as i32;
                            story_context.write().target_paragraph_id = Some(goto.clone());
                            if let Some(on_choice_click) = &state.on_choice_click {
                                on_choice_click.call(goto);
                            }
                        }
                    }
                }
                event.stop_propagation();
            }
            Key::Enter => {
                if state.selected_index >= 0 && state.selected_index < state.choices.len() as i32 {
                    let choice = &state.choices[state.selected_index as usize];
                    let goto = choice.action.to.clone();
                    if state.enabled_choices.contains(&goto) {
                        story_context.write().target_paragraph_id = Some(goto.clone());
                        if let Some(on_choice_click) = &state.on_choice_click {
                            on_choice_click.call(goto);
                        }
                    }
                }
                event.stop_propagation();
            }
            _ => {}
        }
    };

    rsx! {
        main {
            class: "min-h-screen bg-gray-100 dark:bg-gray-900",
            tabindex: "0",
            onkeydown: handle_key_press,
            Navbar { closure_signal: closure_signal }
            div {
                class: "container mx-auto px-4 py-8",
                Outlet::<Route> {}
            }
        }
    }
}

#[allow(dead_code)]
fn handle_choice_selection(
    state: &KeyboardState,
    idx: usize,
    story_context: &mut Signal<StoryContext>,
) {
    if idx < state.choices.len() {
        let choice = &state.choices[idx];
        let goto = choice.action.to.clone();
        if state.enabled_choices.contains(&goto) {
            if let Some(on_choice_click) = &state.on_choice_click {
                on_choice_click.call(goto.clone());
                story_context.write().target_paragraph_id = Some(goto);
            }
        }
    }
}

#[component]
pub fn Home() -> Element {
    let navigator = use_navigator();
    let default_lang = Route::default_language();
    let mut state = use_context::<Signal<LanguageState>>();
    
    use_effect(move || {
        state.write().set_language(&default_lang);
        
        let lang = default_lang.clone();
        navigator.replace(Route::Story { lang });
        (move || ())()
    });
    
    rsx! {
        div { "Redirecting..." }
    }
}

