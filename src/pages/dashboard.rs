use dioxus::prelude::{fc_to_builder, rsx, Element, component, dioxus_core, GlobalSignal, Readable};

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        crate::pages::layout::Layout { title: "Dashboard" }
    }
}
