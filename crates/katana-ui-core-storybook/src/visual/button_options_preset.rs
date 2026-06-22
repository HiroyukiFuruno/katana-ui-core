use super::model::{
    StorybookButtonCommandMode, StorybookButtonHeightMode, StorybookButtonIconPosition,
    StorybookButtonLayoutPreset, StorybookButtonOptionControl, StorybookButtonOptions,
    StorybookButtonTabIndex, StorybookButtonWidthMode, StorybookButtonZIndex,
};

const VISIBLE_PRESET_INDEX: usize = 4;
const DISABLED_PRESET_INDEX: usize = 5;
const FOCUSABLE_PRESET_INDEX: usize = 6;
const WIDTH_PRESET_INDEX: usize = 7;
const HEIGHT_PRESET_INDEX: usize = 8;
const BORDER_PRESET_INDEX: usize = 9;
const LABEL_PRESET_INDEX: usize = 10;
const TAB_INDEX_PRESET_INDEX: usize = 11;
const Z_INDEX_PRESET_INDEX: usize = 12;
const COMMAND_PRESET_INDEX: usize = 13;
const KEYBOARD_PRESET_INDEX: usize = 14;
const ICON_POSITION_PRESET_INDEX: usize = 15;
const LAYOUT_PRESET_INDEX: usize = 16;
const SVG_SOURCE_PRESET_INDEX: usize = 17;
const ARIA_LABEL_PRESET_INDEX: usize = 18;

pub(in crate::visual) fn preset_button_options(preset_index: usize) -> StorybookButtonOptions {
    let mut options = StorybookButtonOptions::default();
    match preset_index {
        VISIBLE_PRESET_INDEX => options.visible = false,
        DISABLED_PRESET_INDEX => options.disabled = true,
        FOCUSABLE_PRESET_INDEX => options.focusable = false,
        WIDTH_PRESET_INDEX => options.width_mode = StorybookButtonWidthMode::Px,
        HEIGHT_PRESET_INDEX => options.height_mode = StorybookButtonHeightMode::Tall,
        BORDER_PRESET_INDEX => options.border = false,
        LABEL_PRESET_INDEX => options.japanese_label = true,
        TAB_INDEX_PRESET_INDEX => options.tab_index = StorybookButtonTabIndex::One,
        Z_INDEX_PRESET_INDEX => options.z_index = StorybookButtonZIndex::Raised,
        COMMAND_PRESET_INDEX => options.command_mode = StorybookButtonCommandMode::Open,
        KEYBOARD_PRESET_INDEX => options.keyboard_activation = false,
        ICON_POSITION_PRESET_INDEX => options.icon_position = StorybookButtonIconPosition::Trailing,
        LAYOUT_PRESET_INDEX => options.layout_preset = StorybookButtonLayoutPreset::Dense,
        SVG_SOURCE_PRESET_INDEX => options.external_svg_source = true,
        ARIA_LABEL_PRESET_INDEX => options.aria_label = true,
        _ => {}
    }
    options
}

pub(in crate::visual) const fn preset_index_for_control(
    control: StorybookButtonOptionControl,
) -> usize {
    match control {
        StorybookButtonOptionControl::Visible => VISIBLE_PRESET_INDEX,
        StorybookButtonOptionControl::Disabled => DISABLED_PRESET_INDEX,
        StorybookButtonOptionControl::Focusable => FOCUSABLE_PRESET_INDEX,
        StorybookButtonOptionControl::Width => WIDTH_PRESET_INDEX,
        StorybookButtonOptionControl::Height => HEIGHT_PRESET_INDEX,
        StorybookButtonOptionControl::Border => BORDER_PRESET_INDEX,
        StorybookButtonOptionControl::Label => LABEL_PRESET_INDEX,
        StorybookButtonOptionControl::TabIndex => TAB_INDEX_PRESET_INDEX,
        StorybookButtonOptionControl::ZIndex => Z_INDEX_PRESET_INDEX,
        StorybookButtonOptionControl::Command => COMMAND_PRESET_INDEX,
        StorybookButtonOptionControl::KeyboardActivation => KEYBOARD_PRESET_INDEX,
        StorybookButtonOptionControl::IconPosition => ICON_POSITION_PRESET_INDEX,
        StorybookButtonOptionControl::LayoutPreset => LAYOUT_PRESET_INDEX,
        StorybookButtonOptionControl::SvgSource => SVG_SOURCE_PRESET_INDEX,
        StorybookButtonOptionControl::AriaLabel => ARIA_LABEL_PRESET_INDEX,
    }
}
