use crate::component::ComponentAction;
use crate::interaction::UiAction;
use crate::interaction::UiActionResult;
use crate::molecule::selection::ChoiceItem;
use crate::molecule::state::MoleculeState;
use crate::render_model::{UiNode, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentedToggle {
    label: String,
    state: MoleculeState,
    items: Vec<ChoiceItem>,
    keyboard_navigation: String,
    children: Vec<UiNode>,
}

impl SegmentedToggle {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state: MoleculeState::new(UiNodeKind::SegmentedToggle),
            items: Vec::new(),
            keyboard_navigation: String::new(),
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn item(mut self, item: ChoiceItem) -> Self {
        self.items.push(item);
        self.state.item_count = self.items.len();
        self
    }

    #[must_use]
    pub fn child(mut self, child: impl Into<UiNode>) -> Self {
        self.children.push(child.into());
        self
    }

    #[must_use]
    pub fn selected_index(mut self, value: usize) -> Self {
        self.state.has_selection = true;
        self.state.selected_index = value;
        self
    }

    #[must_use]
    pub fn item_count(mut self, value: usize) -> Self {
        self.state.item_count = value;
        self
    }

    #[must_use]
    pub fn keyboard_navigation(mut self, value: impl Into<String>) -> Self {
        self.keyboard_navigation = value.into();
        self
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state.state_id
    }

    #[must_use]
    pub fn items(&self) -> &[ChoiceItem] {
        &self.items
    }

    #[must_use]
    pub fn keyboard_navigation_model(&self) -> &str {
        &self.keyboard_navigation
    }
}

impl ComponentAction for SegmentedToggle {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = self.state.interaction();
        if action.target() != &self.state.state_id || self.state.disabled {
            return UiActionResult::ignored(self.state.state_id.clone(), before);
        }
        let UiAction::SetSelectedIndex {
            selected_index,
            selected,
            ..
        } = action
        else {
            return self.state.apply_action(action, false);
        };
        if self.is_disabled_segment(*selected_index) {
            return UiActionResult::ignored(self.state.state_id.clone(), before);
        }
        self.state.has_selection = *selected;
        self.state.selected_index = *selected_index;
        UiActionResult::handled(
            self.state.state_id.clone(),
            action,
            before,
            self.state.interaction(),
        )
    }
}

impl From<SegmentedToggle> for UiNode {
    fn from(value: SegmentedToggle) -> Self {
        let mut node = value.state.node(UiNodeKind::SegmentedToggle, value.label);
        for child in value.children {
            node = node.child(child);
        }
        node
    }
}

impl SegmentedToggle {
    fn is_disabled_segment(&self, selected_index: usize) -> bool {
        self.items
            .get(selected_index)
            .map(|item| item.disabled)
            .unwrap_or(false)
    }
}
