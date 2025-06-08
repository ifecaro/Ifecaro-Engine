use dioxus::prelude::*;
use crate::enums::route::Route;
use crate::enums::style::NavbarStyle;
use dioxus_i18n::t;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::Event;
use crate::contexts::language_context::LanguageState;
use crate::components::dropdown::Dropdown;
use crate::components::language_selector::{AVAILABLE_LANGUAGES, Language, display_language};
use crate::components::settings::Settings;

#[component]
pub fn Navbar(closure_signal: Signal<Option<Closure<dyn FnMut(Event)>>>) -> Element {
    let navigator = use_navigator();
    let route: Route = use_route();
    let mut state = use_context::<Signal<LanguageState>>();
    let current_lang = state.read().current_language.clone();
    let mut is_open = use_signal(|| false);
    let mut search_query = use_signal(|| String::new());

    let filtered_languages = use_memo(move || {
        let query = search_query.read().to_lowercase();
        AVAILABLE_LANGUAGES.iter()
            .filter(|l| {
                l.name.to_lowercase().contains(&query) || 
                l.code.to_lowercase().contains(&query)
            })
            .collect::<Vec<_>>()
    });

    let current_language = {
        let lang_code = state.read().current_language.clone();
        AVAILABLE_LANGUAGES.iter()
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

        (move || {
            if let Some(closure) = closure_signal.read().as_ref() {
                document.remove_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
            }
        })()
    });

    rsx! {
        div { 
            class: "fixed bottom-0 sm:top-0 sm:bottom-auto left-0 right-0 w-full bg-white dark:bg-gray-900 z-[9999]",
            div { 
                class: "container mx-auto px-0 sm:px-6 py-3",
                div { 
                    class: "flex items-center sm:justify-end sm:space-x-6 w-full",
                    Link { 
                        to: Route::Story { lang: current_lang.clone() },
                        class: NavbarStyle::Link.class(),
                        "{t!(\"story\")}" 
                    }
                    Link { 
                        to: Route::Dashboard { lang: current_lang.clone() },
                        class: NavbarStyle::Link.class(),
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
                                _ => {}
                            };
                            is_open.set(false);
                            search_query.set(String::new());
                        },
                        display_fn: display_language,
                        class: "flex-1 sm:flex-none".to_string(),
                        search_placeholder: t!("search_language"),
                        button_class: Some(NavbarStyle::Dropdown.class().to_string()),
                        show_arrow: false,
                        label_class: String::new(),
                        dropdown_width: Some("min-w-max".to_string()),
                        dropdown_position: None,
                        show_search: true,
                        option_class: NavbarStyle::DropdownOption.class().to_string(),
                    }
                    Settings {}
                }
            }
        }
    }
}
