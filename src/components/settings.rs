use dioxus::prelude::*;
use crate::services::indexeddb::clear_choices_and_random_choices;
use crate::components::toast::ToastType;
use crate::contexts::toast_context::use_toast;
use dioxus_i18n::t;
use crate::components::dropdown::Dropdown;

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

    rsx! {
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
            option_class: "text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/50",
        }
    }
} 