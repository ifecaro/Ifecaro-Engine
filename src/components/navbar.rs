use crate::{constants::config::config::LANGUAGES, enums::route::Route};
use dioxus::{
    hooks::use_context,
    prelude::{dioxus_core, dioxus_elements, fc_to_builder, rsx, Element, IntoDynNode, Link, component},
};
// use dioxus_router::prelude::Link;

#[component]
pub fn Navbar() -> Element {
    let mut lang = use_context::<&str>();

    rsx! {
        div { class: "fixed top-0 right-0 px-6 py-3",
            div { class: "dark:text-white grid grid-cols-4 gap-x-4 w-fit text-gray-900 text-center",
                Link { to: Route::Story {}, "Story" }
                Link { to: Route::Dashboard {}, "Dashboard" }
                button { "Settings" }
                button { onclick: move |_| { lang = if lang == "zh-TW" { "en-US" } else { "zh-TW" } },
                    {LANGUAGES.iter().find(|language| language.code == lang).and_then(|lang_found| {
                        Some(
                            lang_found.name
                        )
                    }).unwrap()
                    }
                }
            }
        }
    }
}
