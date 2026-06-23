use super::render_context::ScenarioContext;
use super::screen_state::StorybookScreenState;
use super::{dedicated_dod_atom_progress, palette};
use katana_ui_core::theme::ThemeSnapshot;

const PAGE: &str = "progress-bar";
const SPEED_PRESET: usize = 4;
const SEGMENT_COUNT_PRESET: usize = 5;
const REDUCED_MOTION_PRESET: usize = 6;
const PROGRESS_BLOCK_COUNT: usize = 4;
const PROGRESS_LABEL_COUNT: usize = 3;
const TRACK_INDEX: usize = 0;
const SEGMENT_INDEX: usize = 3;
const PERCENT_LABEL_INDEX: usize = 1;
const STATE_LABEL_INDEX: usize = 2;

#[test]
fn progress_bar_indeterminate_segment_and_reduced_motion_tokens_render() {
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let speed = progress_blocks(SPEED_PRESET);
    let segmented = progress_blocks(SEGMENT_COUNT_PRESET);
    let reduced = progress_blocks(REDUCED_MOTION_PRESET);

    assert!(segmented[SEGMENT_INDEX].rect.width > speed[SEGMENT_INDEX].rect.width);
    assert!(
        speed[SEGMENT_INDEX].rect.y >= speed[TRACK_INDEX].rect.y
            && speed[SEGMENT_INDEX].rect.y + speed[SEGMENT_INDEX].rect.height
                <= speed[TRACK_INDEX].rect.y + speed[TRACK_INDEX].rect.height,
        "indeterminate segment must live inside the progress track, not as a detached second bar"
    );
    assert_ne!(
        speed[TRACK_INDEX].fill, speed[SEGMENT_INDEX].fill,
        "speed preset segment must be visible against the track"
    );
    assert_ne!(colors.surface, reduced[SEGMENT_INDEX].fill);
    assert_eq!(
        "speed=96ms",
        progress_labels(SPEED_PRESET)[STATE_LABEL_INDEX]
    );
    assert_eq!(
        "segments=5",
        progress_labels(SEGMENT_COUNT_PRESET)[STATE_LABEL_INDEX]
    );
    assert_eq!(
        "reduced motion",
        progress_labels(REDUCED_MOTION_PRESET)[STATE_LABEL_INDEX]
    );
}

#[test]
fn progress_bar_indeterminate_presets_do_not_show_determinate_percent_label() {
    assert_eq!(
        "indeterminate",
        progress_labels(SPEED_PRESET)[PERCENT_LABEL_INDEX]
    );
    assert_eq!(
        "indeterminate",
        progress_labels(SEGMENT_COUNT_PRESET)[PERCENT_LABEL_INDEX]
    );
}

fn progress_blocks(
    preset_index: usize,
) -> [dedicated_dod_atom_progress::ProgressBlockSnapshot; PROGRESS_BLOCK_COUNT] {
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let screen_state = StorybookScreenState::default();
    let scenario = scenario(preset_index, &screen_state);

    dedicated_dod_atom_progress::progress_blocks_for_test(&colors, scenario)
}

fn progress_labels(preset_index: usize) -> [&'static str; PROGRESS_LABEL_COUNT] {
    let screen_state = StorybookScreenState::default();
    let scenario = scenario(preset_index, &screen_state);

    dedicated_dod_atom_progress::progress_labels_for_test(scenario)
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
