use dioxus::prelude::*;
use crate::services::indexeddb::clear_choices_and_random_choices;
use crate::components::toast::ToastType;
use crate::contexts::toast_context::use_toast;

#[component]
pub fn Settings() -> Element {
    let mut is_open = use_signal(|| false);
    let mut toast = use_toast();

    let handle_clear_choices = move |_| {
        clear_choices_and_random_choices();
        toast.write().show("Choices cleared successfully.".to_string(), ToastType::Success, 5000);
        is_open.set(false);
    };

    rsx! {
        div {
            class: "relative",
            button {
                class: "p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700",
                onclick: move |_| is_open.toggle(),
                svg {
                    class: "w-6 h-6 text-gray-700 dark:text-white",
                    fill: "none",
                    stroke: "currentColor",
                    "stroke-width": "2",
                    "viewBox": "0 0 24 24",
                    path {
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        d: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                    }
                    path {
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                    }
                }
            }

            if *is_open.read() {
                div {
                    class: "absolute right-0 mt-2 w-48 bg-white dark:bg-gray-800 rounded-md shadow-lg py-1 z-50",
                    div {
                        class: "px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer",
                        onclick: handle_clear_choices,
                        "Clear Choices"
                    }
                }
            }
        }
    }
} 