use super::interaction_spec::StorybookInteractionSpec;
use super::render_context::ScenarioContext;
use super::screen_state::StorybookScreenState;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::atom::ColorSwatch;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{RgbaActionValue, UiAction};
use katana_ui_core::render_model::UiNode;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "color-swatch";
const PALETTE_PRESET: usize = 0;
const SELECT_PRESET: usize = 1;
const DISABLED_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SWATCH_X: usize = 18;
const SWATCH_Y: usize = 38;
const SWATCH_SAMPLE_OFFSET: usize = 4;
const SWATCH_COUNT: usize = 5;
const SELECTED_INDEX: usize = 2;
const COLOR_RED: u8 = 64;
const COLOR_GREEN: u8 = 128;
const COLOR_BLUE: u8 = 255;
const COLOR_ALPHA: u8 = 204;
const COLOR_HUE: u16 = 215;
const COLOR_RGBA: &str = "rgba(64, 128, 255, 204)";

#[test]
fn color_swatch_exposes_leaf_presets_options_and_color_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("color_select", spec.action);
    assert_eq!("color_changed", spec.event);
    assert_eq!("color_swatch.selected_color", spec.option);
    assert_eq!("rgba(64,128,255,1)", spec.after);
    assert_eq!("color=accent", spec.state);
}

#[test]
fn color_swatch_core_action_updates_selected_rgba_props() {
    let mut swatch = ColorSwatch::new("Accent");
    let result = swatch.apply_action(&UiAction::color_drag(
        swatch.state_id().clone(),
        RgbaActionValue::new(COLOR_RED, COLOR_GREEN, COLOR_BLUE, COLOR_ALPHA),
        COLOR_HUE,
        false,
    ));
    let node: UiNode = swatch.into();
    let props = node.props();

    assert!(result.handled);
    assert_eq!(COLOR_RGBA, props.color_swatch.selected_color);
    assert_eq!(COLOR_RGBA, props.interaction.value);
    assert_eq!("color_drag", result.callback_log[0].action);
}

#[test]
fn color_swatch_presets_render_distinct_palette_bodies() {
    let palette = StorybookVisual.render_preset(DARK_THEME, PAGE, PALETTE_PRESET, 0);
    let selected = StorybookVisual.render_preset(DARK_THEME, PAGE, SELECT_PRESET, 0);
    let disabled = StorybookVisual.render_preset(DARK_THEME, PAGE, DISABLED_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &palette, &selected) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &selected, &disabled) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &palette, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn color_swatch_geometry_covers_selected_contrast_and_theme_ring() {
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let palette_swatches = swatches(PALETTE_PRESET);
    let selected_swatches = swatches(SELECT_PRESET);
    let disabled_swatches = swatches(DISABLED_PRESET);
    let themed_swatches = swatches(THEME_PRESET);

    assert_eq!(SWATCH_COUNT, palette_swatches.len());
    assert_eq!(colors.accent, palette_swatches[0].fill);
    assert_ne!(palette_swatches[0].fill, palette_swatches[1].fill);
    assert!(selected_swatches[SELECTED_INDEX].selected);
    assert_eq!(colors.accent, themed_swatches[SELECTED_INDEX].ring);
    assert_ne!(disabled_swatches[0].fill, disabled_swatches[1].fill);
}

#[test]
fn color_swatch_setting_option_updates_palette_state() {
    assert_settings_page_changes_body(PAGE);
}

fn swatches(
    preset_index: usize,
) -> [super::dedicated_dod_atom_swatch_live::ColorSwatchSnapshot; SWATCH_COUNT] {
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let screen_state = StorybookScreenState::default();
    let scenario = scenario(preset_index, &screen_state);

    super::dedicated_dod_atom_swatch_live::swatches_for_test(&colors, scenario)
}

fn scenario<'a>(
    preset_index: usize,
    screen_state: &'a StorybookScreenState,
) -> ScenarioContext<'a> {
    ScenarioContext {
        selected_page: PAGE,
        selected_instance_id: crate::visual::window_interaction::DEFAULT_INSTANCE_ID,
        preset_index,
        preset_tab_scroll_x: 0,
        tree_expansion: Default::default(),
        scrollbar_visible: true,
        panel_scroll: Default::default(),
        screen_state,
        show_navigation_lines: true,
        show_navigation_text_connectors: false,
    }
}

#[test]
fn color_swatch_preview_action_updates_palette_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn color_swatch_light_and_dark_first_swatch_uses_theme_accent() {
    assert_first_swatch_token(DARK_THEME, ThemeSnapshot::dark());
    assert_first_swatch_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_first_swatch_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, PALETTE_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.accent),
        pixel_at(
            &canvas,
            component.x + SWATCH_X + SWATCH_SAMPLE_OFFSET,
            component.y + SWATCH_Y + SWATCH_SAMPLE_OFFSET
        )
    );
}
