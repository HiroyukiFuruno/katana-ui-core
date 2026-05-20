use super::{BeforeAfterReport, SettingsMutationReport, StoryExample, TypedOptionMutationReport};

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    let Some(example) = examples.iter().find(|it| it.page == "search-control-strip") else {
        return Vec::new();
    };
    let state_id = example.tree.root().props().state_id.as_str().to_string();
    search_options()
        .into_iter()
        .map(|option| report(option, &state_id))
        .collect()
}

fn report(option: SearchSettingOption, state_id: &str) -> SettingsMutationReport {
    let marker = "catalog-search-control-strip".to_string();
    SettingsMutationReport {
        page: "search-control-strip".to_string(),
        ui_marker: marker.clone(),
        action: format!("set_{}", option.name),
        event: "search_control_strip_settings_changed".to_string(),
        target_state_id: state_id.to_string(),
        option: TypedOptionMutationReport {
            name: option.name.to_string(),
            value_type: option.value_type.to_string(),
            before_value: option.before.to_string(),
            after_value: option.after.to_string(),
        },
        state: BeforeAfterReport {
            before: format!("search strip option:{}={}", option.name, option.before),
            after: format!("search strip option:{}={}", option.name, option.after),
        },
        preview: BeforeAfterReport {
            before: format!("{marker}:preview:{}={}", option.name, option.before),
            after: format!("{marker}:preview:{}={}", option.name, option.after),
        },
    }
}

fn search_options() -> [SearchSettingOption; 7] {
    [
        SearchSettingOption {
            name: "search_control.query",
            value_type: "String",
            before: "head",
            after: "heading",
        },
        SearchSettingOption {
            name: "search_control.match_case",
            value_type: "bool",
            before: "false",
            after: "true",
        },
        SearchSettingOption {
            name: "search_control.whole_word",
            value_type: "bool",
            before: "false",
            after: "true",
        },
        SearchSettingOption {
            name: "search_control.use_regex",
            value_type: "bool",
            before: "false",
            after: "true",
        },
        SearchSettingOption {
            name: "search_control.replace_mode",
            value_type: "ReplaceMode",
            before: "Visible",
            after: "Disabled",
        },
        SearchSettingOption {
            name: "search_control.result_count",
            value_type: "usize",
            before: "12",
            after: "0",
        },
        SearchSettingOption {
            name: "search_control.active_index",
            value_type: "Option<usize>",
            before: "Some(2)",
            after: "None",
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct SearchSettingOption {
    name: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
}
