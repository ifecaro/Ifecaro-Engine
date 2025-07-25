use dioxus::prelude::*;
use crate::contexts::toast_context::use_toast;
use gloo_timers::callback::Timeout;

#[derive(Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum ToastType {
    Success,
    Error,
    Warning,
    Info,
}

#[derive(Clone, PartialEq)]
pub struct Toast {
    pub id: u32,
    pub message: String,
    pub toast_type: ToastType,
    pub duration: u64,
}

impl Toast {
    pub fn new(message: String, toast_type: ToastType, duration: u64) -> Self {
        Self {
            id: rand::random(),
            message,
            toast_type,
            duration,
        }
    }
}

#[component]
pub fn ToastContainer() -> Element {
    let toast_manager = use_toast();
    let mut toasts = toast_manager.read().get_toasts();

    rsx! {
        div {
            class: "fixed bottom-4 right-4 flex flex-col space-y-2 z-[9999]",
            for toast in toasts.read().iter() {
                ToastItem {
                    key: "{toast.id}",
                    toast: toast.clone(),
                    on_dismiss: move |id| {
                        toasts.write().retain(|t| t.id != id);
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ToastItemProps {
    toast: Toast,
    on_dismiss: EventHandler<u32>,
}

#[component]
fn ToastItem(props: ToastItemProps) -> Element {
    let toast = props.toast.clone();

    // Track visibility to drive enter/exit animations.
    // We start in the visible state so the toast plays the "in" animation immediately after mount.
    let mut is_visible = use_signal(|| true);

    let bg_color = match toast.toast_type {
        ToastType::Success => "bg-green-500",
        ToastType::Error => "bg-red-500",
        ToastType::Warning => "bg-yellow-500",
        ToastType::Info => "bg-blue-500",
    };

    // Start auto-dismiss timer with exit animation support
    use_effect(move || {
        let on_dismiss = props.on_dismiss.clone();
        let id = toast.id;
        let mut is_visible = is_visible.clone();

        // After `duration`, trigger fade/slide-out, then remove from list after 400 ms (animation length)
        Timeout::new(toast.duration as u32, move || {
            is_visible.set(false);
            Timeout::new(400, move || {
                on_dismiss.call(id);
            })
            .forget();
        })
        .forget();
    });

    // Use custom animation utility classes generated in `tailwind.css`
    // (see `toast-in` and `toast-out` keyframes defined under the `@layer utilities` section).
    // These classes handle both the opacity and translate-Y transitions so we only need to
    // toggle between them depending on visibility.
    let animation_class = if *is_visible.read() {
        "toast-animate-in"
    } else {
        "toast-animate-out"
    };

    rsx! {
        div {
            class: "text-white px-6 py-3 rounded shadow-lg transform will-change-transform {bg_color} {animation_class}",
            onclick: move |_| {
                // Trigger exit animation first, then remove after it finishes (400ms)
                is_visible.set(false);
                Timeout::new(400, move || {
                    let on_dismiss = props.on_dismiss.clone();
                    let id = toast.id;
                    on_dismiss.call(id);
                }).forget();
            },
            "{toast.message}"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toast_creation_with_all_types() {
        // Test Success type
        let success_toast = Toast::new(
            "Success message".to_string(),
            ToastType::Success,
            5000
        );
        assert_eq!(success_toast.message, "Success message");
        assert_eq!(success_toast.toast_type, ToastType::Success);
        assert_eq!(success_toast.duration, 5000);

        // Test Error type
        let error_toast = Toast::new(
            "Error message".to_string(),
            ToastType::Error,
            3000
        );
        assert_eq!(error_toast.message, "Error message");
        assert_eq!(error_toast.toast_type, ToastType::Error);
        assert_eq!(error_toast.duration, 3000);

        // Test Warning type
        let warning_toast = Toast::new(
            "Warning message".to_string(),
            ToastType::Warning,
            4000
        );
        assert_eq!(warning_toast.message, "Warning message");
        assert_eq!(warning_toast.toast_type, ToastType::Warning);
        assert_eq!(warning_toast.duration, 4000);

        // Test Info type
        let info_toast = Toast::new(
            "Info message".to_string(),
            ToastType::Info,
            2000
        );
        assert_eq!(info_toast.message, "Info message");
        assert_eq!(info_toast.toast_type, ToastType::Info);
        assert_eq!(info_toast.duration, 2000);
    }

    #[test]
    fn test_toast_type_matching() {
        let success_bg = match ToastType::Success {
            ToastType::Success => "bg-green-500",
            ToastType::Error => "bg-red-500",
            ToastType::Warning => "bg-yellow-500",
            ToastType::Info => "bg-blue-500",
        };
        assert_eq!(success_bg, "bg-green-500");

        let error_bg = match ToastType::Error {
            ToastType::Success => "bg-green-500",
            ToastType::Error => "bg-red-500",
            ToastType::Warning => "bg-yellow-500",
            ToastType::Info => "bg-blue-500",
        };
        assert_eq!(error_bg, "bg-red-500");

        let warning_bg = match ToastType::Warning {
            ToastType::Success => "bg-green-500",
            ToastType::Error => "bg-red-500",
            ToastType::Warning => "bg-yellow-500",
            ToastType::Info => "bg-blue-500",
        };
        assert_eq!(warning_bg, "bg-yellow-500");

        let info_bg = match ToastType::Info {
            ToastType::Success => "bg-green-500",
            ToastType::Error => "bg-red-500",
            ToastType::Warning => "bg-yellow-500",
            ToastType::Info => "bg-blue-500",
        };
        assert_eq!(info_bg, "bg-blue-500");
    }
} 