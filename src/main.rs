mod components;
mod enums;
mod i18n;
mod layout;
mod pages;
mod contexts;
mod constants;

use dioxus::{
    prelude::*,
    document::Stylesheet,
};
use crate::{
    enums::route::Route,
    components::navbar::Navbar,
    contexts::language_context::{LanguageProvider, LanguageState},
    components::story_content::Choice,
    contexts::story_context::{use_story_context, provide_story_context},
};

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    provide_story_context();
    
    rsx! {
        head {
            Stylesheet { href: asset!("public/tailwind.css") }
        }
        LanguageProvider {
            Router::<Route> {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyboardState {
    pub selected_index: i32,
    pub choices: Vec<Choice>,
    pub enabled_choices: Vec<String>,
    pub on_choice_click: Option<EventHandler<String>>,
}

#[component]
pub fn Layout() -> Element {
    let route = use_route::<Route>();
    let mut state = use_context::<Signal<LanguageState>>();
    let mut keyboard_state = use_signal(|| KeyboardState { 
        selected_index: 0,
        choices: vec![],
        enabled_choices: vec![],
        on_choice_click: None,
    });
    let mut story_context = use_story_context();
    
    use_effect(move || {
        match &route {
            Route::Home {} => {
                state.write().set_language("zh-TW");
            }
            Route::Story { lang } => {
                state.write().set_language(lang);
            }
            Route::Dashboard { lang } => {
                state.write().set_language(lang);
            }
            Route::PageNotFound { .. } => {
                state.write().set_language("zh-TW");
            }
        }
    });
    
    provide_context(keyboard_state);
    
    rsx! {
        main {
            class: "min-h-screen bg-gray-100 dark:bg-gray-900",
            tabindex: "0",
            onkeydown: move |event: Event<KeyboardData>| {
                match event.data.key() {
                    Key::ArrowUp => {
                        let mut state = keyboard_state.write();
                        if state.selected_index > 0 {
                            state.selected_index -= 1;
                        }
                        event.stop_propagation();
                    }
                    Key::ArrowDown => {
                        let mut state = keyboard_state.write();
                        if state.selected_index < state.choices.len() as i32 - 1 {
                            state.selected_index += 1;
                        }
                        event.stop_propagation();
                    }
                    Key::Character(key) => {
                        if let Ok(num) = key.parse::<usize>() {
                            let state = keyboard_state.read();
                            if num > 0 && num <= state.choices.len() {
                                let idx = num - 1;
                                if idx < state.choices.len() {
                                    let choice = &state.choices[idx];
                                    let goto = choice.action.to.clone();
                                    if state.enabled_choices.contains(&goto) {
                                        if let Some(on_choice_click) = &state.on_choice_click {
                                            on_choice_click.call(goto.clone());
                                            story_context.write().target_paragraph_id = Some(goto.clone());
                                        }
                                    }
                                }
                            }
                            event.stop_propagation();
                        }
                    }
                    Key::Enter => {
                        let state = keyboard_state.read();
                        let idx = state.selected_index as usize;
                        if idx < state.choices.len() {
                            let choice = &state.choices[idx];
                            let goto = choice.action.to.clone();
                            if state.enabled_choices.contains(&goto) {
                                if let Some(on_choice_click) = &state.on_choice_click {
                                    on_choice_click.call(goto.clone());
                                    story_context.write().target_paragraph_id = Some(goto.clone());
                                }
                            }
                        }
                        event.stop_propagation();
                    }
                    _ => {}
                }
            },
            Navbar {}
            div {
                class: "container mx-auto px-4 py-8",
                Outlet::<Route> {}
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

