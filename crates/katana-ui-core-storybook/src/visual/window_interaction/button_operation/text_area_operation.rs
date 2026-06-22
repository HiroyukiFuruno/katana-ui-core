use super::{StorybookButtonOperation, StorybookWindowState};
use crate::visual::dedicated_dod_form_input_live as input_live;
use crate::visual::preview_detail;

const DISABLED_PRESET_INDEX: usize = 16;
const READONLY_PRESET_INDEX: usize = 17;

pub(super) fn operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    if state.selected_page != "text-area" {
        return None;
    }
    let origin = preview_detail::component_action_hit_rect(state.selected_page);
    let instance = component_instance(state);
    let (readonly, disabled) = mode_for(state, instance);
    if state.preset_index == input_live::TEXT_AREA_TRAILING_BUTTONS_PRESET_INDEX
        && input_live::text_area_trailing_icon_button_rects(origin.x, origin.y)
            .into_iter()
            .any(|rect| rect.contains(x, y))
    {
        return Some(StorybookButtonOperation::TextAreaIconButton);
    }
    if state.preset_index == input_live::TEXT_AREA_CLEAR_ACTION_PRESET_INDEX
        && input_live::text_area_clear_action_rect(origin.x, origin.y).contains(x, y)
    {
        return Some(StorybookButtonOperation::TextAreaClearAction { readonly, disabled });
    }
    if input_live::text_area_rect_for_screen_state_instance(
        origin.x,
        origin.y,
        &state.screen_state,
        instance,
    )
    .contains(x, y)
    {
        return Some(StorybookButtonOperation::TextAreaFocus { readonly, disabled });
    }
    None
}

pub(super) fn hovered_icon_button_index_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<usize> {
    if state.selected_page != "text-area"
        || state.preset_index != input_live::TEXT_AREA_TRAILING_BUTTONS_PRESET_INDEX
    {
        return None;
    }
    let origin = preview_detail::component_action_hit_rect(state.selected_page);
    input_live::text_area_trailing_icon_button_rects(origin.x, origin.y)
        .into_iter()
        .position(|rect| rect.contains(x, y))
}

pub(super) fn hovered_clear_action_at(state: &StorybookWindowState, x: usize, y: usize) -> bool {
    if state.selected_page != "text-area"
        || state.preset_index != input_live::TEXT_AREA_CLEAR_ACTION_PRESET_INDEX
    {
        return false;
    }
    let origin = preview_detail::component_action_hit_rect(state.selected_page);
    input_live::text_area_clear_action_rect(origin.x, origin.y).contains(x, y)
}

fn mode_for(state: &StorybookWindowState, instance: &'static str) -> (bool, bool) {
    (
        state.preset_index == READONLY_PRESET_INDEX
            || state.screen_state.text_area_readonly_for(instance),
        state.preset_index == DISABLED_PRESET_INDEX
            || state.screen_state.text_area_disabled_for(instance),
    )
}

fn component_instance(state: &StorybookWindowState) -> &'static str {
    super::super::component_instance_id_for_page(state.selected_page, state.selected_instance_id)
}
