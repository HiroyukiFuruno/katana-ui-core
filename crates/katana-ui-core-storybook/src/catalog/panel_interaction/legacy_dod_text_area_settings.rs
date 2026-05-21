use super::{BeforeAfterReport, SettingsMutationReport, TypedOptionMutationReport};
use crate::catalog::StoryExample;

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    let Some(example) = examples.iter().find(|it| it.page == "text-area") else {
        return Vec::new();
    };
    let state_id = example.tree.root().props().state_id.as_str().to_string();
    text_area_options()
        .into_iter()
        .map(|option| text_area_report(option, &state_id))
        .collect()
}

fn text_area_report(option: TextAreaSettingOption, state_id: &str) -> SettingsMutationReport {
    let marker = "catalog-text-area".to_string();
    SettingsMutationReport {
        page: "text-area".to_string(),
        ui_marker: marker.clone(),
        action: format!("set_{}", option.name),
        event: "text_area_settings_changed".to_string(),
        target_state_id: state_id.to_string(),
        option: TypedOptionMutationReport {
            name: option.name.to_string(),
            value_type: option.value_type.to_string(),
            before_value: option.before.to_string(),
            after_value: option.after.to_string(),
        },
        state: BeforeAfterReport {
            before: format!("text_area option:{}={}", option.name, option.before),
            after: format!("text_area option:{}={}", option.name, option.after),
        },
        preview: BeforeAfterReport {
            before: format!("{marker}:preview:{}={}", option.name, option.before),
            after: format!("{marker}:preview:{}={}", option.name, option.after),
        },
    }
}

fn text_area_options() -> Vec<TextAreaSettingOption> {
    vec![
        TextAreaSettingOption {
            name: "text_area.submit_key",
            value_type: "TextAreaSubmitKey",
            before: "Enter",
            after: "ModEnter",
        },
        TextAreaSettingOption {
            name: "text_area.newline_key",
            value_type: "TextAreaNewlineKey",
            before: "ShiftEnter",
            after: "Enter",
        },
        TextAreaSettingOption {
            name: "text_area.tab_behavior",
            value_type: "TextAreaTabBehavior",
            before: "MoveFocus",
            after: "InsertTab",
        },
        TextAreaSettingOption {
            name: "text_area.auto_grow",
            value_type: "bool",
            before: "true",
            after: "false",
        },
        TextAreaSettingOption {
            name: "text_area.wrap_policy",
            value_type: "TextAreaWrapPolicy",
            before: "Soft",
            after: "Hard",
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct TextAreaSettingOption {
    name: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
}
