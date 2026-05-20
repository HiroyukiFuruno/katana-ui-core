use super::{BeforeAfterReport, SettingsMutationReport, TypedOptionMutationReport};
use crate::catalog::StoryExample;

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    let Some(example) = examples.iter().find(|it| it.page == "empty-state") else {
        return Vec::new();
    };
    let state_id = example.tree.root().props().state_id.as_str().to_string();
    empty_state_options()
        .into_iter()
        .map(|option| empty_state_report(option, &state_id))
        .collect()
}

fn empty_state_report(option: EmptyStateSettingOption, state_id: &str) -> SettingsMutationReport {
    let marker = "catalog-empty-state".to_string();
    SettingsMutationReport {
        page: "empty-state".to_string(),
        ui_marker: marker.clone(),
        action: format!("set_{}", option.name),
        event: "empty_state_settings_changed".to_string(),
        target_state_id: state_id.to_string(),
        option: TypedOptionMutationReport {
            name: option.name.to_string(),
            value_type: option.value_type.to_string(),
            before_value: option.before.to_string(),
            after_value: option.after.to_string(),
        },
        state: BeforeAfterReport {
            before: format!("empty_state option:{}={}", option.name, option.before),
            after: format!("empty_state option:{}={}", option.name, option.after),
        },
        preview: BeforeAfterReport {
            before: format!("{marker}:preview:{}={}", option.name, option.before),
            after: format!("{marker}:preview:{}={}", option.name, option.after),
        },
    }
}

fn empty_state_options() -> [EmptyStateSettingOption; 4] {
    [
        EmptyStateSettingOption {
            name: "empty_state.tone",
            value_type: "EmptyStateTone",
            before: "Accent",
            after: "Danger",
        },
        EmptyStateSettingOption {
            name: "empty_state.size",
            value_type: "EmptyStateSize",
            before: "Default",
            after: "Large",
        },
        EmptyStateSettingOption {
            name: "empty_state.alignment",
            value_type: "EmptyStateAlignment",
            before: "Center",
            after: "Leading",
        },
        EmptyStateSettingOption {
            name: "empty_state.actions",
            value_type: "EmptyStateActions",
            before: "Primary",
            after: "Primary+Secondary",
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct EmptyStateSettingOption {
    name: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
}
