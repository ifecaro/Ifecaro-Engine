use crate::{constants::config::config::LANGUAGES, enums::route::Route, enums::translations::Translations};
use dioxus::{
    hooks::{use_context, use_signal, use_effect},
    prelude::{
        component, rsx, Element, Link, Readable, dioxus_core, dioxus_elements, GlobalSignal, fc_to_builder, IntoDynNode, Writable
    },
    signals::Signal,
};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::Event;

#[component]
pub fn Navbar() -> Element {
    let mut current_lang = use_context::<Signal<&str>>();
    let t = Translations::get(current_lang());
    let mut is_open = use_signal(|| false);
    let mut closure_signal = use_signal(|| None);

    use_effect(move || {
        let handler = move |event: Event| {
            if let Some(target) = event.target() {
                if let Some(element) = target.dyn_into::<web_sys::Element>().ok() {
                    if element.closest(".language-dropdown").ok().flatten().is_none() {
                        is_open.set(false);
                    }
                }
            }
        };

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
        let ref_ = closure.as_ref().unchecked_ref();
        document.add_event_listener_with_callback("click", ref_).unwrap();

        closure_signal.set(Some(closure));
    });

    let dropdown_class = if *is_open.read() {
        "translate-y-0 opacity-100"
    } else {
        "-translate-y-2 opacity-0 pointer-events-none"
    };

    rsx! {
        div { 
            class: "fixed top-0 right-0 px-6 py-3",
            div { 
                class: "dark:text-white grid grid-cols-4 space-x-4 w-fit text-gray-900 text-center",
                Link { to: Route::Story {}, "{t.story}" }
                Link { to: Route::Dashboard {}, "{t.dashboard}" }
                button { 
                    class: "cursor-pointer",
                    "{t.settings}" 
                }
                div { 
                    class: "relative language-dropdown",
                    button {
                        class: "bg-transparent border-none text-sm font-medium text-gray-900 dark:text-white outline-none focus:outline-none focus:ring-0 focus:ring-offset-0 transition-all duration-200 ease-in-out transform hover:scale-105",
                        onclick: move |_| {
                            let current = *is_open.read();
                            is_open.set(!current);
                        },
                        {LANGUAGES.iter().find(|l| l.code == current_lang()).map(|l| l.name).unwrap_or("")}
                    }
                    div {
                        class: "absolute right-0 mt-2 w-48 rounded-md shadow-lg bg-white dark:bg-gray-800 ring-1 ring-black ring-opacity-5 transition-all duration-200 ease-in-out transform origin-top-right {dropdown_class}",
                        {LANGUAGES.iter().map(|language| {
                            rsx! {
                                button {
                                    class: "block w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors duration-150",
                                    onclick: move |_| {
                                        current_lang.set(language.code);
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
