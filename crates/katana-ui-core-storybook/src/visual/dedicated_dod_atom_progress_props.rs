use super::render_context::ScenarioContext;
use katana_ui_core::atom::ProgressBar;
use katana_ui_core::render_model::{UiNode, UiProps};

const PROGRESS_CHANGE_PRESET: usize = 1;
const PROGRESS_EMPTY_PRESET: usize = 2;
const SPEED_PRESET: usize = 4;
const SEGMENT_COUNT_PRESET: usize = 5;
const REDUCED_MOTION_PRESET: usize = 6;
const PROGRESS_EMPTY_PERCENT: u8 = 0;
const PROGRESS_DEFAULT_PERCENT: u8 = 65;
const PROGRESS_CHANGED_PERCENT: u8 = 82;
pub(super) const SPEED_PRESET_MS: u16 = 96;
pub(super) const SEGMENT_COUNT: u8 = 5;

pub(super) fn core_progress_props(scenario: ScenarioContext<'_>) -> UiProps {
    let label = progress_component_label(scenario);
    let mut progress = ProgressBar::new(label).progress(
        progress_is_determinate(scenario),
        progress_percent(scenario),
    );
    if label == "Syncing" {
        progress = progress.loading_label("Syncing");
    }
    if progress_uses_speed_ms(scenario) {
        progress = progress.speed_ms(SPEED_PRESET_MS);
    }
    if progress_uses_segment_count(scenario) {
        progress = progress.dot_count(SEGMENT_COUNT);
    }
    if progress_uses_reduced_motion(scenario) {
        progress = progress.reduced_motion(true);
    }
    let node: UiNode = progress.into();
    node.props().clone()
}

pub(super) fn component_label(scenario: ScenarioContext<'_>) -> &'static str {
    let props = core_progress_props(scenario);
    progress_label_for_props(&props)
}

pub(super) fn progress_has_speed_preset(props: &UiProps) -> bool {
    props.loading_indicator.speed_ms == SPEED_PRESET_MS
}

pub(super) fn progress_has_segment_count_preset(props: &UiProps) -> bool {
    props.loading_indicator.dot_count == SEGMENT_COUNT
}

fn progress_label_for_props(props: &UiProps) -> &'static str {
    if props.loading_indicator.label == "Syncing" || props.label == "Syncing" {
        return "Syncing";
    }
    "Progress"
}

fn progress_component_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_setting == "loading.label"
        && scenario.screen_state.last_setting_value == "Syncing"
    {
        return "Syncing";
    }
    "Progress"
}

fn progress_percent(scenario: ScenarioContext<'_>) -> u8 {
    if scenario.preset_index == PROGRESS_EMPTY_PRESET {
        return PROGRESS_EMPTY_PERCENT;
    }
    if scenario.screen_state.has_progress_state() {
        return scenario.screen_state.progress_percent();
    }
    if scenario.screen_state.has_settings_override()
        || scenario.preset_index == PROGRESS_CHANGE_PRESET
    {
        return PROGRESS_CHANGED_PERCENT;
    }
    PROGRESS_DEFAULT_PERCENT
}

fn progress_is_determinate(scenario: ScenarioContext<'_>) -> bool {
    !progress_uses_speed_ms(scenario)
        && !progress_uses_segment_count(scenario)
        && !progress_uses_reduced_motion(scenario)
}

fn progress_uses_speed_ms(scenario: ScenarioContext<'_>) -> bool {
    scenario.preset_index == SPEED_PRESET
        || (scenario.screen_state.last_setting == "loading.speed_ms"
            && scenario.screen_state.last_setting_value == "96")
}

fn progress_uses_segment_count(scenario: ScenarioContext<'_>) -> bool {
    scenario.preset_index == SEGMENT_COUNT_PRESET
        || (scenario.screen_state.last_setting == "loading.dot_count"
            && scenario.screen_state.last_setting_value == "5")
}

fn progress_uses_reduced_motion(scenario: ScenarioContext<'_>) -> bool {
    scenario.preset_index == REDUCED_MOTION_PRESET
        || (scenario.screen_state.last_setting == "loading.reduced_motion"
            && scenario.screen_state.last_setting_value == "true")
}
