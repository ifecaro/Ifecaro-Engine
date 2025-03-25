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
use dioxus_router::prelude::*;
use crate::{
    enums::route::Route,
    components::navbar::Navbar,
    pages::{story::Story, dashboard::Dashboard},
};
use unic_langid::langid;
use tracing as log;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn App() -> Element {
    let i18n = use_init_i18n(|| i18n::create_i18n_store());
    let mut lang = use_signal(|| i18n.language().to_string());
    
    provide_context(lang.clone());
    
    rsx! {
        head {
            Stylesheet { href: asset!("public/tailwind.css") }
        }
        Router::<Route> {}
    }
}

#[component]
pub fn Layout() -> Element {
    let route = use_route::<Route>();
    log::info!("Current route: {:?}", route);
    
    let mut i18n = use_init_i18n(|| i18n::create_i18n_store());
    let mut lang = use_context::<Signal<String>>();
    
    use_effect(move || {
        match &route {
            Route::Home {} => {
                i18n.set_language(langid!("zh-TW"));
                lang.set(i18n.language().to_string());
            }
            Route::Story { lang: route_lang } => {
                match route_lang.as_str() {
                    "zh-TW" => i18n.set_language(langid!("zh-TW")),
                    "en-US" => i18n.set_language(langid!("en-US")),
                    "es-ES" => i18n.set_language(langid!("es-ES")),
                    "es-CL" => i18n.set_language(langid!("es-CL")),
                    _ => i18n.set_language(langid!("zh-TW")),
                }
                lang.set(i18n.language().to_string());
            }
            Route::Dashboard { lang: route_lang } => {
                match route_lang.as_str() {
                    "zh-TW" => i18n.set_language(langid!("zh-TW")),
                    "en-US" => i18n.set_language(langid!("en-US")),
                    "es-ES" => i18n.set_language(langid!("es-ES")),
                    "es-CL" => i18n.set_language(langid!("es-CL")),
                    _ => i18n.set_language(langid!("zh-TW")),
                }
                lang.set(i18n.language().to_string());
            }
            Route::PageNotFound { .. } => {
                i18n.set_language(langid!("zh-TW"));
                lang.set(i18n.language().to_string());
            }
        }
    });
    
    rsx! {
        main {
            class: "min-h-screen bg-gray-100 dark:bg-gray-900",
            Navbar {}
            div {
                class: "container mx-auto px-4 py-8",
                Outlet::<Route> {}
            }
        }
    }
}

#[component]
pub fn Home() -> Element {
    let navigator = use_navigator();
    let default_lang = Route::default_language();
    
    use_effect(move || {
        let lang = default_lang.clone();
        navigator.replace(Route::Story { lang });
        (move || ())()
    });
    
    rsx! {
        div { "Redirecting..." }
    }
}

