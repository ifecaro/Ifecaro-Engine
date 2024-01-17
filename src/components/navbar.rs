use crate::enums::route::Route;
use dioxus::prelude::{dioxus_elements, fc_to_builder, rsx, Element, GlobalAttributes, Scope};
use dioxus_router::prelude::Link;

#[allow(non_snake_case)]
pub fn Navbar(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "fixed top-0 right-0 px-6 py-3",
            div { class: "dark:text-white grid grid-cols-4 gap-x-4 w-fit text-gray-900 text-center",
                Link { to: Route::Story {}, "Story" }
                Link { to: Route::Dashboard {}, "Dashboard" }
                button { "Settings" }
            }
        }
    })
}
