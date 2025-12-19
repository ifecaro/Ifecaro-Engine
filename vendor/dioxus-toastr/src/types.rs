use std::time::Duration;

pub type ToastId = u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastKind {
    Error,
    Info,
    Success,
    Warning,
}

impl ToastKind {
    pub(crate) fn class_name(self) -> &'static str {
        match self {
            ToastKind::Error => "toast-error",
            ToastKind::Info => "toast-info",
            ToastKind::Success => "toast-success",
            ToastKind::Warning => "toast-warning",
        }
    }

    pub(crate) fn aria_role(self) -> &'static str {
        match self {
            ToastKind::Error => "alert",
            ToastKind::Info => "status",
            ToastKind::Success => "status",
            ToastKind::Warning => "alert",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ToastOptions {
    pub tap_to_dismiss: bool,
    pub toast_class: &'static str,
    pub container_id: &'static str,
    pub position_class: &'static str,
    pub time_out: Duration,
    pub extended_time_out: Duration,
    pub close_button: bool,
    pub newest_on_top: bool,
    pub prevent_duplicates: bool,
    pub progress_bar: bool,
    pub rtl: bool,
}

impl Default for ToastOptions {
    fn default() -> Self {
        Self {
            tap_to_dismiss: true,
            toast_class: "toast",
            container_id: "toast-container",
            position_class: "toast-top-right",
            time_out: Duration::from_millis(5000),
            extended_time_out: Duration::from_millis(1000),
            close_button: false,
            newest_on_top: true,
            prevent_duplicates: false,
            progress_bar: false,
            rtl: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ToastRequest {
    pub kind: ToastKind,
    pub message: String,
    pub title: Option<String>,
    pub time_out: Option<Duration>,
}

impl ToastRequest {
    pub fn new(kind: ToastKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            title: None,
            time_out: None,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.time_out = Some(timeout);
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Toast {
    pub id: ToastId,
    pub kind: ToastKind,
    pub message: String,
    pub title: Option<String>,
    pub time_out: Duration,
}
