use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const CHAT_USAGE_PRESET_INDEX: usize = 1;
const LINTER_PRESET_INDEX: usize = 2;
const PROGRESS_PRESET_INDEX: usize = 3;
const POPOVER_PRESET_INDEX: usize = 4;
const PROGRESS_COMPACT_WIDTH: usize = 80;
const PROGRESS_FULL_WIDTH: usize = 128;

pub(super) fn segment_fill(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    index: usize,
) -> u32 {
    if scenario.screen_state.has_widget_action() || scenario.screen_state.has_settings_override() {
        return palette.accent;
    }
    if scenario.preset_index == LINTER_PRESET_INDEX && index == 1 {
        return common::WARN;
    }
    if scenario.preset_index == POPOVER_PRESET_INDEX && index == 0 {
        return common::TOKEN;
    }
    palette.surface
}

pub(super) fn progress_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == PROGRESS_PRESET_INDEX {
        return common::SUCCESS;
    }
    palette.accent
}

pub(super) fn progress_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == PROGRESS_PRESET_INDEX {
        return PROGRESS_FULL_WIDTH;
    }
    PROGRESS_COMPACT_WIDTH
}

pub(super) fn center_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        CHAT_USAGE_PRESET_INDEX => "tokens 42%",
        LINTER_PRESET_INDEX => "2 warnings",
        PROGRESS_PRESET_INDEX => "indexing 72%",
        POPOVER_PRESET_INDEX => "main ahead",
        _ => "Ln 12, Col 4",
    }
}

pub(super) fn status_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        CHAT_USAGE_PRESET_INDEX => "mode=chat",
        LINTER_PRESET_INDEX => "lint=warning",
        PROGRESS_PRESET_INDEX => "progress=72",
        POPOVER_PRESET_INDEX => "popover=branch",
        _ => "segments=4",
    }
}
