use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiModalPresentation {
    NativeWindow,
    OverlayDialog,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiModalParentInteraction {
    Block,
    Allow,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiModalSize {
    Small,
    Medium,
    Large,
    Custom { width_px: u16 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiModalProps {
    pub presentation: UiModalPresentation,
    pub title: String,
    pub size: UiModalSize,
    pub footer: String,
    pub backdrop: String,
    pub focus_trap: bool,
    pub focus_return: String,
    pub dismiss_policy: String,
    pub dismiss_on_escape: bool,
    pub dismiss_on_backdrop: bool,
    pub parent_interaction: UiModalParentInteraction,
}

impl Default for UiModalProps {
    fn default() -> Self {
        Self {
            presentation: UiModalPresentation::OverlayDialog,
            title: String::new(),
            size: UiModalSize::Medium,
            footer: String::new(),
            backdrop: String::new(),
            focus_trap: false,
            focus_return: String::new(),
            dismiss_policy: String::new(),
            dismiss_on_escape: false,
            dismiss_on_backdrop: false,
            parent_interaction: UiModalParentInteraction::Block,
        }
    }
}
