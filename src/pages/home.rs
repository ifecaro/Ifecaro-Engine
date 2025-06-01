use dioxus::prelude::*;
use dioxus_i18n::t;
use crate::{
    enums::route::Route,
    contexts::language_context::LanguageState,
};

#[component]
pub fn Home() -> Element {
    let navigator = use_navigator();
    let default_lang = Route::default_language();
    let mut state = use_context::<Signal<LanguageState>>();
    
    use_effect(move || {
        state.write().set_language(&default_lang);
        
        let lang = default_lang.clone();
        navigator.replace(Route::Story { lang });
        (move || ())()
    });
    
    rsx! {
        div { "{t!(\"redirecting\")}" }
    }
} 