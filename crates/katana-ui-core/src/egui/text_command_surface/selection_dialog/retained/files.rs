use super::{
    FileSelectionProjection, KucDialogTarget, SelectionDialogEvent, SelectionDialogOperation,
    SelectionDialogRetainedState, TextCommandSurfaceStyle, TextSurfacePaintOperation, activated,
};

impl SelectionDialogRetainedState {
    pub(super) fn files(
        &mut self,
        ui: &mut egui::Ui,
        files: &FileSelectionProjection,
        style: &TextCommandSurfaceStyle,
        operations: &mut Vec<TextSurfacePaintOperation>,
        events: &mut Vec<SelectionDialogEvent>,
    ) -> Result<(), String> {
        let allowed = !self.lease.disabled && !self.lease.readonly && files.failure.is_none();
        if let Some(message) = &files.failure {
            self.painter.label(
                ui,
                message,
                "kuc.dialog.failure",
                true,
                false,
                false,
                style,
                operations,
            )?;
            if !self.failure_reported {
                self.event(
                    events,
                    SelectionDialogOperation::Failed {
                        message: message.clone(),
                    },
                );
                self.failure_reported = true;
            }
        }
        let candidates = files
            .items
            .iter()
            .filter(|item| !item.disabled && (item.selectable || item.navigable))
            .map(|item| item.target.clone())
            .collect::<Vec<_>>();
        let keyboard_selected = self.navigate_rows(ui, &candidates, allowed);
        let mut keyboard_focus_id = None;
        for item in &files.items {
            let enabled = allowed && !item.disabled && (item.selectable || item.navigable);
            let selected = self.selected.as_ref() == Some(&item.target);
            let label = format!("{}{}", "  ".repeat(usize::from(item.depth)), item.label);
            let response = self.painter.label(
                ui,
                &label,
                &format!("kuc.dialog.file.{}", item.target.interaction_key()),
                enabled,
                true,
                selected,
                style,
                operations,
            )?;
            if keyboard_selected.as_ref() == Some(&item.target) {
                response.request_focus();
                keyboard_focus_id = Some(response.id);
                response.scroll_to_me(Some(egui::Align::Center));
            }
            if enabled && activated(ui, &response) {
                self.selected = Some(item.target.clone());
                if item.navigable {
                    self.event(
                        events,
                        SelectionDialogOperation::Navigated {
                            target: item.target.clone(),
                        },
                    );
                } else if item.selectable {
                    self.event(
                        events,
                        SelectionDialogOperation::FileSelected {
                            target: item.target.clone(),
                        },
                    );
                    self.lease.open = false;
                }
            }
        }
        let selected = files.items.iter().find(|item| {
            self.selected.as_ref() == Some(&item.target) && item.selectable && !item.disabled
        });
        let label = self.lease.controls.select.clone();
        let response = self.painter.label(
            ui,
            &label,
            "kuc.dialog.file.select",
            allowed && selected.is_some(),
            true,
            false,
            style,
            operations,
        )?;
        if allowed
            && self.lease.open
            && activated(ui, &response)
            && let Some(item) = selected
        {
            self.event(
                events,
                SelectionDialogOperation::FileSelected {
                    target: item.target.clone(),
                },
            );
            self.lease.open = false;
        }
        if let Some(id) = keyboard_focus_id {
            ui.memory_mut(|memory| {
                memory.request_focus(id);
                /* WHY: 行移動をこの一覧で消費するため、egui の方向キー移動で Cancel に戻さない。 */
                memory.move_focus(egui::FocusDirection::None);
            });
        }
        Ok(())
    }

    pub(super) fn navigate_rows(
        &mut self,
        ui: &egui::Ui,
        candidates: &[KucDialogTarget],
        allowed: bool,
    ) -> Option<KucDialogTarget> {
        if !allowed || candidates.is_empty() {
            return None;
        }
        let delta = ui.input_mut(|input| {
            if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) {
                1
            } else if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) {
                -1
            } else {
                0
            }
        });
        if delta == 0 {
            return None;
        }
        let current = self
            .selected
            .as_ref()
            .and_then(|selected| candidates.iter().position(|item| item == selected));
        let next = match (current, delta) {
            (None, 1) => 0,
            (None, _) => candidates.len() - 1,
            (Some(index), 1) => (index + 1) % candidates.len(),
            (Some(index), _) => (index + candidates.len() - 1) % candidates.len(),
        };
        self.selected = Some(candidates[next].clone());
        self.selected.clone()
    }
}
