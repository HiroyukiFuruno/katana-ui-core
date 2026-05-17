use super::{UiNodeId, UiNodeKind, UiStateId};
use crate::theme::ThemeSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiProps {
    pub label: String,
    pub state_id: UiStateId,
    pub disabled: bool,
    pub focusable: bool,
    pub accessibility_label: String,
    pub interaction: UiInteractionState,
    pub theme_id: String,
    pub style_classes: Vec<String>,
}

impl UiProps {
    #[must_use]
    pub fn new(label: impl Into<String>, state_id: UiStateId) -> Self {
        Self {
            label: label.into(),
            state_id,
            disabled: false,
            focusable: false,
            accessibility_label: String::new(),
            interaction: UiInteractionState::default(),
            theme_id: String::new(),
            style_classes: Vec::new(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiInteractionState {
    pub open: bool,
    pub has_selection: bool,
    pub selected_index: usize,
    pub item_count: usize,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiNode {
    id: UiNodeId,
    kind: UiNodeKind,
    props: UiProps,
    children: Vec<UiNode>,
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
    pub fn disabled(mut self, value: bool) -> Self {
        self.props.disabled = value;
        self
    }

    #[must_use]
    pub fn focusable(mut self, value: bool) -> Self {
        self.props.focusable = value;
        self
    }

    #[must_use]
    pub fn accessibility_label(mut self, value: impl Into<String>) -> Self {
        self.props.accessibility_label = value.into();
        self
    }

    #[must_use]
    pub fn interaction(mut self, value: UiInteractionState) -> Self {
        self.props.interaction = value;
        self
    }

    #[must_use]
    pub fn theme(mut self, value: &ThemeSnapshot) -> Self {
        self.props.theme_id = value.id.as_str().to_string();
        self
    }

    #[must_use]
    pub fn style_class(mut self, value: impl Into<String>) -> Self {
        self.props.style_classes.push(value.into());
        self
    }

    #[must_use]
    pub fn style_classes(mut self, values: impl IntoIterator<Item = String>) -> Self {
        self.props.style_classes.extend(values);
        self
    }

    #[must_use]
    pub fn child(mut self, child: impl Into<UiNode>) -> Self {
        self.children.push(child.into());
        self
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTree {
    root: UiNode,
}

impl UiTree {
    #[must_use]
    pub fn new(root: impl Into<UiNode>) -> Self {
        Self { root: root.into() }
    }

    #[must_use]
    pub fn root(&self) -> &UiNode {
        &self.root
    }
}
