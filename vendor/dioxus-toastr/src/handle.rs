use crate::store::ToastStore;
use crate::types::{ToastId, ToastKind, ToastRequest};
use dioxus::prelude::use_context;

#[derive(Clone)]
pub struct ToastHandle {
    store: ToastStore,
}

impl ToastHandle {
    pub fn push(&self, request: ToastRequest) -> ToastId {
        self.store.push(request)
    }

    pub fn success(&self, message: impl Into<String>) -> ToastId {
        self.store.push(ToastRequest::new(ToastKind::Success, message))
    }

    pub fn info(&self, message: impl Into<String>) -> ToastId {
        self.store.push(ToastRequest::new(ToastKind::Info, message))
    }

    pub fn warning(&self, message: impl Into<String>) -> ToastId {
        self.store.push(ToastRequest::new(ToastKind::Warning, message))
    }

    pub fn error(&self, message: impl Into<String>) -> ToastId {
        self.store.push(ToastRequest::new(ToastKind::Error, message))
    }

    pub fn remove(&self, id: ToastId) {
        self.store.remove(id);
    }

    pub fn clear(&self) {
        self.store.clear();
    }
}

pub fn use_toast() -> ToastHandle {
    let store = use_context::<ToastStore>();
    ToastHandle { store }
}
