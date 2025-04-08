use dioxus::prelude::*;
use unic_langid::langid;
use dioxus_i18n::prelude::*;

#[derive(Clone)]
pub struct LanguageState {
    pub current_language: String,
    pub i18n: I18n,
}

impl LanguageState {
    pub fn new(i18n: I18n) -> Self {
        Self {
            current_language: "zh-TW".to_string(),
            i18n,
        }
    }

    pub fn set_language(&mut self, lang: &str) {
        self.current_language = lang.to_string();
        match lang {
            "zh-TW" => self.i18n.set_language(langid!("zh-TW")),
            "zh-CN" => self.i18n.set_language(langid!("zh-CN")),
            "en-US" => self.i18n.set_language(langid!("en-US")),
            "es-ES" => self.i18n.set_language(langid!("es-ES")),
            "es-CL" => self.i18n.set_language(langid!("es-CL")),
            _ => self.i18n.set_language(langid!("zh-TW")),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct LanguageProviderProps {
    children: Element,
}

#[component]
pub fn LanguageProvider(props: LanguageProviderProps) -> Element {
    let i18n = use_init_i18n(|| crate::i18n::create_i18n_store());
    let state = use_signal(|| LanguageState::new(i18n));
    
    provide_context(state);
    
    rsx! {
        {props.children}
    }
} 