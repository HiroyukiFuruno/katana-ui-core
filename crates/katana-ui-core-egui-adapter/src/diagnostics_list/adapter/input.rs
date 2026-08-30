//! Keyboard focus and one-shot closure handling for diagnostics.

use super::super::accessibility::DiagnosticsAccessibility;
use super::super::identity::DiagnosticsTargetIdentity;
use super::super::types::EguiDiagnosticsListOutput;
use super::EguiDiagnosticsListAdapter;
use katana_ui_core::molecule::{DiagnosticKeyboardInput, DiagnosticsList, DiagnosticsListAction};

impl EguiDiagnosticsListAdapter {
    pub(super) fn process_keyboard(
        &mut self,
        ui: &egui::Ui,
        output: &mut EguiDiagnosticsListOutput,
        row_heights: &[f32],
        viewport: egui::Rect,
        max_scroll_y: f32,
        diagnostics: &mut DiagnosticsList,
    ) {
        let snapshot = diagnostics.render_snapshot();
        let focused_response = ui.memory(|memory| memory.focused());
        let item_has_focus = self
            .focused_item
            .as_deref()
            .is_some_and(|id| focused_response == Some(self.id.with(id)));
        let scope_has_focus = self.focused_scope.as_deref().is_some_and(|scope| {
            focused_response == Some(self.id.with(DiagnosticsTargetIdentity::scope(scope)))
        });
        let is_diagnostics_focused = item_has_focus || scope_has_focus;
        let scope_keyboard = scope_has_focus
            .then(|| {
                ui.input(|input| {
                    if input.key_pressed(egui::Key::ArrowRight) {
                        Some(DiagnosticKeyboardInput::ScopeNext)
                    } else if input.key_pressed(egui::Key::ArrowLeft) {
                        Some(DiagnosticKeyboardInput::ScopePrevious)
                    } else {
                        None
                    }
                })
            })
            .flatten();
        if let Some(keyboard) = scope_keyboard {
            output
                .events
                .extend(diagnostics.apply_action(DiagnosticsListAction::Keyboard(keyboard)));
        }
        let keyboard = ui.input(|input| {
            let list_keyboard = is_diagnostics_focused
                .then(|| {
                    [
                        (egui::Key::ArrowUp, DiagnosticKeyboardInput::ArrowUp),
                        (egui::Key::ArrowDown, DiagnosticKeyboardInput::ArrowDown),
                        (egui::Key::ArrowLeft, DiagnosticKeyboardInput::ArrowLeft),
                        (egui::Key::ArrowRight, DiagnosticKeyboardInput::ArrowRight),
                        (egui::Key::Enter, DiagnosticKeyboardInput::Enter),
                        (egui::Key::Space, DiagnosticKeyboardInput::Space),
                    ]
                    .into_iter()
                    .find_map(|(key, action)| input.key_pressed(key).then_some(action))
                })
                .flatten();
            list_keyboard.or_else(|| {
                input
                    .key_pressed(egui::Key::F8)
                    .then_some(if input.modifiers.shift {
                        DiagnosticKeyboardInput::ShiftF8
                    } else {
                        DiagnosticKeyboardInput::F8
                    })
            })
        });
        if let Some(keyboard) = keyboard.filter(|_| scope_keyboard.is_none()) {
            let disclosure_action = self.focused_item.as_deref().and_then(|id| {
                let item_id = katana_ui_core::molecule::DiagnosticId::new(id);
                let expanded = snapshot.state.expanded_ids.contains(&item_id);
                match keyboard {
                    DiagnosticKeyboardInput::ArrowRight if !expanded => Some(item_id),
                    DiagnosticKeyboardInput::ArrowLeft if expanded => Some(item_id),
                    _ => None,
                }
            });
            if let Some(id) = disclosure_action {
                output
                    .events
                    .extend(diagnostics.apply_action(DiagnosticsListAction::ToggleFixPreview(id)));
            } else {
                output
                    .events
                    .extend(diagnostics.apply_action(DiagnosticsListAction::Keyboard(keyboard)));
            }
            if let Some(selected_id) = diagnostics.render_snapshot().state.selected_id
                && let Some(index) = snapshot
                    .visible
                    .visible_ids
                    .iter()
                    .position(|id| id == &selected_id)
            {
                let selected_top = row_heights.iter().take(index).sum::<f32>();
                let selected_bottom = selected_top
                    + row_heights
                        .get(index)
                        .copied()
                        .unwrap_or(super::super::paint::DIAGNOSTICS_DEFAULT_ROW_HEIGHT);
                if selected_top < self.scroll_y {
                    self.scroll_y = selected_top;
                } else if selected_bottom > self.scroll_y + viewport.height() {
                    self.scroll_y = (selected_bottom - viewport.height()).min(max_scroll_y);
                }
            }
        }
    }

    pub(super) fn process_focus_closure(
        &mut self,
        ui: &egui::Ui,
        output: &mut EguiDiagnosticsListOutput,
        surface: egui::Rect,
        diagnostics: &mut DiagnosticsList,
    ) {
        let snapshot = diagnostics.render_snapshot();
        let close_requested =
            self.focused_item.is_some() && ui.input(|input| input.key_pressed(egui::Key::Escape));
        let outside_requested = DiagnosticsAccessibility::pointer_pressed_outside(ui, surface);
        if (close_requested || outside_requested)
            && let Some(id) = self.focused_item.take()
            && snapshot
                .state
                .expanded_ids
                .contains(&katana_ui_core::molecule::DiagnosticId::new(id.clone()))
        {
            output.events.extend(diagnostics.apply_action(
                DiagnosticsListAction::ToggleFixPreview(
                    katana_ui_core::molecule::DiagnosticId::new(id),
                ),
            ));
        }
    }
}

#[cfg(test)]
#[path = "input_tests.rs"]
mod tests;
