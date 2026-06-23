use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{
    StorybookWindowState, apply_click, apply_clickable_keyboard_activation_for_audit,
    apply_hover_at, apply_split_pane_drag_for_audit, apply_split_pane_resize_for_audit,
    focus_clickable_at_for_audit,
};
use super::{dedicated_dod_molecule_split_pane, preview_detail, render};

const SPLIT_PANE_PAGE: &str = "split-pane";
const SURFACE_SAMPLE_X_OFFSET: usize = 20;
const SURFACE_SAMPLE_Y_OFFSET: usize = 42;

#[test]
fn split_pane_live_operations_update_core_ratio_state_and_body() {
    assert_split_pane_live_operation(
        "split_pane_drag",
        apply_split_pane_drag_for_audit,
        "split_pane_drag_resize",
        "split_pane_ratio_changed",
        "ratio=64",
    );
    assert_split_pane_live_operation(
        "split_pane_hover",
        apply_hover_at,
        "split_pane_hover",
        "hover_start",
        "hover=handle",
    );
    assert_split_pane_live_operation(
        "split_pane_focus",
        focus_clickable_at_for_audit,
        "split_pane_focus",
        "focus",
        "focus=handle",
    );

    let mut keyboard_state = StorybookWindowState {
        selected_page: SPLIT_PANE_PAGE,
        ..StorybookWindowState::default()
    };
    let keyboard_target = preview_detail::component_action_hit_rect(SPLIT_PANE_PAGE);
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
        "split_pane_keyboard_resize",
        keyboard_state.screen_state.last_action
    );
    assert_eq!(
        "split_pane_ratio_changed",
        keyboard_state.screen_state.last_event
    );
    assert_eq!("keyboard=58", keyboard_state.screen_state.state_label);
    assert_eq!(58, keyboard_state.screen_state.split_pane.ratio_percent());
    let after = render_layout_window_state(&keyboard_state);
    assert!(component_body_pixel_diff(SPLIT_PANE_PAGE, &before, &after) > 0);

    assert_split_pane_live_operation(
        "split_pane_resize",
        apply_split_pane_resize_for_audit,
        "split_pane_resize",
        "split_pane_ratio_changed",
        "resize=40",
    );
}

#[test]
fn split_pane_visible_affordances_route_window_clicks_to_core_actions() {
    let mut drag_state = StorybookWindowState {
        selected_page: SPLIT_PANE_PAGE,
        ..StorybookWindowState::default()
    };
    let origin = preview_detail::component_action_hit_rect(SPLIT_PANE_PAGE);
    let handle = dedicated_dod_molecule_split_pane::handle_drag_rect(origin.x, origin.y);

    assert!(apply_click(&mut drag_state, handle.x + 1, handle.y + 1));
    assert_eq!(
        "split_pane_drag_resize",
        drag_state.screen_state.last_action
    );
    assert_eq!(
        "split_pane_ratio_changed",
        drag_state.screen_state.last_event
    );
    assert_eq!("ratio=64", drag_state.screen_state.state_label);

    let mut resize_state = StorybookWindowState {
        selected_page: SPLIT_PANE_PAGE,
        ..StorybookWindowState::default()
    };
    let resize = dedicated_dod_molecule_split_pane::resize_handle_rect(origin.x, origin.y);

    assert!(apply_click(&mut resize_state, resize.x + 1, resize.y + 1));
    assert_eq!("split_pane_resize", resize_state.screen_state.last_action);
    assert_eq!(
        "split_pane_ratio_changed",
        resize_state.screen_state.last_event
    );
    assert_eq!("resize=40", resize_state.screen_state.state_label);
}

fn assert_split_pane_live_operation(
    label: &str,
    operation: impl FnOnce(&mut StorybookWindowState, usize, usize) -> bool,
    expected_action: &str,
    expected_event: &str,
    expected_state: &str,
) {
    let mut state = StorybookWindowState {
        selected_page: SPLIT_PANE_PAGE,
        ..StorybookWindowState::default()
    };
    let before = render_layout_window_state(&state);
    let target = preview_detail::component_action_hit_rect(SPLIT_PANE_PAGE);

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
        component_body_pixel_diff(SPLIT_PANE_PAGE, &before, &after) > 0,
        "{label} should update the split-pane component body"
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
