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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation_maps_live_blocking_and_hue_state() {
        let rect = preview_detail::component_action_hit_rect(PAGE);

        let mut disabled = StorybookWindowState {
            selected_page: PAGE,
            ..StorybookWindowState::default()
        };
        disabled
            .screen_state
            .color_picker
            .apply_option("color_picker.disabled");
        assert_eq!(
            Some(ColorPickerAction::DisabledBlocked),
            operation_at(&disabled, rect.x, rect.y)
        );

        let mut readonly = StorybookWindowState {
            selected_page: PAGE,
            ..StorybookWindowState::default()
        };
        readonly
            .screen_state
            .color_picker
            .apply_option("color_picker.readonly");
        assert_eq!(
            Some(ColorPickerAction::ReadonlyBlocked),
            operation_at(&readonly, rect.x, rect.y)
        );

        let hue = StorybookWindowState {
            selected_page: PAGE,
            preset_index: HUE_PRESET_INDEX,
            ..StorybookWindowState::default()
        };
        assert_eq!(
            Some(ColorPickerAction::HueDrag),
            operation_at(&hue, rect.x, rect.y)
        );
    }
}
