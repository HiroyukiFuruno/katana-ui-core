use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{
    StorybookWindowState, apply_clickable_keyboard_activation_for_audit, apply_hover_at,
    focus_clickable_at_for_audit,
};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use crate::visual::render;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "card";
const SLOTS_PRESET: usize = 0;
const CLICK_PRESET: usize = 1;
const NESTED_PRESET: usize = 2;
const THEME_BORDER_PRESET: usize = 3;
const LABEL_PRESET: usize = 4;
const HEADER_PRESET: usize = 5;
const FOOTER_PRESET: usize = 6;
const PADDING_PRESET: usize = 7;
const REQUIRED_PRESET_COUNT: usize = 8;
const REQUIRED_OPTION_COUNT: usize = 8;
const BODY_DIFF_THRESHOLD: usize = 80;
const CARD_X: usize = 22;
const CARD_Y: usize = 30;
const CARD_SAMPLE_X_OFFSET: usize = 260;
const CARD_SAMPLE_Y_OFFSET: usize = 64;
const CLICK_OFFSET: usize = 4;

#[test]
fn card_exposes_leaf_presets_options_and_activation_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    for option in [
        "card.label",
        "card.header",
        "card.footer",
        "card.variant",
        "card.padding",
        "card.clickable",
        "card.nested_controls",
        "card.child_state",
    ] {
        assert!(
            options.iter().any(|it| it.setting == option),
            "card option is not exposed: {option}"
        );
        assert!(
            rows.iter()
                .any(|row| row.starts_with(&format!("{option}:"))),
            "card settings row is not exposed: {option}"
        );
    }
    assert_eq!("card_click", spec.action);
    assert_eq!("card_activated", spec.event);
    assert_eq!("interaction.active", spec.option);
    assert_eq!("true", spec.after);
    assert_eq!("active=true", spec.state);
}

#[test]
fn card_presets_render_distinct_slot_click_nested_and_theme_states() {
    let slots = StorybookVisual.render_preset(DARK_THEME, PAGE, SLOTS_PRESET, 0);
    let click = StorybookVisual.render_preset(DARK_THEME, PAGE, CLICK_PRESET, 0);
    let nested = StorybookVisual.render_preset(DARK_THEME, PAGE, NESTED_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_BORDER_PRESET, 0);
    let label = StorybookVisual.render_preset(DARK_THEME, PAGE, LABEL_PRESET, 0);
    let header = StorybookVisual.render_preset(DARK_THEME, PAGE, HEADER_PRESET, 0);
    let footer = StorybookVisual.render_preset(DARK_THEME, PAGE, FOOTER_PRESET, 0);
    let padding = StorybookVisual.render_preset(DARK_THEME, PAGE, PADDING_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &slots, &click) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &click, &nested) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &nested, &themed) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &themed, &label) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &label, &header) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &header, &footer) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &footer, &padding) > BODY_DIFF_THRESHOLD);
}

#[test]
fn card_setting_option_updates_card_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn card_preview_action_updates_activation_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn card_hover_focus_and_keyboard_update_body_and_state() {
    let target = preview_detail::component_action_hit_rect(PAGE);
    let mut hover = page_state();
    let before_hover = render_state(&hover);

    assert!(apply_hover_at(
        &mut hover,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET
    ));
    assert!(hover.screen_state.preview_hovered);
    assert!(component_body_pixel_diff(PAGE, &before_hover, &render_state(&hover)) > 0);

    let mut focus = page_state();
    let before_focus = render_state(&focus);
    assert!(focus_clickable_at_for_audit(
        &mut focus,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET
    ));
    assert_eq!("card_focus", focus.screen_state.last_action);
    assert_eq!("card_focused", focus.screen_state.last_event);
    assert_eq!("focused=true", focus.screen_state.state_label);
    assert!(focus.screen_state.is_button_focused());
    assert!(component_body_pixel_diff(PAGE, &before_focus, &render_state(&focus)) > 0);

    let before_keyboard = render_state(&focus);
    assert!(apply_clickable_keyboard_activation_for_audit(&mut focus));
    assert_eq!("card_click", focus.screen_state.last_action);
    assert_eq!("card_activated", focus.screen_state.last_event);
    assert_eq!("active=true", focus.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_keyboard, &render_state(&focus)) > 0);
}

#[test]
fn card_light_and_dark_surface_uses_theme_surface() {
    assert_card_token(DARK_THEME, ThemeSnapshot::dark());
    assert_card_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_card_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, SLOTS_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + CARD_X + CARD_SAMPLE_X_OFFSET,
            component.y + CARD_Y + CARD_SAMPLE_Y_OFFSET
        )
    );
}

fn page_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}
