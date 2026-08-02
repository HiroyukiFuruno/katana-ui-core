use crate::visual::{
    dedicated_dod_atom_progress_motion::{
        ProgressSegmentMotionSnapshot, progress_segment_motion_snapshot,
    },
    live_interaction_audit::{
        StorybookLiveInteractionScenario, component_body_pixel_diff, page_state, render_state,
        scenario,
    },
    render_context::ScenarioContext,
    window_interaction::{DEFAULT_INSTANCE_ID, StorybookWindowState},
};

const PROGRESS_BAR_PAGE: &str = "progress-bar";
const PROGRESS_TICK_MS: u16 = 250;
const SPEED_PRESET_INDEX: usize = 4;

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    if page != PROGRESS_BAR_PAGE {
        return Vec::new();
    }
    vec![
        progress_timed_tick_scenario(),
        progress_timed_cycle_scenario(),
        progress_indeterminate_segment_motion_scenario(),
    ]
}

pub(in crate::visual) fn progress_timed_tick_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PROGRESS_BAR_PAGE);
    let before = render_state(PROGRESS_BAR_PAGE, &state);
    let before_percent = state.screen_state.progress_percent();
    state
        .screen_state
        .register_progress_bar_timed_tick(PROGRESS_TICK_MS);
    let after = render_state(PROGRESS_BAR_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PROGRESS_BAR_PAGE, &before, &after);
    let changed = state.screen_state.progress_percent() > before_percent;
    let passed = changed
        && state.screen_state.last_action == "progress_tick"
        && state.screen_state.last_event == "progress_changed"
        && body_pixel_diff > 0;
    scenario(
        PROGRESS_BAR_PAGE,
        "progress_timed_tick",
        "timed_tick",
        changed,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(in crate::visual) fn progress_timed_cycle_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PROGRESS_BAR_PAGE);
    state
        .screen_state
        .register_progress_bar_timed_tick(PROGRESS_TICK_MS);
    state
        .screen_state
        .register_progress_bar_timed_tick(PROGRESS_TICK_MS);
    let at_max = render_state(PROGRESS_BAR_PAGE, &state);
    state
        .screen_state
        .register_progress_bar_timed_tick(PROGRESS_TICK_MS);
    let after = render_state(PROGRESS_BAR_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PROGRESS_BAR_PAGE, &at_max, &after);
    let cycled = state.screen_state.progress_percent() == 0;
    let passed = cycled
        && state.screen_state.last_action == "progress_tick"
        && state.screen_state.last_event == "progress_changed"
        && state.screen_state.state_label == "percent=0"
        && body_pixel_diff > 0;
    scenario(
        PROGRESS_BAR_PAGE,
        "progress_timed_cycle",
        "timed_tick",
        cycled,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(in crate::visual) fn progress_indeterminate_segment_motion_scenario()
-> StorybookLiveInteractionScenario {
    let mut state = page_state(PROGRESS_BAR_PAGE);
    state.preset_index = SPEED_PRESET_INDEX;
    let before = render_state(PROGRESS_BAR_PAGE, &state);
    let before_segment = progress_segment_snapshot(&state);
    state
        .screen_state
        .register_progress_bar_timed_tick(PROGRESS_TICK_MS);
    let after = render_state(PROGRESS_BAR_PAGE, &state);
    let after_segment = progress_segment_snapshot(&state);
    let body_pixel_diff = component_body_pixel_diff(PROGRESS_BAR_PAGE, &before, &after);
    let moved = segment_moved_within_track(before_segment, after_segment);
    let passed = moved
        && state.screen_state.last_action == "progress_tick"
        && state.screen_state.last_event == "progress_changed"
        && state.screen_state.state_label == "percent=82"
        && body_pixel_diff > 0;
    scenario(
        PROGRESS_BAR_PAGE,
        "progress_indeterminate_segment_motion",
        "timed_tick",
        moved,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn progress_segment_snapshot(
    state: &StorybookWindowState,
) -> Option<ProgressSegmentMotionSnapshot> {
    progress_segment_motion_snapshot(ScenarioContext {
        selected_page: PROGRESS_BAR_PAGE,
        selected_instance_id: DEFAULT_INSTANCE_ID,
        preset_index: state.preset_index,
        preset_tab_scroll_x: state.preset_tab_scroll_x,
        tree_expansion: state.tree_expansion,
        scrollbar_visible: state.scrollbar_visible,
        panel_scroll: state.panel_scroll,
        screen_state: &state.screen_state,
        show_navigation_lines: state.show_navigation_lines,
        show_navigation_text_connectors: state.show_navigation_text_connectors,
    })
}

fn segment_moved_within_track(
    before: Option<ProgressSegmentMotionSnapshot>,
    after: Option<ProgressSegmentMotionSnapshot>,
) -> bool {
    let (Some(before), Some(after)) = (before, after) else {
        return false;
    };
    before.x != after.x && segment_inside_track(after)
}

fn segment_inside_track(segment: ProgressSegmentMotionSnapshot) -> bool {
    segment.x >= segment.track_x
        && segment.width > 0
        && segment.x.saturating_add(segment.width)
            <= segment.track_x.saturating_add(segment.track_width)
}

#[cfg(test)]
mod tests {
    use super::segment_moved_within_track;

    #[test]
    fn missing_progress_segment_cannot_report_motion() {
        assert!(!segment_moved_within_track(None, None));
    }
}
