use super::{
    SearchResultProjection, SelectionDialogEvent, SelectionDialogOperation,
    SelectionDialogRetainedState, TextCommandSurfaceStyle, TextSurfacePaintOperation, activated,
};

impl SelectionDialogRetainedState {
    pub(super) fn search(
        &mut self,
        ui: &mut egui::Ui,
        search: &mut SearchResultProjection,
        style: &TextCommandSurfaceStyle,
        first: bool,
        operations: &mut Vec<TextSurfacePaintOperation>,
        events: &mut Vec<SelectionDialogEvent>,
    ) -> Result<(), String> {
        let allowed = !self.lease.disabled && !self.lease.readonly;
        let mut mode_changed = false;
        for mode in &search.modes {
            let response = self.painter.label(
                ui,
                &mode.label,
                &format!("kuc.dialog.mode.{}", mode.target.interaction_key()),
                allowed,
                true,
                mode.target == search.active_mode,
                style,
                operations,
            )?;
            if allowed && activated(ui, &response) && mode.target != search.active_mode {
                search.active_mode = mode.target.clone();
                mode_changed = true;
                self.selected = None;
                self.event(
                    events,
                    SelectionDialogOperation::ModeChanged {
                        mode: mode.target.clone(),
                    },
                );
            }
        }
        let index = search
            .modes
            .iter()
            .position(|mode| mode.target == search.active_mode)
            .ok_or_else(|| "active dialog mode is absent".to_owned())?;
        let mode = &mut search.modes[index];
        let target = mode.target.clone();
        let mode_key = target.interaction_key();
        if self.input(
            ui,
            format!("kuc.dialog.query.{mode_key}"),
            &mode.label,
            &mut mode.query,
            style,
            first || mode_changed,
            operations,
        )? {
            self.selected = None;
            self.event(
                events,
                SelectionDialogOperation::QueryChanged {
                    mode: target.clone(),
                    value: mode.query.clone(),
                },
            );
        }
        for (include, value) in [(true, &mut mode.include), (false, &mut mode.exclude)] {
            if let Some(value) = value {
                let label = if include {
                    self.lease.controls.include.clone()
                } else {
                    self.lease.controls.exclude.clone()
                };
                if self.input(
                    ui,
                    format!("kuc.dialog.filter.{mode_key}.{include}"),
                    &label,
                    value,
                    style,
                    false,
                    operations,
                )? {
                    self.selected = None;
                    self.event(
                        events,
                        if include {
                            SelectionDialogOperation::IncludeChanged {
                                mode: target.clone(),
                                value: value.clone(),
                            }
                        } else {
                            SelectionDialogOperation::ExcludeChanged {
                                mode: target.clone(),
                                value: value.clone(),
                            }
                        },
                    );
                }
            }
        }
        for option in &mut mode.options {
            let enabled = allowed && option.enabled;
            let response = self.painter.label(
                ui,
                &option.label,
                &format!(
                    "kuc.dialog.option.{mode_key}.{}",
                    option.target.interaction_key()
                ),
                enabled,
                true,
                option.selected,
                style,
                operations,
            )?;
            if enabled && activated(ui, &response) {
                option.selected = !option.selected;
                self.selected = None;
                self.event(
                    events,
                    SelectionDialogOperation::OptionChanged {
                        mode: target.clone(),
                        option: option.target.clone(),
                        selected: option.selected,
                    },
                );
            }
        }
        self.search_actions(ui, mode, style, operations, events)?;
        Ok(())
    }
}
