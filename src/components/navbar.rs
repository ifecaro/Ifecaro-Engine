use crate::{constants::config::config::LANGUAGES, enums::route::Route, Language};
use dioxus::{
    hooks::use_shared_state,
    prelude::{dioxus_elements, fc_to_builder, rsx, Element, GlobalAttributes, IntoDynNode, Scope},
};
use dioxus_router::prelude::Link;

#[allow(non_snake_case)]
pub fn Navbar(cx: Scope) -> Element {
    let lang = use_shared_state::<Language>(cx).unwrap();

    cx.render(rsx! {
        div { class: "fixed top-0 right-0 px-6 py-3",
            div { class: "dark:text-white grid grid-cols-4 gap-x-4 w-fit text-gray-900 text-center",
                Link { to: Route::Story {}, "Story" }
                Link { to: Route::Dashboard {}, "Dashboard" }
                button { "Settings" }
                button { onclick: |_| { lang.write().0 = if lang.read().0 == "zh-TW" { "en-US" } else { "zh-TW" } },
                    {LANGUAGES.iter().find(|language| language.code == lang.read().0).and_then(|lang_found| {
                        Some(
                            lang_found.name
                        )
                    }).unwrap()
                    }
                }
            }
        }
    })
}
