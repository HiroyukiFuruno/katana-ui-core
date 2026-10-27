use super::{
    SearchQueryArrowKeyPolicy, SearchResultMode, SelectionDialogEvent, SelectionDialogOperation,
    SelectionDialogRetainedState, TextCommandSurfaceStyle, TextSurfacePaintOperation, activated,
};

impl SelectionDialogRetainedState {
    pub(super) fn search_actions(
        &mut self,
        ui: &mut egui::Ui,
        mode: &mut SearchResultMode,
        style: &TextCommandSurfaceStyle,
        operations: &mut Vec<TextSurfacePaintOperation>,
        events: &mut Vec<SelectionDialogEvent>,
    ) -> Result<(), String> {
        let allowed = !self.lease.disabled && !self.lease.readonly;
        let target = mode.target.clone();
        let mode_key = target.interaction_key();
        let label = self.lease.controls.search.clone();
        let response = self.painter.label(
            ui,
            &label,
            "kuc.dialog.search",
            allowed,
            true,
            false,
            style,
            operations,
        )?;
        let search_clicked = activated(ui, &response);
        let focused_input = |key: String| {
            self.inputs
                .get(&key)
                .map(|input| &input.surface.state().text_area)
                .filter(|input| input.focused)
        };
        let focused_inputs = [
            focused_input(format!("kuc.dialog.query.{mode_key}")),
            mode.include
                .as_ref()
                .and_then(|_| focused_input(format!("kuc.dialog.filter.{mode_key}.true"))),
            mode.exclude
                .as_ref()
                .and_then(|_| focused_input(format!("kuc.dialog.filter.{mode_key}.false"))),
        ];
        let text_input_focused = focused_inputs.iter().any(Option::is_some);
        let query_focused = focused_inputs[0].is_some();
        let composition_active = focused_inputs
            .iter()
            .flatten()
            .any(|input| input.composition.is_some());
        /* WHY: 確定でcompositionが消えるframeも、Enterは変換処理にだけ渡す。 */
        let ime_event = ui.input(|input| {
            input
                .events
                .iter()
                .any(|event| matches!(event, egui::Event::Ime(_)))
        });
        let entered = text_input_focused
            && !composition_active
            && !ime_event
            && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
        if allowed
            && entered
            && !search_clicked
            && let Some(selected) = &self.selected
            && mode
                .results
                .iter()
                .any(|row| row.target == *selected && !row.disabled)
        {
            self.event(
                events,
                SelectionDialogOperation::ResultSelected {
                    mode: target.clone(),
                    target: selected.clone(),
                },
            );
        } else if allowed && (search_clicked || entered) {
            self.event(
                events,
                SelectionDialogOperation::SearchRequested {
                    mode: target.clone(),
                },
            );
        }
        let candidates = mode
            .results
            .iter()
            .filter(|item| !item.disabled)
            .map(|item| item.target.clone())
            .collect::<Vec<_>>();
        let history_lane =
            query_focused && self.lease.arrow_policy(&target) == SearchQueryArrowKeyPolicy::History;
        let keyboard_selected = if composition_active || ime_event || history_lane {
            None
        } else {
            self.navigate_rows(ui, &candidates, allowed)
        };
        let mut deleted = None;
        for (index, row) in mode.history.iter().enumerate() {
            let highlights = if row.snippet.is_none() {
                row.highlights.as_slice()
            } else {
                &[]
            };
            let response = self.painter.highlighted_button(
                ui,
                (&row.label, highlights),
                &format!(
                    "kuc.dialog.history.{mode_key}.{}",
                    row.target.interaction_key()
                ),
                allowed && !row.disabled,
                false,
                style,
                operations,
            )?;
            if allowed && !row.disabled && activated(ui, &response) {
                self.history_traversal = None;
                mode.query = row.label.clone();
                self.selected = None;
                self.event(
                    events,
                    SelectionDialogOperation::HistorySelected {
                        mode: target.clone(),
                        target: row.target.clone(),
                    },
                );
            }
            let label = self.lease.controls.delete_history.clone();
            let response = self.painter.label(
                ui,
                &label,
                &format!(
                    "kuc.dialog.history.delete.{mode_key}.{}",
                    row.target.interaction_key()
                ),
                allowed,
                true,
                false,
                style,
                operations,
            )?;
            if allowed && activated(ui, &response) {
                deleted = Some(index);
                self.event(
                    events,
                    SelectionDialogOperation::HistoryDeleted {
                        mode: target.clone(),
                        target: row.target.clone(),
                    },
                );
            }
        }
        if let Some(index) = deleted {
            self.history_traversal = None;
            mode.history.remove(index);
        }
        let label = self.lease.controls.clear_history.clone();
        let response = self.painter.label(
            ui,
            &label,
            "kuc.dialog.history.clear",
            allowed && !mode.history.is_empty(),
            true,
            false,
            style,
            operations,
        )?;
        if allowed && !mode.history.is_empty() && activated(ui, &response) {
            self.history_traversal = None;
            mode.history.clear();
            self.event(
                events,
                SelectionDialogOperation::HistoryCleared {
                    mode: target.clone(),
                },
            );
        }
        for row in &mode.results {
            let row_key = row.target.interaction_key();
            let enabled = allowed && !row.disabled;
            let selected = self.selected.as_ref() == Some(&row.target);
            let highlights = if row.snippet.is_none() {
                row.highlights.as_slice()
            } else {
                &[]
            };
            let response = self.painter.highlighted_button(
                ui,
                (&row.label, highlights),
                &format!("kuc.dialog.result.{mode_key}.{row_key}"),
                enabled,
                selected,
                style,
                operations,
            )?;
            if keyboard_selected.as_ref() == Some(&row.target) {
                response.scroll_to_me(Some(egui::Align::Center));
            }
            if enabled && activated(ui, &response) {
                self.selected = Some(row.target.clone());
                self.event(
                    events,
                    SelectionDialogOperation::ResultSelected {
                        mode: target.clone(),
                        target: row.target.clone(),
                    },
                );
            }
            if let Some(label) = &row.auxiliary_label {
                self.painter.label(
                    ui,
                    label,
                    &format!("kuc.dialog.auxiliary.{mode_key}.{row_key}"),
                    true,
                    false,
                    false,
                    style,
                    operations,
                )?;
            }
            if let Some(snippet) = &row.snippet {
                self.painter.snippet(
                    ui,
                    (snippet, &row.highlights),
                    &format!("kuc.dialog.snippet.{mode_key}.{row_key}"),
                    style,
                    operations,
                )?;
            }
        }
        if mode.searched && mode.results.is_empty() {
            let label = self.lease.controls.empty.clone();
            self.painter.label(
                ui,
                &label,
                "kuc.dialog.empty",
                true,
                false,
                false,
                style,
                operations,
            )?;
        }
        Ok(())
    }
}
