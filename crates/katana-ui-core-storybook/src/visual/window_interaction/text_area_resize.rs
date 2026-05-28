use super::super::dedicated_dod_form_input_live as input_live;
use super::super::preview_detail;
use super::StorybookWindowState;

pub(super) fn handle_at(state: &StorybookWindowState, x: usize, y: usize) -> bool {
    if state.selected_page != "text-area"
        || !input_live::text_area_resize_enabled_for(state.preset_index, &state.screen_state)
    {
        return false;
    }
    let origin = preview_detail::component_action_hit_rect(state.selected_page);
    input_live::text_area_resize_grip_rect_for(
        origin.x,
        origin.y,
        state.preset_index,
        &state.screen_state,
    )
    .is_some_and(|rect| rect.contains(x, y))
}

pub(super) fn apply_drag_at(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if state.selected_page != "text-area" {
        return false;
    }
    if !input_live::text_area_resize_enabled_for(state.preset_index, &state.screen_state) {
        return false;
    }
    let origin = preview_detail::component_action_hit_rect(state.selected_page);
    let (width_delta, height_delta) =
        input_live::text_area_resize_delta_for_pointer(origin.x, origin.y, x, y);
    state
        .screen_state
        .register_text_area_resize_drag(width_delta, height_delta)
}
