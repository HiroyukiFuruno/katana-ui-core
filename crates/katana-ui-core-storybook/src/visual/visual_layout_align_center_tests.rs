use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{
    StorybookWindowState, apply_align_center_resize_for_audit, apply_click,
    apply_clickable_keyboard_activation_for_audit, apply_hover_at, focus_clickable_at_for_audit,
};
use super::{preview_detail, render};

const ALIGN_CENTER_PAGE: &str = "align-center";
const LAYOUT_DIFF_THRESHOLD: usize = 80;
const SURFACE_SAMPLE_X_OFFSET: usize = 20;
const SURFACE_SAMPLE_Y_OFFSET: usize = 42;

#[test]
fn align_center_window_interaction_click_updates_preview_state() {
    let mut state = StorybookWindowState {
        selected_page: ALIGN_CENTER_PAGE,
        ..StorybookWindowState::default()
    };
    let before = render_layout_window_state(&state);
    let target = preview_detail::component_action_hit_rect(ALIGN_CENTER_PAGE);

    assert!(apply_click(&mut state, target.x + 1, target.y + 1));
    assert_eq!(1, state.screen_state.action_count);
    assert_eq!("align_measure", state.screen_state.last_action);
    assert_eq!("alignment_changed", state.screen_state.last_event);
    assert_eq!("centered=true", state.screen_state.state_label);
    let after = render_layout_window_state(&state);

    assert!(component_body_pixel_diff(ALIGN_CENTER_PAGE, &before, &after) > LAYOUT_DIFF_THRESHOLD);
}

#[test]
fn align_center_live_operations_update_center_state_and_body() {
    assert_align_center_live_operation(
        "align_center_hover",
        apply_hover_at,
        "align_center_hover",
        "hover_start",
        "hover=center",
    );
    assert_align_center_live_operation(
        "align_center_focus",
        focus_clickable_at_for_audit,
        "align_center_focus",
        "focus",
        "focus=center",
    );

    let mut keyboard_state = StorybookWindowState {
        selected_page: ALIGN_CENTER_PAGE,
        ..StorybookWindowState::default()
    };
    let keyboard_target = preview_detail::component_action_hit_rect(ALIGN_CENTER_PAGE);
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
        "align_center_keyboard_measure",
        keyboard_state.screen_state.last_action
    );
    assert_eq!("alignment_changed", keyboard_state.screen_state.last_event);
    assert_eq!("keyboard=center", keyboard_state.screen_state.state_label);
    let after = render_layout_window_state(&keyboard_state);
    assert!(component_body_pixel_diff(ALIGN_CENTER_PAGE, &before, &after) > 0);

    assert_align_center_live_operation(
        "align_center_resize",
        apply_align_center_resize_for_audit,
        "align_center_resize",
        "layout_resized",
        "resize=center",
    );
}

fn assert_align_center_live_operation(
    label: &str,
    operation: impl FnOnce(&mut StorybookWindowState, usize, usize) -> bool,
    expected_action: &str,
    expected_event: &str,
    expected_state: &str,
) {
    let mut state = StorybookWindowState {
        selected_page: ALIGN_CENTER_PAGE,
        ..StorybookWindowState::default()
    };
    let before = render_layout_window_state(&state);
    let target = preview_detail::component_action_hit_rect(ALIGN_CENTER_PAGE);

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
        component_body_pixel_diff(ALIGN_CENTER_PAGE, &before, &after) > 0,
        "{label} should update the align-center component body"
    );
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
