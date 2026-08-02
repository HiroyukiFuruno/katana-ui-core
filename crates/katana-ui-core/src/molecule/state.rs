use crate::interaction::{UiAction, UiActionResult};
use crate::render_model::{
    UiCommonProps, UiInteractionState, UiNode, UiNodeKind, UiStateId, UiStatusProps,
};
use crate::text_selection::{UiTextSelectionModel, UiTextSelectionRange};
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
    pub(crate) invalid: bool,
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
            invalid: false,
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
            .invalid(self.invalid)
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
                UiAction::SetValue { .. }
                    | UiAction::ClearValue { .. }
                    | UiAction::PasteText { .. }
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
            UiAction::CopySelection { .. } => {}
            UiAction::PasteText { text, .. } => self.paste_text(text),
            UiAction::Dismiss { .. } => self.open = false,
            UiAction::InvokeCallback { .. }
            | UiAction::ScrollTo { .. }
            | UiAction::ScrollBy { .. }
            | UiAction::ScrollIntoView { .. }
            | UiAction::SetScrollbarVisibility { .. }
            | UiAction::SplitPaneSetRatio { .. }
            | UiAction::SplitPaneResizeBy { .. }
            | UiAction::SplitPaneResetRatio { .. }
            | UiAction::SplitPaneStartResize { .. }
            | UiAction::SplitPaneEndResize { .. }
            | UiAction::TabSelect { .. }
            | UiAction::TabAdd { .. }
            | UiAction::TabClose { .. }
            | UiAction::TabCloseOthers { .. }
            | UiAction::TabCloseToRight { .. }
            | UiAction::TabCloseToLeft { .. }
            | UiAction::TabCloseAll { .. }
            | UiAction::TabRestoreClosed { .. }
            | UiAction::TabPin { .. }
            | UiAction::TabMove { .. }
            | UiAction::TabMoveToGroup { .. }
            | UiAction::TabMoveToNewGroup { .. }
            | UiAction::TabMoveGroup { .. }
            | UiAction::TabRenameGroup { .. }
            | UiAction::TabSetGroupColor { .. }
            | UiAction::TabUngroup { .. }
            | UiAction::TabCloseGroup { .. }
            | UiAction::TabToggleGroupCollapse { .. } => {}
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

    fn paste_text(&mut self, text: &str) {
        let selection =
            UiTextSelectionRange::new(self.transient.selection_start, self.transient.selection_end);
        let result = UiTextSelectionModel::replace_grapheme_range(&self.value, selection, text);
        self.value = result.text;
        self.transient.cursor = result.selection.caret_position();
        self.transient.selection_start = self.transient.cursor;
        self.transient.selection_end = self.transient.cursor;
    }
}

#[cfg(test)]
mod tests {
    use super::MoleculeState;
    use crate::interaction::UiAction;
    use crate::render_model::{UiNodeKind, UiStateId};

    #[test]
    fn molecule_state_applies_complete_neutral_interaction_sequence() {
        let mut state = MoleculeState::new(UiNodeKind::ComboBox);
        let target = state.state_id.clone();
        state.open = true;
        state.value = "ab".to_string();

        for action in [
            UiAction::press(target.clone()),
            UiAction::focus(target.clone()),
            UiAction::hover(target.clone(), true),
            UiAction::active(target.clone(), true),
            UiAction::dragging(target.clone(), true),
            UiAction::animation_tick(target.clone(), 7),
            UiAction::reduced_motion(target.clone(), true),
            UiAction::cursor_selection(target.clone(), 1, 0, 1),
            UiAction::paste_text(target.clone(), "Z"),
            UiAction::set_open(target.clone(), true),
            UiAction::set_selected_index(target.clone(), 3),
            UiAction::set_value(target.clone(), "value"),
            UiAction::copy_selection(target.clone()),
            UiAction::invoke_callback(target.clone(), "noop"),
        ] {
            assert!(state.apply_action(&action, false).handled);
        }

        let interaction = state.interaction();
        assert!(interaction.focused);
        assert!(interaction.hovered);
        assert!(interaction.active);
        assert!(interaction.dragging);
        assert_eq!(7, interaction.animation_phase);
        assert!(interaction.reduced_motion);
        assert!(interaction.has_selection);
        assert_eq!(3, interaction.selected_index);
        assert_eq!("value", interaction.value);

        assert!(
            state
                .apply_action(&UiAction::clear_value(target.clone()), false)
                .handled
        );
        assert!(state.value.is_empty());
        assert!(
            state
                .apply_action(&UiAction::dismiss(target.clone()), false)
                .handled
        );
        assert!(!state.open);

        let node = state.node(UiNodeKind::ComboBox, "Combo");
        assert_eq!(UiNodeKind::ComboBox, node.kind());
        assert_eq!(target, node.props().state_id);
    }

    #[test]
    fn molecule_state_enforces_target_disabled_readonly_and_close_policy() {
        let mut state = MoleculeState::new(UiNodeKind::Input);
        let target = state.state_id.clone();
        let foreign = UiAction::set_value(UiStateId::new("foreign"), "ignored");
        assert!(!state.apply_action(&foreign, false).handled);

        state.disabled = true;
        assert!(
            !state
                .apply_action(&UiAction::set_value(target.clone(), "ignored"), false)
                .handled
        );
        state.disabled = false;
        state.readonly = true;
        for action in [
            UiAction::set_value(target.clone(), "ignored"),
            UiAction::clear_value(target.clone()),
            UiAction::paste_text(target.clone(), "ignored"),
        ] {
            assert!(!state.apply_action(&action, false).handled);
        }

        state.readonly = false;
        state.open = true;
        assert!(
            state
                .apply_action(&UiAction::set_selected_index(target, 2), true)
                .handled
        );
        assert!(!state.open);
    }
}
