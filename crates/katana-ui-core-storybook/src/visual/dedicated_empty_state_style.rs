use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const SEARCH_PRESET_INDEX: usize = 1;
const CLEAN_PRESET_INDEX: usize = 2;
const HISTORY_PRESET_INDEX: usize = 3;
const ERROR_PRESET_INDEX: usize = 4;
const ALIGNMENT_LEADING_X: usize = 36;
const ALIGNMENT_CENTER_X: usize = 116;
const ALIGNMENT_TRAILING_X: usize = 196;

pub(super) fn illustration_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action() || scenario.screen_state.has_settings_override() {
        return common::DANGER;
    }
    match scenario.preset_index {
        SEARCH_PRESET_INDEX => common::TOKEN,
        CLEAN_PRESET_INDEX => common::SUCCESS,
        HISTORY_PRESET_INDEX => common::WARN,
        ERROR_PRESET_INDEX => common::DANGER,
        _ => palette.accent,
    }
}

pub(super) fn panel_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == ERROR_PRESET_INDEX {
        return palette.panel;
    }
    palette.surface
}

pub(super) fn primary_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action() || scenario.screen_state.has_settings_override() {
        return common::DANGER;
    }
    if scenario.preset_index == CLEAN_PRESET_INDEX {
        return common::SUCCESS;
    }
    palette.accent
}

pub(super) fn secondary_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == HISTORY_PRESET_INDEX {
        return common::WARN;
    }
    palette.panel
}

pub(super) fn alignment_marker_x(scenario: ScenarioContext<'_>) -> usize {
    if scenario.screen_state.has_settings_override() || scenario.preset_index == SEARCH_PRESET_INDEX
    {
        return ALIGNMENT_LEADING_X;
    }
    if scenario.preset_index == HISTORY_PRESET_INDEX {
        return ALIGNMENT_TRAILING_X;
    }
    ALIGNMENT_CENTER_X
}

pub(super) fn heading_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        SEARCH_PRESET_INDEX => "No matches",
        CLEAN_PRESET_INDEX => "All clear",
        HISTORY_PRESET_INDEX => "No history",
        ERROR_PRESET_INDEX => "Load failed",
        _ => "No diagnostics",
    }
}

pub(super) fn body_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.has_settings_override() {
        return "alignment=leading";
    }
    if scenario.screen_state.has_widget_action() {
        return "action=reload";
    }
    match scenario.preset_index {
        SEARCH_PRESET_INDEX => "try another query",
        CLEAN_PRESET_INDEX => "nothing to fix",
        HISTORY_PRESET_INDEX => "run a task first",
        ERROR_PRESET_INDEX => "retry or open docs",
        _ => "日本語 mixed text",
    }
}

pub(super) fn status_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        SEARCH_PRESET_INDEX => "tone=neutral",
        CLEAN_PRESET_INDEX => "tone=success",
        HISTORY_PRESET_INDEX => "tone=warning",
        ERROR_PRESET_INDEX => "tone=danger",
        _ => "tone=accent",
    }
}
