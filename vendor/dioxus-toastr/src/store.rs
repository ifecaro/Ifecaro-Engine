use crate::types::{Toast, ToastId, ToastOptions, ToastRequest};
use dioxus::prelude::*;
use futures_timer::Delay;
use std::time::Duration;

#[derive(Clone)]
pub struct ToastStore {
    toasts: Signal<Vec<Toast>>,
    next_id: Signal<ToastId>,
    options: Signal<ToastOptions>,
}

impl ToastStore {
    pub fn new(options: ToastOptions) -> Self {
        Self {
            toasts: Signal::new(Vec::new()),
            next_id: Signal::new(1),
            options: Signal::new(options),
        }
    }

    pub fn options(&self) -> ToastOptions {
        self.options.read().clone()
    }

    pub fn update_options(&self, options: ToastOptions) {
        let mut signal = self.options;
        signal.set(options);
    }

    pub fn toasts(&self) -> Signal<Vec<Toast>> {
        self.toasts
    }

    pub fn clear(&self) {
        let mut signal = self.toasts;
        signal.set(Vec::new());
    }

    pub fn remove(&self, id: ToastId) {
        let mut signal = self.toasts;
        let mut items = signal.write();
        items.retain(|toast| toast.id != id);
    }

    pub fn push(&self, request: ToastRequest) -> ToastId {
        let options = self.options.read().clone();
        let time_out = request.time_out.unwrap_or(options.time_out);
        if options.prevent_duplicates {
            let items = self.toasts.read();
            let is_duplicate = items.iter().any(|toast| {
                toast.message == request.message
                    && toast.title == request.title
                    && toast.kind == request.kind
            });
            if is_duplicate {
                return 0;
            }
        }

        let id = {
            let mut signal = self.next_id;
            let mut next_id = signal.write();
            let id = *next_id;
            *next_id += 1;
            id
        };

        let toast = Toast {
            id,
            kind: request.kind,
            message: request.message,
            title: request.title,
            time_out,
        };

        {
            let mut signal = self.toasts;
            let mut items = signal.write();
            if options.newest_on_top {
                items.insert(0, toast);
            } else {
                items.push(toast);
            }
        }

        if time_out != Duration::from_millis(0) {
            let mut toasts = self.toasts;
            spawn(async move {
                Delay::new(time_out).await;
                let mut items = toasts.write();
                items.retain(|toast| toast.id != id);
            });
        }

        id
    }
}
