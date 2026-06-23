use super::{BeforeAfterReport, SettingsMutationReport, StoryExample, TypedOptionMutationReport};

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    let Some(example) = examples.iter().find(|it| it.page == "split-pane") else {
        return Vec::new();
    };
    let state_id = example.tree.root().props().state_id.as_str().to_string();
    split_pane_options()
        .into_iter()
        .map(|option| report(option, &state_id))
        .collect()
}

fn report(option: SplitPaneOption, state_id: &str) -> SettingsMutationReport {
    let marker = "catalog-split-pane".to_string();
    SettingsMutationReport {
        page: "split-pane".to_string(),
        ui_marker: marker.clone(),
        action: format!("set_{}", option.name),
        event: "split_pane_settings_changed".to_string(),
        target_state_id: state_id.to_string(),
        option: TypedOptionMutationReport {
            name: option.name.to_string(),
            value_type: option.value_type.to_string(),
            before_value: option.before.to_string(),
            after_value: option.after.to_string(),
        },
        state: BeforeAfterReport {
            before: format!("split pane option:{}={}", option.name, option.before),
            after: format!("split pane option:{}={}", option.name, option.after),
        },
        preview: BeforeAfterReport {
            before: format!("{marker}:preview:{}={}", option.name, option.before),
            after: format!("{marker}:preview:{}={}", option.name, option.after),
        },
    }
}

fn split_pane_options() -> Vec<SplitPaneOption> {
    vec![
        SplitPaneOption {
            name: "split_pane.axis",
            value_type: "SplitPaneAxis",
            before: "Horizontal",
            after: "Vertical",
        },
        SplitPaneOption {
            name: "split_pane.gap",
            value_type: "Length",
            before: "0",
            after: "12",
        },
        SplitPaneOption {
            name: "split_pane.alignment",
            value_type: "Alignment",
            before: "Start",
            after: "Center",
        },
        SplitPaneOption {
            name: "split_pane.overflow",
            value_type: "OverflowBehavior",
            before: "Fit",
            after: "Scroll",
        },
        SplitPaneOption {
            name: "split_pane.ratio_percent",
            value_type: "Percent",
            before: "50",
            after: "64",
        },
        SplitPaneOption {
            name: "split_pane.min_percent",
            value_type: "Percent",
            before: "20",
            after: "24",
        },
        SplitPaneOption {
            name: "split_pane.max_percent",
            value_type: "Percent",
            before: "80",
            after: "76",
        },
        SplitPaneOption {
            name: "split_pane.reset_percent",
            value_type: "Percent",
            before: "50",
            after: "55",
        },
        SplitPaneOption {
            name: "split_pane.handle_width_px",
            value_type: "Pixels",
            before: "8",
            after: "10",
        },
        SplitPaneOption {
            name: "split_pane.resize_mode",
            value_type: "SplitPaneResizeMode",
            before: "PointerAndKeyboard",
            after: "KeyboardOnly",
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct SplitPaneOption {
    name: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
}
