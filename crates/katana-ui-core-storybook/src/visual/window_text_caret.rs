use super::window_interaction::{StorybookWindowState, component_instance_id_for_page};

pub(super) fn show_active_text_caret(state: &mut StorybookWindowState) -> bool {
    match state.selected_page {
        "text-input" => {
            let instance =
                component_instance_id_for_page(state.selected_page, state.selected_instance_id);
            state.screen_state.show_text_input_caret_for(instance)
        }
        "text-area" => state
            .screen_state
            .show_text_area_caret_for(component_instance_id_for_page(
                state.selected_page,
                state.selected_instance_id,
            )),
        _ => false,
    }
}

pub(super) fn update_active_text_caret(
    state: &mut StorybookWindowState,
    frame_index: usize,
    text_caret_epoch_frame: usize,
) -> bool {
    let elapsed_frames = frame_index.saturating_sub(text_caret_epoch_frame);
    match state.selected_page {
        "text-input" => {
            let instance =
                component_instance_id_for_page(state.selected_page, state.selected_instance_id);
            state
                .screen_state
                .update_text_input_caret_visibility_for(instance, elapsed_frames)
        }
        "text-area" => state.screen_state.update_text_area_caret_visibility_for(
            component_instance_id_for_page(state.selected_page, state.selected_instance_id),
            elapsed_frames,
        ),
        _ => false,
    }
}
