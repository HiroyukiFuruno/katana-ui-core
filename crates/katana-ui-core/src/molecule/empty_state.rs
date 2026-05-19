use crate::atom::{Button, Icon, Text};
use crate::render_model::{UiNode, UiNodeKind, UiSize, UiStateId, UiTone};
use serde::{Deserialize, Serialize};

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
        EmptyStateLayoutSnapshot {
            size: self.size,
            alignment: self.alignment,
            has_body: self.body.is_some(),
            action_count: usize::from(self.primary_action.is_some())
                + usize::from(self.secondary_action.is_some()),
        }
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }
}

impl From<EmptyState> for UiNode {
    fn from(value: EmptyState) -> Self {
        let mut node = UiNode::from_state(UiNodeKind::EmptyState, value.heading, value.state_id)
            .tone(value.tone.into())
            .size(value.size.into());
        if let Some(icon) = value.icon {
            node = node.child(Icon::new(icon));
        }
        if let Some(illustration) = value.illustration {
            node = node.child(Icon::new(illustration));
        }
        if let Some(body) = value.body {
            node = node.child(Text::new(body));
        }
        if let Some(action) = value.primary_action {
            node = node.child(Button::new(action.label));
        }
        if let Some(action) = value.secondary_action {
            node = node.child(Button::new(action.label));
        }
        node
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmptyStateAction {
    pub id: String,
    pub label: String,
}

impl EmptyStateAction {
    #[must_use]
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmptyStateActionId {
    Primary,
    Secondary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmptyStateEvent {
    Actioned {
        id: EmptyStateActionId,
        action_id: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmptyStateContractViolation {
    MissingHeading,
    IconAndIllustrationConflict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmptyStateTone {
    Neutral,
    Subtle,
    Accent,
    Warning,
    Danger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmptyStateSize {
    Compact,
    Default,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmptyStateAlignment {
    Center,
    Leading,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmptyStateLayoutSnapshot {
    pub size: EmptyStateSize,
    pub alignment: EmptyStateAlignment,
    pub has_body: bool,
    pub action_count: usize,
}

impl From<EmptyStateTone> for UiTone {
    fn from(value: EmptyStateTone) -> Self {
        match value {
            EmptyStateTone::Neutral => Self::Neutral,
            EmptyStateTone::Subtle | EmptyStateTone::Accent => Self::Accent,
            EmptyStateTone::Warning => Self::Warning,
            EmptyStateTone::Danger => Self::Danger,
        }
    }
}

impl From<EmptyStateSize> for UiSize {
    fn from(value: EmptyStateSize) -> Self {
        match value {
            EmptyStateSize::Compact => Self::Small,
            EmptyStateSize::Default => Self::Medium,
            EmptyStateSize::Large => Self::Large,
        }
    }
}
