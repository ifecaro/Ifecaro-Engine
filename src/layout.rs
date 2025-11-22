use crate::{
    components::{navbar::Navbar, story_content::Choice},
    contexts::{
        language_context::LanguageState,
        settings_context::use_settings_context,
        story_context::{use_story_context, StoryContext},
    },
    enums::route::Route,
};
use dioxus::prelude::*;
use std::collections::HashSet;
use std::{rc::Rc, sync::Arc};
use wasm_bindgen::closure::Closure;
use web_sys::Event as WebEvent;

#[cfg(target_arch = "wasm32")]
fn prefers_dark_mode() -> bool {
    web_sys::window()
        .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
        .map(|mql| mql.matches())
        .unwrap_or(false)
}

#[cfg(not(target_arch = "wasm32"))]
fn prefers_dark_mode() -> bool {
    false
}

#[cfg(target_arch = "wasm32")]
fn apply_theme_class(mode: &str) {
    fn update_class_list(class_list: web_sys::DomTokenList, mode: &str) {
        let _ = class_list.remove_1("dark");
        let _ = class_list.remove_1("paper");

        match mode {
            "dark" => {
                let _ = class_list.add_1("dark");
            }
            "paper" => {
                let _ = class_list.add_1("paper");
            }
            "light" => {}
            _ => {
                if prefers_dark_mode() {
                    let _ = class_list.add_1("dark");
                }
            }
        }
    }

    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        if let Some(element) = document.document_element() {
            update_class_list(element.class_list(), mode);
        }

        if let Some(body) = document.body() {
            update_class_list(body.class_list(), mode);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn apply_theme_class(_mode: &str) {}

#[derive(Debug, Clone)]
pub struct KeyboardState {
    pub selected_index: i32,
    pub choices: Rc<[Choice]>,
    pub enabled_choices: Rc<HashSet<String>>,
    pub on_choice_click: Option<Arc<EventHandler<String>>>,
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self {
            selected_index: 0,
            choices: Rc::<[Choice]>::from(Vec::<Choice>::new()),
            enabled_choices: Rc::new(HashSet::new()),
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
    let settings_context = use_settings_context();
    let closure_signal = use_signal(|| None::<Closure<dyn FnMut(WebEvent)>>);

    use_effect(move || {
        let lang = match &route {
            Route::Home {} => "zh-TW",
            Route::Story { lang }
            | Route::Dashboard { lang }
            | Route::InviteRequest { lang }
            | Route::InviteCheckEmail { lang }
            | Route::Register { lang }
            | Route::Login { lang } => lang,
            Route::PageNotFound { .. } => "zh-TW",
        };
        state.write().set_language(lang);
    });

    let theme_mode = settings_context
        .read()
        .settings
        .get("theme_mode")
        .cloned()
        .unwrap_or_else(|| "auto".to_string());

    {
        let theme_mode = theme_mode.clone();
        use_effect(move || {
            apply_theme_class(&theme_mode);
        });
    }

    let is_dark_theme = match theme_mode.as_str() {
        "dark" => true,
        "light" => false,
        "paper" => false,
        _ => prefers_dark_mode(),
    };

    let main_theme_class = match theme_mode.as_str() {
        "paper" => "bg-[#fdf6e3] text-[#2f2417]",
        _ if is_dark_theme => "bg-gray-900 text-gray-100",
        _ => "bg-gray-100 text-gray-900",
    };

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
            let goto = &choice.action.to;
            if enabled_choices.contains(&goto.as_ref().to_string()) {
                keyboard_state.write().selected_index = -1;
                story_context.write().target_paragraph_id = Some(goto.to_string());
                on_choice_click.call(goto.to_string());
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
                        let goto_owned = state.choices[idx].action.to.to_string();
                        if state.enabled_choices.contains(&goto_owned) {
                            state.selected_index = idx as i32;
                            story_context.write().target_paragraph_id = Some(goto_owned.clone());
                            if let Some(on_choice_click) = &state.on_choice_click {
                                on_choice_click.call(goto_owned);
                            }
                        }
                    }
                }
                event.stop_propagation();
            }
            Key::Enter => {
                if state.selected_index >= 0 && state.selected_index < state.choices.len() as i32 {
                    let goto_owned = state.choices[state.selected_index as usize]
                        .action
                        .to
                        .to_string();
                    if state.enabled_choices.contains(&goto_owned) {
                        story_context.write().target_paragraph_id = Some(goto_owned.clone());
                        if let Some(on_choice_click) = &state.on_choice_click {
                            on_choice_click.call(goto_owned);
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
            class: format!("min-h-screen {}", main_theme_class),
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
        let goto_owned = state.choices[idx].action.to.to_string();
        if state.enabled_choices.contains(&goto_owned) {
            if let Some(on_choice_click) = &state.on_choice_click {
                on_choice_click.call(goto_owned.clone());
                story_context.write().target_paragraph_id = Some(goto_owned);
            }
        }
    }
}
