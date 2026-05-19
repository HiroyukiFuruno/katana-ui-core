use crate::interaction::{UiAction, UiActionResult};
use crate::render_model::{
    UiCommonProps, UiInteractionState, UiNode, UiNodeKind, UiStateId, UiStatusProps,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MoleculeState {
    pub(crate) state_id: UiStateId,
    pub(crate) common: UiCommonProps,
    pub(crate) open: bool,
    pub(crate) has_selection: bool,
    pub(crate) selected_index: usize,
    pub(crate) item_count: usize,
    pub(crate) value: String,
    pub(crate) placeholder: String,
    pub(crate) disabled: bool,
    pub(crate) readonly: bool,
    pub(crate) status: UiStatusProps,
    pub(crate) transient: UiInteractionState,
}

impl MoleculeState {
    pub(crate) fn new(kind: UiNodeKind) -> Self {
        Self {
            state_id: UiStateId::next_for(kind),
            common: UiCommonProps::default(),
            open: false,
            has_selection: false,
            selected_index: 0,
            item_count: 0,
            value: String::new(),
            placeholder: String::new(),
            disabled: false,
            readonly: false,
            status: UiStatusProps::default(),
            transient: UiInteractionState::default(),
        }
    }

    pub(crate) fn interaction(&self) -> UiInteractionState {
        let mut interaction = self.transient.clone();
        interaction.open = self.open;
        interaction.has_selection = self.has_selection;
        interaction.selected_index = self.selected_index;
        interaction.item_count = self.item_count;
        interaction.value = self.value.clone();
        interaction
    }

    pub(crate) fn node(&self, kind: UiNodeKind, label: impl Into<String>) -> UiNode {
        UiNode::from_state(kind, label, self.state_id.clone())
            .common(self.common.clone())
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
            UiAction::SetFocus { focused, .. } => self.focused(*focused),
            UiAction::SetHover { hovered, .. } => self.hovered(*hovered),
            UiAction::SetActive { active, .. } => self.active(*active),
            UiAction::SetDragging { dragging, .. } => self.dragging(*dragging),
            UiAction::AnimationTick { phase, .. } => self.animation_phase(*phase),
            UiAction::SetReducedMotion { reduced_motion, .. } => {
                self.reduced_motion(*reduced_motion);
            }
            UiAction::SetCursorSelection {
                cursor,
                selection_start,
                selection_end,
                ..
            } => {
                self.cursor_selection(*cursor, *selection_start, *selection_end);
            }
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

    fn focused(&mut self, focused: bool) {
        self.transient.focused = focused;
    }

    fn hovered(&mut self, hovered: bool) {
        self.transient.hovered = hovered;
    }

    fn active(&mut self, active: bool) {
        self.transient.active = active;
    }

    fn dragging(&mut self, dragging: bool) {
        self.transient.dragging = dragging;
    }

    fn animation_phase(&mut self, phase: u16) {
        self.transient.animation_phase = phase;
    }

    fn reduced_motion(&mut self, reduced_motion: bool) {
        self.transient.reduced_motion = reduced_motion;
    }

    fn cursor_selection(&mut self, cursor: usize, selection_start: usize, selection_end: usize) {
        self.transient.cursor = cursor;
        self.transient.selection_start = selection_start;
        self.transient.selection_end = selection_end;
    }
}
