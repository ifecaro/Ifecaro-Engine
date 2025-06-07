use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SettingsContext {
    pub settings: HashMap<String, String>,
    pub loaded: bool,
    pub settings_done: Signal<bool>,
}

pub fn use_settings_context() -> Signal<SettingsContext> {
    use_context::<Signal<SettingsContext>>()
} 