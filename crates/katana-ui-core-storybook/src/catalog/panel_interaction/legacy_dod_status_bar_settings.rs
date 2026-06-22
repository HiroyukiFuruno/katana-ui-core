use super::{BeforeAfterReport, SettingsMutationReport, TypedOptionMutationReport};
use crate::catalog::StoryExample;

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    let Some(example) = examples.iter().find(|it| it.page == "status-bar") else {
        return Vec::new();
    };
    let state_id = example.tree.root().props().state_id.as_str().to_string();
    status_bar_options()
        .into_iter()
        .map(|option| status_bar_report(option, &state_id))
        .collect()
}

fn status_bar_report(option: StatusBarSettingOption, state_id: &str) -> SettingsMutationReport {
    let marker = "catalog-status-bar".to_string();
    SettingsMutationReport {
        page: "status-bar".to_string(),
        ui_marker: marker.clone(),
        action: format!("set_{}", option.name),
        event: "status_bar_settings_changed".to_string(),
        target_state_id: state_id.to_string(),
        option: TypedOptionMutationReport {
            name: option.name.to_string(),
            value_type: option.value_type.to_string(),
            before_value: option.before.to_string(),
            after_value: option.after.to_string(),
        },
        state: BeforeAfterReport {
            before: format!("status bar option:{}={}", option.name, option.before),
            after: format!("status bar option:{}={}", option.name, option.after),
        },
        preview: BeforeAfterReport {
            before: format!("{marker}:preview:{}={}", option.name, option.before),
            after: format!("{marker}:preview:{}={}", option.name, option.after),
        },
    }
}

fn status_bar_options() -> Vec<StatusBarSettingOption> {
    vec![
        StatusBarSettingOption {
            name: "status_bar.mode",
            value_type: "StatusBarMode",
            before: "SingleMessage",
            after: "MultiSegment",
        },
        StatusBarSettingOption {
            name: "status_bar.segments",
            value_type: "usize",
            before: "1",
            after: "4",
        },
        StatusBarSettingOption {
            name: "status_bar.density",
            value_type: "StatusBarDensity",
            before: "Default",
            after: "Compact",
        },
        StatusBarSettingOption {
            name: "status_bar.message",
            value_type: "Option<String>",
            before: "None",
            after: "Ready",
        },
        StatusBarSettingOption {
            name: "status_bar.severity",
            value_type: "UiTone",
            before: "Neutral",
            after: "Warning",
        },
        StatusBarSettingOption {
            name: "status_bar.dismiss",
            value_type: "UiDismissAction",
            before: "None",
            after: "Available",
        },
        StatusBarSettingOption {
            name: "status_bar.segment_a11y",
            value_type: "String",
            before: "default",
            after: "custom",
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct StatusBarSettingOption {
    name: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
}
