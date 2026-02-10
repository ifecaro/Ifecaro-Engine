use dioxus_i18n::prelude::*;
use include_dir::{include_dir, Dir};
use unic_langid::{langid, LanguageIdentifier};

static I18N_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/i18n");

pub fn create_i18n_store() -> I18nConfig {
    let mut config = I18nConfig::new(langid!("en-US")).with_fallback(langid!("en-US"));

    for file in I18N_DIR.files() {
        let Some(language_code) = file.path().file_stem().and_then(|stem| stem.to_str()) else {
            continue;
        };

        let Ok(langid) = language_code.parse::<LanguageIdentifier>() else {
            continue;
        };

        let Some(contents) = file.contents_utf8() else {
            continue;
        };

        config = config.with_locale(Locale::new_static(langid, contents));
    }

    config
}

// Get list of all available languages
#[allow(dead_code)]
pub fn get_available_languages() -> Vec<String> {
    I18N_DIR
        .files()
        .filter_map(|file| file.path().file_stem()?.to_str())
        .map(String::from)
        .collect()
}
