use super::{BeforeAfterReport, SettingsMutationReport, TypedOptionMutationReport};
use crate::catalog::StoryExample;

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    let Some(example) = examples.iter().find(|it| it.page == "toast-stack-manager") else {
        return Vec::new();
    };
    let state_id = example.tree.root().props().state_id.as_str().to_string();
    toast_stack_options()
        .into_iter()
        .map(|option| toast_stack_report(option, &state_id))
        .collect()
}

fn toast_stack_report(option: ToastStackSettingOption, state_id: &str) -> SettingsMutationReport {
    let marker = "catalog-toast-stack-manager".to_string();
    SettingsMutationReport {
        page: "toast-stack-manager".to_string(),
        ui_marker: marker.clone(),
        action: format!("set_{}", option.name),
        event: "toast_stack_settings_changed".to_string(),
        target_state_id: state_id.to_string(),
        option: TypedOptionMutationReport {
            name: option.name.to_string(),
            value_type: option.value_type.to_string(),
            before_value: option.before.to_string(),
            after_value: option.after.to_string(),
        },
        state: BeforeAfterReport {
            before: format!("toast stack option:{}={}", option.name, option.before),
            after: format!("toast stack option:{}={}", option.name, option.after),
        },
        preview: BeforeAfterReport {
            before: format!("{marker}:preview:{}={}", option.name, option.before),
            after: format!("{marker}:preview:{}={}", option.name, option.after),
        },
    }
}

fn toast_stack_options() -> Vec<ToastStackSettingOption> {
    vec![
        ToastStackSettingOption {
            name: "toast_stack.position",
            value_type: "ToastPosition",
            before: "BottomEnd",
            after: "TopCenter",
        },
        ToastStackSettingOption {
            name: "toast_stack.max_visible",
            value_type: "usize",
            before: "2",
            after: "4",
        },
        ToastStackSettingOption {
            name: "toast_stack.dedup",
            value_type: "ToastDedupStrategy",
            before: "ById",
            after: "ByIdAndSeverity",
        },
        ToastStackSettingOption {
            name: "toast_stack.duration",
            value_type: "u64",
            before: "8000",
            after: "3000",
        },
        ToastStackSettingOption {
            name: "toast_stack.pause_on_hover",
            value_type: "bool",
            before: "true",
            after: "false",
        },
        ToastStackSettingOption {
            name: "toast_stack.stack_gap",
            value_type: "u16",
            before: "10",
            after: "16",
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct ToastStackSettingOption {
    name: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
}
