use super::render_context::ScenarioContext;
use super::screen_state::StorybookScreenState;
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{dedicated_dod_atom_progress, palette};
use katana_ui_core::theme::ThemeSnapshot;

const PAGE: &str = "progress-bar";
const DEFAULT_PRESET: usize = 0;
const PROGRESS_BLOCK_COUNT: usize = 4;
const PROGRESS_LABEL_COUNT: usize = 3;
const TRACK_INDEX: usize = 0;
const VALUE_INDEX: usize = 1;
const SEGMENT_INDEX: usize = 3;
const PERCENT_LABEL_INDEX: usize = 1;
const STATE_LABEL_INDEX: usize = 2;
const PROGRESS_SPEED_SETTING_INDEX: usize = 4;
const PROGRESS_DOT_COUNT_SETTING_INDEX: usize = 5;
const PROGRESS_REDUCED_MOTION_SETTING_INDEX: usize = 6;

#[test]
fn progress_bar_loading_speed_option_reaches_core_props_and_segment_render() {
    let mut state = progress_window_state();
    let before = progress_blocks_for_screen_state(&state.screen_state);
    click_setting(&mut state, PROGRESS_SPEED_SETTING_INDEX);

    let after = progress_blocks_for_screen_state(&state.screen_state);
    let labels = progress_labels_for_screen_state(&state.screen_state);
    assert_eq!("settings_loading_option", state.screen_state.last_action);
    assert_eq!("atom_settings_changed", state.screen_state.last_event);
    assert_eq!("progress_bar.speed_ms=96", state.screen_state.state_label);
    assert_eq!("indeterminate", labels[PERCENT_LABEL_INDEX]);
    assert_eq!("speed=96ms", labels[STATE_LABEL_INDEX]);
    assert_eq!(0, before[SEGMENT_INDEX].rect.width);
    assert!(after[SEGMENT_INDEX].rect.width > before[SEGMENT_INDEX].rect.width);
    assert_eq!(0, after[VALUE_INDEX].rect.width);
}

#[test]
fn progress_bar_loading_dot_count_option_reaches_core_props_and_segment_render() {
    let mut state = progress_window_state();
    click_setting(&mut state, PROGRESS_DOT_COUNT_SETTING_INDEX);

    let after = progress_blocks_for_screen_state(&state.screen_state);
    let labels = progress_labels_for_screen_state(&state.screen_state);
    assert_eq!("settings_loading_option", state.screen_state.last_action);
    assert_eq!("atom_settings_changed", state.screen_state.last_event);
    assert_eq!("progress_bar.dot_count=5", state.screen_state.state_label);
    assert_eq!("indeterminate", labels[PERCENT_LABEL_INDEX]);
    assert_eq!("segments=5", labels[STATE_LABEL_INDEX]);
    assert!(
        after[SEGMENT_INDEX].rect.width > after[VALUE_INDEX].rect.width,
        "dot-count option should switch the render into a wide segmented loading indicator"
    );
}

#[test]
fn progress_bar_reduced_motion_option_reaches_core_props_and_segment_render() {
    let mut state = progress_window_state();
    let before = progress_blocks_for_screen_state(&state.screen_state);
    click_setting(&mut state, PROGRESS_REDUCED_MOTION_SETTING_INDEX);

    let after = progress_blocks_for_screen_state(&state.screen_state);
    let labels = progress_labels_for_screen_state(&state.screen_state);
    assert_eq!("settings_loading_option", state.screen_state.last_action);
    assert_eq!("atom_settings_changed", state.screen_state.last_event);
    assert_eq!(
        "progress_bar.reduced_motion=true",
        state.screen_state.state_label
    );
    assert_eq!("indeterminate", labels[PERCENT_LABEL_INDEX]);
    assert_eq!("reduced motion", labels[STATE_LABEL_INDEX]);
    assert_eq!(0, before[SEGMENT_INDEX].rect.width);
    assert!(after[SEGMENT_INDEX].rect.width > before[SEGMENT_INDEX].rect.width);
    assert_eq!(0, after[VALUE_INDEX].rect.width);
    assert_ne!(after[SEGMENT_INDEX].fill, after[TRACK_INDEX].fill);
}

fn progress_window_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn click_setting(state: &mut StorybookWindowState, index: usize) {
    let row = super::layout_metrics::inspector_setting_row_hit_rect(index);
    assert!(apply_click(state, row.x + 1, row.y + 1));
}

fn progress_blocks_for_screen_state(
    screen_state: &StorybookScreenState,
) -> [dedicated_dod_atom_progress::ProgressBlockSnapshot; PROGRESS_BLOCK_COUNT] {
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    dedicated_dod_atom_progress::progress_blocks_for_test(&colors, scenario(screen_state))
}

fn progress_labels_for_screen_state(
    screen_state: &StorybookScreenState,
) -> [&'static str; PROGRESS_LABEL_COUNT] {
    dedicated_dod_atom_progress::progress_labels_for_test(scenario(screen_state))
}

fn scenario(screen_state: &StorybookScreenState) -> ScenarioContext<'_> {
    ScenarioContext {
        selected_page: PAGE,
        selected_instance_id: crate::visual::window_interaction::DEFAULT_INSTANCE_ID,
        preset_index: DEFAULT_PRESET,
        preset_tab_scroll_x: 0,
        tree_expansion: Default::default(),
        scrollbar_visible: true,
        panel_scroll: Default::default(),
        screen_state,
        show_navigation_lines: true,
        show_navigation_text_connectors: false,
    }
}
