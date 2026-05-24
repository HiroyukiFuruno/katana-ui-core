use super::super::{StorybookWindowState, apply_scroll_delta_at, apply_scroll_delta_x_at};
use crate::visual::layout_metrics;

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
fn panel_preview_wheel_input_is_inert_when_preview_content_does_not_overflow() {
    let mut state = StorybookWindowState {
        selected_page: "panel",
        scrollbar_visible: true,
        ..StorybookWindowState::default()
    };

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
