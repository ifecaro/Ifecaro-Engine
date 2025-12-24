use crate::store::ToastStore;
use crate::types::{Toast, ToastOptions};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastProviderProps {
    #[props(optional)]
    pub options: Option<ToastOptions>,
    pub children: Element,
}

#[component]
pub fn ToastProvider(props: ToastProviderProps) -> Element {
    let options = props.options.clone().unwrap_or_default();
    let store = use_context_provider(|| ToastStore::new(options.clone()));
    store.update_options(options.clone());

    let toasts = store.toasts();
    let render_items = toasts.read().clone();
    let layout_class = if store.options().rtl { "toast-rtl" } else { "" };
    let position_class = store.options().position_class;
    let container_id = store.options().container_id;

    rsx! {
        div {
            id: "{container_id}",
            class: "{position_class} {layout_class}",
            for toast in render_items {
                ToastItem { toast: toast.clone() }
            }
        }
        {props.children}
    }
}

#[derive(Props, Clone, PartialEq)]
struct ToastItemProps {
    toast: Toast,
}

#[component]
fn ToastItem(props: ToastItemProps) -> Element {
    let store = use_context::<ToastStore>();
    let options = store.options();
    let close_button = options.close_button;
    let tap_to_dismiss = options.tap_to_dismiss;
    let progress_bar = options.progress_bar;
    let class_name = format!("{} {}", options.toast_class, props.toast.kind.class_name());
    let role = props.toast.kind.aria_role();
    let timeout_ms = props.toast.time_out.as_millis();

    let on_click_store = store.clone();
    let on_click = move |_| {
        if tap_to_dismiss {
            on_click_store.remove(props.toast.id);
        }
    };
    let close_store = store.clone();

    rsx! {
        div {
            class: "{class_name}",
            role: "{role}",
            onclick: on_click,
            if close_button {
                button {
                    class: "toast-close-button",
                    "aria-label": "close",
                    onclick: move |_| close_store.remove(props.toast.id),
                    "×"
                }
            }
            if let Some(title) = &props.toast.title {
                div { class: "toast-title", "{title}" }
            }
            div { class: "toast-message", "{props.toast.message}" }
            if progress_bar {
                div {
                    class: "toast-progress",
                    style: "animation: toast-progress linear {timeout_ms}ms;",
                }
            }
        }
    }
}
