use crate::render_model::{UiStateId, UiTone};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToastPayload {
    pub id: String,
    pub message: String,
    pub title: Option<String>,
    pub severity: UiTone,
    pub duration_ms: Option<u64>,
    pub actions: Vec<ToastAction>,
}

impl ToastPayload {
    #[must_use]
    pub fn new(id: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            message: message.into(),
            title: None,
            severity: UiTone::Neutral,
            duration_ms: None,
            actions: Vec::new(),
        }
    }

    #[must_use]
    pub fn severity(mut self, value: UiTone) -> Self {
        self.severity = value;
        self
    }

    #[must_use]
    pub fn duration_ms(mut self, value: u64) -> Self {
        self.duration_ms = Some(value);
        self
    }

    #[must_use]
    pub fn action(mut self, value: ToastAction) -> Self {
        self.actions.push(value);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToastAction {
    pub id: String,
    pub label: String,
    pub kind: ToastActionKind,
}

impl ToastAction {
    #[must_use]
    pub fn new(id: impl Into<String>, label: impl Into<String>, kind: ToastActionKind) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            kind,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveToast {
    pub state_id: UiStateId,
    pub payload: ToastPayload,
    pub remaining_duration_ms: Option<u64>,
}

impl ActiveToast {
    #[must_use]
    pub fn new(payload: ToastPayload, default_duration_ms: u64) -> Self {
        let duration = payload.duration_ms.unwrap_or(default_duration_ms);
        Self {
            state_id: UiStateId::next_for(crate::render_model::UiNodeKind::NotificationToast),
            payload,
            remaining_duration_ms: (duration > 0).then_some(duration),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToastActionKind {
    Primary,
    Secondary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToastPosition {
    TopStart,
    TopCenter,
    TopEnd,
    BottomStart,
    BottomCenter,
    BottomEnd,
}

impl ToastPosition {
    #[must_use]
    pub const fn stack_direction(self) -> ToastStackDirection {
        match self {
            Self::TopStart | Self::TopCenter | Self::TopEnd => ToastStackDirection::Down,
            Self::BottomStart | Self::BottomCenter | Self::BottomEnd => ToastStackDirection::Up,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToastStackDirection {
    Down,
    Up,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToastDedupStrategy {
    None,
    ById,
    ByIdAndSeverity,
}
