use super::{UiNodeId, UiNodeKind, UiProps, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiNode {
    id: UiNodeId,
    kind: UiNodeKind,
    pub(super) props: UiProps,
    pub(super) children: Vec<UiNode>,
}

impl UiNode {
    #[must_use]
    pub fn new(kind: UiNodeKind, label: impl Into<String>) -> Self {
        Self::from_state(kind, label, UiStateId::next_for(kind))
    }

    pub(crate) fn from_state(
        kind: UiNodeKind,
        label: impl Into<String>,
        state_id: UiStateId,
    ) -> Self {
        Self {
            id: UiNodeId::next_for(kind),
            kind,
            props: UiProps::new(label, state_id),
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn stable_node_id(mut self, value: impl Into<UiNodeId>) -> Self {
        self.id = value.into();
        self
    }

    #[must_use]
    pub fn stable_state_id(mut self, value: impl Into<UiStateId>) -> Self {
        self.props.state_id = value.into();
        self
    }

    #[must_use]
    pub fn state_id(self, value: impl Into<UiStateId>) -> Self {
        self.stable_state_id(value)
    }

    #[must_use]
    pub fn kind(&self) -> UiNodeKind {
        self.kind
    }

    #[must_use]
    pub fn children(&self) -> &[UiNode] {
        &self.children
    }

    #[must_use]
    pub fn id(&self) -> &UiNodeId {
        &self.id
    }

    #[must_use]
    pub fn props(&self) -> &UiProps {
        &self.props
    }
}
