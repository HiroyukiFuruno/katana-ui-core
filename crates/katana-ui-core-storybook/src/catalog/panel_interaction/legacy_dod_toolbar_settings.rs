use super::{BeforeAfterReport, SettingsMutationReport, TypedOptionMutationReport};
use crate::catalog::StoryExample;

#[derive(Clone, Copy)]
struct ToolbarSetting {
    option: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
}

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    let Some(example) = examples.iter().find(|it| it.page == "toolbar") else {
        return Vec::new();
    };
    toolbar_settings()
        .into_iter()
        .map(|setting| report(setting, example))
        .collect()
}

fn toolbar_settings() -> Vec<ToolbarSetting> {
    vec![
        setting("toolbar.action_count", "usize", "4", "5"),
        setting("toolbar.priority", "Priority", "search=10", "search=90"),
        setting("toolbar.overflow_strategy", "Strategy", "Menu", "Hide"),
        setting(
            "toolbar.display_mode",
            "DisplayMode",
            "IconLeading",
            "LabelOnly",
        ),
        setting("toolbar.density", "Density", "Default", "Compact"),
    ]
}

const fn setting(
    option: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
) -> ToolbarSetting {
    ToolbarSetting {
        option,
        value_type,
        before,
        after,
    }
}

fn report(setting: ToolbarSetting, example: &StoryExample) -> SettingsMutationReport {
    let marker = "catalog-toolbar".to_string();
    SettingsMutationReport {
        page: "toolbar".to_string(),
        ui_marker: marker.clone(),
        action: format!("set_{}", setting.option),
        event: "toolbar_settings_changed".to_string(),
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

fn before_after(setting: ToolbarSetting, prefix: &str) -> BeforeAfterReport {
    BeforeAfterReport {
        before: format!("{prefix} option:{}={}", setting.option, setting.before),
        after: format!("{prefix} option:{}={}", setting.option, setting.after),
    }
}
