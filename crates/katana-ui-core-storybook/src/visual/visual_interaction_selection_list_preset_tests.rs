use super::canvas::Canvas;
use super::dedicated_dod_form_selection_list_live;
use super::layout_metrics::LayoutRect;
use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{preview_detail, render};

const DIFF_THRESHOLD: usize = 80;
const PRIMARY_INSTANCE: &str = "selection.primary";
const SECONDARY_INSTANCE: &str = "selection.secondary";
const SELECTION_LIST_PAGE: &str = "selection-list";
const CLICK_INSET: usize = 2;
const SELECT_PRESET: usize = 1;
const MULTI_PRESET: usize = 2;
const DYNAMIC_ROW_INDEX: usize = 1;
const DYNAMIC_MULTI_MASK: u8 = 0b10;

#[test]
fn selection_list_window_interaction_keeps_instance_state_isolated_across_presets() {
    let mut state = state_for_selection_list();

    state.select_preset(SELECT_PRESET);
    state.select_instance(PRIMARY_INSTANCE);
    click_select_row(&mut state);
    let primary_select = state.screen_state.clone();
    let primary_select_canvas = render_state(&state);
    assert_eq!(
        Some(DYNAMIC_ROW_INDEX),
        primary_select.selection.selection_list_selected_index
    );

    state.select_preset(MULTI_PRESET);
    assert_eq!(
        None,
        state.screen_state.selection.selection_list_selected_index
    );
    click_multi_toggle(&mut state);
    let primary_multi = state.screen_state.clone();
    assert_eq!(
        DYNAMIC_MULTI_MASK,
        primary_multi.selection.selection_list_multi_mask
    );

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!("none", state.screen_state.last_action);
    assert_eq!(0, state.screen_state.selection.selection_list_multi_mask);

    state.select_preset(SELECT_PRESET);
    assert_eq!(
        None,
        state.screen_state.selection.selection_list_selected_index
    );
    let secondary_select_canvas = render_state(&state);

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary_select.last_action, state.screen_state.last_action);
    assert_eq!(
        primary_select.selection.selection_list_selected_index,
        state.screen_state.selection.selection_list_selected_index
    );

    state.select_preset(MULTI_PRESET);
    assert_eq!(primary_multi.last_action, state.screen_state.last_action);
    assert_eq!(
        primary_multi.selection.selection_list_multi_mask,
        state.screen_state.selection.selection_list_multi_mask
    );
    assert!(
        component_body_pixel_diff(
            SELECTION_LIST_PAGE,
            &primary_select_canvas,
            &secondary_select_canvas,
        ) > DIFF_THRESHOLD
    );
}

fn click_select_row(state: &mut StorybookWindowState) {
    click_rect(
        state,
        dedicated_dod_form_selection_list_live::selection_list_select_row_button_rect(
            component().x,
            component().y,
        ),
    );
}

fn click_multi_toggle(state: &mut StorybookWindowState) {
    click_rect(
        state,
        dedicated_dod_form_selection_list_live::selection_list_multi_toggle_button_rect(
            component().x,
            component().y,
        ),
    );
}

fn click_rect(state: &mut StorybookWindowState, rect: LayoutRect) {
    assert!(apply_click(
        state,
        rect.x + CLICK_INSET,
        rect.y + CLICK_INSET,
    ));
}

fn component() -> LayoutRect {
    preview_detail::component_action_hit_rect(SELECTION_LIST_PAGE)
}

fn state_for_selection_list() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: SELECTION_LIST_PAGE,
        ..StorybookWindowState::default()
    }
}

fn render_state(state: &StorybookWindowState) -> Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}
