use dioxus::prelude::*;
use crate::services::indexeddb::clear_choices_and_random_choices;
#[cfg(target_arch = "wasm32")]
use crate::services::indexeddb::set_setting_to_indexeddb;
use crate::components::toast::ToastType;
use crate::contexts::toast_context::use_toast;
use dioxus_i18n::t;
use crate::contexts::settings_context::use_settings_context;
use crate::enums::style::NavbarStyle;

#[component]
pub fn Settings() -> Element {
    let mut is_open = use_signal(|| false);
    let mut toast = use_toast();
    let settings_context = use_settings_context();
    let reader_mode = settings_context.read().settings.get("reader_mode").map(|v| v == "true").unwrap_or(false);

    rsx! {
        div {
            class: "relative",
            button {
                class: NavbarStyle::Dropdown.class(),
                onclick: move |_| is_open.toggle(),
                {t!("settings")}
            }
            if *is_open.read() {
                div {
                    class: "fixed inset-0 w-screen h-screen z-[999]",
                    onclick: move |_| is_open.set(false),
                }
                div {
                    class: "absolute right-0 mt-2 w-48 rounded-md shadow-lg bg-white dark:bg-gray-800 ring-1 ring-black ring-opacity-5 z-[1000]",
                    div {
                        class: "py-1",
                        button {
                            class: "w-full px-4 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center justify-between",
                            onclick: move |_| {
                                let mut settings_context = settings_context.clone();
                                let new_reader_mode = !reader_mode;
                                settings_context.write().settings.insert("reader_mode".to_string(), new_reader_mode.to_string());
                                #[cfg(target_arch = "wasm32")]
                                {
                                    set_setting_to_indexeddb("reader_mode", &new_reader_mode.to_string());
                                }
                            },
                            span { {t!("reader_mode")} }
                            span {
                                class: "text-sm text-gray-500 dark:text-gray-400",
                                if reader_mode { "{t!(\"on\")}" } else { "{t!(\"off\")}" }
                            }
                        }
                        div {
                            class: "border-t border-gray-200 dark:border-gray-700 my-1",
                        }
                        button {
                            class: "w-full px-4 py-2 text-sm text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 flex items-center justify-between",
                            onclick: move |_| {
                                clear_choices_and_random_choices();
                                toast.write().show("Choices cleared successfully.".to_string(), ToastType::Success, 5000);
                                is_open.set(false);
                            },
                            span { {t!("clear_all_story_choices")} }
                        }
                    }
                }
            }
        }
    }
} 