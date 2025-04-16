use dioxus_i18n::prelude::*;
use unic_langid::langid;
use crate::enums::translations::Translations;
use crate::constants::config::LANGUAGES;

pub fn create_i18n_store() -> I18nConfig {
    I18nConfig::new(langid!("zh-TW"))
        .with_locale(Locale::new_static(langid!("en-US"), include_str!("../i18n/en.ftl")))
        .with_locale(Locale::new_static(langid!("zh-TW"), include_str!("../i18n/zh-TW.ftl")))
        .with_locale(Locale::new_static(langid!("es-ES"), include_str!("../i18n/es-ES.ftl")))
        .with_locale(Locale::new_static(langid!("es-CL"), include_str!("../i18n/es-CL.ftl")))
}

// 輔助函數：獲取當前語言的翻譯
#[allow(dead_code)]
pub fn get_translations(lang: &str) -> Translations {
    Translations::get(lang)
}

// 獲取所有可用的語言列表
#[allow(dead_code)]
pub fn get_available_languages() -> Vec<&'static str> {
    LANGUAGES.iter().map(|lang| lang.code).collect()
} 