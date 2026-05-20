use super::{BeforeAfterReport, SettingsMutationReport, TypedOptionMutationReport};
use crate::catalog::StoryExample;

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    let Some(example) = examples.iter().find(|it| it.page == "shortcut-combo") else {
        return Vec::new();
    };
    let state_id = example.tree.root().props().state_id.as_str().to_string();
    shortcut_options()
        .into_iter()
        .map(|option| shortcut_report(option, &state_id))
        .collect()
}

fn shortcut_report(option: ShortcutSettingOption, state_id: &str) -> SettingsMutationReport {
    let marker = "catalog-shortcut-combo".to_string();
    SettingsMutationReport {
        page: "shortcut-combo".to_string(),
        ui_marker: marker.clone(),
        action: format!("set_{}", option.name),
        event: "shortcut_combo_settings_changed".to_string(),
        target_state_id: state_id.to_string(),
        option: TypedOptionMutationReport {
            name: option.name.to_string(),
            value_type: option.value_type.to_string(),
            before_value: option.before.to_string(),
            after_value: option.after.to_string(),
        },
        state: BeforeAfterReport {
            before: format!("shortcut combo option:{}={}", option.name, option.before),
            after: format!("shortcut combo option:{}={}", option.name, option.after),
        },
        preview: BeforeAfterReport {
            before: format!("{marker}:preview:{}={}", option.name, option.before),
            after: format!("{marker}:preview:{}={}", option.name, option.after),
        },
    }
}

fn shortcut_options() -> [ShortcutSettingOption; 4] {
    [
        ShortcutSettingOption {
            name: "shortcut_combo.platform_display",
            value_type: "ShortcutPlatform",
            before: "Auto",
            after: "MacOS",
        },
        ShortcutSettingOption {
            name: "shortcut_combo.separator",
            value_type: "ShortcutSeparator",
            before: "Plus",
            after: "None",
        },
        ShortcutSettingOption {
            name: "shortcut_combo.size",
            value_type: "UiSize",
            before: "Medium",
            after: "Large",
        },
        ShortcutSettingOption {
            name: "shortcut_combo.tone",
            value_type: "UiTone",
            before: "Neutral",
            after: "Accent",
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct ShortcutSettingOption {
    name: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
}
