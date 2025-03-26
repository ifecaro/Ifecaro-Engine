use dioxus::prelude::*;
use crate::enums::route::Route;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            class: "container mx-auto px-4 pt-16",
            div {
                class: "max-w-2xl mx-auto text-center",
                h1 { class: "text-4xl font-bold text-gray-900 dark:text-white mb-4", "404 - 找不到頁面" }
                p { class: "text-gray-600 dark:text-gray-400", "抱歉，您請求的頁面不存在。" }
            }
        }
    }
}
