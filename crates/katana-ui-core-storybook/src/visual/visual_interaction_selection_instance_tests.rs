use super::dedicated_dod_form_combo_live;
use super::dedicated_dod_form_input_live;
use super::dedicated_dod_form_select_live;
use super::dedicated_dod_form_selection_list_live;
use super::layout_metrics::LayoutRect;
use super::selection_control_metrics as metrics;
use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{preview_detail, render};

const DIFF_THRESHOLD: usize = 80;
const PRIMARY_INSTANCE: &str = "selection.primary";
const SECONDARY_INSTANCE: &str = "selection.secondary";
const SELECT_BOX_PAGE: &str = "select-box";
const COMBO_BOX_PAGE: &str = "combo-box";
const SEARCH_BOX_PAGE: &str = "search-box";
const SELECTION_LIST_PAGE: &str = "selection-list";
const CLICK_INSET: usize = 2;
const SELECT_DARK_OPTION_Y_OFFSET: usize = 90;
const COMBO_TWO_OPTION_Y_OFFSET: usize = 80;

#[test]
fn select_box_window_interaction_keeps_instance_state_isolated() {
    let mut state = state_for(SELECT_BOX_PAGE);

    state.select_instance(PRIMARY_INSTANCE);
    click_select_trigger(&mut state);
    click_component_offset(
        &mut state,
        SELECT_BOX_PAGE,
        metrics::TRIGGER_X + metrics::OPTION_ROW_INSET,
        SELECT_DARK_OPTION_Y_OFFSET,
    );
    let primary = state.screen_state.clone();
    let primary_canvas = render_state(&state);
    assert_eq!("select_option", primary.last_action);
    assert_eq!(Some(2), primary.selection.select_selected_index);

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!("none", state.screen_state.last_action);
    click_rect(
        &mut state,
        dedicated_dod_form_select_live::select_state_read_button_rect(
            component(SELECT_BOX_PAGE).x,
            component(SELECT_BOX_PAGE).y,
        ),
    );
    let secondary_canvas = render_state(&state);
    assert_eq!("select_state_read", state.screen_state.last_action);
    assert_eq!(None, state.screen_state.selection.select_selected_index);

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary.last_action, state.screen_state.last_action);
    assert_eq!(primary.state_label, state.screen_state.state_label);
    assert!(
        component_body_pixel_diff(SELECT_BOX_PAGE, &primary_canvas, &secondary_canvas)
            > DIFF_THRESHOLD
    );
}

#[test]
fn combo_box_window_interaction_keeps_instance_state_isolated() {
    let mut state = state_for(COMBO_BOX_PAGE);

    state.select_instance(PRIMARY_INSTANCE);
    click_combo_trigger(&mut state);
    click_component_offset(
        &mut state,
        COMBO_BOX_PAGE,
        metrics::TRIGGER_X + metrics::OPTION_ROW_INSET,
        COMBO_TWO_OPTION_Y_OFFSET,
    );
    let primary = state.screen_state.clone();
    let primary_canvas = render_state(&state);
    assert_eq!("combo_select", primary.last_action);
    assert_eq!(Some(1), primary.selection.combo_selected_index);

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!("none", state.screen_state.last_action);
    click_rect(
        &mut state,
        dedicated_dod_form_combo_live::combo_state_read_button_rect(
            component(COMBO_BOX_PAGE).x,
            component(COMBO_BOX_PAGE).y,
        ),
    );
    let secondary_canvas = render_state(&state);
    assert_eq!("combo_state_read", state.screen_state.last_action);
    assert_eq!(None, state.screen_state.selection.combo_selected_index);

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary.last_action, state.screen_state.last_action);
    assert_eq!(primary.state_label, state.screen_state.state_label);
    assert!(
        component_body_pixel_diff(COMBO_BOX_PAGE, &primary_canvas, &secondary_canvas)
            > DIFF_THRESHOLD
    );
}

#[test]
fn search_box_window_interaction_keeps_instance_state_isolated() {
    let mut state = state_for(SEARCH_BOX_PAGE);

    state.select_instance(PRIMARY_INSTANCE);
    click_rect(
        &mut state,
        dedicated_dod_form_input_live::search_type_query_button_rect(
            component(SEARCH_BOX_PAGE).x,
            component(SEARCH_BOX_PAGE).y,
        ),
    );
    let primary = state.screen_state.clone();
    let primary_canvas = render_state(&state);
    assert_eq!("search_type_query", primary.last_action);
    assert!(primary.search_box.typed);

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!("none", state.screen_state.last_action);
    click_rect(
        &mut state,
        dedicated_dod_form_input_live::search_state_read_button_rect(
            component(SEARCH_BOX_PAGE).x,
            component(SEARCH_BOX_PAGE).y,
        ),
    );
    let secondary_canvas = render_state(&state);
    assert_eq!("search_state_read", state.screen_state.last_action);
    assert!(!state.screen_state.search_box.typed);

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary.last_action, state.screen_state.last_action);
    assert_eq!(primary.state_label, state.screen_state.state_label);
    assert!(
        component_body_pixel_diff(SEARCH_BOX_PAGE, &primary_canvas, &secondary_canvas)
            > DIFF_THRESHOLD
    );
}

#[test]
fn selection_list_window_interaction_keeps_instance_state_isolated() {
    let mut state = state_for(SELECTION_LIST_PAGE);

    state.select_instance(PRIMARY_INSTANCE);
    click_rect(
        &mut state,
        dedicated_dod_form_selection_list_live::selection_list_select_row_button_rect(
            component(SELECTION_LIST_PAGE).x,
            component(SELECTION_LIST_PAGE).y,
        ),
    );
    let primary = state.screen_state.clone();
    let primary_canvas = render_state(&state);
    assert_eq!("selection_list_select_row", primary.last_action);
    assert_eq!(Some(1), primary.selection.selection_list_selected_index);

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!("none", state.screen_state.last_action);
    click_rect(
        &mut state,
        dedicated_dod_form_selection_list_live::selection_list_state_read_button_rect(
            component(SELECTION_LIST_PAGE).x,
            component(SELECTION_LIST_PAGE).y,
        ),
    );
    let secondary_canvas = render_state(&state);
    assert_eq!("selection_list_state_read", state.screen_state.last_action);
    assert_eq!(
        None,
        state.screen_state.selection.selection_list_selected_index
    );

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary.last_action, state.screen_state.last_action);
    assert_eq!(primary.state_label, state.screen_state.state_label);
    assert!(
        component_body_pixel_diff(SELECTION_LIST_PAGE, &primary_canvas, &secondary_canvas)
            > DIFF_THRESHOLD
    );
}

fn click_select_trigger(state: &mut StorybookWindowState) {
    click_component_offset(
        state,
        SELECT_BOX_PAGE,
        metrics::TRIGGER_X + CLICK_INSET,
        metrics::TRIGGER_Y + CLICK_INSET,
    );
}

fn click_combo_trigger(state: &mut StorybookWindowState) {
    click_component_offset(
        state,
        COMBO_BOX_PAGE,
        metrics::TRIGGER_X + CLICK_INSET,
        metrics::TRIGGER_Y + CLICK_INSET,
    );
}

fn click_component_offset(
    state: &mut StorybookWindowState,
    page: &'static str,
    x_offset: usize,
    y_offset: usize,
) {
    let rect = preview_detail::component_action_hit_rect(page);
    assert!(apply_click(state, rect.x + x_offset, rect.y + y_offset));
}

fn click_rect(state: &mut StorybookWindowState, rect: LayoutRect) {
    assert!(apply_click(
        state,
        rect.x + CLICK_INSET,
        rect.y + CLICK_INSET,
    ));
}

fn component(page: &'static str) -> LayoutRect {
    preview_detail::component_action_hit_rect(page)
}

fn state_for(page: &'static str) -> StorybookWindowState {
    StorybookWindowState {
        selected_page: page,
        ..StorybookWindowState::default()
    }
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}
