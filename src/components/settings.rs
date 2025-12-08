use crate::components::toast::ToastType;
use crate::contexts::language_context::LanguageState;
use crate::contexts::settings_context::use_settings_context;
use crate::contexts::toast_context::use_toast;
use crate::enums::route::Route;
use crate::enums::style::NavbarStyle;
#[cfg(target_arch = "wasm32")]
use crate::services::indexeddb::clear_all_disabled_choices_from_indexeddb;
use crate::services::indexeddb::clear_choices_and_random_choices;
#[cfg(target_arch = "wasm32")]
use crate::services::indexeddb::set_setting_to_indexeddb;
use crate::utils::theme::{apply_theme_class, ThemeMode};
use dioxus::prelude::*;
use dioxus_i18n::t;
use wasm_bindgen_futures::spawn_local;
#[cfg(target_arch = "wasm32")]
use web_sys::window;

#[derive(Props, Clone, PartialEq)]
pub struct SettingsProps {
    is_desktop: Signal<bool>,
}

#[component]
pub fn Settings(props: SettingsProps) -> Element {
    let mut is_open = use_signal(|| false);
    let navigator = use_navigator();
    let mut toast = use_toast();
    let language_state = use_context::<Signal<LanguageState>>();
    let current_lang = language_state.read().current_language.clone();
    let settings_context = use_settings_context();
    let reader_mode = settings_context
        .read()
        .settings
        .get("reader_mode")
        .map(|v| v == "true")
        .unwrap_or(false);
    let theme_mode = settings_context
        .read()
        .settings
        .get("theme_mode")
        .cloned()
        .unwrap_or_else(|| "auto".to_string());

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
        let raw = window()
            .expect("no global `window` exists")
            .location()
            .search()
            .unwrap_or_default();
        raw.split('?').nth(1).unwrap_or("").split('&').any(|pair| {
            let mut iter = pair.split('=');
            iter.next() == Some("debugmode") && iter.next() == Some("true")
        })
    });
    #[cfg(not(target_arch = "wasm32"))]
    let debugmode_signal = use_signal(|| false);
    let show_clear = cfg!(debug_assertions) || *debugmode_signal.read();
    let theme_layout_class = if *props.is_desktop.read() {
        "grid grid-cols-2 gap-2"
    } else {
        "grid grid-cols-1 gap-2"
    };

    rsx! {
        div {
            class: "relative flex-1 sm:flex-none",
            button {
                class: NavbarStyle::Dropdown.class(),
                onclick: move |_| is_open.toggle(),
                span { class: "pencil-lite", "{t!(\"settings\")}" }
            }
            if *is_open.read() {
                div {
                    class: "fixed inset-0 w-screen h-screen z-[999] bg-black/50",
                    onclick: move |_| is_open.set(false),
                }
            }
            div {
                class: format!("{position_class} w-full sm:min-w-[16rem] sm:max-w-[60vw] shadow-lg bg-white dark:bg-gray-800 paper:bg-[#fef8e7] paper:text-[#1f2937] ring-1 ring-black ring-opacity-5 paper:ring-[#d4c29a] paper:ring-opacity-60 z-[1000] transition duration-200 ease-in-out transform {animation_class} will-change-transform will-change-opacity"),
                div {
                    class: "py-1",
                    button {
                        class: "w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-300 paper:text-[#374151] hover:bg-gray-100 dark:hover:bg-gray-700 paper:hover:bg-[#f0e6cf]",
                        onclick: move |_| {
                            let _ = navigator.push(Route::Login { lang: current_lang.clone() });
                            is_open.set(false);
                        },
                        span { class: "pencil-lite", "{t!(\"login\")}" }
                    }
                    button {
                        class: "w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-300 paper:text-[#374151] hover:bg-gray-100 dark:hover:bg-gray-700 paper:hover:bg-[#f0e6cf]",
                        onclick: move |_| {
                            let mut settings_context = settings_context.clone();
                            let new_reader_mode = !reader_mode;
                            settings_context
                                .write()
                                .settings
                                .insert("reader_mode".to_string(), new_reader_mode.to_string());
                            #[cfg(target_arch = "wasm32")]
                            {
                                set_setting_to_indexeddb("reader_mode", &new_reader_mode.to_string());
                            }
                        },
                        span { class: "pencil-lite", "{t!(\"toggle_reader_mode\")}" }
                        div {
                            class: "flex items-center justify-between w-full",
                            span { class: "font-medium pencil-lite", "{t!(\"reader_mode\")}" }
                            br {}
                            span { class: "text-xs text-gray-500 dark:text-gray-400 ml-2 pencil-lite", "{reader_mode_status}" }
                        }
                    }
                }
                div { class: "border-t border-gray-200 dark:border-gray-700 my-1" }
                div {
                    class: "px-4 py-2",
                    div { class: "text-sm font-medium text-gray-800 dark:text-gray-100 mb-2 pencil-lite", "{t!(\"theme_mode\")}" }
                    div {
                        class: theme_layout_class,
                        {["auto", "light", "dark", "paper"].iter().map(|mode| {
                            let label = match *mode {
                                "auto" => t!("theme_mode_auto"),
                                "light" => t!("theme_mode_light"),
                                "dark" => t!("theme_mode_dark"),
                                "paper" => t!("theme_mode_paper"),
                                _ => String::new(),
                            };
                            let is_active = theme_mode == *mode;
                            let mode_value = mode.to_string();
                            let mut settings_context = settings_context.clone();

                            rsx! {
                                button {
                                    key: "{mode}",
                                    class: format!(
                                        "w-full px-3 py-2 text-sm rounded-md border transition-colors duration-150 {}",
                                        if is_active {
                                            "border-blue-500 text-blue-700 dark:text-blue-300 paper:text-[#1f2937] bg-blue-50 dark:bg-blue-900/30 paper:bg-[#eae0c9] paper:border-[#c6b17e]"
                                        } else {
                                            "border-gray-200 dark:border-gray-700 paper:border-[#e4d5b2] text-gray-700 dark:text-gray-300 paper:text-[#374151] hover:bg-gray-100 dark:hover:bg-gray-700 paper:hover:bg-[#f0e6cf]"
                                        }
                                    ),
                                    onclick: move |_| {
                                        settings_context
                                            .write()
                                            .settings
                                            .insert("theme_mode".to_string(), mode_value.clone());
                                        #[cfg(target_arch = "wasm32")]
                                        {
                                            set_setting_to_indexeddb("theme_mode", &mode_value);
                                        }
                                        apply_theme_class(ThemeMode::from_value(&mode_value));
                                    },
                                    span { class: "pencil-lite", "{label}" }
                                }
                            }
                        })}
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
                        span { class: "pencil-lite", "{t!(\"clear_all_story_choices\")}" }
                    }
                }
            }
        }
    }
}
