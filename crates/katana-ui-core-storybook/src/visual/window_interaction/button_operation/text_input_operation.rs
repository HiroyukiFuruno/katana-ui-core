use super::{StorybookButtonOperation, StorybookWindowState};
use crate::visual::dedicated_dod_form_input_live as input_live;
use crate::visual::dedicated_dod_form_input_live_values::input_static_value_for_preset;
use crate::visual::preview_detail;

const ICON_BUTTONS_PRESET_INDEX: usize = 6;
const READONLY_PRESET_INDEX: usize = 2;
const CLEAR_ACTION_PRESET_INDEX: usize = 12;

pub(super) fn operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    if state.selected_page != "text-input" {
        return None;
    }
    let origin = preview_detail::component_action_hit_rect(state.selected_page);
    if state.preset_index == ICON_BUTTONS_PRESET_INDEX
        && icon_button_index_at(origin.x, origin.y, x, y).is_some()
    {
        return Some(StorybookButtonOperation::TextInputIconButton);
    }
    if state.preset_index == CLEAR_ACTION_PRESET_INDEX
        && input_live::search_inline_clear_rect(origin.x, origin.y).contains(x, y)
    {
        return Some(StorybookButtonOperation::TextInputClearAction {
            initial_value: input_static_value_for_preset(state.preset_index),
            readonly: false,
        });
    }
    if input_live::search_field_rect(origin.x, origin.y).contains(x, y) {
        return Some(StorybookButtonOperation::TextInputFocus {
            initial_value: input_static_value_for_preset(state.preset_index),
            readonly: state.preset_index == READONLY_PRESET_INDEX,
        });
    }
    None
}

pub(super) fn hovered_icon_button_index_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<usize> {
    if state.selected_page != "text-input" || state.preset_index != ICON_BUTTONS_PRESET_INDEX {
        return None;
    }
    let origin = preview_detail::component_action_hit_rect(state.selected_page);
    icon_button_index_at(origin.x, origin.y, x, y)
}

pub(super) fn hovered_clear_action_at(state: &StorybookWindowState, x: usize, y: usize) -> bool {
    if state.selected_page != "text-input" || state.preset_index != CLEAR_ACTION_PRESET_INDEX {
        return false;
    }
    let origin = preview_detail::component_action_hit_rect(state.selected_page);
    input_live::search_inline_clear_rect(origin.x, origin.y).contains(x, y)
}

fn icon_button_index_at(origin_x: usize, origin_y: usize, x: usize, y: usize) -> Option<usize> {
    input_live::text_input_trailing_icon_button_rects(origin_x, origin_y)
        .into_iter()
        .position(|rect| rect.contains(x, y))
}
