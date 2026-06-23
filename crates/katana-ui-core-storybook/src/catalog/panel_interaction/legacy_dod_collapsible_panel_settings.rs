use super::{BeforeAfterReport, SettingsMutationReport, TypedOptionMutationReport};
use crate::catalog::StoryExample;

#[derive(Clone, Copy)]
struct CollapsiblePanelSetting {
    option: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
}

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    let Some(example) = examples.iter().find(|it| it.page == "collapsible-panel") else {
        return Vec::new();
    };
    collapsible_panel_settings()
        .into_iter()
        .map(|setting| report(setting, example))
        .collect()
}

fn collapsible_panel_settings() -> Vec<CollapsiblePanelSetting> {
    vec![
        setting(
            "collapsible_panel.mode",
            "PanelMode",
            "Expanded",
            "IconOnly",
        ),
        setting("collapsible_panel.width", "u16", "240", "320"),
        setting("collapsible_panel.pinned", "bool", "true", "false"),
        setting("collapsible_panel.expand_on_hover", "bool", "false", "true"),
        setting("collapsible_panel.resize_handle", "bool", "false", "true"),
    ]
}

const fn setting(
    option: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
) -> CollapsiblePanelSetting {
    CollapsiblePanelSetting {
        option,
        value_type,
        before,
        after,
    }
}

fn report(setting: CollapsiblePanelSetting, example: &StoryExample) -> SettingsMutationReport {
    let marker = "catalog-collapsible-panel".to_string();
    SettingsMutationReport {
        page: "collapsible-panel".to_string(),
        ui_marker: marker.clone(),
        action: format!("set_{}", setting.option),
        event: "collapsible_panel_settings_changed".to_string(),
        target_state_id: example.tree.root().props().state_id.as_str().to_string(),
        option: TypedOptionMutationReport {
            name: setting.option.to_string(),
            value_type: setting.value_type.to_string(),
            before_value: setting.before.to_string(),
            after_value: setting.after.to_string(),
        },
        state: before_after(setting, "state"),
        preview: before_after(setting, &format!("{marker}:preview")),
    }
}

fn before_after(setting: CollapsiblePanelSetting, prefix: &str) -> BeforeAfterReport {
    BeforeAfterReport {
        before: format!("{prefix} option:{}={}", setting.option, setting.before),
        after: format!("{prefix} option:{}={}", setting.option, setting.after),
    }
}
