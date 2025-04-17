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
        
        // 根據語言代碼設置 i18n
        match lang {
            "zh-TW" => self.i18n.set_language(langid!("zh-TW")),
            "zh-CN" => self.i18n.set_language(langid!("zh-CN")),
            "en-US" | "en-GB" | "en" => self.i18n.set_language(langid!("en-US")),
            "es-ES" | "es-CL" | "es" => self.i18n.set_language(langid!("es-ES")),
            "ja" => self.i18n.set_language(langid!("ja")),
            "ko" => self.i18n.set_language(langid!("ko")),
            "fr" => self.i18n.set_language(langid!("fr")),
            "de" => self.i18n.set_language(langid!("de")),
            "it" => self.i18n.set_language(langid!("it")),
            "pt" => self.i18n.set_language(langid!("pt")),
            "ru" => self.i18n.set_language(langid!("ru")),
            // 如果找不到匹配的語言，默認為英語
            _ => self.i18n.set_language(langid!("en-US")),
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
    let state = use_signal(|| {
        let mut state = LanguageState::new(i18n);
        state.set_language("zh-TW");
        state
    });
    
    provide_context(state);
    
    rsx! {
        {props.children}
    }
} 