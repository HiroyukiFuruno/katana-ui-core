use super::render_context::ScenarioContext;
use super::screen_state::StorybookScreenState;
use super::{live_interaction_audit, palette};
use katana_ui_core::theme::ThemeSnapshot;

const PAGE: &str = "progress-bar";
const DEFAULT_PRESET: usize = 0;
const SPEED_PRESET: usize = 4;
const PROGRESS_BLOCK_COUNT: usize = 4;
const VALUE_INDEX: usize = 1;
const SEGMENT_INDEX: usize = 3;
const BODY_DIFF_THRESHOLD: usize = 80;
const TICK_MS: u16 = 250;
const FRAME_MS: u16 = 16;
const PRE_TICK_FRAME_COUNT: usize = 15;
const DEFAULT_PERCENT: u8 = 65;
const TICK_TARGET_PERCENT: u8 = 82;
const MAX_PERCENT: u8 = 99;

#[test]
fn progress_bar_timed_tick_advances_via_core_progress_action() {
    let before = progress_blocks(DEFAULT_PRESET);
    let mut screen_state = StorybookScreenState::default();

    screen_state.register_progress_bar_timed_tick(TICK_MS);
    let after_first = progress_blocks_for_screen_state(DEFAULT_PRESET, &screen_state);

    screen_state.register_progress_bar_timed_tick(TICK_MS);
    let after_second = progress_blocks_for_screen_state(DEFAULT_PRESET, &screen_state);

    assert_eq!("progress_tick", screen_state.last_action);
    assert_eq!("progress_changed", screen_state.last_event);
    assert_eq!("percent=99", screen_state.state_label);
    assert_eq!(MAX_PERCENT, screen_state.progress_percent());
    assert!(after_first[VALUE_INDEX].rect.width > before[VALUE_INDEX].rect.width);
    assert!(after_second[VALUE_INDEX].rect.width > after_first[VALUE_INDEX].rect.width);
}

#[test]
fn progress_bar_track_value_and_motion_indicator_are_rounded() {
    let blocks = progress_blocks(DEFAULT_PRESET);

    for block in blocks {
        assert!(
            block.radius > 0,
            "progress bar blocks must not render as flat legacy rectangles"
        );
    }
}

#[test]
fn progress_bar_timed_tick_cycles_after_reaching_maximum() {
    let mut screen_state = StorybookScreenState::default();

    screen_state.register_progress_bar_timed_tick(TICK_MS);
    screen_state.register_progress_bar_timed_tick(TICK_MS);
    let at_max = progress_blocks_for_screen_state(DEFAULT_PRESET, &screen_state);

    screen_state.register_progress_bar_timed_tick(TICK_MS);
    let cycled = progress_blocks_for_screen_state(DEFAULT_PRESET, &screen_state);

    assert_eq!("progress_tick", screen_state.last_action);
    assert_eq!("progress_changed", screen_state.last_event);
    assert_eq!(0, screen_state.progress_percent());
    assert_eq!("percent=0", screen_state.state_label);
    assert!(cycled[VALUE_INDEX].rect.width < at_max[VALUE_INDEX].rect.width);
}

#[test]
fn progress_bar_live_audit_reports_timed_tick_progress_contract() {
    let scenario = live_interaction_audit::progress_timed_tick_scenario();

    assert_eq!("timed_tick", scenario.operation_kind);
    assert!(scenario.passed);
    assert_eq!("progress_tick", scenario.action);
    assert_eq!("progress_changed", scenario.event);
    assert_eq!("percent=82", scenario.state);
    assert!(scenario.body_pixel_diff > BODY_DIFF_THRESHOLD);
}

#[test]
fn progress_bar_live_audit_reports_timed_cycle_after_maximum() {
    let scenario = live_interaction_audit::progress_timed_cycle_scenario();

    assert_eq!("timed_tick", scenario.operation_kind);
    assert!(scenario.passed);
    assert_eq!("progress_tick", scenario.action);
    assert_eq!("progress_changed", scenario.event);
    assert_eq!("percent=0", scenario.state);
    assert!(scenario.body_pixel_diff > BODY_DIFF_THRESHOLD);
}

#[test]
fn progress_bar_live_audit_reports_indeterminate_segment_motion() {
    let scenario = live_interaction_audit::progress_indeterminate_segment_motion_scenario();

    assert_eq!("timed_tick", scenario.operation_kind);
    assert!(scenario.passed);
    assert_eq!("progress_tick", scenario.action);
    assert_eq!("progress_changed", scenario.event);
    assert_eq!("percent=82", scenario.state);
    assert!(scenario.body_pixel_diff > BODY_DIFF_THRESHOLD);
}

#[test]
fn progress_bar_runtime_frame_ticks_accumulate_before_advancing() {
    let mut screen_state = StorybookScreenState::default();

    for _ in 0..PRE_TICK_FRAME_COUNT {
        screen_state.register_progress_bar_timed_tick(FRAME_MS);
    }
    assert_eq!(DEFAULT_PERCENT, screen_state.progress_percent());
    assert_eq!("none", screen_state.last_action);

    screen_state.register_progress_bar_timed_tick(FRAME_MS);

    assert_eq!(TICK_TARGET_PERCENT, screen_state.progress_percent());
    assert_eq!("progress_tick", screen_state.last_action);
    assert_eq!("progress_changed", screen_state.last_event);
}

#[test]
fn progress_bar_repeated_preview_actions_advance_meter_state() {
    let before = progress_blocks(DEFAULT_PRESET);
    let mut screen_state = StorybookScreenState::default();

    screen_state.register_preview_action(PAGE);
    let after_first = progress_blocks_for_screen_state(DEFAULT_PRESET, &screen_state);

    screen_state.register_preview_action(PAGE);
    let after_second = progress_blocks_for_screen_state(DEFAULT_PRESET, &screen_state);

    assert_eq!("progress_change", screen_state.last_action);
    assert_eq!("progress_changed", screen_state.last_event);
    assert_eq!("percent=99", screen_state.state_label);
    assert!(after_first[VALUE_INDEX].rect.width > before[VALUE_INDEX].rect.width);
    assert!(after_second[VALUE_INDEX].rect.width > after_first[VALUE_INDEX].rect.width);
}

#[test]
fn progress_bar_indeterminate_segment_moves_on_runtime_tick() {
    let before = progress_blocks(SPEED_PRESET);
    let mut screen_state = StorybookScreenState::default();

    screen_state.register_progress_bar_timed_tick(TICK_MS);
    let after = progress_blocks_for_screen_state(SPEED_PRESET, &screen_state);

    assert_eq!("progress_tick", screen_state.last_action);
    assert_eq!("progress_changed", screen_state.last_event);
    assert_ne!(
        before[SEGMENT_INDEX].rect.x, after[SEGMENT_INDEX].rect.x,
        "indeterminate progress segment must move on runtime tick"
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
