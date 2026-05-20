use super::{BeforeAfterReport, SettingsMutationReport, TypedOptionMutationReport};
use crate::catalog::StoryExample;

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    chip_options()
        .into_iter()
        .filter_map(|option| chip_report(examples, option))
        .collect()
}

fn chip_report(
    examples: &[StoryExample],
    option: ChipSettingOption,
) -> Option<SettingsMutationReport> {
    let example = examples.iter().find(|it| it.page == option.page)?;
    let state_id = example.tree.root().props().state_id.as_str().to_string();
    let marker = format!("catalog-{}", option.page);
    Some(SettingsMutationReport {
        page: option.page.to_string(),
        ui_marker: marker.clone(),
        action: format!("set_{}", option.name),
        event: option.event.to_string(),
        target_state_id: state_id,
        option: TypedOptionMutationReport {
            name: option.name.to_string(),
            value_type: option.value_type.to_string(),
            before_value: option.before.to_string(),
            after_value: option.after.to_string(),
        },
        state: BeforeAfterReport {
            before: format!(
                "{} option:{}={}",
                option.state_prefix, option.name, option.before
            ),
            after: format!(
                "{} option:{}={}",
                option.state_prefix, option.name, option.after
            ),
        },
        preview: BeforeAfterReport {
            before: format!("{marker}:preview:{}={}", option.name, option.before),
            after: format!("{marker}:preview:{}={}", option.name, option.after),
        },
    })
}

fn chip_options() -> [ChipSettingOption; 6] {
    [
        ChipSettingOption {
            page: "chip",
            name: "chip.variant",
            value_type: "ChipVariant",
            before: "Outline",
            after: "Filled",
            event: "chip_settings_changed",
            state_prefix: "chip",
        },
        ChipSettingOption {
            page: "chip",
            name: "chip.tone",
            value_type: "ChipTone",
            before: "Accent",
            after: "Danger",
            event: "chip_settings_changed",
            state_prefix: "chip",
        },
        ChipSettingOption {
            page: "chip",
            name: "chip.size",
            value_type: "ChipSize",
            before: "Medium",
            after: "Large",
            event: "chip_settings_changed",
            state_prefix: "chip",
        },
        ChipSettingOption {
            page: "attachment-chip",
            name: "attachment.status",
            value_type: "AttachmentStatus",
            before: "Uploading",
            after: "Error",
            event: "attachment_chip_settings_changed",
            state_prefix: "attachment",
        },
        ChipSettingOption {
            page: "attachment-chip",
            name: "attachment.progress",
            value_type: "AttachmentProgress",
            before: "42",
            after: "100",
            event: "attachment_chip_settings_changed",
            state_prefix: "attachment",
        },
        ChipSettingOption {
            page: "chip-group",
            name: "chip_group.overflow",
            value_type: "ChipGroupOverflow",
            before: "Menu",
            after: "ScrollHorizontal",
            event: "chip_group_settings_changed",
            state_prefix: "chip_group",
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct ChipSettingOption {
    page: &'static str,
    name: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
    event: &'static str,
    state_prefix: &'static str,
}
