use crate::interaction::{UiAction, UiActionResult};
use crate::render_model::{UiInteractionState, UiNode, UiNodeKind, UiStateId, UiStatusProps};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MoleculeState {
    pub(crate) state_id: UiStateId,
    pub(crate) open: bool,
    pub(crate) has_selection: bool,
    pub(crate) selected_index: usize,
    pub(crate) item_count: usize,
    pub(crate) value: String,
    pub(crate) placeholder: String,
    pub(crate) disabled: bool,
    pub(crate) readonly: bool,
    pub(crate) status: UiStatusProps,
}

impl MoleculeState {
    pub(crate) fn new(kind: UiNodeKind) -> Self {
        Self {
            state_id: UiStateId::next_for(kind),
            open: false,
            has_selection: false,
            selected_index: 0,
            item_count: 0,
            value: String::new(),
            placeholder: String::new(),
            disabled: false,
            readonly: false,
            status: UiStatusProps::default(),
        }
    }

    pub(crate) fn interaction(&self) -> UiInteractionState {
        UiInteractionState {
            open: self.open,
            has_selection: self.has_selection,
            selected_index: self.selected_index,
            item_count: self.item_count,
            value: self.value.clone(),
        }
    }

    pub(crate) fn node(&self, kind: UiNodeKind, label: impl Into<String>) -> UiNode {
        UiNode::from_state(kind, label, self.state_id.clone())
            .interaction(self.interaction())
            .placeholder(self.placeholder.clone())
            .disabled(self.disabled)
            .readonly(self.readonly)
            .status(self.status.clone())
    }

    pub(crate) fn apply_action(
        &mut self,
        action: &UiAction,
        close_on_select: bool,
    ) -> UiActionResult {
        let before = self.interaction();
        if action.target() != &self.state_id || self.disabled || self.readonly_blocks(action) {
            return UiActionResult::ignored(self.state_id.clone(), before);
        }
        self.apply_interaction(action, close_on_select);
        UiActionResult::handled(self.state_id.clone(), action, before, self.interaction())
    }

    fn readonly_blocks(&self, action: &UiAction) -> bool {
        self.readonly
            && matches!(
                action,
                UiAction::SetValue { .. } | UiAction::ClearValue { .. }
            )
    }

    fn apply_interaction(&mut self, action: &UiAction, close_on_select: bool) {
        match action {
            UiAction::Press { .. } => {}
            UiAction::SetOpen { open, .. } => self.open = *open,
            UiAction::SetSelectedIndex { selected_index, .. } => {
                self.has_selection = true;
                self.selected_index = *selected_index;
                if close_on_select {
                    self.open = false;
                }
            }
            UiAction::SetValue { value, .. } => self.value = value.clone(),
            UiAction::ClearValue { .. } => self.value.clear(),
            UiAction::Dismiss { .. } => self.open = false,
        }
    }
}
