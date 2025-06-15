use dioxus::prelude::*;
use dioxus_i18n::t;
use smallvec::SmallVec;

#[component]
#[allow(unused_variables)]
pub fn PageNotFound(route: SmallVec<[String; 8]>) -> Element {
    rsx! {
        div {
            class: "container mx-auto px-4 pt-16",
            div {
                class: "max-w-2xl mx-auto text-center",
                h1 { class: "text-4xl font-bold text-gray-900 dark:text-white mb-4", "{t!(\"page_not_found\")}" }
                p { class: "text-gray-600 dark:text-gray-400", "{t!(\"page_not_found_message\")}" }
            }
        }
    }
}
