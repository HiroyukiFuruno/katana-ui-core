use super::super::{StorybookWindowState, apply_scroll_delta_at, apply_scroll_delta_x_at};
use crate::visual::layout_metrics;
use crate::visual::panel_screen_state::PanelChildKey;
use crate::visual::preview_detail;
use crate::visual::render;

#[test]
fn hidden_preview_scrollbar_does_not_accept_wheel_input_on_button_page() {
    let mut state = StorybookWindowState::default();

    assert!(!apply_scroll_delta_at(
        &mut state,
        layout_metrics::PREVIEW_X + 8,
        layout_metrics::PRESET_ACTIVE_Y + 40,
        -1.0,
    ));
    assert!(!apply_scroll_delta_x_at(
        &mut state,
        layout_metrics::PREVIEW_X + 8,
        layout_metrics::PRESET_ACTIVE_Y + 40,
        -1.0,
    ));
    assert_eq!(0, state.panel_scroll.preview_y);
    assert_eq!(0, state.panel_scroll.preview_x);
}

#[test]
fn panel_preview_wheel_input_updates_inner_panel_offsets() {
    let mut state = StorybookWindowState {
        selected_page: "panel",
        scrollbar_visible: true,
        ..StorybookWindowState::default()
    };

    assert!(apply_scroll_delta_at(
        &mut state,
        preview_child_x(),
        preview_child_y(),
        -1.0,
    ));
    assert!(apply_scroll_delta_x_at(
        &mut state,
        preview_child_x(),
        preview_child_y(),
        -1.0,
    ));
    assert_eq!(0, state.panel_scroll.preview_y);
    assert_eq!(0, state.panel_scroll.preview_x);
    assert_eq!(
        72 + layout_metrics::SCROLL_STEP as u32,
        state
            .screen_state
            .panel
            .child(PanelChildKey::Preview)
            .scroll_y
    );
    assert_eq!(
        96 + layout_metrics::SCROLL_STEP as u32,
        state
            .screen_state
            .panel
            .child(PanelChildKey::Preview)
            .scroll_x
    );
}

#[test]
fn tree_view_preview_wheel_input_updates_tree_scroll_offset_and_repaints() {
    let mut state = StorybookWindowState {
        selected_page: "tree-view",
        ..StorybookWindowState::default()
    };
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    let target = preview_detail::component_action_hit_rect("tree-view");

    assert!(apply_scroll_delta_at(
        &mut state,
        target.x + 32,
        target.y + 48,
        -1.0,
    ));

    assert!(state.screen_state.tree_view_scroll_offset > 0);
    assert_eq!("tree_scroll_retained", state.screen_state.last_action);
    assert_eq!("tree_scroll_offset_kept", state.screen_state.last_event);
    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    assert!(component_pixel_diff(&before, &after, target) > 0);
}

#[test]
fn tree_view_preview_wheel_input_accumulates_and_reverses_scroll_offset() {
    let mut state = StorybookWindowState {
        selected_page: "tree-view",
        ..StorybookWindowState::default()
    };
    let target = preview_detail::component_action_hit_rect("tree-view");

    assert!(apply_scroll_delta_at(
        &mut state,
        target.x + 32,
        target.y + 48,
        -1.0,
    ));
    let first = state.screen_state.tree_view_scroll_offset;
    assert!(apply_scroll_delta_at(
        &mut state,
        target.x + 32,
        target.y + 48,
        -1.0,
    ));
    let second = state.screen_state.tree_view_scroll_offset;
    assert!(second > first);

    assert!(apply_scroll_delta_at(
        &mut state,
        target.x + 32,
        target.y + 48,
        1.0,
    ));
    assert!(state.screen_state.tree_view_scroll_offset < second);
}

fn preview_child_x() -> usize {
    preview_detail::HERO_PREVIEW_X_FOR_TEST + 210 + 12
}

fn preview_child_y() -> usize {
    preview_detail::HERO_PREVIEW_Y_FOR_TEST + 58 + 12
}

fn component_pixel_diff(
    before: &crate::visual::Canvas,
    after: &crate::visual::Canvas,
    rect: crate::visual::layout_metrics::LayoutRect,
) -> usize {
    let mut diff = 0;
    for y in rect.y..rect.bottom().min(before.height()).min(after.height()) {
        for x in rect.x..rect.right().min(before.width()).min(after.width()) {
            let index = y * before.width() + x;
            if before.pixels()[index] != after.pixels()[index] {
                diff += 1;
            }
        }
    }
    diff
}
