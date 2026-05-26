use super::super::{StorybookWindowState, apply_scroll_delta_at, apply_scroll_delta_x_at};
use crate::visual::layout_metrics;
use crate::visual::panel_screen_state::PanelChildKey;
use crate::visual::preview_detail;

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

fn preview_child_x() -> usize {
    preview_detail::HERO_PREVIEW_X_FOR_TEST + 210 + 12
}

fn preview_child_y() -> usize {
    preview_detail::HERO_PREVIEW_Y_FOR_TEST + 58 + 12
}
