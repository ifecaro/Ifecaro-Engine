#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Auto,
    Light,
    Dark,
    Paper,
}

impl ThemeMode {
    pub fn from_value(value: &str) -> Self {
        match value {
            "light" => Self::Light,
            "dark" => Self::Dark,
            "paper" => Self::Paper,
            _ => Self::Auto,
        }
    }

    pub fn resolve(self) -> ResolvedTheme {
        let prefers_dark = prefers_dark_mode();

        match self {
            Self::Auto => {
                if prefers_dark {
                    ResolvedTheme::dark()
                } else {
                    ResolvedTheme::light()
                }
            }
            Self::Light => ResolvedTheme::light(),
            Self::Dark => ResolvedTheme::dark(),
            Self::Paper => ResolvedTheme::paper(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ResolvedTheme {
    pub data_value: &'static str,
    pub is_dark: bool,
}

impl ResolvedTheme {
    const fn light() -> Self {
        Self {
            data_value: "light",
            is_dark: false,
        }
    }

    const fn dark() -> Self {
        Self {
            data_value: "dark",
            is_dark: true,
        }
    }

    const fn paper() -> Self {
        Self {
            data_value: "paper",
            is_dark: false,
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn prefers_dark_mode() -> bool {
    web_sys::window()
        .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
        .map(|mql| mql.matches())
        .unwrap_or(false)
}

#[cfg(not(target_arch = "wasm32"))]
fn prefers_dark_mode() -> bool {
    false
}

#[cfg(target_arch = "wasm32")]
pub fn apply_theme_class(mode: ThemeMode) {
    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        if let Some(element) = document.document_element() {
            let resolved = mode.resolve();
            let class_list = element.class_list();

            let _ = class_list.remove_1("dark");

            if resolved.is_dark {
                let _ = class_list.add_1("dark");
            }

            let _ = element.set_attribute("data-theme", resolved.data_value);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn apply_theme_class(_mode: ThemeMode) {}
