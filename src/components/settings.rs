use dioxus::prelude::*;
use crate::services::indexeddb::clear_choices_and_random_choices;
use crate::components::toast::ToastType;
use crate::contexts::toast_context::use_toast;
use dioxus_i18n::t;

#[component]
pub fn Settings() -> Element {
    let mut is_open = use_signal(|| false);
    let mut toast = use_toast();

    let handle_clear_choices = move |_| {
        clear_choices_and_random_choices();
        toast.write().show("Choices cleared successfully.".to_string(), ToastType::Success, 5000);
        is_open.set(false);
    };

    let dropdown_animation_class = if *is_open.read() {
        "opacity-100 max-h-[500px]"
    } else {
        "opacity-0 max-h-0 pointer-events-none"
    };

    rsx! {
        div {
            class: "relative",
            button {
                class: "w-full text-xs text-gray-700 dark:text-white hover:text-gray-900 dark:hover:text-gray-300 transition-colors duration-200 py-2",
                onclick: move |_| is_open.toggle(),
                {t!("settings")}
            }

            div {
                class: "absolute right-0 mt-2 w-48 bg-white dark:bg-gray-800 rounded-md shadow-lg py-1 z-50 overflow-hidden transition-all duration-300 ease-in-out {dropdown_animation_class}",
                div {
                    class: "px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer",
                    onclick: handle_clear_choices,
                    "Clear Choices"
                }
            }
        }
    }
} 