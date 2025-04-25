use crate::enums::route::Route;
use dioxus::prelude::{component, dioxus_core, fc_to_builder, rsx, Element, Outlet, GlobalSignal, Readable, use_signal};
use wasm_bindgen::closure::Closure;
use web_sys::Event;

#[component]
pub fn Layout() -> Element {
    let closure_signal = use_signal(|| None::<Closure<dyn FnMut(Event)>>);
    
    rsx! {
        crate::components::navbar::Navbar { closure_signal: closure_signal }
        Outlet::<Route> {}
    }
}
