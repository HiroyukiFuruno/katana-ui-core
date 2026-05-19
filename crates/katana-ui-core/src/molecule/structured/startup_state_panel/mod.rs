mod actions;
mod render;
mod state;
mod types;

use crate::render_model::{UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

pub use actions::{StartupStatePanelAction, StartupStatePanelEvent};
pub use state::StartupState;
pub use types::StartupStatePanelOptions;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartupStatePanel {
    pub(super) label: String,
    pub(super) state_id: UiStateId,
    pub(super) options: StartupStatePanelOptions,
    pub(super) state: StartupState,
}

impl StartupStatePanel {
    const DEFAULT_LIVE_REGION_LABEL: &'static str = "Startup status";

    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::StartupStatePanel),
            options: StartupStatePanelOptions::default()
                .live_region_label(Self::DEFAULT_LIVE_REGION_LABEL),
            state: StartupState::Idle,
        }
    }

    #[must_use]
    pub fn option(mut self, value: StartupStatePanelOptions) -> Self {
        self.options = value;
        self
    }

    #[must_use]
    pub fn state(mut self, value: StartupState) -> Self {
        self.state = value;
        self
    }

    #[must_use]
    pub fn live_region_label(mut self, value: impl Into<String>) -> Self {
        self.options.live_region_label = value.into();
        self
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub fn state_model(&self) -> &StartupState {
        &self.state
    }

    #[must_use]
    pub fn options_model(&self) -> &StartupStatePanelOptions {
        &self.options
    }

    #[must_use]
    pub fn live_region_label_model(&self) -> &str {
        self.options.live_region_label.as_str()
    }

    #[must_use]
    pub fn accessibility_role(&self) -> &'static str {
        self.state.accessibility_role()
    }

    pub fn apply_action(&mut self, action: StartupStatePanelAction) -> Vec<StartupStatePanelEvent> {
        actions::apply(self, action)
    }
}
