use dioxus::prelude::*;
use crate::enums::route::Route;
use dioxus_i18n::t;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::Event;
use crate::contexts::language_context::LanguageState;

struct Language {
    code: &'static str,
    name: &'static str,
}

const LANGUAGES: &[Language] = &[
    Language { code: "zh-TW", name: "繁體中文" },
    Language { code: "zh-CN", name: "简体中文" },
    Language { code: "en-US", name: "English (US)" },
    Language { code: "en-GB", name: "English (UK)" },
    Language { code: "es-ES", name: "Español (España)" },
    Language { code: "es-CL", name: "Español (Chile)" },
];

#[component]
pub fn Navbar(closure_signal: Signal<Option<Closure<dyn FnMut(Event)>>>) -> Element {
    let navigator = use_navigator();
    let route: Route = use_route();
    let mut state = use_context::<Signal<LanguageState>>();
    let current_lang = state.read().current_language.clone();
    
    let mut is_open = use_signal(|| false);

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

        (move || {
            if let Some(closure) = closure_signal.read().as_ref() {
                document.remove_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
            }
        })()
    });

    let dropdown_class = if *is_open.read() {
        "translate-y-0 opacity-100"
    } else {
        "-translate-y-2 opacity-0 pointer-events-none"
    };

    let current_language = LANGUAGES.iter()
        .find(|l| l.code == current_lang)
        .map(|l| l.name)
        .unwrap_or("中文");
    
    rsx! {
        div { 
            class: "fixed top-0 left-0 right-0 w-full bg-white dark:bg-gray-900 z-[9999]",
            div { 
                class: "container mx-auto px-0 sm:px-6 py-3",
                div { 
                    class: "flex items-center justify-between sm:justify-end space-x-0 sm:space-x-6 w-full",
                    Link { 
                        to: Route::Story { lang: current_lang.clone() },
                        class: "flex-1 sm:flex-none text-center text-xs text-gray-700 dark:text-white hover:text-gray-900 dark:hover:text-gray-300 transition-colors duration-200 py-2",
                        "{t!(\"story\")}" 
                    }
                    Link { 
                        to: Route::Dashboard { lang: current_lang.clone() },
                        class: "flex-1 sm:flex-none text-center text-xs text-gray-700 dark:text-white hover:text-gray-900 dark:hover:text-gray-300 transition-colors duration-200 py-2",
                        "{t!(\"dashboard\")}" 
                    }
                    div {
                        class: "flex-1 sm:flex-none relative language-dropdown text-center",
                        button {
                            class: "w-full bg-transparent border-none text-xs font-medium text-gray-700 dark:text-white hover:text-gray-900 dark:hover:text-gray-300 outline-none focus:outline-none focus:ring-0 focus:ring-offset-0 transition-all duration-200 ease-in-out transform hover:scale-105 py-2",
                            onclick: move |_| {
                                let current = *is_open.read();
                                is_open.set(!current);
                            },
                            {current_language}
                        }
                        div {
                            class: "absolute right-0 mt-2 w-48 rounded-md shadow-lg bg-white dark:bg-gray-800 ring-1 ring-black ring-opacity-5 transition-all duration-200 ease-in-out transform origin-top-right {dropdown_class}",
                            {LANGUAGES.iter().map(|language| {
                                let route = route.clone();
                                let lang_code = language.code.to_string();
                                rsx! {
                                    button {
                                        class: "block w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors duration-150",
                                        onclick: move |_| {
                                            state.write().set_language(&lang_code);
                                            match route {
                                                Route::Story { .. } => {
                                                    let _ = navigator.push(Route::Story { lang: lang_code.clone() });
                                                }
                                                Route::Dashboard { .. } => {
                                                    let _ = navigator.push(Route::Dashboard { lang: lang_code.clone() });
                                                }
                                                _ => {}
                                            };
                                            is_open.set(false);
                                        },
                                        {language.name}
                                    }
                                }
                            })}
                        }
                    }
                }
            }
        }
    }
}
