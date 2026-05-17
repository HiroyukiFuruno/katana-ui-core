use std::rc::Rc;

/// Severity level for NotificationToast.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotificationToastSeverity {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

/// Position for a stacked toast group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotificationToastPosition {
    #[default]
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

#[derive(Clone)]
pub(super) struct NotificationToastAction {
    pub(super) label: String,
    pub(super) on_action: Rc<dyn Fn()>,
}

#[derive(Clone)]
pub(super) struct NotificationToastProps {
    pub(super) message: String,
    pub(super) severity: NotificationToastSeverity,
    pub(super) action: Option<NotificationToastAction>,
    pub(super) duration: Option<u64>,
    pub(super) on_dismiss: Rc<dyn Fn()>,
}

/// Parameters for a single toast.
#[derive(Clone)]
pub struct NotificationToast {
    pub(super) props: NotificationToastProps,
}

#[derive(Clone)]
pub(super) struct ActiveToast {
    pub(super) id: u64,
    pub(super) toast: NotificationToast,
}

/// Parameters for a stacked toast group.
#[derive(Clone)]
pub struct NotificationToastStack {
    pub(super) props: Vec<NotificationToast>,
    pub(super) position: NotificationToastPosition,
    pub(super) max_visible: usize,
    pub(super) gap: f32,
}
