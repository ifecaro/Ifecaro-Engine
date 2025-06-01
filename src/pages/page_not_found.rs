use dioxus::prelude::*;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            class: "container mx-auto px-4 pt-16",
            div {
                class: "max-w-2xl mx-auto text-center",
                h1 { class: "text-4xl font-bold text-gray-900 dark:text-white mb-4", "404 - Page Not Found" }
                p { class: "text-gray-600 dark:text-gray-400", "Sorry, the page you requested does not exist." }
            }
        }
    }
}
