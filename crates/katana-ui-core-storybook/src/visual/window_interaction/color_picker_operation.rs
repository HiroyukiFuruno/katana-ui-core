use super::StorybookWindowState;
use crate::visual::preview_detail;

const PAGE: &str = "color-picker-rgba";
const HUE_PRESET_INDEX: usize = 3;
const ALPHA_PRESET_INDEX: usize = 4;
const EYEDROPPER_PRESET_INDEX: usize = 12;
const READONLY_PRESET_INDEX: usize = 13;
const DISABLED_PRESET_INDEX: usize = 14;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum ColorPickerAction {
    Drag,
    HueDrag,
    AlphaDrag,
    Eyedropper,
    Focus,
    Hover,
    ReadonlyBlocked,
    DisabledBlocked,
}

pub(super) fn operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<ColorPickerAction> {
    if state.selected_page != PAGE {
        return None;
    }
    if !preview_detail::component_action_hit_rect(PAGE).contains(x, y) {
        return None;
    }
    if state.screen_state.color_picker.blocks_focus() {
        return Some(ColorPickerAction::DisabledBlocked);
    }
    if state.screen_state.color_picker.blocks_writes() {
        return Some(ColorPickerAction::ReadonlyBlocked);
    }
    Some(match state.preset_index {
        HUE_PRESET_INDEX => ColorPickerAction::HueDrag,
        ALPHA_PRESET_INDEX => ColorPickerAction::AlphaDrag,
        EYEDROPPER_PRESET_INDEX => ColorPickerAction::Eyedropper,
        READONLY_PRESET_INDEX => ColorPickerAction::ReadonlyBlocked,
        DISABLED_PRESET_INDEX => ColorPickerAction::DisabledBlocked,
        _ => ColorPickerAction::Drag,
    })
}
