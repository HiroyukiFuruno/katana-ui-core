use super::{MotionPrimitiveKind, MotionSpec};
use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};
use crate::render_model::{
    UiInteractionState, UiLoadingProps, UiNode, UiNodeKind, UiProgressMode, UiStateId,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplashStatus {
    Idle,
    Loading,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplashBackground {
    Solid,
    Gradient,
    Image,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplashSize {
    Embedded,
    Window,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplashEvent {
    None,
    StatusChanged(SplashStatus),
    Retried,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SplashScreen {
    label: String,
    state_id: UiStateId,
    status: SplashStatus,
    background: SplashBackground,
    size: SplashSize,
    progress_percent: Option<u8>,
    motion: MotionSpec,
    last_event: SplashEvent,
}

impl SplashScreen {
    #[must_use]
    pub fn new(label: impl Into<String>, motion: MotionSpec) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::SplashScreen),
            status: SplashStatus::Idle,
            background: SplashBackground::Solid,
            size: SplashSize::Embedded,
            progress_percent: None,
            motion,
            last_event: SplashEvent::None,
        }
    }

    #[must_use]
    pub fn status(mut self, status: SplashStatus) -> Self {
        self.status = status;
        self
    }

    #[must_use]
    pub fn progress(mut self, progress_percent: Option<u8>) -> Self {
        self.progress_percent = progress_percent;
        self
    }

    #[must_use]
    pub fn size(mut self, size: SplashSize) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub fn accessibility_role(&self) -> &'static str {
        if self.status == SplashStatus::Error {
            "alert"
        } else {
            "status"
        }
    }

    #[must_use]
    pub fn last_event(&self) -> &SplashEvent {
        &self.last_event
    }
}

impl ComponentAction for SplashScreen {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = state(self);
        if action.target() != &self.state_id {
            return UiActionResult::ignored(self.state_id.clone(), before);
        }
        match action {
            UiAction::SetSelectedIndex { selected_index, .. } => {
                self.status = match selected_index {
                    1 => SplashStatus::Loading,
                    2 => SplashStatus::Error,
                    _ => SplashStatus::Idle,
                };
                self.last_event = SplashEvent::StatusChanged(self.status);
            }
            UiAction::Press { .. } => {
                self.status = SplashStatus::Idle;
                self.last_event = SplashEvent::Retried;
            }
            UiAction::Dismiss { .. } => self.last_event = SplashEvent::Cancelled,
            _ => return UiActionResult::ignored(self.state_id.clone(), before),
        }
        UiActionResult::handled(self.state_id.clone(), action, before, state(self))
    }
}

impl From<SplashScreen> for UiNode {
    fn from(value: SplashScreen) -> Self {
        let state = state(&value);
        let accessibility_role = value.accessibility_role();
        let loading = UiLoadingProps {
            mode: if value.progress_percent.is_some() {
                UiProgressMode::Determinate
            } else {
                UiProgressMode::Indeterminate
            },
            ..UiLoadingProps::default()
        };
        UiNode::from_state(UiNodeKind::SplashScreen, value.label, value.state_id)
            .interaction(state)
            .loading_indicator(loading)
            .progress(
                value.progress_percent.is_some(),
                value.progress_percent.unwrap_or(0),
            )
            .accessibility_label(accessibility_role)
            .style_class(format!("{:?}", value.background))
            .style_class(format!("{:?}", value.size))
            .style_class(format!("{:?}", value.motion.primitive))
    }
}

fn state(value: &SplashScreen) -> UiInteractionState {
    UiInteractionState {
        active: value.status == SplashStatus::Loading,
        has_selection: value.status == SplashStatus::Error,
        value: value.progress_percent.unwrap_or(0).to_string(),
        reduced_motion: value.motion.primitive == MotionPrimitiveKind::Fade
            && value.motion.duration_ms == 0,
        ..UiInteractionState::default()
    }
}
