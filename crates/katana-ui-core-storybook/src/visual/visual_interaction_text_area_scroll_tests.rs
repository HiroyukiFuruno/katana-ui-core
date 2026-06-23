use super::layout_metrics::LayoutRect;
use super::palette::VisualPalette;
use super::preview_detail;
use super::storybook_ui_option_contract::StorybookUiOptionContract;
use super::visual_interaction_test_support::{component_body_pixel_diff, pixel_at};
use super::window_interaction::{
    StorybookWindowState, apply_scroll_delta_at_for_test, apply_scroll_delta_x_at_for_test,
};
use super::{render, window_interaction};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const PAGE: &str = "text-area";
const SCROLL_DELTA: f32 = 1.0;
const BODY_DIFF_THRESHOLD: usize = 80;
const VERTICAL_THUMB_X_OFFSET: usize = 5;
const VERTICAL_THUMB_Y_OFFSET: usize = 34;
const HORIZONTAL_THUMB_X_OFFSET: usize = 12;
const HORIZONTAL_THUMB_Y_OFFSET: usize = 4;

#[test]
fn text_area_vertical_scroll_requires_enabled_axis_and_updates_preview() -> Result<(), String> {
    let mut state = text_area_state_for_preset("chat composer")?;
    let field = text_area_field_rect();

    assert!(!apply_scroll_delta_at_for_test(
        &mut state,
        field.x + 2,
        field.y + 2,
        SCROLL_DELTA
    ));
    assert_eq!(0, state.screen_state.text_area_scroll_offset());

    let mut enabled = text_area_state_for_preset("vertical scroll")?;
    let before = render_with_state(&enabled);
    assert!(apply_scroll_delta_at_for_test(
        &mut enabled,
        field.x + 2,
        field.y + 2,
        SCROLL_DELTA
    ));
    let after = render_with_state(&enabled);
    assert_eq!(1, enabled.screen_state.text_area_scroll_offset());
    assert_eq!("text_area_scroll_y", enabled.screen_state.last_action);
    assert_eq!("text_area_scroll_changed", enabled.screen_state.last_event);
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
    Ok(())
}

#[test]
fn text_area_horizontal_scroll_requires_enabled_axis_and_updates_preview() -> Result<(), String> {
    let mut state = text_area_state_for_preset("chat composer")?;
    let field = text_area_field_rect();

    assert!(!apply_scroll_delta_x_at_for_test(
        &mut state,
        field.x + 2,
        field.y + 2,
        SCROLL_DELTA
    ));
    assert_eq!(0, state.screen_state.text_area_scroll_x_offset());

    let mut enabled = text_area_state_for_preset("horizontal scroll")?;
    let before = render_with_state(&enabled);
    assert!(apply_scroll_delta_at_for_test(
        &mut enabled,
        field.x + 2,
        field.y + 2,
        SCROLL_DELTA
    ));
    let after = render_with_state(&enabled);
    assert!(enabled.screen_state.text_area_scroll_x_offset() > 0);
    assert_eq!("text_area_scroll_x", enabled.screen_state.last_action);
    assert_eq!("text_area_scroll_changed", enabled.screen_state.last_event);
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
    Ok(())
}

#[test]
fn text_area_horizontal_scroll_accepts_native_delta_x() -> Result<(), String> {
    let mut enabled = text_area_state_for_preset("horizontal scroll")?;
    let field = text_area_field_rect();

    assert!(apply_scroll_delta_x_at_for_test(
        &mut enabled,
        field.x + 2,
        field.y + 2,
        SCROLL_DELTA
    ));
    assert!(enabled.screen_state.text_area_scroll_x_offset() > 0);
    Ok(())
}

#[test]
fn text_area_resize_drag_requires_enabled_option_and_updates_preview() -> Result<(), String> {
    let field = text_area_field_rect();
    let mut disabled = text_area_state_for_preset("chat composer")?;

    assert!(
        !window_interaction::apply_text_area_resize_drag_at_for_test(
            &mut disabled,
            field.right() + 36,
            field.bottom() + 8
        )
    );

    let mut enabled = text_area_state_for_preset("resize handle")?;
    let before = render_with_state(&enabled);
    assert!(window_interaction::apply_text_area_resize_drag_at_for_test(
        &mut enabled,
        field.right() + 36,
        field.bottom() + 8
    ));
    let after = render_with_state(&enabled);

    assert!(enabled.screen_state.text_area_resize_width_delta() > 0);
    assert!(enabled.screen_state.text_area_resize_height_delta() > 0);
    assert_eq!("text_area_resize_drag", enabled.screen_state.last_action);
    assert_eq!("text_area_resized", enabled.screen_state.last_event);
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
    Ok(())
}

#[test]
fn text_area_scrollbar_visibility_requires_enabled_storybook_axis() -> Result<(), String> {
    let default_state = text_area_state_for_preset("chat composer")?;
    let vertical_state = text_area_state_for_preset("vertical scroll")?;
    let horizontal_state = text_area_state_for_preset("horizontal scroll")?;
    let vertical_bar_state = text_area_state_for_preset("vertical scrollbar")?;
    let horizontal_bar_state = text_area_state_for_preset("horizontal scrollbar")?;
    let field = text_area_field_rect();
    let palette = VisualPalette::from_theme(&ThemeSnapshot::dark());
    let vertical_thumb = (
        field.right() - VERTICAL_THUMB_X_OFFSET,
        field.y + VERTICAL_THUMB_Y_OFFSET,
    );
    let horizontal_thumb = (
        field.x + HORIZONTAL_THUMB_X_OFFSET,
        field.bottom() - HORIZONTAL_THUMB_Y_OFFSET,
    );

    let default_canvas = render_with_state(&default_state);
    let vertical_canvas = render_with_state(&vertical_state);
    let horizontal_canvas = render_with_state(&horizontal_state);
    let vertical_bar_canvas = render_with_state(&vertical_bar_state);
    let horizontal_bar_canvas = render_with_state(&horizontal_bar_state);

    assert_ne!(
        Some(palette.accent),
        pixel_at(&default_canvas, vertical_thumb.0, vertical_thumb.1)
    );
    assert_ne!(
        Some(palette.accent),
        pixel_at(&default_canvas, horizontal_thumb.0, horizontal_thumb.1)
    );
    assert_ne!(
        Some(palette.accent),
        pixel_at(&vertical_canvas, vertical_thumb.0, vertical_thumb.1)
    );
    assert_ne!(
        Some(palette.accent),
        pixel_at(&horizontal_canvas, horizontal_thumb.0, horizontal_thumb.1)
    );
    assert_eq!(
        Some(palette.accent),
        pixel_at(&vertical_bar_canvas, vertical_thumb.0, vertical_thumb.1)
    );
    assert_eq!(
        Some(palette.accent),
        pixel_at(
            &horizontal_bar_canvas,
            horizontal_thumb.0,
            horizontal_thumb.1
        )
    );
    Ok(())
}

#[test]
fn text_area_scrollbar_visibility_state_mutation_requires_scroll_enabled_axis() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };

    state.screen_state.register_settings_contract_change(
        PAGE,
        StorybookUiOptionContract::new("text_area.vertical_scrollbar_visible", "false", "true"),
    );
    assert!(!state.screen_state.text_area_vertical_scroll_enabled());
    assert!(!state.screen_state.text_area_vertical_scrollbar_visible());
    assert_eq!(
        "text_area.vertical_scrollbar_visible",
        state.screen_state.last_setting
    );

    state.screen_state.register_settings_contract_change(
        PAGE,
        StorybookUiOptionContract::new("text_area.vertical_scroll_enabled", "false", "true"),
    );
    state.screen_state.register_settings_contract_change(
        PAGE,
        StorybookUiOptionContract::new("text_area.vertical_scrollbar_visible", "false", "true"),
    );
    assert!(state.screen_state.text_area_vertical_scroll_enabled());
    assert!(state.screen_state.text_area_vertical_scrollbar_visible());

    let mut horizontal_state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    horizontal_state
        .screen_state
        .register_settings_contract_change(
            PAGE,
            StorybookUiOptionContract::new(
                "text_area.horizontal_scrollbar_visible",
                "false",
                "true",
            ),
        );
    assert!(
        !horizontal_state
            .screen_state
            .text_area_horizontal_scroll_enabled()
    );
    assert!(
        !horizontal_state
            .screen_state
            .text_area_horizontal_scrollbar_visible()
    );

    horizontal_state
        .screen_state
        .register_settings_contract_change(
            PAGE,
            StorybookUiOptionContract::new("text_area.horizontal_scroll_enabled", "false", "true"),
        );
    horizontal_state
        .screen_state
        .register_settings_contract_change(
            PAGE,
            StorybookUiOptionContract::new(
                "text_area.horizontal_scrollbar_visible",
                "false",
                "true",
            ),
        );
    assert!(
        horizontal_state
            .screen_state
            .text_area_horizontal_scroll_enabled()
    );
    assert!(
        horizontal_state
            .screen_state
            .text_area_horizontal_scrollbar_visible()
    );
}

fn text_area_state_for_preset(label: &str) -> Result<StorybookWindowState, String> {
    Ok(StorybookWindowState {
        selected_page: PAGE,
        preset_index: preset_index(label)?,
        ..StorybookWindowState::default()
    })
}

fn preset_index(label: &str) -> Result<usize, String> {
    StoryPresetLabels::for_page(PAGE)
        .iter()
        .position(|it| *it == label)
        .ok_or_else(|| "text-area preset".to_string())
}

fn text_area_field_rect() -> LayoutRect {
    let rect = preview_detail::component_action_hit_rect(PAGE);
    super::dedicated_dod_form_input_live::text_area_rect(rect.x, rect.y)
}

fn render_with_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}
