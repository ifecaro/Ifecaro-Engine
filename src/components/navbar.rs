use crate::components::dropdown::Dropdown;
use crate::components::language_selector::{display_language, Language, AVAILABLE_LANGUAGES};
use crate::components::settings::Settings;
use crate::contexts::language_context::LanguageState;
use crate::enums::route::Route;
use crate::enums::style::NavbarStyle;
use dioxus::prelude::*;
use dioxus_core::use_drop;
use dioxus_i18n::t;
use gloo_timers::callback::Timeout;
use wasm_bindgen::JsValue;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::window;
use web_sys::Event;

#[component]
pub fn Navbar(closure_signal: Signal<Option<Closure<dyn FnMut(Event)>>>) -> Element {
    let navigator = use_navigator();
    let route: Route = use_route();
    // debugmode detection initial mount
    #[cfg(target_arch = "wasm32")]
    let debugmode_signal = use_signal(|| {
        let raw = window().unwrap().location().search().unwrap_or_default();
        raw.split('?').nth(1).unwrap_or("").split('&').any(|pair| {
            let mut iter = pair.split('=');
            iter.next() == Some("debugmode") && iter.next() == Some("true")
        })
    });
    #[cfg(not(target_arch = "wasm32"))]
    let debugmode_signal = use_signal(|| false);
    let debugmode = *debugmode_signal.read();
    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        if debugmode {
            let win = window().unwrap();
            let path = win.location().pathname().unwrap_or_default();
            let new_url = format!("{}?debugmode=true", path);
            let _ =
                win.history()
                    .unwrap()
                    .replace_state_with_url(&JsValue::NULL, "", Some(&new_url));
        }
    });
    let mut state = use_context::<Signal<LanguageState>>();
    let current_lang = state.read().current_language.clone();
    let story_lang = current_lang.clone();
    let dashboard_lang = current_lang.clone();
    let mut is_open = use_signal(|| false);
    let mut search_query = use_signal(|| String::new());
    let mut is_desktop = use_signal(|| false);
    let mut resize_closure: Signal<Option<Closure<dyn FnMut(Event)>>> = use_signal(|| None);

    // Check if we are in desktop mode and listen for window resize
    use_effect(move || {
        let win = window().unwrap();
        let width = win.inner_width().unwrap().as_f64().unwrap();
        let is_desktop_mode = width >= 640.0; // sm breakpoint is 640px
        is_desktop.set(is_desktop_mode);

        // Create resize event listener
        let closure = Closure::wrap(Box::new(move |_event: Event| {
            let win = window().unwrap();
            let width = win.inner_width().unwrap().as_f64().unwrap();
            let is_desktop_mode = width >= 640.0;
            is_desktop.set(is_desktop_mode);
        }) as Box<dyn FnMut(Event)>);

        win.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
            .unwrap();

        resize_closure.set(Some(closure));
    });

    use_drop(move || {
        if let Some(closure) = resize_closure.take() {
            let win = window().unwrap();
            win.remove_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                .unwrap();
        }
    });

    let filtered_languages = use_memo(move || {
        let query = search_query.read().to_lowercase();
        AVAILABLE_LANGUAGES
            .iter()
            .filter(|l| {
                l.name.to_lowercase().contains(&query) || l.code.to_lowercase().contains(&query)
            })
            .collect::<Vec<_>>()
    });

    let current_language = {
        let lang_code = state.read().current_language.clone();
        AVAILABLE_LANGUAGES
            .iter()
            .find(|l| l.code == lang_code)
            .map(|l| l.name)
            .unwrap_or("English")
            .to_string()
    };

    use_effect(move || {
        let document = web_sys::window().unwrap().document().unwrap();
        let closure = Closure::wrap(Box::new(move |_event: Event| {
            if let Some(target) = _event.target() {
                let element = target.dyn_into::<web_sys::Element>().unwrap();
                if !element.closest(".language-dropdown").unwrap().is_some() {
                    is_open.set(false);
                }
            }
        }) as Box<dyn FnMut(Event)>);

        document
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();

        closure_signal.set(Some(closure));
    });

    rsx! {
        div {
            class: "fixed bottom-0 sm:top-0 sm:bottom-auto left-0 right-0 w-full bg-white dark:bg-gray-900 paper:bg-transparent paper:text-[#1f2937] z-[9999] h-14 sm:h-auto transition-colors duration-200 paper-surface",
            div {
                class: "container mx-auto px-0 sm:px-6 h-full flex items-center",
                div {
                    class: "flex items-center sm:justify-end sm:space-x-6 w-full",
                    Link {
                        to: Route::Story { lang: story_lang.clone() },
                        class: NavbarStyle::Link.class(),
                        onclick: move |_| {
                            let _ = navigator.push(Route::Story { lang: story_lang.clone() });
                            #[cfg(target_arch = "wasm32")]
                            if debugmode {
                                let win = window().unwrap();
                                let path = win.location().pathname().unwrap_or_default();
                                let new_url = format!("{}?debugmode=true", path);
                                Timeout::new(0, move || {
                                    let _ = window().unwrap().history().unwrap().replace_state_with_url(&JsValue::NULL, "", Some(&new_url));
                                }).forget();
                            }
                        },
                        "{t!(\"story\")}"
                    }
                    Link {
                        to: Route::Dashboard { lang: dashboard_lang.clone() },
                        class: NavbarStyle::Link.class(),
                        onclick: move |_| {
                            let _ = navigator.push(Route::Dashboard { lang: dashboard_lang.clone() });
                            #[cfg(target_arch = "wasm32")]
                            if debugmode {
                                let win = window().unwrap();
                                let path = win.location().pathname().unwrap_or_default();
                                let new_url = format!("{}?debugmode=true", path);
                                Timeout::new(0, move || {
                                    let _ = window().unwrap().history().unwrap().replace_state_with_url(&JsValue::NULL, "", Some(&new_url));
                                }).forget();
                            }
                        },
                        "{t!(\"dashboard\")}"
                    }
                    Dropdown {
                        label: String::new(),
                        value: current_language,
                        options: filtered_languages.read().clone(),
                        is_open: *is_open.read(),
                        search_query: search_query.read().to_string(),
                        on_toggle: move |_| {
                            let current = *is_open.read();
                            is_open.set(!current);
                        },
                        on_search: move |query| search_query.set(query),
                        on_select: move |lang: &Language| {
                            let lang_code = lang.code.to_string();
                            state.write().set_language(&lang_code);
                            match route {
                                Route::Story { .. } => {
                                    let _ = navigator.push(Route::Story { lang: lang_code.clone() });
                                }
                                Route::Dashboard { .. } => {
                                    let _ = navigator.push(Route::Dashboard { lang: lang_code.clone() });
                                }
                                Route::InviteRequest { .. } => {
                                    let _ = navigator.push(Route::InviteRequest { lang: lang_code.clone() });
                                }
                                Route::InviteCheckEmail { .. } => {
                                    let _ = navigator.push(Route::InviteCheckEmail { lang: lang_code.clone() });
                                }
                                Route::Register { .. } => {
                                    let _ = navigator.push(Route::Register { lang: lang_code.clone() });
                                }
                                Route::Login { .. } => {
                                    let _ = navigator.push(Route::Login { lang: lang_code.clone() });
                                }
                                _ => {}
                            };
                            is_open.set(false);
                            search_query.set(String::new());
                        },
                        display_fn: display_language,
                        class: "flex-1 sm:flex-none language-dropdown".to_string(),
                        search_placeholder: t!("search_language"),
                        button_class: Some(NavbarStyle::Dropdown.class().to_string()),
                        show_arrow: false,
                        label_class: String::new(),
                        dropdown_position: Some(if *is_desktop.read() {
                            "absolute right-0 top-full left-auto bottom-auto rounded-md"
                        } else {
                            "fixed bottom-14 left-0 right-0 rounded-t-lg"
                        }.to_string()),
                        show_search: true,
                        option_class: NavbarStyle::DropdownOption.class().to_string(),
                        is_desktop: *is_desktop.read(),
                    }
                    Settings {
                        is_desktop: is_desktop
                    }
                }
            }
        }
    }
}
