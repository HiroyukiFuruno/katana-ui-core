use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{
    StorybookWindowState, apply_hover_at, focus_clickable_at_for_audit,
};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "hover-card";
const DELAYED_PRESET: usize = 0;
const POINTER_PRESET: usize = 1;
const FOCUS_PRESET: usize = 2;
const RICH_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const COMPONENT_HIT_INSET: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const CARD_X: usize = 142;
const CARD_Y: usize = 28;
const CARD_SAMPLE_OFFSET: usize = 8;

#[test]
fn hover_card_exposes_leaf_presets_options_and_open_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("hover_card_open", spec.action);
    assert_eq!("hover_card_opened", spec.event);
    assert_eq!("interaction.open", spec.option);
    assert_eq!("true", spec.after);
    assert_eq!("open=true", spec.state);
}

#[test]
fn hover_card_presets_render_distinct_delay_pointer_focus_and_content_bodies() {
    let delayed = StorybookVisual.render_preset(DARK_THEME, PAGE, DELAYED_PRESET, 0);
    let pointer = StorybookVisual.render_preset(DARK_THEME, PAGE, POINTER_PRESET, 0);
    let focus = StorybookVisual.render_preset(DARK_THEME, PAGE, FOCUS_PRESET, 0);
    let rich = StorybookVisual.render_preset(DARK_THEME, PAGE, RICH_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &delayed, &pointer) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &pointer, &focus) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &delayed, &rich) > BODY_DIFF_THRESHOLD);
}

#[test]
fn hover_card_setting_option_updates_card_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn hover_card_preview_action_opens_card_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn hover_card_live_hover_focus_and_inner_focus_use_core_actions() {
    let mut hover_state = page_state();
    let hover_before = render_state(&hover_state);
    assert!(apply_hover_at(
        &mut hover_state,
        hover_card_x(),
        hover_card_y()
    ));
    let hover_after = render_state(&hover_state);

    assert_eq!("hover_card_hover", hover_state.screen_state.last_action);
    assert_eq!("hover_card_opened", hover_state.screen_state.last_event);
    assert_eq!("hover=true open=true", hover_state.screen_state.state_label);
    assert!(hover_state.screen_state.preview_hovered);
    assert!(component_body_pixel_diff(PAGE, &hover_before, &hover_after) > 0);

    let mut focus_state = page_state();
    let focus_before = render_state(&focus_state);
    assert!(focus_clickable_at_for_audit(
        &mut focus_state,
        hover_card_x(),
        hover_card_y()
    ));
    let focus_after = render_state(&focus_state);

    assert_eq!("hover_card_focus", focus_state.screen_state.last_action);
    assert_eq!("hover_card_opened", focus_state.screen_state.last_event);
    assert_eq!("focus=true open=true", focus_state.screen_state.state_label);
    assert!(focus_state.screen_state.is_button_focused());
    assert!(component_body_pixel_diff(PAGE, &focus_before, &focus_after) > 0);

    let inner_before = render_state(&focus_state);
    focus_state
        .screen_state
        .register_hover_card_inner_focus_keep_open();
    let inner_after = render_state(&focus_state);

    assert_eq!(
        "hover_card_inner_focus",
        focus_state.screen_state.last_action
    );
    assert_eq!("hover_card_kept_open", focus_state.screen_state.last_event);
    assert_eq!(
        "inner_focus=kept_open",
        focus_state.screen_state.state_label
    );
    assert!(component_body_pixel_diff(PAGE, &inner_before, &inner_after) > 0);
}

#[test]
fn hover_card_light_and_dark_card_uses_theme_surface() {
    assert_card_token(DARK_THEME, ThemeSnapshot::dark());
    assert_card_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_card_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, DELAYED_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + CARD_X + CARD_SAMPLE_OFFSET,
            component.y + CARD_Y + CARD_SAMPLE_OFFSET
        )
    );
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    super::render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn page_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn hover_card_x() -> usize {
    preview_detail::component_action_hit_rect(PAGE).x + COMPONENT_HIT_INSET
}

fn hover_card_y() -> usize {
    preview_detail::component_action_hit_rect(PAGE).y + COMPONENT_HIT_INSET
}
