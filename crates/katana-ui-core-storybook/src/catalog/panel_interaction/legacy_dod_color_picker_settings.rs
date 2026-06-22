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

fn color_picker_options() -> Vec<ColorPickerSettingOption> {
    vec![
        ColorPickerSettingOption {
            name: "color_picker.rgba",
            value_type: "RgbaColor",
            before: "64,128,255,204",
            after: "72,136,240,188",
        },
        ColorPickerSettingOption {
            name: "color_picker.value",
            value_type: "String",
            before: "rgba(64,128,255,204)",
            after: "rgba(72,136,240,188)",
        },
        ColorPickerSettingOption {
            name: "color_picker.open",
            value_type: "bool",
            before: "false",
            after: "true",
        },
        ColorPickerSettingOption {
            name: "color_picker.hue",
            value_type: "u16",
            before: "214",
            after: "226",
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
            name: "color_picker.color_area",
            value_type: "ColorAreaMode",
            before: "SaturationValue",
            after: "HueSaturation",
        },
        ColorPickerSettingOption {
            name: "color_picker.trigger_size",
            value_type: "UiSize",
            before: "Large",
            after: "Small",
        },
        ColorPickerSettingOption {
            name: "color_picker.title",
            value_type: "String",
            before: "Brand color",
            after: "Accent color",
        },
        ColorPickerSettingOption {
            name: "color_picker.rgba_mode",
            value_type: "bool",
            before: "true",
            after: "false",
        },
        ColorPickerSettingOption {
            name: "color_picker.panel_scale_percent",
            value_type: "u8",
            before: "75",
            after: "100",
        },
        ColorPickerSettingOption {
            name: "color_picker.trigger_border",
            value_type: "bool",
            before: "true",
            after: "false",
        },
        ColorPickerSettingOption {
            name: "color_picker.eyedropper_callback",
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
