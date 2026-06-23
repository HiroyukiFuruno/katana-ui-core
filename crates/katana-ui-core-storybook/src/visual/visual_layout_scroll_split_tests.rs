use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{
    StorybookWindowState, apply_click, apply_clickable_keyboard_activation_for_audit,
    apply_hover_at, apply_scroll_area_drag_for_audit, apply_scroll_area_resize_for_audit,
    apply_scroll_area_scroll_for_audit, apply_scroll_delta_at_for_test,
    focus_clickable_at_for_audit,
};
use super::{
    dedicated_dod_layout_scroll_area, layout_metrics, panel_scroll_state, preview_detail, render,
};

const DARK_THEME: &str = "dark";
const SCROLL_AREA_PAGE: &str = "scroll-area";
const PRIMARY_INSTANCE: &str = "scroll-area.primary";
const SECONDARY_INSTANCE: &str = "scroll-area.secondary";
const DEFAULT_PRESET: usize = 0;
const LAYOUT_DIFF_THRESHOLD: usize = 80;
const SURFACE_SAMPLE_X_OFFSET: usize = 20;
const SURFACE_SAMPLE_Y_OFFSET: usize = 42;

#[test]
fn scroll_area_preview_scroll_offsets_move_inner_rows() {
    let before = render_scroll_area_with_preview_offset(0);
    let after = render_scroll_area_with_preview_offset(32);

    assert!(component_body_pixel_diff(SCROLL_AREA_PAGE, &before, &after) > LAYOUT_DIFF_THRESHOLD);
}

#[test]
fn scroll_area_window_interaction_scroll_updates_preview_state() {
    let mut state = StorybookWindowState {
        selected_page: SCROLL_AREA_PAGE,
        scrollbar_visible: true,
        ..StorybookWindowState::default()
    };
    let before = render_layout_window_state(&state);
    let target = preview_detail::component_action_hit_rect(SCROLL_AREA_PAGE);

    assert!(apply_scroll_delta_at_for_test(
        &mut state,
        target.x + SURFACE_SAMPLE_X_OFFSET,
        target.y + SURFACE_SAMPLE_Y_OFFSET,
        -1.0,
    ));
    assert_eq!(layout_metrics::SCROLL_STEP, state.panel_scroll.preview_y);
    let after = render_layout_window_state(&state);

    assert!(component_body_pixel_diff(SCROLL_AREA_PAGE, &before, &after) > LAYOUT_DIFF_THRESHOLD);
}

#[test]
fn scroll_area_window_interaction_keeps_instance_scroll_state_isolated() {
    let mut state = StorybookWindowState {
        selected_page: SCROLL_AREA_PAGE,
        scrollbar_visible: true,
        ..StorybookWindowState::default()
    };
    let target = preview_detail::component_action_hit_rect(SCROLL_AREA_PAGE);

    state.select_instance(PRIMARY_INSTANCE);
    assert!(apply_scroll_delta_at_for_test(
        &mut state,
        target.x + SURFACE_SAMPLE_X_OFFSET,
        target.y + SURFACE_SAMPLE_Y_OFFSET,
        -1.0,
    ));
    let primary_scroll_y = state.panel_scroll.preview_y;
    let primary_canvas = render_layout_window_state(&state);

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!(0, state.panel_scroll.preview_y);
    let secondary_canvas = render_layout_window_state(&state);

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary_scroll_y, state.panel_scroll.preview_y);
    assert!(primary_scroll_y > 0);
    assert!(
        component_body_pixel_diff(SCROLL_AREA_PAGE, &primary_canvas, &secondary_canvas)
            > LAYOUT_DIFF_THRESHOLD
    );
}

#[test]
fn scroll_area_live_operations_update_core_scroll_state_and_body() {
    assert_scroll_area_live_operation(
        "scroll_area_scroll",
        apply_scroll_area_scroll_for_audit,
        "scroll_area_scroll",
        "scroll_area_scrolled",
        "scroll=48",
    );
    assert_scroll_area_live_operation(
        "scroll_area_drag",
        apply_scroll_area_drag_for_audit,
        "scroll_area_drag_thumb",
        "scroll_area_scrolled",
        "drag=72",
    );
    assert_scroll_area_live_operation(
        "scroll_area_hover",
        apply_hover_at,
        "scroll_area_hover",
        "hover_start",
        "hover=viewport",
    );
    assert_scroll_area_live_operation(
        "scroll_area_focus",
        focus_clickable_at_for_audit,
        "scroll_area_focus",
        "focus",
        "focus=viewport",
    );

    let mut keyboard_state = StorybookWindowState {
        selected_page: SCROLL_AREA_PAGE,
        ..StorybookWindowState::default()
    };
    let keyboard_target = preview_detail::component_action_hit_rect(SCROLL_AREA_PAGE);
    assert!(focus_clickable_at_for_audit(
        &mut keyboard_state,
        keyboard_target.x + SURFACE_SAMPLE_X_OFFSET,
        keyboard_target.y + SURFACE_SAMPLE_Y_OFFSET,
    ));
    let before = render_layout_window_state(&keyboard_state);
    assert!(apply_clickable_keyboard_activation_for_audit(
        &mut keyboard_state
    ));
    assert_eq!(
        "scroll_area_keyboard_scroll",
        keyboard_state.screen_state.last_action
    );
    assert_eq!(
        "scroll_area_scrolled",
        keyboard_state.screen_state.last_event
    );
    assert_eq!("keyboard=36", keyboard_state.screen_state.state_label);
    let after = render_layout_window_state(&keyboard_state);
    assert!(component_body_pixel_diff(SCROLL_AREA_PAGE, &before, &after) > LAYOUT_DIFF_THRESHOLD);

    assert_scroll_area_live_operation(
        "scroll_area_resize",
        apply_scroll_area_resize_for_audit,
        "scrollbar_visibility_changed",
        "scroll_area_resized",
        "resize=viewport",
    );
}

#[test]
fn scroll_area_visible_affordances_route_window_clicks_to_core_actions() {
    let mut drag_state = StorybookWindowState {
        selected_page: SCROLL_AREA_PAGE,
        ..StorybookWindowState::default()
    };
    let origin = preview_detail::component_action_hit_rect(SCROLL_AREA_PAGE);
    let thumb = dedicated_dod_layout_scroll_area::scrollbar_drag_rect(origin.x, origin.y);

    assert!(apply_click(&mut drag_state, thumb.x + 1, thumb.y + 1));
    assert_eq!(
        "scroll_area_drag_thumb",
        drag_state.screen_state.last_action
    );
    assert_eq!("scroll_area_scrolled", drag_state.screen_state.last_event);
    assert_eq!("drag=72", drag_state.screen_state.state_label);

    let mut resize_state = StorybookWindowState {
        selected_page: SCROLL_AREA_PAGE,
        ..StorybookWindowState::default()
    };
    let resize = dedicated_dod_layout_scroll_area::resize_handle_rect(origin.x, origin.y);

    assert!(apply_click(&mut resize_state, resize.x + 1, resize.y + 1));
    assert_eq!(
        "scrollbar_visibility_changed",
        resize_state.screen_state.last_action
    );
    assert_eq!("scroll_area_resized", resize_state.screen_state.last_event);
    assert_eq!("resize=viewport", resize_state.screen_state.state_label);
}

fn assert_scroll_area_live_operation(
    label: &str,
    operation: impl FnOnce(&mut StorybookWindowState, usize, usize) -> bool,
    expected_action: &str,
    expected_event: &str,
    expected_state: &str,
) {
    let mut state = StorybookWindowState {
        selected_page: SCROLL_AREA_PAGE,
        ..StorybookWindowState::default()
    };
    let before = render_layout_window_state(&state);
    let target = preview_detail::component_action_hit_rect(SCROLL_AREA_PAGE);

    assert!(
        operation(
            &mut state,
            target.x + SURFACE_SAMPLE_X_OFFSET,
            target.y + SURFACE_SAMPLE_Y_OFFSET,
        ),
        "{label} should be handled"
    );
    assert_eq!(
        expected_action, state.screen_state.last_action,
        "{label} action"
    );
    assert_eq!(
        expected_event, state.screen_state.last_event,
        "{label} event"
    );
    assert_eq!(
        expected_state, state.screen_state.state_label,
        "{label} state"
    );
    let after = render_layout_window_state(&state);

    assert!(
        component_body_pixel_diff(SCROLL_AREA_PAGE, &before, &after) > LAYOUT_DIFF_THRESHOLD,
        "{label} should update the scroll-area component body"
    );
}

fn render_scroll_area_with_preview_offset(preview_y: usize) -> super::Canvas {
    render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
        theme_id: DARK_THEME,
        selected_page: SCROLL_AREA_PAGE,
        selected_instance_id: crate::visual::window_interaction::DEFAULT_INSTANCE_ID,
        preset_index: DEFAULT_PRESET,
        preset_tab_scroll_x: 0,
        scroll_y: 0,
        scrollbar_visible: true,
        panel_scroll: panel_scroll_state::PanelScrollOffsets {
            preview_y,
            ..Default::default()
        },
        tree_expansion: Default::default(),
        show_navigation_lines: true,
        show_navigation_text_connectors: false,
        screen_state: Default::default(),
    })
}

fn render_layout_window_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
        theme_id: state.theme_id,
        selected_page: state.selected_page,
        selected_instance_id: state.selected_instance_id,
        preset_index: state.preset_index,
        preset_tab_scroll_x: state.preset_tab_scroll_x,
        scroll_y: state.scroll_y,
        scrollbar_visible: state.scrollbar_visible,
        panel_scroll: state.panel_scroll,
        tree_expansion: state.tree_expansion,
        show_navigation_lines: state.show_navigation_lines,
        show_navigation_text_connectors: state.show_navigation_text_connectors,
        screen_state: state.screen_state.clone(),
    })
}
