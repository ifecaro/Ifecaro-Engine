mod components;
mod constants;
mod enums;
mod i18n;
mod layout;
mod pages;
mod contexts;

use dioxus::{
    prelude::*,
    document::Stylesheet,
};
use dioxus_i18n::prelude::*;
use dioxus_router::prelude::*;
use crate::{
    enums::route::Route,
    components::navbar::Navbar,
    pages::{story::Story, dashboard::Dashboard},
    contexts::language_context::{LanguageProvider, LanguageState},
};
use unic_langid::langid;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        head {
            Stylesheet { href: asset!("public/tailwind.css") }
        }
        LanguageProvider {
            Router::<Route> {}
        }
    }
}

#[component]
pub fn Layout() -> Element {
    let route = use_route::<Route>();
    let mut state = use_context::<Signal<LanguageState>>();
    
    use_effect(move || {
        match &route {
            Route::Home {} => {
                state.write().set_language("zh-TW");
            }
            Route::Story { lang } => {
                state.write().set_language(lang);
            }
            Route::Dashboard { lang } => {
                state.write().set_language(lang);
            }
            Route::PageNotFound { .. } => {
                state.write().set_language("zh-TW");
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

