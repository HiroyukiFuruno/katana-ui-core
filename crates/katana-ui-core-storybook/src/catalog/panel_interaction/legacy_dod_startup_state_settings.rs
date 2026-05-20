use super::{BeforeAfterReport, SettingsMutationReport, StoryExample, TypedOptionMutationReport};

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    let Some(example) = examples.iter().find(|it| it.page == "startup-state-panel") else {
        return Vec::new();
    };
    let state_id = example.tree.root().props().state_id.as_str().to_string();
    options()
        .into_iter()
        .map(|option| report(option, &state_id))
        .collect()
}

fn report(option: StartupStateOption, state_id: &str) -> SettingsMutationReport {
    let marker = "catalog-startup-state-panel".to_string();
    SettingsMutationReport {
        page: "startup-state-panel".to_string(),
        ui_marker: marker.clone(),
        action: format!("set_{}", option.name),
        event: "startup_state_settings_changed".to_string(),
        target_state_id: state_id.to_string(),
        option: TypedOptionMutationReport {
            name: option.name.to_string(),
            value_type: option.value_type.to_string(),
            before_value: option.before.to_string(),
            after_value: option.after.to_string(),
        },
        state: BeforeAfterReport {
            before: format!("startup option:{}={}", option.name, option.before),
            after: format!("startup option:{}={}", option.name, option.after),
        },
        preview: BeforeAfterReport {
            before: format!("{marker}:preview:{}={}", option.name, option.before),
            after: format!("{marker}:preview:{}={}", option.name, option.after),
        },
    }
}

fn options() -> [StartupStateOption; 5] {
    [
        StartupStateOption {
            name: "startup_state.state",
            value_type: "StartupState",
            before: "Loading",
            after: "Error",
        },
        StartupStateOption {
            name: "startup_state.progress",
            value_type: "Option<u8>",
            before: "None",
            after: "64",
        },
        StartupStateOption {
            name: "startup_state.label",
            value_type: "String",
            before: "Preparing session",
            after: "Loading workspace",
        },
        StartupStateOption {
            name: "startup_state.retry",
            value_type: "bool",
            before: "false",
            after: "true",
        },
        StartupStateOption {
            name: "startup_state.cancel",
            value_type: "bool",
            before: "false",
            after: "true",
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct StartupStateOption {
    name: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
}
