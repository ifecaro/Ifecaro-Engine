use dioxus::prelude::*;
use crate::services::indexeddb::clear_choices_and_random_choices;
#[cfg(target_arch = "wasm32")]
use crate::services::indexeddb::set_setting_to_indexeddb;
use crate::components::toast::ToastType;
use crate::contexts::toast_context::use_toast;
use dioxus_i18n::t;
use crate::contexts::settings_context::use_settings_context;
use crate::enums::style::NavbarStyle;
use wasm_bindgen_futures::spawn_local;
#[cfg(target_arch = "wasm32")]
use crate::services::indexeddb::clear_all_disabled_choices_from_indexeddb;
#[cfg(target_arch = "wasm32")]
use web_sys::window;

#[derive(Props, Clone, PartialEq)]
pub struct SettingsProps {
    is_desktop: Signal<bool>,
}

#[component]
pub fn Settings(props: SettingsProps) -> Element {
    let mut is_open = use_signal(|| false);
    let mut toast = use_toast();
    let settings_context = use_settings_context();
    let reader_mode = settings_context.read().settings.get("reader_mode").map(|v| v == "true").unwrap_or(false);

    let animation_class = if *is_open.read() {
        "translate-y-0 opacity-100"
    } else {
        if *props.is_desktop.read() {
            "-translate-y-2 opacity-0 pointer-events-none"
        } else {
            "translate-y-2 opacity-0 pointer-events-none"
        }
    };

    let position_class = if *props.is_desktop.read() {
        "absolute right-0 top-full left-auto bottom-auto rounded-md mt-2"
    } else {
        "fixed bottom-14 left-0 right-0 rounded-t-lg"
    };
    
    let reader_mode_status = if reader_mode { t!("on") } else { t!("off") };

    // debugmode detection initial mount
    #[cfg(target_arch = "wasm32")]
    let debugmode_signal = use_signal(|| {
        let raw = window().expect("no global `window` exists").location().search().unwrap_or_default();
        raw.split('?').nth(1).unwrap_or("").split('&').any(|pair| {
            let mut iter = pair.split('=');
            iter.next() == Some("debugmode") && iter.next() == Some("true")
        })
    });
    #[cfg(not(target_arch = "wasm32"))]
    let debugmode_signal = use_signal(|| false);
    let show_clear = cfg!(debug_assertions) || *debugmode_signal.read();

    rsx! {
        div {
            class: "relative flex-1 sm:flex-none",
            button {
                class: NavbarStyle::Dropdown.class(),
                onclick: move |_| is_open.toggle(),
                "{t!(\"settings\")}"
            }
            if *is_open.read() {
                div {
                    class: "fixed inset-0 w-screen h-screen z-[999] bg-black/50",
                    onclick: move |_| is_open.set(false),
                }
            }
            div {
                class: format!("{position_class} w-full sm:max-w-[60vw] shadow-lg bg-white dark:bg-gray-800 ring-1 ring-black ring-opacity-5 z-[1000] transition duration-200 ease-in-out transform {animation_class} will-change-transform will-change-opacity"),
                div {
                    class: "py-1",
                    button {
                        class: "w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700",
                        onclick: move |_| {
                            let mut settings_context = settings_context.clone();
                            let new_reader_mode = !reader_mode;
                            settings_context.write().settings.insert("reader_mode".to_string(), new_reader_mode.to_string());
                            #[cfg(target_arch = "wasm32")]
                            {
                                set_setting_to_indexeddb("reader_mode", &new_reader_mode.to_string());
                            }
                        },
                        div {
                            class: "flex items-center justify-between w-full",
                            span { class: "font-medium", "{t!(\"reader_mode\")}" }
                            br {}
                            span { class: "text-xs text-gray-500 dark:text-gray-400 ml-2", "{reader_mode_status}" }
                        }
                    }
                    if show_clear {
                        div {
                            class: "border-t border-gray-200 dark:border-gray-700 my-1",
                        }
                        button {
                            class: "w-full text-left px-4 py-2 text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 font-medium",
                            onclick: move |_| {
                                spawn_local(async move {
                                    let _ = clear_choices_and_random_choices().await;
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        let _ = clear_all_disabled_choices_from_indexeddb().await;
                                    }
                                });
                                toast.write().show("Choices cleared successfully.".to_string(), ToastType::Success, 5000);
                                is_open.set(false);
                            },
                            "{t!(\"clear_all_story_choices\")}"
                        }
                    }
                }
            }
        }
    }
} 