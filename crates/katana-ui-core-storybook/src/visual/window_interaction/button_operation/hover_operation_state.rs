use super::{
    StorybookWindowState, preview, preview_detail, text_area_operation, text_input_operation,
    toolbar_operation,
};

pub(super) fn apply(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    let checkbox_hover_changed = state
        .screen_state
        .set_checkbox_hovered_index(checkbox_hovered_index_at(state, x, y));
    let icon_button_changed = state.screen_state.set_hovered_text_input_icon_button_index(
        text_input_operation::hovered_icon_button_index_at(state, x, y),
    );
    let input_clear_changed = state.screen_state.set_hovered_text_input_clear_action(
        text_input_operation::hovered_clear_action_at(state, x, y),
    );
    let text_area_icon_changed = state.screen_state.set_hovered_text_area_icon_button_index(
        text_area_operation::hovered_icon_button_index_at(state, x, y),
    );
    let text_area_clear_changed = state.screen_state.set_hovered_text_area_clear_action(
        text_area_operation::hovered_clear_action_at(state, x, y),
    );
    let toolbar_action_changed = state.screen_state.set_hovered_toolbar_action_index(
        toolbar_operation::hovered_action_index_at(state.selected_page, x, y),
    );
    let summary_changed = state
        .screen_state
        .set_hovered_summary_index(preview::summary_control_index_at(x, y));
    let hovered = state.selected_page != "tooltip"
        && preview_detail::component_action_hit_rect(state.selected_page).contains(x, y);
    state.screen_state.set_preview_hovered(hovered)
        || summary_changed
        || icon_button_changed
        || input_clear_changed
        || text_area_icon_changed
        || text_area_clear_changed
        || toolbar_action_changed
        || checkbox_hover_changed
}

fn checkbox_hovered_index_at(state: &StorybookWindowState, x: usize, y: usize) -> Option<usize> {
    if state.selected_page != "checkbox" {
        return None;
    }
    let component = preview_detail::component_action_hit_rect("checkbox");
    (0..2).find(|index| {
        crate::visual::dedicated_dod_form_binary_choice_live::row_rect(
            *index,
            component.x,
            component.y,
        )
        .contains(x, y)
    })
}
