use super::interaction_spec::StorybookInteractionSpec;
use super::render_context::ScenarioContext;
use super::screen_state::StorybookScreenState;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::atom::ProgressBar;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::render_model::{UiNode, UiProgressMode};
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "progress-bar";
const DEFAULT_PRESET: usize = 0;
const CHANGE_PRESET: usize = 1;
const EMPTY_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const SIZE_PRESET: usize = 8;
const REQUIRED_PRESET_COUNT: usize = 9;
const REQUIRED_OPTION_COUNT: usize = 9;
const BODY_DIFF_THRESHOLD: usize = 80;
const SAMPLE_X_OFFSET: usize = 24;
const SAMPLE_Y_OFFSET: usize = 48;
const PROGRESS_BLOCK_COUNT: usize = 4;
const PROGRESS_LABEL_COUNT: usize = 3;
const TRACK_INDEX: usize = 0;
const VALUE_INDEX: usize = 1;
const COMPONENT_LABEL_INDEX: usize = 0;
const PERCENT_LABEL_INDEX: usize = 1;
const ACTION_PERCENT: u8 = 82;
const CORE_PROGRESS_TRACK_WIDTH: usize = 244;
const PERCENT_SCALE: usize = 100;
const PROGRESS_PERCENT_SETTING_INDEX: usize = 1;
const PROGRESS_LABEL_SETTING_INDEX: usize = 3;

#[test]
fn progress_bar_exposes_leaf_presets_options_and_progress_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("progress_change", spec.action);
    assert_eq!("progress_changed", spec.event);
    assert_eq!("percent=82", spec.state);
}

#[test]
fn progress_bar_presets_render_distinct_meter_bodies() {
    let determinate = StorybookVisual.render_preset(DARK_THEME, PAGE, DEFAULT_PRESET, 0);
    let changed = StorybookVisual.render_preset(DARK_THEME, PAGE, CHANGE_PRESET, 0);
    let empty = StorybookVisual.render_preset(DARK_THEME, PAGE, EMPTY_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &determinate, &changed) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &changed, &empty) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &empty, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn progress_bar_core_action_updates_determinate_percent_and_mode() {
    let mut progress = ProgressBar::new("Progress").progress(false, 0);
    let target = progress.state_id().clone();
    let result = progress.apply_action(&UiAction::progress_changed(target, true, ACTION_PERCENT));
    let node: UiNode = progress.into();
    let props = node.props();

    assert!(result.handled);
    assert!(props.determinate);
    assert_eq!(ACTION_PERCENT, props.progress_percent);
    assert_eq!(UiProgressMode::Determinate, props.loading_indicator.mode);
}

#[test]
fn progress_bar_geometry_covers_empty_changed_size_and_bounds() {
    let determinate = progress_blocks(DEFAULT_PRESET);
    let changed = progress_blocks(CHANGE_PRESET);
    let empty = progress_blocks(EMPTY_PRESET);
    let sized = progress_blocks(SIZE_PRESET);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert!(changed[VALUE_INDEX].rect.width > determinate[VALUE_INDEX].rect.width);
    assert_eq!(0, empty[VALUE_INDEX].rect.width);
    assert!(sized[VALUE_INDEX].rect.width > determinate[VALUE_INDEX].rect.width);
    for block in [
        determinate[TRACK_INDEX],
        changed[VALUE_INDEX],
        sized[VALUE_INDEX],
    ] {
        assert!(component.contains(component.x + block.rect.x, component.y + block.rect.y));
        assert!(component.contains(
            component.x + block.rect.x + block.rect.width - 1,
            component.y + block.rect.y + block.rect.height - 1
        ));
    }
}

#[test]
fn progress_bar_percent_labels_cover_zero_changed_and_default() {
    assert_eq!("65%", progress_labels(DEFAULT_PRESET)[PERCENT_LABEL_INDEX]);
    assert_eq!("82%", progress_labels(CHANGE_PRESET)[PERCENT_LABEL_INDEX]);
    assert_eq!("0%", progress_labels(EMPTY_PRESET)[PERCENT_LABEL_INDEX]);
}

#[test]
fn progress_bar_loading_label_option_reaches_core_props_and_render_label() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let row = super::layout_metrics::inspector_setting_row_hit_rect(PROGRESS_LABEL_SETTING_INDEX);

    assert!(apply_click(&mut state, row.x + 1, row.y + 1));

    let labels = progress_labels_for_screen_state(DEFAULT_PRESET, &state.screen_state);
    assert_eq!("settings_loading_option", state.screen_state.last_action);
    assert_eq!("atom_settings_changed", state.screen_state.last_event);
    assert_eq!("progress_bar.label=Syncing", state.screen_state.state_label);
    assert_eq!("Syncing", labels[COMPONENT_LABEL_INDEX]);
}

#[test]
fn progress_bar_setting_option_updates_meter_width() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn progress_bar_preview_action_updates_meter_width() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn progress_bar_preview_action_updates_progress_width_and_action_state() {
    let before = progress_blocks(DEFAULT_PRESET);
    let screen_state = super::clicked_preset_screen_state(PAGE, DEFAULT_PRESET);
    let after = progress_blocks_for_screen_state(DEFAULT_PRESET, &screen_state);

    assert_eq!("progress_change", screen_state.last_action);
    assert_eq!("progress_changed", screen_state.last_event);
    assert!(after[VALUE_INDEX].rect.width > before[VALUE_INDEX].rect.width);
}

#[test]
fn progress_bar_percent_option_updates_state_and_meter_width() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let before = progress_blocks_for_screen_state(DEFAULT_PRESET, &state.screen_state);
    let row = super::layout_metrics::inspector_setting_row_hit_rect(PROGRESS_PERCENT_SETTING_INDEX);

    assert!(apply_click(&mut state, row.x + 1, row.y + 1));

    let after = progress_blocks_for_screen_state(DEFAULT_PRESET, &state.screen_state);
    assert_eq!("settings_progress_option", state.screen_state.last_action);
    assert_eq!("atom_settings_changed", state.screen_state.last_event);
    assert_eq!("progress_bar.percent=82", state.screen_state.state_label);
    assert_eq!(82, state.screen_state.progress_percent());
    assert!(state.screen_state.has_progress_state());
    assert!(after[VALUE_INDEX].rect.width > before[VALUE_INDEX].rect.width);
}

#[test]
fn progress_bar_meter_width_matches_core_progress_props() {
    let mut screen_state = StorybookScreenState::default();
    screen_state.register_preview_action(PAGE);
    let blocks = progress_blocks_for_screen_state(DEFAULT_PRESET, &screen_state);

    assert_eq!(
        core_progress_width(screen_state.progress_percent()),
        blocks[VALUE_INDEX].rect.width
    );
}

#[test]
fn progress_bar_dedicated_render_uses_core_progress_bar_public_api() {
    let source = include_str!("dedicated_dod_atom_progress_props.rs");

    assert!(source.contains("ProgressBar::new"));
    assert!(source.contains(".progress("));
    assert!(source.contains(".loading_label("));
    assert!(source.contains(".speed_ms("));
    assert!(source.contains(".dot_count("));
    assert!(source.contains(".reduced_motion("));
}

#[test]
fn progress_bar_motion_presets_are_derived_from_core_public_props() {
    let source = concat!(
        include_str!("dedicated_dod_atom_progress.rs"),
        include_str!("dedicated_dod_atom_progress_motion.rs")
    );

    assert!(source.contains("props.loading_indicator.speed_ms"));
    assert!(source.contains("props.loading_indicator.dot_count"));
    assert!(source.contains("props.loading_indicator.reduced_motion"));
}

#[test]
fn progress_bar_light_and_dark_surfaces_use_theme_tokens() {
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

fn progress_blocks(
    preset_index: usize,
) -> [super::dedicated_dod_atom_progress::ProgressBlockSnapshot; PROGRESS_BLOCK_COUNT] {
    let screen_state = StorybookScreenState::default();
    progress_blocks_for_screen_state(preset_index, &screen_state)
}

fn progress_blocks_for_screen_state(
    preset_index: usize,
    screen_state: &StorybookScreenState,
) -> [super::dedicated_dod_atom_progress::ProgressBlockSnapshot; PROGRESS_BLOCK_COUNT] {
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let scenario = scenario(preset_index, screen_state);

    super::dedicated_dod_atom_progress::progress_blocks_for_test(&colors, scenario)
}

fn progress_labels(preset_index: usize) -> [&'static str; PROGRESS_LABEL_COUNT] {
    let screen_state = StorybookScreenState::default();
    progress_labels_for_screen_state(preset_index, &screen_state)
}

fn progress_labels_for_screen_state(
    preset_index: usize,
    screen_state: &StorybookScreenState,
) -> [&'static str; PROGRESS_LABEL_COUNT] {
    let scenario = scenario(preset_index, screen_state);

    super::dedicated_dod_atom_progress::progress_labels_for_test(scenario)
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

fn core_progress_width(percent: u8) -> usize {
    let node: UiNode = ProgressBar::new("Progress").progress(true, percent).into();
    CORE_PROGRESS_TRACK_WIDTH * usize::from(node.props().progress_percent) / PERCENT_SCALE
}
