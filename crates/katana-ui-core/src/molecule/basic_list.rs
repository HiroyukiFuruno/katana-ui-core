use crate::interaction::{VirtualRange, VirtualizationConfig};
use crate::molecule::virtualization::MoleculeVirtualization;
use crate::render_model::{UiCommonProps, UiInteractionState, UiNode, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct List {
    label: String,
    state_id: UiStateId,
    common: UiCommonProps,
    virtualization: Option<VirtualizationConfig>,
    selected_index: Option<usize>,
    empty_state: Option<UiNode>,
    row_theme_slot: String,
    children: Vec<UiNode>,
}

impl List {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::List),
            common: UiCommonProps::default(),
            virtualization: None,
            selected_index: None,
            empty_state: None,
            row_theme_slot: String::new(),
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn common(mut self, value: UiCommonProps) -> Self {
        self.common = value;
        self
    }

    #[must_use]
    pub fn child(mut self, child: impl Into<UiNode>) -> Self {
        self.children.push(child.into());
        self
    }

    #[must_use]
    pub fn children(&self) -> &[UiNode] {
        &self.children
    }

    #[must_use]
    pub fn selected_index(mut self, value: usize) -> Self {
        self.selected_index = Some(value);
        self
    }

    #[must_use]
    pub fn empty_state(mut self, child: impl Into<UiNode>) -> Self {
        self.empty_state = Some(child.into());
        self
    }

    #[must_use]
    pub fn row_theme_slot(mut self, value: impl Into<String>) -> Self {
        self.row_theme_slot = value.into();
        self
    }

    #[must_use]
    pub fn virtualization(mut self, value: VirtualizationConfig) -> Self {
        self.virtualization = Some(value);
        self
    }

    #[must_use]
    pub fn virtual_range_model(&self) -> Option<VirtualRange> {
        MoleculeVirtualization::range(&self.virtualization, self.children.len())
    }
}

impl From<List> for UiNode {
    fn from(value: List) -> Self {
        let range = value.virtual_range_model();
        let item_count = value.children.len();
        let mut common = value.common;
        if !value.row_theme_slot.is_empty() {
            common.theme_slot = value.row_theme_slot;
        }
        let mut node = UiNode::from_state(UiNodeKind::List, value.label, value.state_id)
            .common(common)
            .interaction(list_interaction(
                item_count,
                value.selected_index,
                range.as_ref(),
            ));
        let children = MoleculeVirtualization::slice_by_range(value.children, range.as_ref());
        if children.is_empty() {
            if let Some(empty_state) = value.empty_state {
                node = node.child(empty_state);
            }
            return node;
        }
        for child in children {
            node = node.child(child);
        }
        node
    }
}

fn list_interaction(
    item_count: usize,
    selected_index: Option<usize>,
    range: Option<&VirtualRange>,
) -> UiInteractionState {
    let base = UiInteractionState {
        item_count,
        has_selection: selected_index.is_some(),
        selected_index: selected_index.unwrap_or_default(),
        ..UiInteractionState::default()
    };
    MoleculeVirtualization::interaction(base, range)
}
