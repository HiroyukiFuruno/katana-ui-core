use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToastStackAction {
    Enqueue(super::ToastPayload),
    Dismiss(String),
    DismissAll,
    PauseHover(bool),
    FocusInside(bool),
    Resume,
    Tick(u64),
    ActivateToastAction { toast_id: String, action_id: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToastStackEvent {
    ToastShown {
        id: String,
    },
    ToastTimedOut {
        id: String,
    },
    ToastDismissed {
        id: String,
        reason: ToastDismissReason,
    },
    ToastQueued {
        id: String,
    },
    ToastReplaced {
        id: String,
        kind: ToastReplaceKind,
    },
    ToastQueueOverflow {
        dropped_id: String,
    },
    ToastPaused,
    ToastResumed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToastDismissReason {
    Manual,
    Timeout,
    Action(String),
    DismissAll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToastReplaceKind {
    Visible,
    Queued,
}
