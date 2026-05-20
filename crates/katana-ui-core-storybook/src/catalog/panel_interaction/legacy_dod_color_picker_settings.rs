use super::{BeforeAfterReport, SettingsMutationReport, TypedOptionMutationReport};
use crate::catalog::StoryExample;

const PAGE: &str = "color-picker-rgba";
const EVENT: &str = "color_picker_settings_changed";

pub(super) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
    let Some(example) = examples.iter().find(|it| it.page == PAGE) else {
        return Vec::new();
    };
    let state_id = example.tree.root().props().state_id.as_str().to_string();
    color_picker_options()
        .into_iter()
        .map(|option| color_picker_report(option, &state_id))
        .collect()
}

fn color_picker_report(option: ColorPickerSettingOption, state_id: &str) -> SettingsMutationReport {
    let marker = "legacy-22-color-picker".to_string();
    SettingsMutationReport {
        page: PAGE.to_string(),
        ui_marker: marker.clone(),
        action: format!("set_{}", option.name),
        event: EVENT.to_string(),
        target_state_id: state_id.to_string(),
        option: TypedOptionMutationReport {
            name: option.name.to_string(),
            value_type: option.value_type.to_string(),
            before_value: option.before.to_string(),
            after_value: option.after.to_string(),
        },
        state: BeforeAfterReport {
            before: format!("color picker option:{}={}", option.name, option.before),
            after: format!("color picker option:{}={}", option.name, option.after),
        },
        preview: BeforeAfterReport {
            before: format!("{marker}:preview:{}={}", option.name, option.before),
            after: format!("{marker}:preview:{}={}", option.name, option.after),
        },
    }
}

fn color_picker_options() -> [ColorPickerSettingOption; 9] {
    [
        ColorPickerSettingOption {
            name: "color_picker.mode",
            value_type: "ColorPickerMode",
            before: "RGBA",
            after: "RGB",
        },
        ColorPickerSettingOption {
            name: "color_picker.red",
            value_type: "u8",
            before: "64",
            after: "72",
        },
        ColorPickerSettingOption {
            name: "color_picker.green",
            value_type: "u8",
            before: "128",
            after: "136",
        },
        ColorPickerSettingOption {
            name: "color_picker.blue",
            value_type: "u8",
            before: "255",
            after: "240",
        },
        ColorPickerSettingOption {
            name: "color_picker.alpha",
            value_type: "u8",
            before: "204",
            after: "188",
        },
        ColorPickerSettingOption {
            name: "color_picker.blending",
            value_type: "ColorBlendingMode",
            before: "Normal",
            after: "Additive",
        },
        ColorPickerSettingOption {
            name: "color_picker.eyedropper",
            value_type: "bool",
            before: "false",
            after: "true",
        },
        ColorPickerSettingOption {
            name: "color_picker.readonly",
            value_type: "bool",
            before: "false",
            after: "true",
        },
        ColorPickerSettingOption {
            name: "color_picker.disabled",
            value_type: "bool",
            before: "false",
            after: "true",
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct ColorPickerSettingOption {
    name: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
}
