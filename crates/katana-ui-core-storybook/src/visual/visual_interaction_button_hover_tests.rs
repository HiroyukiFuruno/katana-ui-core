use super::visual_interaction_test_support::pixel_at;
use super::window_interaction::{StorybookWindowState, apply_hover_at};
use super::{palette, preview_detail, render};
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const BUTTON_HOVER_BORDER_SAMPLE_X_OFFSET: usize = 8;

#[test]
fn hover_draws_visible_border_for_all_button_surfaces() {
    for page in ["button", "text-button", "svg-button", "icon-text-button"] {
        let mut state = StorybookWindowState {
            selected_page: page,
            ..StorybookWindowState::default()
        };
        let before = render::render_storybook_canvas_with_screen_state(
            DARK_THEME,
            state.selected_page,
            state.preset_index,
            state.screen_state.clone(),
        );
        let rect = preview_detail::button_action_hit_rect(page);

        assert!(apply_hover_at(
            &mut state,
            rect.x + rect.width / 2,
            rect.y + rect.height / 2
        ));

        let after = render::render_storybook_canvas_with_screen_state(
            DARK_THEME,
            state.selected_page,
            state.preset_index,
            state.screen_state.clone(),
        );
        let hover_border = pixel_at(&after, rect.x + BUTTON_HOVER_BORDER_SAMPLE_X_OFFSET, rect.y);
        assert_ne!(
            pixel_at(
                &before,
                rect.x + BUTTON_HOVER_BORDER_SAMPLE_X_OFFSET,
                rect.y
            ),
            hover_border,
            "{page} hover border must be visible"
        );
        assert_eq!(
            Some(palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).hover_border),
            hover_border,
            "{page} hover border must use the shared hover border token"
        );
        assert_ne!(
            Some(palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).text),
            hover_border,
            "{page} dark hover border must not use text color"
        );
    }
}
