use crate::enums::route::Route;
use dioxus::prelude::{component, dioxus_core, fc_to_builder, rsx, Element, Outlet};

#[component]
pub fn Layout() -> Element {
    rsx! {
        crate::components::navbar::Navbar {}
        Outlet::<Route> {}
    }
}
