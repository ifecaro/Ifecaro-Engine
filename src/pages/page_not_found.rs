use dioxus::prelude::{dioxus_core, dioxus_elements, rsx, Element, component, Props, GlobalSignal, Readable};

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div { "404" }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
