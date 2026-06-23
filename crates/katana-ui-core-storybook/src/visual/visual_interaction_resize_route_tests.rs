use super::preview_detail;
use super::window_interaction::{StorybookWindowState, apply_click};

#[test]
fn component_lower_right_click_routes_to_resize_actions() {
    for (page, expected_action, expected_event, expected_state) in [
        ("panel", "panel_resize", "panel_resized", "resize=preview"),
        ("row", "row_resize", "layout_resized", "resize=row"),
        ("column", "column_resize", "layout_resized", "resize=column"),
        ("stack", "stack_resize", "layout_resized", "resize=stack"),
        ("grid", "grid_resize", "layout_resized", "resize=grid"),
        (
            "align-center",
            "align_center_resize",
            "layout_resized",
            "resize=center",
        ),
        (
            "theme-tokens",
            "theme_token_resize_spacing",
            "theme_spacing_changed",
            "resize=spacing",
        ),
    ] {
        let mut state = StorybookWindowState {
            selected_page: page,
            ..StorybookWindowState::default()
        };
        let component = preview_detail::component_action_hit_rect(page);
        let x = component.right().saturating_sub(2);
        let y = component.bottom().saturating_sub(2);

        assert!(apply_click(&mut state, x, y), "{page} resize click");
        assert_eq!(expected_action, state.screen_state.last_action, "{page}");
        assert_eq!(expected_event, state.screen_state.last_event, "{page}");
        assert_eq!(expected_state, state.screen_state.state_label, "{page}");
    }
}
