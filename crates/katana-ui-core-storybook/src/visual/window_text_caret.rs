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

#[cfg(test)]
mod tests {
    use super::{show_active_text_caret, update_active_text_caret};
    use crate::visual::window_interaction::StorybookWindowState;

    #[test]
    fn active_caret_routes_text_input_text_area_and_passive_pages() {
        let mut input = state_for("text-input");
        input.screen_state.register_text_input_focus_for(
            crate::visual::text_input_screen_state::DEFAULT_TEXT_INPUT_INSTANCE,
            "value",
            false,
        );
        assert!(!show_active_text_caret(&mut input));
        assert!(update_active_text_caret(&mut input, 30, 0));
        assert!(!update_active_text_caret(&mut input, 60, 30));

        let mut area = state_for("text-area");
        area.screen_state.register_text_area_focus_for(
            crate::visual::text_area_screen_state::DEFAULT_TEXT_AREA_INSTANCE,
            false,
            false,
        );
        assert!(!show_active_text_caret(&mut area));
        assert!(update_active_text_caret(&mut area, 30, 0));
        assert!(!update_active_text_caret(&mut area, 60, 30));

        let mut passive = state_for("button");
        assert!(!show_active_text_caret(&mut passive));
        assert!(!update_active_text_caret(&mut passive, 60, 0));
    }

    fn state_for(selected_page: &'static str) -> StorybookWindowState {
        StorybookWindowState {
            selected_page,
            ..StorybookWindowState::default()
        }
    }
}
