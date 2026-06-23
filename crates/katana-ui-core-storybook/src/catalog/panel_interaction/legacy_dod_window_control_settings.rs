use super::{BeforeAfterReport, SettingsMutationReport, StoryExample, TypedOptionMutationReport};

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    let Some(example) = examples
        .iter()
        .find(|it| it.page == "window-control-button-group")
    else {
        return Vec::new();
    };
    let state_id = example.tree.root().props().state_id.as_str().to_string();
    options()
        .into_iter()
        .map(|option| report(option, &state_id))
        .collect()
}

fn report(option: WindowControlOption, state_id: &str) -> SettingsMutationReport {
    let marker = "catalog-window-control-button-group".to_string();
    SettingsMutationReport {
        page: "window-control-button-group".to_string(),
        ui_marker: marker.clone(),
        action: format!("set_{}", option.name),
        event: "window_control_settings_changed".to_string(),
        target_state_id: state_id.to_string(),
        option: TypedOptionMutationReport {
            name: option.name.to_string(),
            value_type: option.value_type.to_string(),
            before_value: option.before.to_string(),
            after_value: option.after.to_string(),
        },
        state: BeforeAfterReport {
            before: format!("window control option:{}={}", option.name, option.before),
            after: format!("window control option:{}={}", option.name, option.after),
        },
        preview: BeforeAfterReport {
            before: format!("{marker}:preview:{}={}", option.name, option.before),
            after: format!("{marker}:preview:{}={}", option.name, option.after),
        },
    }
}

fn options() -> Vec<WindowControlOption> {
    vec![
        WindowControlOption {
            name: "window_control.position",
            value_type: "WindowControlsPosition",
            before: "Leading",
            after: "Trailing",
        },
        WindowControlOption {
            name: "window_control.size",
            value_type: "WindowControlSize",
            before: "Compact",
            after: "Tall",
        },
        WindowControlOption {
            name: "window_control.controls",
            value_type: "Vec<WindowControlKind>",
            before: "Close+Minimize+Maximize",
            after: "Close",
        },
        WindowControlOption {
            name: "window_control.visibility",
            value_type: "WindowControlVisibility",
            before: "Always",
            after: "Hover",
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct WindowControlOption {
    name: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
}
