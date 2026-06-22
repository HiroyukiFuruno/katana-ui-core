use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{
    StorybookWindowState, apply_click, apply_clickable_keyboard_activation_for_audit,
    apply_column_resize_for_audit, apply_hover_at, apply_row_resize_for_audit,
    focus_clickable_at_for_audit,
};
use super::{layout_metrics, preview_detail, render};

const DARK_THEME: &str = "dark";
const ROW_PAGE: &str = "row";
const COLUMN_PAGE: &str = "column";
const ALIGNMENT_OPTION_ROW: usize = 3;
const ROW_DIFF_THRESHOLD: usize = 80;

#[test]
fn row_window_interaction_updates_live_layout_state() {
    assert_layout_click_updates_state(
        ROW_PAGE,
        "row_align",
        "layout_changed",
        "alignment=center",
        "callback=layout",
    );
}

#[test]
fn row_live_hover_focus_keyboard_and_resize_update_component_body() {
    let target = preview_detail::component_action_hit_rect(ROW_PAGE);
    let action_x = target.x + 4;
    let action_y = target.y + 4;

    let mut hover_state = page_state(ROW_PAGE);
    let before_hover = render_layout_window_state(&hover_state);
    assert!(apply_hover_at(&mut hover_state, action_x, action_y));
    let after_hover = render_layout_window_state(&hover_state);
    assert_eq!("row_hover", hover_state.screen_state.last_action);
    assert_eq!("hover_start", hover_state.screen_state.last_event);
    assert!(hover_state.screen_state.layout.hovered());
    assert!(component_body_pixel_diff(ROW_PAGE, &before_hover, &after_hover) > ROW_DIFF_THRESHOLD);

    let mut focus_state = page_state(ROW_PAGE);
    let before_focus = render_layout_window_state(&focus_state);
    assert!(focus_clickable_at_for_audit(
        &mut focus_state,
        action_x,
        action_y
    ));
    let after_focus = render_layout_window_state(&focus_state);
    assert_eq!("row_focus", focus_state.screen_state.last_action);
    assert_eq!("focus", focus_state.screen_state.last_event);
    assert!(focus_state.screen_state.layout.focused());
    assert!(component_body_pixel_diff(ROW_PAGE, &before_focus, &after_focus) > ROW_DIFF_THRESHOLD);

    let before_keyboard = render_layout_window_state(&focus_state);
    assert!(apply_clickable_keyboard_activation_for_audit(
        &mut focus_state
    ));
    let after_keyboard = render_layout_window_state(&focus_state);
    assert_eq!("row_keyboard_align", focus_state.screen_state.last_action);
    assert_eq!("layout_changed", focus_state.screen_state.last_event);
    assert!(
        component_body_pixel_diff(ROW_PAGE, &before_keyboard, &after_keyboard) > ROW_DIFF_THRESHOLD
    );

    let mut resize_state = page_state(ROW_PAGE);
    let before_resize = render_layout_window_state(&resize_state);
    assert!(apply_row_resize_for_audit(
        &mut resize_state,
        action_x,
        action_y
    ));
    let after_resize = render_layout_window_state(&resize_state);
    assert_eq!("row_resize", resize_state.screen_state.last_action);
    assert_eq!("layout_resized", resize_state.screen_state.last_event);
    assert!(resize_state.screen_state.layout.resized());
    assert!(
        component_body_pixel_diff(ROW_PAGE, &before_resize, &after_resize) > ROW_DIFF_THRESHOLD
    );
}

#[test]
fn column_window_interaction_updates_live_layout_state() {
    assert_layout_click_updates_state(
        COLUMN_PAGE,
        "column_align",
        "layout_changed",
        "alignment=center",
        "callback=layout",
    );
}

#[test]
fn column_live_hover_focus_keyboard_and_resize_update_component_body() {
    let target = preview_detail::component_action_hit_rect(COLUMN_PAGE);
    let action_x = target.x + 4;
    let action_y = target.y + 4;

    let mut hover_state = page_state(COLUMN_PAGE);
    let before_hover = render_layout_window_state(&hover_state);
    assert!(apply_hover_at(&mut hover_state, action_x, action_y));
    let after_hover = render_layout_window_state(&hover_state);
    assert_eq!("column_hover", hover_state.screen_state.last_action);
    assert_eq!("hover_start", hover_state.screen_state.last_event);
    assert!(hover_state.screen_state.layout.hovered());
    assert!(
        component_body_pixel_diff(COLUMN_PAGE, &before_hover, &after_hover) > ROW_DIFF_THRESHOLD
    );

    let mut focus_state = page_state(COLUMN_PAGE);
    let before_focus = render_layout_window_state(&focus_state);
    assert!(focus_clickable_at_for_audit(
        &mut focus_state,
        action_x,
        action_y
    ));
    let after_focus = render_layout_window_state(&focus_state);
    assert_eq!("column_focus", focus_state.screen_state.last_action);
    assert_eq!("focus", focus_state.screen_state.last_event);
    assert!(focus_state.screen_state.layout.focused());
    assert!(
        component_body_pixel_diff(COLUMN_PAGE, &before_focus, &after_focus) > ROW_DIFF_THRESHOLD
    );

    let before_keyboard = render_layout_window_state(&focus_state);
    assert!(apply_clickable_keyboard_activation_for_audit(
        &mut focus_state
    ));
    let after_keyboard = render_layout_window_state(&focus_state);
    assert_eq!(
        "column_keyboard_align",
        focus_state.screen_state.last_action
    );
    assert_eq!("layout_changed", focus_state.screen_state.last_event);
    assert!(
        component_body_pixel_diff(COLUMN_PAGE, &before_keyboard, &after_keyboard)
            > ROW_DIFF_THRESHOLD
    );

    let mut resize_state = page_state(COLUMN_PAGE);
    let before_resize = render_layout_window_state(&resize_state);
    assert!(apply_column_resize_for_audit(
        &mut resize_state,
        action_x,
        action_y
    ));
    let after_resize = render_layout_window_state(&resize_state);
    assert_eq!("column_resize", resize_state.screen_state.last_action);
    assert_eq!("layout_resized", resize_state.screen_state.last_event);
    assert!(resize_state.screen_state.layout.resized());
    assert!(
        component_body_pixel_diff(COLUMN_PAGE, &before_resize, &after_resize) > ROW_DIFF_THRESHOLD
    );
}

#[test]
fn row_inspector_alignment_updates_live_layout_state() {
    assert_layout_inspector_updates_state(
        ROW_PAGE,
        "layout_option_changed",
        "layout_option_changed",
        "row.alignment=center",
    );
}

#[test]
fn column_inspector_alignment_updates_live_layout_state() {
    assert_layout_inspector_updates_state(
        COLUMN_PAGE,
        "layout_option_changed",
        "layout_option_changed",
        "column.alignment=center",
    );
}

fn assert_layout_click_updates_state(
    page: &'static str,
    action: &str,
    event: &str,
    state_label: &str,
    callback: &str,
) {
    let mut state = StorybookWindowState {
        selected_page: page,
        ..StorybookWindowState::default()
    };
    let before = render_layout_window_state(&state);
    let target = preview_detail::component_action_hit_rect(page);

    assert!(apply_click(&mut state, target.x + 1, target.y + 1));
    assert_eq!(1, state.screen_state.action_count);
    assert_eq!(action, state.screen_state.last_action);
    assert_eq!(event, state.screen_state.last_event);
    assert_eq!(state_label, state.screen_state.state_label);
    assert_eq!(callback, state.screen_state.layout.callback());
    let after = render_layout_window_state(&state);

    assert!(component_body_pixel_diff(page, &before, &after) > ROW_DIFF_THRESHOLD);
}

fn assert_layout_inspector_updates_state(
    page: &'static str,
    action: &str,
    event: &str,
    state_label: &str,
) {
    let mut state = StorybookWindowState {
        selected_page: page,
        ..StorybookWindowState::default()
    };
    let setting = layout_metrics::inspector_setting_row_hit_rect(ALIGNMENT_OPTION_ROW);

    assert!(apply_click(&mut state, setting.x + 1, setting.y + 1));
    assert_eq!(1, state.screen_state.settings_revision);
    assert_eq!(ALIGNMENT_OPTION_ROW, state.preset_index);
    assert_eq!(action, state.screen_state.last_action);
    assert_eq!(event, state.screen_state.last_event);
    assert_eq!("alignment", state.screen_state.last_setting);
    assert_eq!("center", state.screen_state.last_setting_value);
    assert_eq!(state_label, state.screen_state.state_label);
}

fn render_layout_window_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
        theme_id: DARK_THEME,
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

fn page_state(page: &'static str) -> StorybookWindowState {
    StorybookWindowState {
        selected_page: page,
        ..StorybookWindowState::default()
    }
}
