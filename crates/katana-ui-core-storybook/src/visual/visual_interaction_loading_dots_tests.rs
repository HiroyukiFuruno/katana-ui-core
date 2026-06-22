use super::interaction_spec::StorybookInteractionSpec;
use super::render_context::ScenarioContext;
use super::screen_state::StorybookScreenState;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "loading-dots";
const DEFAULT_PRESET: usize = 0;
const PHASE_PRESET: usize = 1;
const REDUCED_MOTION_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const SPEED_PRESET: usize = 4;
const DOT_COUNT_PRESET: usize = 5;
const REQUIRED_PRESET_COUNT: usize = 8;
const REQUIRED_OPTION_COUNT: usize = 8;
const BODY_DIFF_THRESHOLD: usize = 80;
const SAMPLE_X_OFFSET: usize = 34;
const SAMPLE_Y_OFFSET: usize = 50;
const LOADING_DOT_BLOCK_COUNT: usize = 5;
const PRIMARY_DOT_INDEX: usize = 0;
const SECONDARY_DOT_INDEX: usize = 1;
const TERTIARY_DOT_INDEX: usize = 2;
const QUATERNARY_DOT_INDEX: usize = 3;
const REDUCED_MOTION_BADGE_INDEX: usize = 4;

#[test]
fn loading_dots_exposes_leaf_presets_options_and_phase_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("animation_tick", spec.action);
    assert_eq!("loading_phase_changed", spec.event);
    assert_eq!("phase=1", spec.state);
}

#[test]
fn loading_dots_presets_render_distinct_phase_bodies() {
    let running = StorybookVisual.render_preset(DARK_THEME, PAGE, DEFAULT_PRESET, 0);
    let phase = StorybookVisual.render_preset(DARK_THEME, PAGE, PHASE_PRESET, 0);
    let reduced = StorybookVisual.render_preset(DARK_THEME, PAGE, REDUCED_MOTION_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &running, &phase) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &phase, &reduced) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &reduced, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn loading_dots_geometry_covers_phase_speed_dot_count_and_bounds() {
    let running = loading_dot_blocks(DEFAULT_PRESET);
    let phase = loading_dot_blocks(PHASE_PRESET);
    let speed = loading_dot_blocks(SPEED_PRESET);
    let dot_count = loading_dot_blocks(DOT_COUNT_PRESET);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert!(phase[PRIMARY_DOT_INDEX].rect.width > running[PRIMARY_DOT_INDEX].rect.width);
    assert!(phase[SECONDARY_DOT_INDEX].rect.y > running[SECONDARY_DOT_INDEX].rect.y);
    assert!(speed[SECONDARY_DOT_INDEX].rect.width > running[SECONDARY_DOT_INDEX].rect.width);
    assert!(dot_count[TERTIARY_DOT_INDEX].rect.width > running[TERTIARY_DOT_INDEX].rect.width);
    assert!(dot_count[QUATERNARY_DOT_INDEX].rect.width > running[QUATERNARY_DOT_INDEX].rect.width);
    for dot in [
        running[PRIMARY_DOT_INDEX],
        phase[PRIMARY_DOT_INDEX],
        speed[SECONDARY_DOT_INDEX],
        dot_count[QUATERNARY_DOT_INDEX],
    ] {
        assert!(component.contains(component.x + dot.rect.x, component.y + dot.rect.y));
        assert!(component.contains(
            component.x + dot.rect.x + dot.rect.width - 1,
            component.y + dot.rect.y + dot.rect.height - 1
        ));
    }
}

#[test]
fn loading_dots_reduced_motion_uses_static_motion_token() {
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let reduced = loading_dot_blocks(REDUCED_MOTION_PRESET);

    assert_eq!(colors.panel, reduced[TERTIARY_DOT_INDEX].fill);
    assert_ne!(
        colors.surface, reduced[REDUCED_MOTION_BADGE_INDEX].fill,
        "reduced motion badge should visibly change from default surface"
    );
    assert_eq!(
        "reduced motion on",
        loading_dots_motion_label(REDUCED_MOTION_PRESET)
    );
}

#[test]
fn loading_dots_phase_labels_cover_tick_speed_and_dot_count() {
    assert_eq!(
        "phase=4 speed=fast label=Loading",
        phase_label(PHASE_PRESET)
    );
    assert_eq!("speed=96ms", phase_label(SPEED_PRESET));
    assert_eq!("dot_count=5", phase_label(DOT_COUNT_PRESET));
}

#[test]
fn loading_dots_setting_option_updates_phase_style() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn loading_dots_preview_action_updates_phase_style() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn loading_dots_light_and_dark_surfaces_use_theme_tokens() {
    let dark = StorybookVisual.render_preset(DARK_THEME, PAGE, DEFAULT_PRESET, 0);
    let light = StorybookVisual.render_preset(LIGHT_THEME, PAGE, DEFAULT_PRESET, 0);
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let sample_x = rect.x + SAMPLE_X_OFFSET;
    let sample_y = rect.y + SAMPLE_Y_OFFSET;

    assert_ne!(
        pixel_at(&dark, sample_x, sample_y),
        pixel_at(&light, sample_x, sample_y)
    );
}

fn loading_dot_blocks(
    preset_index: usize,
) -> [super::dedicated_dod_atom_loading_dots::LoadingDotSnapshot; LOADING_DOT_BLOCK_COUNT] {
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let screen_state = StorybookScreenState::default();
    let scenario = scenario(preset_index, &screen_state);

    super::dedicated_dod_atom_loading_dots::loading_dot_blocks_for_test(&colors, scenario)
}

fn phase_label(preset_index: usize) -> &'static str {
    let screen_state = StorybookScreenState::default();
    let scenario = scenario(preset_index, &screen_state);

    super::dedicated_dod_atom_loading_dots::loading_dots_phase_label_for_test(scenario)
}

fn loading_dots_motion_label(preset_index: usize) -> &'static str {
    let screen_state = StorybookScreenState::default();
    let scenario = scenario(preset_index, &screen_state);

    super::dedicated_dod_atom_loading_dots::loading_dots_motion_label_for_test(scenario)
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
