use dioxus::prelude::*;
use crate::services::indexeddb::clear_choices_and_random_choices;
use crate::components::toast::ToastType;
use crate::contexts::toast_context::use_toast;
use dioxus_i18n::t;
use crate::components::dropdown::Dropdown;
use crate::contexts::settings_context::use_settings_context;
use crate::services::indexeddb::set_setting_to_indexeddb;

#[derive(Debug, Clone, PartialEq)]
pub struct SettingOption {
    pub value: String,
    pub label: String,
}

fn display_setting_option(option: &SettingOption) -> String {
    option.label.clone()
}

#[component]
pub fn Settings() -> Element {
    let mut is_open = use_signal(|| false);
    let mut toast = use_toast();
    let settings_context = use_settings_context();
    let route: crate::enums::route::Route = use_route();

    let setting_options = vec![
        SettingOption {
            value: "clear_choices".to_string(),
            label: t!("clear_choices"),
        },
    ];

    let handle_select = move |option: SettingOption| {
        match option.value.as_str() {
            "clear_choices" => {
                clear_choices_and_random_choices();
                toast.write().show("Choices cleared successfully.".to_string(), ToastType::Success, 5000);
            }
            _ => {}
        }
        is_open.set(false);
    };

    let reader_mode = settings_context.read().settings.get("reader_mode").map(|v| v == "true").unwrap_or(false);

    rsx! {
        div {
            class: "relative",
            Dropdown {
                label: String::new(),
                value: t!("settings"),
                options: setting_options,
                is_open: *is_open.read(),
                search_query: String::new(),
                on_toggle: move |_| is_open.toggle(),
                on_search: move |_| {},
                on_select: handle_select,
                display_fn: display_setting_option,
                class: String::new(),
                search_placeholder: String::new(),
                button_class: Some("flex-1 sm:flex-none text-center text-xs text-gray-700 dark:text-white hover:text-gray-900 dark:hover:text-gray-300 transition-colors duration-200 py-2 cursor-pointer".to_string()),
                show_arrow: false,
                label_class: String::new(),
                dropdown_width: Some("min-w-max".to_string()),
                dropdown_position: Some("right-0".to_string()),
                show_search: false,
                option_class: "text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-800/50",
            }
            if *is_open.read() {
                div {
                    class: "absolute right-0 mt-2 w-48 rounded-md shadow-lg bg-white dark:bg-gray-800 ring-1 ring-black ring-opacity-5 z-[1000]",
                    div {
                        class: "py-1",
                        if let crate::enums::route::Route::Story { .. } = route {
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
                                    if reader_mode { "開啟" } else { "關閉" }
                                }
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
                            span { {t!("clear_choices")} }
                        }
                    }
                }
            }
        }
    }
} 