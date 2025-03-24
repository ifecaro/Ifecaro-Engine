use crate::{constants::config::config::LANGUAGES, enums::route::Route};
use dioxus::{
    hooks::use_context,
    prelude::{
        component, rsx, Element, Link, Readable, dioxus_core, dioxus_elements, GlobalSignal, fc_to_builder, IntoDynNode, Writable
    },
    signals::Signal,
};
// use dioxus_router::prelude::Link;

#[derive(Clone)]
struct NavbarTranslations {
    story: &'static str,
    dashboard: &'static str,
    settings: &'static str,
}

impl NavbarTranslations {
    fn get(lang: &str) -> Self {
        match lang {
            "en" => Self {
                story: "Story",
                dashboard: "Dashboard",
                settings: "Settings",
            },
            "zh-TW" => Self {
                story: "故事",
                dashboard: "儀表板",
                settings: "設定",
            },
            _ => Self::get("en"),
        }
    }
}

#[component]
pub fn Navbar() -> Element {
    let mut lang = use_context::<Signal<&str>>();
    let t = NavbarTranslations::get(lang());

    rsx! {
        div { class: "fixed top-0 right-0 px-6 py-3",
            div { class: "dark:text-white grid grid-cols-4 space-x-4 w-fit text-gray-900 text-center",
                Link { to: Route::Story {}, "{t.story}" }
                Link { to: Route::Dashboard {}, "{t.dashboard}" }
                button { 
                    class: "cursor-pointer",
                    "{t.settings}" 
                }
                button {
                    class: "cursor-pointer",
                    onclick: move |_| {
                        lang.set(if lang() == "zh-TW" { "en-US" } else { "zh-TW" });
                    },
                    {
                        LANGUAGES
                            .iter()
                            .find(|language| language.code == lang())
                            .and_then(|lang_found| { Some(lang_found.name) })
                            .unwrap()
                    }
                }
            }
        }
    }
}
