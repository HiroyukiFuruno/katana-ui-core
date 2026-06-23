use super::visual_interaction_test_support::pixel_at;
use super::window_interaction::{StorybookWindowState, apply_hover_at};
use super::{palette, preview_detail, render};
use katana_ui_core::theme::ThemeSnapshot;

const PAGE: &str = "text-input";
const DARK_THEME: &str = "dark";

#[test]
fn text_input_field_hover_uses_hover_border_without_mutating_value() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let before = render_with_state(&state);
    let field = text_input_field_rect();
    let initial_value = state.screen_state.text_input_value().to_string();

    assert!(apply_hover_at(&mut state, field.x + 1, field.y + 1));
    assert!(state.screen_state.preview_hovered);
    assert_eq!(initial_value, state.screen_state.text_input_value());
    assert_eq!(0, state.screen_state.action_count);

    let after = render_with_state(&state);
    assert_eq!(
        Some(palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).hover_border),
        pixel_at(&after, field.x, field.y)
    );
    assert_ne!(
        pixel_at(&before, field.x, field.y),
        pixel_at(&after, field.x, field.y)
    );
}

fn render_with_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn text_input_field_rect() -> super::layout_metrics::LayoutRect {
    let rect = preview_detail::component_action_hit_rect(PAGE);
    super::dedicated_dod_form_input_live::search_field_rect(rect.x, rect.y)
}
