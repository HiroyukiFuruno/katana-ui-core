use super::super::{StorybookWindowState, apply_click};
use crate::visual::visual_interaction_test_support::component_body_pixel_diff;
use crate::visual::{layout_metrics::LayoutRect, selection_control_metrics as sc};
use crate::visual::{preview_detail, render};

const SELECT_BOX_PAGE: &str = "select-box";
const COMBO_BOX_PAGE: &str = "combo-box";
const SEARCH_BOX_PAGE: &str = "search-box";
const SELECTION_LIST_PAGE: &str = "selection-list";
const COMPONENT_BODY_DIFF_THRESHOLD: usize = 80;
const TRIGGER_X_OFFSET: usize = 20;
const TRIGGER_Y_OFFSET: usize = 34;
const OPTION_X_OFFSET: usize = 20;
const SELECT_DARK_OPTION_Y_OFFSET: usize = 90;
const COMBO_TWO_OPTION_Y_OFFSET: usize = 80;
const SEARCH_FIELD_Y: usize = 36;
const SEARCH_FIELD_X: usize = 18;
const SEARCH_FIELD_WIDTH: usize = 210;
const SEARCH_FIELD_HEIGHT: usize = 34;
const SEARCH_CLEAR_X: usize = 208;
const SEARCH_CLEAR_Y: usize = 46;
const SEARCH_CLEAR_SIZE: usize = 14;
const SEARCH_STATUS_X: usize = 246;
const SEARCH_STATUS_Y: usize = 76;
const SEARCH_STATUS_WIDTH: usize = 84;
const SEARCH_STATUS_HEIGHT: usize = 20;
const SEARCH_STATUS_GAP: usize = 8;
const SEARCH_STATUS_ROW_COUNT: usize = 3;

#[test]
fn select_box_surface_opens_then_selects_option() {
    let mut state = state_for(SELECT_BOX_PAGE);
    let rect = preview_detail::component_action_hit_rect(SELECT_BOX_PAGE);
    let before = render_state(&state);

    assert!(apply_click(
        &mut state,
        rect.x + TRIGGER_X_OFFSET,
        rect.y + TRIGGER_Y_OFFSET
    ));
    assert_eq!("select_open", state.screen_state.last_action);
    assert_eq!("select_opened", state.screen_state.last_event);
    assert_eq!("open=true", state.screen_state.state_label);
    let opened = render_state(&state);
    assert!(
        component_body_pixel_diff(SELECT_BOX_PAGE, &before, &opened)
            > COMPONENT_BODY_DIFF_THRESHOLD
    );

    assert!(apply_click(
        &mut state,
        rect.x + OPTION_X_OFFSET,
        rect.y + SELECT_DARK_OPTION_Y_OFFSET
    ));
    assert_eq!("select_option", state.screen_state.last_action);
    assert_eq!("select_changed", state.screen_state.last_event);
    assert_eq!("selected=dark", state.screen_state.state_label);
    assert!(
        component_body_pixel_diff(SELECT_BOX_PAGE, &opened, &render_state(&state))
            > COMPONENT_BODY_DIFF_THRESHOLD
    );
}

#[test]
fn combo_box_surface_filters_then_selects_option() {
    let mut state = state_for(COMBO_BOX_PAGE);
    let rect = preview_detail::component_action_hit_rect(COMBO_BOX_PAGE);
    let before = render_state(&state);

    assert!(apply_click(
        &mut state,
        rect.x + TRIGGER_X_OFFSET,
        rect.y + TRIGGER_Y_OFFSET
    ));
    assert_eq!("combo_filter", state.screen_state.last_action);
    assert_eq!("combo_filtered", state.screen_state.last_event);
    assert_eq!("query=tw", state.screen_state.state_label);
    let filtered = render_state(&state);
    assert!(
        component_body_pixel_diff(COMBO_BOX_PAGE, &before, &filtered)
            > COMPONENT_BODY_DIFF_THRESHOLD
    );

    assert!(apply_click(
        &mut state,
        rect.x + OPTION_X_OFFSET,
        rect.y + COMBO_TWO_OPTION_Y_OFFSET
    ));
    assert_eq!("combo_select", state.screen_state.last_action);
    assert_eq!("combo_selected", state.screen_state.last_event);
    assert_eq!("selected=two", state.screen_state.state_label);
    assert!(
        component_body_pixel_diff(COMBO_BOX_PAGE, &filtered, &render_state(&state))
            > COMPONENT_BODY_DIFF_THRESHOLD
    );
}

#[test]
fn select_box_option_click_without_opening_is_ignored() {
    let mut state = state_for(SELECT_BOX_PAGE);
    let rect = preview_detail::component_action_hit_rect(SELECT_BOX_PAGE);

    assert_eq!(None, state.screen_state.selection.select_selected_index);
    assert!(apply_click(
        &mut state,
        rect.x + OPTION_X_OFFSET,
        rect.y + SELECT_DARK_OPTION_Y_OFFSET
    ));
    assert_eq!(None, state.screen_state.selection.select_selected_index);
}

#[test]
fn combo_box_filtered_option_region_is_not_clickable_when_closed() {
    let mut state = state_for(COMBO_BOX_PAGE);
    let rect = preview_detail::component_action_hit_rect(COMBO_BOX_PAGE);

    assert_eq!(None, state.screen_state.selection.combo_selected_index);
    assert!(apply_click(
        &mut state,
        rect.x + OPTION_X_OFFSET,
        rect.y + COMBO_TWO_OPTION_Y_OFFSET
    ));
    assert_eq!(None, state.screen_state.selection.combo_selected_index);
}

#[test]
fn search_box_layout_parts_do_not_overlap_on_base_state() {
    let rect = preview_detail::component_action_hit_rect(SEARCH_BOX_PAGE);

    let field = LayoutRect::new(
        rect.x + SEARCH_FIELD_X,
        rect.y + SEARCH_FIELD_Y,
        SEARCH_FIELD_WIDTH,
        SEARCH_FIELD_HEIGHT,
    );
    let clear = LayoutRect::new(
        rect.x + SEARCH_CLEAR_X,
        rect.y + SEARCH_CLEAR_Y,
        SEARCH_CLEAR_SIZE,
        SEARCH_CLEAR_SIZE,
    );
    let status = LayoutRect::new(
        rect.x + SEARCH_STATUS_X,
        rect.y + SEARCH_STATUS_Y,
        SEARCH_STATUS_WIDTH,
        SEARCH_STATUS_HEIGHT,
    );
    let status_row_span = SEARCH_STATUS_HEIGHT + SEARCH_STATUS_GAP;

    assert!(field.contains(clear.x, clear.y));
    assert!(field.contains(clear.right() - 1, clear.bottom() - 1));
    assert!(!field.overlaps(status));
    let mut prev_row: Option<LayoutRect> = None;
    for row in 0..SEARCH_STATUS_ROW_COUNT {
        let row_rect = LayoutRect::new(
            status.x,
            status.y + row * status_row_span,
            status.width,
            status.height,
        );
        assert_eq!(status.x, row_rect.x);
        assert_eq!(status.width, row_rect.width);
        assert!(row_rect.x >= status.x);
        assert!(row_rect.right() <= status.right());
        if let Some(previous_row) = prev_row {
            assert_eq!(row_rect.y, previous_row.bottom() + SEARCH_STATUS_GAP);
            assert!(!row_rect.overlaps(previous_row));
        }
        assert!(!field.overlaps(row_rect));
        prev_row = Some(row_rect);
    }
}

#[test]
fn select_box_layout_parts_do_not_overlap_in_open_and_base_states() {
    let rect = preview_detail::component_action_hit_rect(SELECT_BOX_PAGE);

    let trigger = LayoutRect::new(
        rect.x + sc::TRIGGER_X,
        rect.y + sc::TRIGGER_Y,
        sc::TRIGGER_WIDTH,
        sc::TRIGGER_HEIGHT,
    );
    let status = LayoutRect::new(
        rect.x + sc::STATUS_X,
        rect.y + sc::STATUS_Y,
        sc::STATUS_WIDTH,
        sc::STATUS_HEIGHT,
    );
    assert!(!trigger.overlaps(status));

    let mut state = state_for(SELECT_BOX_PAGE);
    let opened_rect = {
        let _ = apply_click(
            &mut state,
            rect.x + sc::TRIGGER_X + 1,
            rect.y + sc::TRIGGER_Y + 1,
        );
        LayoutRect::new(
            rect.x + sc::TRIGGER_X,
            rect.y + sc::SELECT_OPTIONS_Y,
            sc::TRIGGER_WIDTH,
            sc::SELECT_OPTION_HEIGHT * sc::SELECT_OPTION_COUNT,
        )
    };
    assert!(!opened_rect.overlaps(status));
}

#[test]
fn selection_list_row_click_updates_list_state_and_component_body() {
    let mut state = state_for(SELECTION_LIST_PAGE);
    let rect = preview_detail::component_action_hit_rect(SELECTION_LIST_PAGE);
    let before = render_state(&state);

    assert_eq!(
        None,
        state.screen_state.selection.selection_list_selected_index
    );
    assert!(apply_click(
        &mut state,
        rect.x + sc::TRIGGER_X + sc::OPTION_ROW_INSET,
        rect.y
            + sc::SELECTION_LIST_Y
            + sc::SELECTION_LIST_ROW_HEIGHT
            + sc::SELECTION_LIST_ROW_HEIGHT / 2,
    ));
    assert_eq!(
        Some(1),
        state.screen_state.selection.selection_list_selected_index
    );
    assert_eq!("selection_toggle", state.screen_state.last_action);
    assert_eq!("selection_changed", state.screen_state.last_event);
    assert_eq!("selected=1", state.screen_state.state_label);
    assert!(
        component_body_pixel_diff(SELECTION_LIST_PAGE, &before, &render_state(&state))
            > COMPONENT_BODY_DIFF_THRESHOLD
    );
}

#[test]
fn selection_list_rows_and_status_do_not_overlap() {
    let rect = preview_detail::component_action_hit_rect(SELECTION_LIST_PAGE);
    let status = LayoutRect::new(
        rect.x + sc::STATUS_X,
        rect.y + sc::STATUS_Y,
        sc::STATUS_WIDTH,
        sc::STATUS_HEIGHT,
    );

    let rows_height = sc::SELECTION_LIST_ROW_HEIGHT * sc::SELECTION_LIST_ROW_COUNT;
    let rows = LayoutRect::new(
        rect.x + sc::TRIGGER_X,
        rect.y + sc::SELECTION_LIST_Y,
        sc::TRIGGER_WIDTH,
        rows_height,
    );
    assert!(!rows.overlaps(status));
    for index in 0..sc::SELECTION_LIST_ROW_COUNT {
        let row = LayoutRect::new(
            rows.x,
            rows.y + index * sc::SELECTION_LIST_ROW_HEIGHT,
            rows.width,
            sc::SELECTION_LIST_ROW_HEIGHT,
        );
        assert!(rows.overlaps(row));
        assert!(rows.contains(row.x, row.y));
        assert!(rows.contains(row.right() - 1, row.bottom() - 1));
    }
}

fn state_for(page: &'static str) -> StorybookWindowState {
    StorybookWindowState {
        selected_page: page,
        ..StorybookWindowState::default()
    }
}

fn render_state(state: &StorybookWindowState) -> crate::visual::Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state,
    )
}
