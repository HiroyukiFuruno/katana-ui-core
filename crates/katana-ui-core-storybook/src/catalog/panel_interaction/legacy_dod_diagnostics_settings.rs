use super::{BeforeAfterReport, SettingsMutationReport, TypedOptionMutationReport};
use crate::catalog::StoryExample;

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    let Some(example) = examples.iter().find(|it| it.page == "diagnostics-list") else {
        return Vec::new();
    };
    let state_id = example.tree.root().props().state_id.as_str().to_string();
    diagnostics_options()
        .into_iter()
        .map(|option| diagnostics_report(option, &state_id))
        .collect()
}

fn diagnostics_report(option: DiagnosticsSettingOption, state_id: &str) -> SettingsMutationReport {
    let marker = "catalog-diagnostics-list".to_string();
    SettingsMutationReport {
        page: "diagnostics-list".to_string(),
        ui_marker: marker.clone(),
        action: format!("set_{}", option.name),
        event: "diagnostics_list_settings_changed".to_string(),
        target_state_id: state_id.to_string(),
        option: TypedOptionMutationReport {
            name: option.name.to_string(),
            value_type: option.value_type.to_string(),
            before_value: option.before.to_string(),
            after_value: option.after.to_string(),
        },
        state: BeforeAfterReport {
            before: format!("diagnostics option:{}={}", option.name, option.before),
            after: format!("diagnostics option:{}={}", option.name, option.after),
        },
        preview: BeforeAfterReport {
            before: format!("{marker}:preview:{}={}", option.name, option.before),
            after: format!("{marker}:preview:{}={}", option.name, option.after),
        },
    }
}

fn diagnostics_options() -> [DiagnosticsSettingOption; 5] {
    [
        DiagnosticsSettingOption {
            name: "diagnostics.group_by",
            value_type: "DiagnosticsGroupBy",
            before: "Severity",
            after: "Source",
        },
        DiagnosticsSettingOption {
            name: "diagnostics.sort_by",
            value_type: "DiagnosticsSortBy",
            before: "Severity",
            after: "Location",
        },
        DiagnosticsSettingOption {
            name: "diagnostics.severity_filter",
            value_type: "BTreeSet<DiagnosticSeverity>",
            before: "Error+Warning",
            after: "Error",
        },
        DiagnosticsSettingOption {
            name: "diagnostics.bulk_action",
            value_type: "DiagnosticsBulkAction",
            before: "Preview",
            after: "Apply",
        },
        DiagnosticsSettingOption {
            name: "diagnostics.fix_preview",
            value_type: "DiagnosticsFixPreviewMode",
            before: "Expanded",
            after: "Collapsed",
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct DiagnosticsSettingOption {
    name: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
}
