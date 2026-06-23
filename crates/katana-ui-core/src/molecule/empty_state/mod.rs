mod layout;
mod render;
mod types;

use crate::render_model::{UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

pub use layout::{EmptyStateLayoutRect, EmptyStateLayoutSnapshot};
pub use types::{
    EmptyStateAction, EmptyStateActionId, EmptyStateAlignment, EmptyStateContractViolation,
    EmptyStateEvent, EmptyStateSize, EmptyStateTone,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmptyState {
    heading: String,
    body: Option<String>,
    icon: Option<String>,
    illustration: Option<String>,
    primary_action: Option<EmptyStateAction>,
    secondary_action: Option<EmptyStateAction>,
    tone: EmptyStateTone,
    size: EmptyStateSize,
    alignment: EmptyStateAlignment,
    state_id: UiStateId,
}

impl EmptyState {
    #[must_use]
    pub fn new(heading: impl Into<String>) -> Self {
        Self {
            heading: heading.into(),
            body: None,
            icon: None,
            illustration: None,
            primary_action: None,
            secondary_action: None,
            tone: EmptyStateTone::Neutral,
            size: EmptyStateSize::Default,
            alignment: EmptyStateAlignment::Center,
            state_id: UiStateId::next_for(UiNodeKind::EmptyState),
        }
    }

    #[must_use]
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    #[must_use]
    pub fn icon(mut self, value: impl Into<String>) -> Self {
        self.icon = Some(value.into());
        self
    }

    #[must_use]
    pub fn illustration(mut self, value: impl Into<String>) -> Self {
        self.illustration = Some(value.into());
        self
    }

    #[must_use]
    pub fn primary_action(mut self, value: EmptyStateAction) -> Self {
        self.primary_action = Some(value);
        self
    }

    #[must_use]
    pub fn secondary_action(mut self, value: EmptyStateAction) -> Self {
        self.secondary_action = Some(value);
        self
    }

    #[must_use]
    pub fn tone(mut self, value: EmptyStateTone) -> Self {
        self.tone = value;
        self
    }

    #[must_use]
    pub fn size(mut self, value: EmptyStateSize) -> Self {
        self.size = value;
        self
    }

    #[must_use]
    pub fn alignment(mut self, value: EmptyStateAlignment) -> Self {
        self.alignment = value;
        self
    }

    pub fn validate(&self) -> Result<(), EmptyStateContractViolation> {
        if self.heading.trim().is_empty() {
            return Err(EmptyStateContractViolation::MissingHeading);
        }
        if self.icon.is_some() && self.illustration.is_some() {
            return Err(EmptyStateContractViolation::IconAndIllustrationConflict);
        }
        Ok(())
    }

    #[must_use]
    pub fn apply_action(&self, action: EmptyStateActionId) -> Option<EmptyStateEvent> {
        let configured = match action {
            EmptyStateActionId::Primary => self.primary_action.as_ref(),
            EmptyStateActionId::Secondary => self.secondary_action.as_ref(),
        };
        configured.map(|it| EmptyStateEvent::Actioned {
            id: action,
            action_id: it.id.clone(),
        })
    }

    #[must_use]
    pub fn layout_snapshot(&self) -> EmptyStateLayoutSnapshot {
        layout::snapshot(self)
    }

    #[must_use]
    pub fn announce_payload(&self) -> String {
        format!("{}: {}", self.tone.announce_label(), self.heading.trim())
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }
}
