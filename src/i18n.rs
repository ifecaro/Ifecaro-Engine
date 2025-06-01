use dioxus_i18n::prelude::*;
use unic_langid::langid;
use crate::constants::config::LANGUAGES;

pub fn create_i18n_store() -> I18nConfig {
    I18nConfig::new(langid!("en-US"))
        .with_locale(Locale::new_static(langid!("en-US"), include_str!("../i18n/en.ftl")))
        .with_locale(Locale::new_static(langid!("zh-TW"), include_str!("../i18n/zh-TW.ftl")))
        .with_locale(Locale::new_static(langid!("zh-CN"), include_str!("../i18n/zh-CN.ftl")))
        .with_locale(Locale::new_static(langid!("es-ES"), include_str!("../i18n/es-ES.ftl")))
        .with_locale(Locale::new_static(langid!("es-CL"), include_str!("../i18n/es-CL.ftl")))
        // Add more language support
        .with_locale(Locale::new_static(langid!("ja"), include_str!("../i18n/en.ftl")))
        .with_locale(Locale::new_static(langid!("ko"), include_str!("../i18n/en.ftl")))
        .with_locale(Locale::new_static(langid!("fr"), include_str!("../i18n/en.ftl")))
        .with_locale(Locale::new_static(langid!("de"), include_str!("../i18n/en.ftl")))
        .with_locale(Locale::new_static(langid!("it"), include_str!("../i18n/en.ftl")))
        .with_locale(Locale::new_static(langid!("pt"), include_str!("../i18n/en.ftl")))
        .with_locale(Locale::new_static(langid!("ru"), include_str!("../i18n/en.ftl")))
}

// Get list of all available languages
#[allow(dead_code)]
pub fn get_available_languages() -> Vec<&'static str> {
    LANGUAGES.iter().map(|lang| lang.code).collect()
} 