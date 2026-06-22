use super::visual_interaction_test_support::pixel_at;
use super::window_interaction::{
    StorybookCursorStyle, StorybookWindowState, TextInputKey, apply_click,
};
use super::{palette, preview_detail, render, window_interaction};
use katana_ui_core::theme::ThemeSnapshot;

const PAGE: &str = "text-input";
const DARK_THEME: &str = "dark";
const ICON_BUTTONS_PRESET: usize = 6;
const STORYBOOK_SEARCH_SVG: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"24\" height=\"24\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"#FFFFFF\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><circle cx=\"11\" cy=\"11\" r=\"8\"/><line x1=\"21\" y1=\"21\" x2=\"16.65\" y2=\"16.65\"/></svg>";

#[test]
fn text_input_typing_updates_event_in_realtime() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let field = text_input_field_rect();

    assert!(apply_click(&mut state, field.x + 1, field.y + 1));
    let before_count = state.screen_state.action_count;
    assert!(window_interaction::apply_text_input_key(
        &mut state,
        TextInputKey::Character('a')
    ));
    assert_eq!(before_count + 1, state.screen_state.action_count);
    assert_eq!("text_input_type", state.screen_state.last_action);
    assert_eq!("text_input_changed", state.screen_state.last_event);
    assert_eq!("value=typing", state.screen_state.state_label);

    assert!(window_interaction::apply_text_input_key(
        &mut state,
        TextInputKey::Character('b')
    ));
    assert_eq!(before_count + 2, state.screen_state.action_count);
    assert!(state.screen_state.text_input_value().ends_with("ab"));
    assert_eq!("text_input_changed", state.screen_state.last_event);
}

#[test]
fn text_input_storybook_uses_external_search_svg_source() -> Result<(), String> {
    let examples = crate::StoryCatalog.examples();
    let input = examples
        .iter()
        .find(|it| it.page == PAGE)
        .ok_or_else(|| "text-input story example".to_string())?;

    assert_eq!(
        STORYBOOK_SEARCH_SVG,
        super::dedicated_dod_form_input_live::text_input_search_svg_fixture_for_test()
    );
    assert_eq!(
        Some(STORYBOOK_SEARCH_SVG),
        input
            .tree
            .root()
            .props()
            .text_entry
            .leading_slot
            .as_ref()
            .and_then(|slot| slot.icon.as_ref())
            .map(|icon| icon.svg_source.as_str())
    );
    Ok(())
}

#[test]
fn text_input_icon_button_hover_draws_border_and_uses_pointing_hand_cursor() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        preset_index: ICON_BUTTONS_PRESET,
        ..StorybookWindowState::default()
    };
    let before = render_with_state(&state);
    let origin = preview_detail::component_action_hit_rect(PAGE);
    let rect = super::dedicated_dod_form_input_live::text_input_trailing_icon_button_rects(
        origin.x, origin.y,
    )[0];

    assert!(window_interaction::apply_hover_at(
        &mut state,
        rect.x + 1,
        rect.y + 1
    ));
    assert_eq!(
        Some(0),
        state.screen_state.hovered_text_input_icon_button_index
    );
    assert_eq!(
        StorybookCursorStyle::PointingHand,
        window_interaction::cursor_style_at_for_test(&state, rect.x + 1, rect.y + 1)
    );

    let after = render_with_state(&state);
    let hover_border = pixel_at(&after, rect.x, rect.y);
    assert_ne!(
        pixel_at(&before, rect.x, rect.y),
        hover_border,
        "text-input icon button hover border must be visible"
    );
    assert_eq!(
        Some(palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).hover_border),
        hover_border,
        "text-input icon button hover border must use the shared hover border token"
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
