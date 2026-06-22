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
const PAGE: &str = "spinner";
const DEFAULT_PRESET: usize = 0;
const PHASE_PRESET: usize = 1;
const PAUSED_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const SPEED_PRESET: usize = 4;
const SEGMENT_COUNT_PRESET: usize = 5;
const TONE_PRESET: usize = 6;
const SIZE_PRESET: usize = 7;
const REQUIRED_PRESET_COUNT: usize = 8;
const REQUIRED_OPTION_COUNT: usize = 8;
const BODY_DIFF_THRESHOLD: usize = 80;
const SAMPLE_X_OFFSET: usize = 64;
const SAMPLE_Y_OFFSET: usize = 42;
const SPINNER_BLOCK_COUNT: usize = 6;
const SPINNER_LABEL_COUNT: usize = 3;
const CONTAINER_INDEX: usize = 0;
const LEADING_SEGMENT_INDEX: usize = 1;
const TOKEN_SEGMENT_INDEX: usize = 2;
const EXTRA_SEGMENT_INDEX: usize = 5;
const TICK_LABEL_INDEX: usize = 0;
const MOTION_LABEL_INDEX: usize = 1;

#[test]
fn spinner_exposes_leaf_presets_options_and_phase_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("spinner_tick", spec.action);
    assert_eq!("spinner_phase_changed", spec.event);
    assert_eq!("phase=paused", spec.state);
}

#[test]
fn spinner_presets_render_distinct_phase_bodies() {
    let running = StorybookVisual.render_preset(DARK_THEME, PAGE, DEFAULT_PRESET, 0);
    let phase = StorybookVisual.render_preset(DARK_THEME, PAGE, PHASE_PRESET, 0);
    let paused = StorybookVisual.render_preset(DARK_THEME, PAGE, PAUSED_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &running, &phase) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &phase, &paused) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &paused, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn spinner_geometry_covers_speed_segment_count_size_and_bounds() {
    let running = spinner_blocks(DEFAULT_PRESET);
    let speed = spinner_blocks(SPEED_PRESET);
    let segments = spinner_blocks(SEGMENT_COUNT_PRESET);
    let sized = spinner_blocks(SIZE_PRESET);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert!(speed[LEADING_SEGMENT_INDEX].rect.width > running[LEADING_SEGMENT_INDEX].rect.width);
    assert!(segments[EXTRA_SEGMENT_INDEX].rect.width > running[EXTRA_SEGMENT_INDEX].rect.width);
    assert!(sized[CONTAINER_INDEX].rect.height > running[CONTAINER_INDEX].rect.height);
    for segment in [
        running[CONTAINER_INDEX],
        speed[LEADING_SEGMENT_INDEX],
        segments[EXTRA_SEGMENT_INDEX],
        sized[TOKEN_SEGMENT_INDEX],
    ] {
        assert!(component.contains(component.x + segment.rect.x, component.y + segment.rect.y));
        assert!(component.contains(
            component.x + segment.rect.x + segment.rect.width.saturating_sub(1),
            component.y + segment.rect.y + segment.rect.height - 1
        ));
    }
}

#[test]
fn spinner_paused_and_tone_presets_use_motion_tokens() {
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let paused = spinner_blocks(PAUSED_PRESET);
    let tone = spinner_blocks(TONE_PRESET);

    assert_eq!(colors.panel, paused[TOKEN_SEGMENT_INDEX].fill);
    assert_ne!(colors.accent, tone[LEADING_SEGMENT_INDEX].fill);
    assert_eq!(
        "reduced motion: on",
        spinner_labels(PAUSED_PRESET)[MOTION_LABEL_INDEX]
    );
}

#[test]
fn spinner_phase_labels_cover_tick_speed_and_segments() {
    assert_eq!(
        "motion tick: 7/12",
        spinner_labels(PHASE_PRESET)[TICK_LABEL_INDEX]
    );
    assert_eq!("speed=96ms", spinner_labels(SPEED_PRESET)[TICK_LABEL_INDEX]);
    assert_eq!(
        "segments=5",
        spinner_labels(SEGMENT_COUNT_PRESET)[TICK_LABEL_INDEX]
    );
}

#[test]
fn spinner_setting_option_updates_phase_style() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn spinner_preview_action_updates_phase_style() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn spinner_light_and_dark_surfaces_use_theme_tokens() {
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

fn spinner_blocks(
    preset_index: usize,
) -> [super::dedicated_dod_atom_motion::SpinnerBlockSnapshot; SPINNER_BLOCK_COUNT] {
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let screen_state = StorybookScreenState::default();
    let scenario = scenario(preset_index, &screen_state);

    super::dedicated_dod_atom_motion::spinner_blocks_for_test(&colors, scenario)
}

fn spinner_labels(preset_index: usize) -> [&'static str; SPINNER_LABEL_COUNT] {
    let screen_state = StorybookScreenState::default();
    let scenario = scenario(preset_index, &screen_state);

    super::dedicated_dod_atom_motion::spinner_labels_for_test(scenario)
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
