//! Keyboard focus and one-shot closure handling for diagnostics.

use super::super::accessibility::DiagnosticsAccessibility;
use super::super::identity::DiagnosticsTargetIdentity;
use super::super::types::EguiDiagnosticsListOutput;
use super::EguiDiagnosticsListAdapter;
use crate::molecule::{DiagnosticKeyboardInput, DiagnosticsList, DiagnosticsListAction};

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
        let stale_focused_scope = self
            .focused_scope
            .as_deref()
            .filter(|focused_scope| {
                !snapshot
                    .scopes
                    .iter()
                    .any(|scope| scope.key.as_str() == *focused_scope)
            })
            .map(str::to_owned);
        if let Some(stale_focused_scope) = stale_focused_scope {
            self.focused_scope = None;
            ui.memory_mut(|memory| {
                memory.surrender_focus(
                    self.id
                        .with(DiagnosticsTargetIdentity::scope(&stale_focused_scope)),
                );
            });
        }
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
                        Some((DiagnosticKeyboardInput::ScopeNext, egui::Key::ArrowRight))
                    } else if input.key_pressed(egui::Key::ArrowLeft) {
                        Some((DiagnosticKeyboardInput::ScopePrevious, egui::Key::ArrowLeft))
                    } else {
                        None
                    }
                })
            })
            .flatten();
        if let Some((keyboard, key)) = scope_keyboard {
            ui.input_mut(|input| {
                let modifiers = input.modifiers;
                input.consume_key(modifiers, key);
            });
            ui.memory_mut(|memory| memory.move_focus(egui::FocusDirection::None));
            output
                .events
                .extend(diagnostics.apply_action(DiagnosticsListAction::Keyboard(keyboard)));
            diagnostics
                .render_snapshot()
                .state
                .selected_scope_key
                .iter()
                .for_each(|scope| {
                    self.focused_item = None;
                    self.focused_scope = Some(scope.as_str().to_owned());
                    ui.memory_mut(|memory| {
                        memory.request_focus(
                            self.id
                                .with(DiagnosticsTargetIdentity::scope(scope.as_str())),
                        );
                    });
                });
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
                    .find_map(|(key, action)| input.key_pressed(key).then_some((action, key)))
                })
                .flatten();
            list_keyboard.or_else(|| {
                input.key_pressed(egui::Key::F8).then_some((
                    if input.modifiers.shift {
                        DiagnosticKeyboardInput::ShiftF8
                    } else {
                        DiagnosticKeyboardInput::F8
                    },
                    egui::Key::F8,
                ))
            })
        });
        if let Some((keyboard, key)) = keyboard.filter(|_| scope_keyboard.is_none()) {
            ui.input_mut(|input| {
                let modifiers = input.modifiers;
                input.consume_key(modifiers, key);
            });
            if matches!(
                keyboard,
                DiagnosticKeyboardInput::ArrowUp
                    | DiagnosticKeyboardInput::ArrowDown
                    | DiagnosticKeyboardInput::ArrowLeft
                    | DiagnosticKeyboardInput::ArrowRight
            ) {
                ui.memory_mut(|memory| memory.move_focus(egui::FocusDirection::None));
            }
            let disclosure_action = self.focused_item.as_deref().and_then(|id| {
                let item_id = crate::molecule::DiagnosticId::new(id);
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
            let selected_id = diagnostics.render_snapshot().state.selected_id;
            if let Some(selected_id) = selected_id.as_ref() {
                self.focused_scope = None;
                self.focused_item = Some(selected_id.as_str().to_owned());
                ui.memory_mut(|memory| {
                    memory.request_focus(self.id.with(selected_id.as_str()));
                });
            }
            if let Some(selected_id) = selected_id
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
        let item_has_focus = self
            .focused_item
            .as_deref()
            .is_some_and(|id| ui.memory(|memory| memory.focused()) == Some(self.id.with(id)));
        let close_requested =
            item_has_focus && ui.input(|input| input.key_pressed(egui::Key::Escape));
        let outside_requested = DiagnosticsAccessibility::pointer_pressed_outside(ui, surface);
        if (close_requested || outside_requested)
            && let Some(id) = self.focused_item.take()
            && snapshot
                .state
                .expanded_ids
                .contains(&crate::molecule::DiagnosticId::new(id.clone()))
        {
            output.events.extend(diagnostics.apply_action(
                DiagnosticsListAction::ToggleFixPreview(crate::molecule::DiagnosticId::new(id)),
            ));
        }
    }
}

#[cfg(test)]
#[path = "input_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "input_focus_tests.rs"]
mod focus_tests;
