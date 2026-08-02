use super::{SettingsList, SettingsListAction};
use crate::render_model::UiHostActionPlan;

pub(super) fn action_from_host_plan(
    list: &SettingsList,
    plan: &UiHostActionPlan,
) -> Option<SettingsListAction> {
    if let Some(target) = plan.settings_field_control_target() {
        return list.activation_action_for_field(&target.field_id);
    }
    plan.settings_section_toggle_target()
        .map(|target| SettingsListAction::ToggleSection {
            section_id: target.section_id,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::molecule::{SettingsControl, SettingsField, SettingsSection, SettingsValue};
    use crate::render_model::UiHostActionPlan;

    #[test]
    fn toggle_control_host_action_returns_settings_update() {
        let list = SettingsList::new("Settings").section(
            SettingsSection::new("display", "Display").field(SettingsField::new(
                "dark",
                "Dark",
                SettingsControl::Toggle { checked: true },
            )),
        );
        let root = crate::render_model::UiNode::from(list.clone());
        let plan = UiHostActionPlan::collect_from_root(&root)
            .into_iter()
            .find_map(|plan| plan.settings_field_control_target().map(|_| plan));

        assert_eq!(
            Some(SettingsListAction::UpdateField {
                field_id: "dark".to_string(),
                value: SettingsValue::Bool(false),
            }),
            plan.as_ref()
                .and_then(|plan| action_from_host_plan(&list, plan))
        );
    }

    #[test]
    fn section_host_action_returns_toggle_section() {
        let list = SettingsList::new("Settings").section(
            SettingsSection::new("display", "Display").field(SettingsField::new(
                "dark",
                "Dark",
                SettingsControl::Toggle { checked: true },
            )),
        );
        let root = crate::render_model::UiNode::from(list.clone());
        let plan = UiHostActionPlan::collect_from_root(&root)
            .into_iter()
            .find_map(|plan| plan.settings_section_toggle_target().map(|_| plan));

        assert_eq!(
            Some(SettingsListAction::ToggleSection {
                section_id: "display".to_string(),
            }),
            plan.as_ref()
                .and_then(|plan| action_from_host_plan(&list, plan))
        );
    }
}
