use crate::{contexts::language_context::LanguageState, enums::route::Route};
use dioxus::prelude::*;
use dioxus_i18n::t;
#[cfg(target_arch = "wasm32")]
use gloo_timers::callback::Timeout;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use web_sys::window;

#[component]
pub fn Home() -> Element {
    let navigator = use_navigator();
    let default_lang = Route::default_language();
    let mut state = use_context::<Signal<LanguageState>>();

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        let (current_search, current_hash) = {
            let win = window().expect("no global `window` exists");
            let location = win.location();
            (
                location.search().unwrap_or_default(),
                location.hash().unwrap_or_default(),
            )
        };

        state.write().set_language(&default_lang);

        let lang = default_lang.clone();
        navigator.replace(Route::Story { lang });

        #[cfg(target_arch = "wasm32")]
        if !current_search.is_empty() || !current_hash.is_empty() {
            Timeout::new(0, move || {
                let win = window().expect("no global `window` exists");
                let location = win.location();
                let path = location.pathname().unwrap_or_default();
                let search = if current_search.is_empty() {
                    String::new()
                } else {
                    format!("?{}", current_search.trim_start_matches('?'))
                };
                let new_url = format!("{path}{search}{current_hash}");
                let _ = win
                    .history()
                    .unwrap()
                    .replace_state_with_url(&JsValue::NULL, "", Some(&new_url));
            })
            .forget();
        }
        (move || ())()
    });

    rsx! {
        div { "{t!(\"redirecting\")}" }
    }
}
