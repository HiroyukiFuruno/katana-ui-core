use super::action_policy::AtomActionPolicy;
use super::state::AtomState;
use crate::interaction::{
    ColorDragAction, ProgressAction, UiAction, UiActionResult, UiActionSource,
};
use crate::render_model::{UiNodeKind, UiProgressMode};
use crate::text_selection::{UiTextSelectionModel, UiTextSelectionRange};

impl AtomState {
    pub(super) fn apply_action_for_kind(
        &mut self,
        kind: UiNodeKind,
        action: &UiAction,
    ) -> UiActionResult {
        let before = self.interaction.clone();
        if action.target() != &self.state_id
            || AtomActionPolicy::blocks(kind, action, self.disabled, self.loading, self.readonly)
        {
            return UiActionResult::ignored(self.state_id.clone(), before);
        }
        self.apply_interaction_action(action);
        UiActionResult::handled(
            self.state_id.clone(),
            action,
            before,
            self.interaction.clone(),
        )
    }

    fn apply_interaction_action(&mut self, action: &UiAction) {
        match action {
            UiAction::Press { .. } => {}
            UiAction::SetFocus { focused, .. } => self.interaction.focused = *focused,
            UiAction::SetHover { hovered, .. } => self.interaction.hovered = *hovered,
            UiAction::SetActive { active, .. } => self.interaction.active = *active,
            UiAction::SetDragging { dragging, .. } => self.interaction.dragging = *dragging,
            UiAction::AnimationTick { phase, .. } => self.interaction.animation_phase = *phase,
            UiAction::SetReducedMotion { reduced_motion, .. } => {
                self.interaction.reduced_motion = *reduced_motion;
                self.loading_indicator.reduced_motion = *reduced_motion;
            }
            UiAction::SetCursorSelection {
                cursor,
                selection_start,
                selection_end,
                ..
            } => self.cursor_selection(*cursor, *selection_start, *selection_end),
            UiAction::CopySelection { .. } => {}
            UiAction::PasteText { text, .. } => self.paste_text(text),
            UiAction::SetSelectedIndex {
                selected_index,
                selected,
                source,
                ..
            } => self.selected_index(*selected_index, *selected, *source),
            UiAction::SetValue {
                value,
                progress,
                color_drag,
                ..
            } => self.apply_value_action(value, progress.as_ref(), color_drag.as_ref()),
            UiAction::ClearValue { .. } => self.interaction.value.clear(),
            _ => {}
        }
    }

    fn cursor_selection(&mut self, cursor: usize, selection_start: usize, selection_end: usize) {
        self.interaction.cursor = cursor;
        self.interaction.selection_start = selection_start;
        self.interaction.selection_end = selection_end;
    }

    fn paste_text(&mut self, text: &str) {
        let selection = UiTextSelectionRange::new(
            self.interaction.selection_start,
            self.interaction.selection_end,
        );
        let result =
            UiTextSelectionModel::replace_grapheme_range(&self.interaction.value, selection, text);
        self.interaction.value = result.text;
        self.interaction.cursor = result.selection.caret_position();
        self.interaction.selection_start = self.interaction.cursor;
        self.interaction.selection_end = self.interaction.cursor;
    }

    fn selected_index(&mut self, selected_index: usize, selected: bool, source: UiActionSource) {
        self.interaction.has_selection = selected;
        self.interaction.selected_index = selected_index;
        self.apply_checked_selection(source, selected);
    }

    fn apply_checked_selection(&mut self, source: UiActionSource, selected: bool) {
        match source {
            UiActionSource::Checkbox | UiActionSource::Radio | UiActionSource::Toggle => {
                self.checked = selected;
            }
            _ => {}
        }
    }

    fn apply_value_action(
        &mut self,
        value: &str,
        progress: Option<&ProgressAction>,
        color_drag: Option<&ColorDragAction>,
    ) {
        if let Some(progress) = progress {
            self.apply_progress_action(progress);
            return;
        }
        if let Some(color_drag) = color_drag {
            self.apply_color_drag_action(color_drag);
            return;
        }
        self.interaction.value = value.to_string();
    }

    fn apply_progress_action(&mut self, progress: &ProgressAction) {
        self.determinate = progress.determinate;
        self.progress_percent = progress.percent;
        self.interaction.value = progress.percent.to_string();
        self.loading_indicator.mode = if progress.determinate {
            UiProgressMode::Determinate
        } else {
            UiProgressMode::Indeterminate
        };
    }

    fn apply_color_drag_action(&mut self, color_drag: &ColorDragAction) {
        let value = color_drag.value.css_rgba();
        self.color_swatch.selected_color.clone_from(&value);
        self.interaction.value = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_model::UiStateId;

    #[test]
    fn internal_interaction_dispatch_ignores_non_atom_action() {
        let mut state = AtomState::enabled(UiNodeKind::Text);
        let before = state.interaction.clone();

        state.apply_interaction_action(&UiAction::dismiss(UiStateId::new("other")));

        assert_eq!(before, state.interaction);
    }

    #[test]
    fn non_checkable_selection_source_preserves_checked_state() {
        let mut state = AtomState::enabled(UiNodeKind::Text);
        state.checked = true;

        state.apply_checked_selection(UiActionSource::SelectBox, false);

        assert!(state.checked);
    }
}
