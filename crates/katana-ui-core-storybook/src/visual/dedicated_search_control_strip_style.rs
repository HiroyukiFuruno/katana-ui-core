use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const EDITOR_FIND_PRESET_INDEX: usize = 1;
const EDITOR_REPLACE_PRESET_INDEX: usize = 2;
const VIEWER_PRESET_INDEX: usize = 3;
const HISTORY_PRESET_INDEX: usize = 4;

pub(super) fn query_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() {
        return common::TOKEN;
    }
    palette.surface
}

pub(super) fn option_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action()
        || scenario.screen_state.has_settings_override()
        || scenario.preset_index == EDITOR_FIND_PRESET_INDEX
    {
        return palette.accent;
    }
    palette.panel
}

pub(super) fn replace_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == EDITOR_REPLACE_PRESET_INDEX {
        return common::SUCCESS;
    }
    if scenario.preset_index == VIEWER_PRESET_INDEX {
        return palette.panel;
    }
    palette.surface
}

pub(super) fn navigation_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == HISTORY_PRESET_INDEX {
        return common::WARN;
    }
    palette.panel
}

pub(super) fn query_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.has_settings_override() {
        return "heading";
    }
    match scenario.preset_index {
        EDITOR_FIND_PRESET_INDEX => "find symbol",
        EDITOR_REPLACE_PRESET_INDEX => "replace title",
        VIEWER_PRESET_INDEX => "viewer text",
        HISTORY_PRESET_INDEX => "recent query",
        _ => "head",
    }
}

pub(super) fn result_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        VIEWER_PRESET_INDEX => "0 / 0",
        HISTORY_PRESET_INDEX => "7 / 18",
        _ => "3 / 12",
    }
}

pub(super) fn replace_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == EDITOR_REPLACE_PRESET_INDEX {
        return "title";
    }
    "replace"
}

pub(super) fn status_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        EDITOR_FIND_PRESET_INDEX => "case=true",
        EDITOR_REPLACE_PRESET_INDEX => "replace=title",
        VIEWER_PRESET_INDEX => "result=0",
        HISTORY_PRESET_INDEX => "active=7",
        _ => "regex=false",
    }
}
