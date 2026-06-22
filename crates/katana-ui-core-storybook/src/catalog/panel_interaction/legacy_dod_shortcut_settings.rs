use super::{BeforeAfterReport, SettingsMutationReport, TypedOptionMutationReport};
use crate::catalog::StoryExample;

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    let combo = examples
        .iter()
        .find(|it| it.page == "shortcut-combo")
        .map(|example| example.tree.root().props().state_id.as_str().to_string())
        .map(|state_id| {
            shortcut_combo_options()
                .into_iter()
                .map(|option| {
                    shortcut_report(
                        option,
                        &state_id,
                        "shortcut-combo",
                        "catalog-shortcut-combo",
                        "shortcut_combo_settings_changed",
                        "shortcut combo",
                    )
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let cheatsheet = examples
        .iter()
        .find(|it| it.page == "shortcut-cheatsheet")
        .map(|example| example.tree.root().props().state_id.as_str().to_string())
        .map(|state_id| {
            shortcut_cheatsheet_options()
                .into_iter()
                .map(|option| {
                    shortcut_report(
                        option,
                        &state_id,
                        "shortcut-cheatsheet",
                        "catalog-shortcut-cheatsheet",
                        "shortcut_cheatsheet_settings_changed",
                        "shortcut cheatsheet",
                    )
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    [combo, cheatsheet].concat()
}

fn shortcut_report(
    option: ShortcutSettingOption,
    state_id: &str,
    page: &str,
    marker: &str,
    event: &str,
    state_prefix: &str,
) -> SettingsMutationReport {
    SettingsMutationReport {
        page: page.to_string(),
        ui_marker: marker.to_string(),
        action: format!("set_{}", option.name),
        event: event.to_string(),
        target_state_id: state_id.to_string(),
        option: TypedOptionMutationReport {
            name: option.name.to_string(),
            value_type: option.value_type.to_string(),
            before_value: option.before.to_string(),
            after_value: option.after.to_string(),
        },
        state: BeforeAfterReport {
            before: format!("{state_prefix} option:{}={}", option.name, option.before),
            after: format!("{state_prefix} option:{}={}", option.name, option.after),
        },
        preview: BeforeAfterReport {
            before: format!("{marker}:preview:{}={}", option.name, option.before),
            after: format!("{marker}:preview:{}={}", option.name, option.after),
        },
    }
}

fn shortcut_combo_options() -> Vec<ShortcutSettingOption> {
    vec![
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
        ShortcutSettingOption {
            name: "shortcut_combo.a11y_label",
            value_type: "Option<String>",
            before: "generated",
            after: "custom",
        },
    ]
}

fn shortcut_cheatsheet_options() -> Vec<ShortcutSettingOption> {
    vec![
        ShortcutSettingOption {
            name: "shortcut_cheatsheet.label",
            value_type: "String",
            before: "Shortcuts",
            after: "Editor keys",
        },
        ShortcutSettingOption {
            name: "shortcut_cheatsheet.groups",
            value_type: "usize",
            before: "2",
            after: "3",
        },
        ShortcutSettingOption {
            name: "shortcut_cheatsheet.group_title",
            value_type: "String",
            before: "Editing",
            after: "Navigation",
        },
        ShortcutSettingOption {
            name: "shortcut_cheatsheet.items",
            value_type: "usize",
            before: "2",
            after: "4",
        },
        ShortcutSettingOption {
            name: "shortcut_cheatsheet.item_combo",
            value_type: "KeyCombo",
            before: "Cmd+F",
            after: "Cmd+Shift+P",
        },
        ShortcutSettingOption {
            name: "shortcut_cheatsheet.group_layout",
            value_type: "ShortcutCheatsheetLayout",
            before: "TwoColumn",
            after: "OneColumn",
        },
        ShortcutSettingOption {
            name: "shortcut_cheatsheet.query",
            value_type: "String",
            before: "format",
            after: "カテゴリ",
        },
        ShortcutSettingOption {
            name: "shortcut_cheatsheet.selected",
            value_type: "Option<String>",
            before: "None",
            after: "format",
        },
        ShortcutSettingOption {
            name: "shortcut_cheatsheet.result_count",
            value_type: "usize",
            before: "2",
            after: "1",
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
