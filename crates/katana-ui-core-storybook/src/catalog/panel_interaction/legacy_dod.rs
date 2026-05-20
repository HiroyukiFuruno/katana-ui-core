use super::legacy_dod_options::{
    option_state_summary, option_value, props_with_option, resolved_after_value,
};
use super::legacy_dod_specs::{LegacyDodSpec, legacy_dod_specs};
use crate::catalog::StoryExample;
use serde::{Deserialize, Serialize};

#[path = "legacy_dod_banner_settings.rs"]
mod legacy_dod_banner_settings;
#[path = "legacy_dod_chip_settings.rs"]
mod legacy_dod_chip_settings;
#[path = "legacy_dod_context_menu.rs"]
mod legacy_dod_context_menu;
#[path = "legacy_dod_diagnostics_settings.rs"]
mod legacy_dod_diagnostics_settings;
#[path = "legacy_dod_drag_settings.rs"]
mod legacy_dod_drag_settings;
#[path = "legacy_dod_empty_state_settings.rs"]
mod legacy_dod_empty_state_settings;
#[path = "legacy_dod_overlay_settings.rs"]
mod legacy_dod_overlay_settings;
#[path = "legacy_dod_settings_list.rs"]
mod legacy_dod_settings_list;
#[path = "legacy_dod_shortcut_settings.rs"]
mod legacy_dod_shortcut_settings;
#[path = "legacy_dod_status_bar_settings.rs"]
mod legacy_dod_status_bar_settings;
#[path = "legacy_dod_text_area_settings.rs"]
mod legacy_dod_text_area_settings;
#[path = "legacy_dod_toast_stack_settings.rs"]
mod legacy_dod_toast_stack_settings;
#[path = "legacy_dod_toolbar_settings.rs"]
mod legacy_dod_toolbar_settings;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsMutationReport {
    pub page: String,
    pub ui_marker: String,
    pub action: String,
    pub event: String,
    pub target_state_id: String,
    pub option: TypedOptionMutationReport,
    pub state: BeforeAfterReport,
    pub preview: BeforeAfterReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedOptionMutationReport {
    pub name: String,
    pub value_type: String,
    pub before_value: String,
    pub after_value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BeforeAfterReport {
    pub before: String,
    pub after: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegacyUiMarkerReport {
    pub page: String,
    pub ui_marker: String,
    pub root_kind: String,
    pub state_id: String,
    pub preview_marker: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresetDifferenceReport {
    pub page: String,
    pub ui_marker: String,
    pub default_marker: String,
    pub interactive_marker: String,
    pub edge_marker: String,
    pub theme_marker: String,
}

pub(crate) struct LegacyDodReports;

impl LegacyDodReports {
    pub(crate) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
        let mut reports: Vec<SettingsMutationReport> =
            examples.iter().map(settings_mutation).collect();
        reports.extend(parity_settings_mutations(examples));
        reports.extend(legacy_dod_drag_settings::drag_and_drop_settings_mutations(
            examples,
        ));
        reports.extend(legacy_dod_banner_settings::settings_mutations(examples));
        reports.extend(legacy_dod_context_menu::settings_mutations(examples));
        reports.extend(legacy_dod_chip_settings::settings_mutations(examples));
        reports.extend(legacy_dod_diagnostics_settings::settings_mutations(
            examples,
        ));
        reports.extend(legacy_dod_empty_state_settings::settings_mutations(
            examples,
        ));
        reports.extend(legacy_dod_overlay_settings::settings_mutations(examples));
        reports.extend(legacy_dod_status_bar_settings::settings_mutations(examples));
        reports.extend(legacy_dod_shortcut_settings::settings_mutations(examples));
        reports.extend(legacy_dod_settings_list::settings_mutations(examples));
        reports.extend(legacy_dod_text_area_settings::settings_mutations(examples));
        reports.extend(legacy_dod_toast_stack_settings::settings_mutations(
            examples,
        ));
        reports.extend(legacy_dod_toolbar_settings::settings_mutations(examples));
        reports.extend(closeable_tab_strip_settings_mutations(examples));
        reports
    }

    pub(crate) fn ui_markers(examples: &[StoryExample]) -> Vec<LegacyUiMarkerReport> {
        legacy_dod_specs()
            .filter_map(|spec| legacy_example(examples, spec).map(|it| ui_marker(spec, it)))
            .collect()
    }

    pub(crate) fn preset_differences(examples: &[StoryExample]) -> Vec<PresetDifferenceReport> {
        legacy_dod_specs()
            .filter(|spec| legacy_example(examples, spec).is_some())
            .map(preset_difference)
            .collect()
    }
}

fn settings_mutation(example: &StoryExample) -> SettingsMutationReport {
    let spec = spec_for(example.page);
    settings_mutation_for_spec(example, spec)
}

fn settings_mutation_for_spec(
    example: &StoryExample,
    spec: Option<&LegacyDodSpec>,
) -> SettingsMutationReport {
    let props = example.tree.root().props();
    let option = spec.map_or(fallback_option(example.page), |it| it.option);
    let before = option_value(option, props);
    let value_type = spec.map_or("StorybookOption", |it| it.value_type);
    let after = resolved_after_value(
        option,
        value_type,
        spec.map_or(fallback_after(example.page), |it| it.after),
        &before,
    );
    let after_props = props_with_option(props, option, &after);
    let actual_after = option_value(option, &after_props);
    let marker = ui_marker_name(spec, example.page);
    SettingsMutationReport {
        page: example.page.to_string(),
        ui_marker: marker.clone(),
        action: format!("set_{option}"),
        event: format!("{}_settings_changed", example.page.replace('-', "_")),
        target_state_id: props.state_id.as_str().to_string(),
        option: TypedOptionMutationReport {
            name: option.to_string(),
            value_type: value_type.to_string(),
            before_value: before.clone(),
            after_value: actual_after.clone(),
        },
        state: BeforeAfterReport {
            before: option_state_summary(option, props),
            after: option_state_summary(option, &after_props),
        },
        preview: BeforeAfterReport {
            before: format!("{marker}:preview:{option}={before}"),
            after: format!("{marker}:preview:{option}={actual_after}"),
        },
    }
}

fn parity_settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    legacy_dod_specs()
        .filter(|it| it.marker == "23-color-picker-parity")
        .filter_map(|spec| {
            legacy_example(examples, spec)
                .map(|example| settings_mutation_for_spec(example, Some(spec)))
        })
        .collect()
}

fn closeable_tab_strip_settings_mutations(
    examples: &[StoryExample],
) -> Vec<SettingsMutationReport> {
    let Some(example) = examples.iter().find(|it| it.page == "closeable-tab-strip") else {
        return Vec::new();
    };
    let state_id = example.tree.root().props().state_id.as_str().to_string();
    closeable_tab_strip_options()
        .into_iter()
        .map(|option| closeable_tab_strip_report(option, &state_id))
        .collect()
}

fn closeable_tab_strip_report(
    option: CloseableTabStripOption,
    state_id: &str,
) -> SettingsMutationReport {
    let marker = "catalog-closeable-tab-strip".to_string();
    SettingsMutationReport {
        page: "closeable-tab-strip".to_string(),
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
            before: format!("tabs option:{}={}", option.name, option.before),
            after: format!("tabs option:{}={}", option.name, option.after),
        },
        preview: BeforeAfterReport {
            before: format!("{marker}:preview:{}={}", option.name, option.before),
            after: format!("{marker}:preview:{}={}", option.name, option.after),
        },
    }
}

fn closeable_tab_strip_options() -> [CloseableTabStripOption; 5] {
    [
        CloseableTabStripOption {
            name: "tab.count",
            value_type: "usize",
            before: "6",
            after: "7",
            action: "add_tab",
            event: "closeable_tab_strip_tab_added",
        },
        CloseableTabStripOption {
            name: "tab.deleted",
            value_type: "bool",
            before: "false",
            after: "true",
            action: "delete_tab",
            event: "closeable_tab_strip_tab_deleted",
        },
        CloseableTabStripOption {
            name: "tab.pinned",
            value_type: "bool",
            before: "false",
            after: "true",
            action: "pin_tab",
            event: "closeable_tab_strip_pin_changed",
        },
        CloseableTabStripOption {
            name: "tab.dirty",
            value_type: "bool",
            before: "false",
            after: "true",
            action: "dirty_toggle",
            event: "closeable_tab_strip_dirty_changed",
        },
        CloseableTabStripOption {
            name: "tab.group",
            value_type: "TabGroup",
            before: "docs",
            after: "preview",
            action: "group_toggle",
            event: "closeable_tab_strip_group_changed",
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct CloseableTabStripOption {
    name: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
    action: &'static str,
    event: &'static str,
}

fn ui_marker(spec: &LegacyDodSpec, example: &StoryExample) -> LegacyUiMarkerReport {
    let props = example.tree.root().props();
    let marker = ui_marker_name(Some(spec), example.page);
    LegacyUiMarkerReport {
        page: example.page.to_string(),
        ui_marker: marker.clone(),
        root_kind: format!("{:?}", example.tree.root().kind()),
        state_id: props.state_id.as_str().to_string(),
        preview_marker: format!("{marker}:{}", spec.presets[0]),
    }
}

fn preset_difference(spec: &LegacyDodSpec) -> PresetDifferenceReport {
    let marker = ui_marker_name(Some(spec), spec.page);
    PresetDifferenceReport {
        page: spec.page.to_string(),
        ui_marker: marker.clone(),
        default_marker: format!("{marker}:default:{}", spec.presets[0]),
        interactive_marker: format!("{marker}:interactive:{}", spec.presets[1]),
        edge_marker: format!("{marker}:edge:{}", spec.presets[2]),
        theme_marker: format!("{marker}:theme:{}", spec.presets[3]),
    }
}

fn legacy_example<'a>(
    examples: &'a [StoryExample],
    spec: &LegacyDodSpec,
) -> Option<&'a StoryExample> {
    examples.iter().find(|it| it.page == spec.page)
}

fn spec_for(page: &str) -> Option<&'static LegacyDodSpec> {
    legacy_dod_specs().find(|it| it.page == page)
}

fn fallback_option(page: &str) -> &'static str {
    if page == "context-menu" {
        return "context_menu.anchor";
    }
    "theme_id"
}

fn fallback_after(page: &str) -> &'static str {
    if page == "context-menu" {
        return "Pointer(192,128)";
    }
    "dark"
}

fn ui_marker_name(spec: Option<&LegacyDodSpec>, page: &str) -> String {
    spec.map_or_else(
        || format!("catalog-{page}"),
        |it| format!("legacy-{}", it.marker),
    )
}
