use super::legacy_dod_options::{
    option_state_summary, option_value, props_with_option, resolved_after_value,
};
use super::legacy_dod_specs::{LegacyDodSpec, legacy_dod_specs};
use crate::catalog::StoryExample;
use serde::{Deserialize, Serialize};

#[path = "legacy_dod_drag_settings.rs"]
mod legacy_dod_drag_settings;

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
