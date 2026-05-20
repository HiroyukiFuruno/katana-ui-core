use super::{BeforeAfterReport, SettingsMutationReport, TypedOptionMutationReport};
use crate::catalog::StoryExample;

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    let Some(example) = examples.iter().find(|it| it.page == "settings-list") else {
        return Vec::new();
    };
    let state_id = example.tree.root().props().state_id.as_str().to_string();
    settings_list_options()
        .into_iter()
        .map(|option| settings_list_report(option, &state_id))
        .collect()
}

fn settings_list_report(
    option: SettingsListSettingOption,
    state_id: &str,
) -> SettingsMutationReport {
    let marker = "catalog-settings-list".to_string();
    SettingsMutationReport {
        page: "settings-list".to_string(),
        ui_marker: marker.clone(),
        action: option.action.to_string(),
        event: option.event.to_string(),
        target_state_id: state_id.to_string(),
        option: TypedOptionMutationReport {
            name: option.name.to_string(),
            value_type: option.value_type.to_string(),
            before_value: option.before.to_string(),
            after_value: option.after.to_string(),
        },
        state: BeforeAfterReport {
            before: format!("settings list option:{}={}", option.name, option.before),
            after: format!("settings list option:{}={}", option.name, option.after),
        },
        preview: BeforeAfterReport {
            before: format!("{marker}:preview:{}={}", option.name, option.before),
            after: format!("{marker}:preview:{}={}", option.name, option.after),
        },
    }
}

fn settings_list_options() -> [SettingsListSettingOption; 6] {
    [
        SettingsListSettingOption {
            name: "settings_list.density",
            value_type: "SettingsListDensity",
            before: "Default",
            after: "Compact",
            action: "set_settings_list.density",
            event: "settings_list_settings_changed",
        },
        SettingsListSettingOption {
            name: "settings_list.dirty_visualization",
            value_type: "SettingsDirtyVisualization",
            before: "Marker",
            after: "Highlight",
            action: "set_settings_list.dirty_visualization",
            event: "settings_list_settings_changed",
        },
        SettingsListSettingOption {
            name: "settings_list.query",
            value_type: "Option<String>",
            before: "None",
            after: "format",
            action: "settings_query_filter",
            event: "settings_list_query_changed",
        },
        SettingsListSettingOption {
            name: "settings_list.sections",
            value_type: "SettingsSectionSet",
            before: "app+chat+lint",
            after: "app+lint",
            action: "settings_toggle_section",
            event: "settings_list_section_collapsed",
        },
        SettingsListSettingOption {
            name: "settings_list.control_kind",
            value_type: "SettingsControlKind",
            before: "Toggle",
            after: "Number",
            action: "settings_update_field",
            event: "settings_list_field_changed",
        },
        SettingsListSettingOption {
            name: "settings_list.reset",
            value_type: "SettingsReset",
            before: "dirty",
            after: "default",
            action: "settings_reset_field",
            event: "settings_list_field_reset",
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct SettingsListSettingOption {
    name: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
    action: &'static str,
    event: &'static str,
}
