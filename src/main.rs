mod components;
mod constants;
mod enums;
mod layout;
mod pages;

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{
    hooks::use_shared_state_provider,
    prelude::{component, fc_to_builder, rsx, Element, Scope},
};
use dioxus_router::prelude::Router;

struct Language<'a>(&'a str);

fn main() {
    // launch the web app
    dioxus_web::launch(App);
    wasm_logger::init(wasm_logger::Config::default());
}

#[component]
#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Language("zh-TW"));

    cx.render(rsx! { Router::<enums::route::Route> {} })
}
