use super::{SettingsList, SettingsListAction};
use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};

impl ComponentAction for SettingsList {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = super::state::interaction_state(self);
        if action.target() != &self.state_id {
            return UiActionResult::ignored(self.state_id.clone(), before);
        }
        let _events = match action {
            UiAction::SetValue { value, .. } => {
                self.apply_settings_action(SettingsListAction::SetQuery(Some(value.clone())))
            }
            UiAction::SetSelectedIndex { selected_index, .. } => {
                let Some(section) = self.sections.get(*selected_index) else {
                    return UiActionResult::ignored(self.state_id.clone(), before);
                };
                self.apply_settings_action(SettingsListAction::ToggleSection {
                    section_id: section.id.clone(),
                })
            }
            UiAction::ClearValue { .. } => {
                let Some(field_id) = self.dirty_field_ids.iter().next().cloned() else {
                    return UiActionResult::ignored(self.state_id.clone(), before);
                };
                self.apply_settings_action(SettingsListAction::ResetField { field_id })
            }
            _ => return UiActionResult::ignored(self.state_id.clone(), before),
        };
        UiActionResult::handled(
            self.state_id.clone(),
            action,
            before,
            super::state::interaction_state(self),
        )
    }
}
