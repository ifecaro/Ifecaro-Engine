use dioxus::prelude::*;
use std::collections::VecDeque;
use crate::components::toast::{Toast, ToastType};

#[derive(Clone, Copy)]
pub struct ToastManager {
    toasts: Signal<VecDeque<Toast>>,
}

impl ToastManager {
    pub fn new() -> Self {
        Self {
            toasts: Signal::new(VecDeque::new()),
        }
    }

    pub fn show(&mut self, message: String, toast_type: ToastType, duration: u64) {
        let toast = Toast::new(message, toast_type, duration);
        self.toasts.write().push_back(toast);
    }

    pub fn get_toasts(&self) -> Signal<VecDeque<Toast>> {
        self.toasts
    }
}

pub fn use_toast() -> Signal<ToastManager> {
    use_context::<Signal<ToastManager>>()
} 