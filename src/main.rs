mod components;
mod constants;
mod enums;
mod i18n;
mod layout;
mod pages;

use dioxus::{
    prelude::*,
    document::Stylesheet,
};
use dioxus_i18n::prelude::*;
use tracing::Level;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn App() -> Element {
    let i18n = use_init_i18n(|| i18n::create_i18n_store());
    use_context_provider(|| Signal::new("zh-TW"));
    
    rsx! {
        head {
            Stylesheet { href: asset!("public/tailwind.css") }
        }
        Router::<enums::route::Route> {}
    }
}