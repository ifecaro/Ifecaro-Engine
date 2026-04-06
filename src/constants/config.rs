// use serde::Deserialize;

#[allow(dead_code)]
pub struct Language<'a> {
    pub name: &'a str,
    pub code: &'a str,
}

pub fn base_api_url() -> &'static str {
    let explicit_base = option_env!("VITE_BASE_API_URL").filter(|value| !value.trim().is_empty());

    let staging_api_url = "https://ifecaro.com/staging/db/api";

    let app_env = app_env_label();

    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(hostname) = window.location().hostname() {
                if hostname == "localhost" || hostname == "127.0.0.1" {
                    return resolve_base_api_url(explicit_base, staging_api_url, app_env, true);
                }
            }
        }
    }

    resolve_base_api_url(explicit_base, staging_api_url, app_env, false)
}

fn resolve_base_api_url(
    explicit_base: Option<&'static str>,
    staging_api_url: &'static str,
    app_env: &'static str,
    is_local_hostname: bool,
) -> &'static str {
    if let Some(base) = explicit_base {
        return base;
    }

    if is_local_hostname {
        return staging_api_url;
    }

    match app_env {
        "production" => "https://ifecaro.com/db/api",
        _ => staging_api_url,
    }
}

#[cfg(target_arch = "wasm32")]
fn debugmode_enabled_from_url() -> bool {
    use web_sys::window;

    let Some(win) = window() else { return false };
    let raw = win.location().search().unwrap_or_default();

    raw.trim_start_matches('?').split('&').any(|pair| {
        let mut iter = pair.split('=');
        iter.next() == Some("debugmode") && iter.next() == Some("true")
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn debugmode_enabled_from_url() -> bool {
    false
}

pub fn should_show_story_debug_info() -> bool {
    if cfg!(debug_assertions) {
        return true;
    }

    matches!(app_env_label(), "staging" | "production") && debugmode_enabled_from_url()
}

pub fn should_show_version_label() -> bool {
    if app_env_label() == "development" {
        return false;
    }

    matches!(app_env_label(), "staging" | "production") && debugmode_enabled_from_url()
}

pub fn app_version_label() -> &'static str {
    option_env!("GHCR_TAG")
        .or(option_env!("IFECARO_APP_VERSION"))
        .unwrap_or(env!("CARGO_PKG_VERSION"))
}
pub fn app_env_label() -> &'static str {
    option_env!("VITE_APP_ENV")
        .or(option_env!("IFECARO_APP_ENV"))
        .unwrap_or("development")
}

pub static PARAGRAPHS: &str = "/collections/paragraphs/records";
pub static CHAPTERS: &str = "/collections/chapters/records";
#[allow(dead_code)]
pub static ACTIONS: &str = "/collections/actions/records";
#[allow(dead_code)]
pub static CHARACTERS: &str = "/collections/characters/records";
#[allow(dead_code)]
pub static ATTRIBUTES: &str = "/collections/attributes/records";
#[allow(dead_code)]
pub static RELATIONSHIPS: &str = "/collections/relationships/records";
#[allow(dead_code)]
pub static PUBLIC_COLLECTIONS: &str = "/collections/public";
#[allow(dead_code)]
pub static AUTH_TOKEN: &str = ""; // No auth token needed, as collections should be accessible to any user
pub static LANGUAGES: [Language; 7] = [
    Language {
        name: "English (US)",
        code: "en-US",
    },
    Language {
        name: "English (UK)",
        code: "en-GB",
    },
    Language {
        name: "Español (España)",
        code: "es-ES",
    },
    Language {
        name: "Español (Chile)",
        code: "es-CL",
    },
    Language {
        name: "Français (France)",
        code: "fr-FR",
    },
    Language {
        name: "中文（台灣）",
        code: "zh-TW",
    },
    Language {
        name: "中文（中国）",
        code: "zh-CN",
    },
];

#[cfg(test)]
mod tests {
    use super::resolve_base_api_url;

    #[test]
    fn defaults_to_staging_in_development() {
        let actual = resolve_base_api_url(
            None,
            "https://ifecaro.com/staging/db/api",
            "development",
            false,
        );

        assert_eq!(actual, "https://ifecaro.com/staging/db/api");
    }

    #[test]
    fn local_hostname_uses_staging_api() {
        let actual = resolve_base_api_url(
            None,
            "https://ifecaro.com/staging/db/api",
            "development",
            true,
        );

        assert_eq!(actual, "https://ifecaro.com/staging/db/api");
    }

    #[test]
    fn explicit_base_url_has_highest_priority() {
        let actual = resolve_base_api_url(
            Some("https://example.com/custom/api"),
            "https://ifecaro.com/staging/db/api",
            "production",
            true,
        );

        assert_eq!(actual, "https://example.com/custom/api");
    }

    #[test]
    fn empty_explicit_base_url_matches_unset_behavior() {
        let explicit_base = Some("").filter(|value| !value.trim().is_empty());
        let expected = resolve_base_api_url(
            None,
            "https://ifecaro.com/staging/db/api",
            "production",
            false,
        );
        let actual = resolve_base_api_url(
            explicit_base,
            "https://ifecaro.com/staging/db/api",
            "production",
            false,
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn production_without_explicit_base_url_uses_production_default() {
        let actual = resolve_base_api_url(
            None,
            "https://ifecaro.com/staging/db/api",
            "production",
            false,
        );
        let expected = "https://ifecaro.com/db/api";

        assert_eq!(actual, expected);
        assert!(!actual.is_empty());
    }

    #[test]
    fn non_production_without_explicit_base_url_uses_staging_default() {
        let actual = resolve_base_api_url(None, "https://ifecaro.com/staging/db/api", "staging", false);

        assert_eq!(actual, "https://ifecaro.com/staging/db/api");
        assert!(!actual.is_empty());
    }
}
