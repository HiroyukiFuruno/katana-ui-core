use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const CHAT_PRESET_INDEX: usize = 1;
const LINT_PRESET_INDEX: usize = 2;
const DIRTY_PRESET_INDEX: usize = 3;
const QUERY_PRESET_INDEX: usize = 4;
const RESET_PRESET_INDEX: usize = 5;
const APP_SECTION_INDEX: usize = 0;
const CHAT_SECTION_INDEX: usize = 1;
const LINT_SECTION_INDEX: usize = 2;

pub(super) fn section_fill(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    section: usize,
) -> u32 {
    if section == active_section(scenario) {
        return palette.surface;
    }
    palette.panel
}

pub(super) fn control_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action()
        || scenario.screen_state.has_settings_override()
        || scenario.preset_index == DIRTY_PRESET_INDEX
    {
        return common::WARN;
    }
    if scenario.preset_index == RESET_PRESET_INDEX {
        return common::SUCCESS;
    }
    palette.accent
}

pub(super) fn query_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == QUERY_PRESET_INDEX || scenario.screen_state.has_settings_override()
    {
        return common::TOKEN;
    }
    palette.panel
}

pub(super) fn dirty_marker_fill(scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == DIRTY_PRESET_INDEX || scenario.screen_state.has_widget_action() {
        return common::WARN;
    }
    common::SUCCESS
}

pub(super) fn section_label(scenario: ScenarioContext<'_>) -> &'static str {
    match active_section(scenario) {
        CHAT_SECTION_INDEX => "Chat settings",
        LINT_SECTION_INDEX => "Lint settings",
        _ => "App settings",
    }
}

pub(super) fn field_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.has_settings_override() {
        return "query=font";
    }
    match scenario.preset_index {
        CHAT_PRESET_INDEX => "Model: GPT-5 Codex",
        LINT_PRESET_INDEX => "Severity: warning",
        DIRTY_PRESET_INDEX => "Font size: dirty",
        QUERY_PRESET_INDEX => "filter: format",
        RESET_PRESET_INDEX => "Font size: default",
        _ => "Format on save",
    }
}

pub(super) fn status_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        DIRTY_PRESET_INDEX => "dirty=true",
        QUERY_PRESET_INDEX => "query=format",
        RESET_PRESET_INDEX => "reset=ready",
        _ => "sections=3",
    }
}

fn active_section(scenario: ScenarioContext<'_>) -> usize {
    match scenario.preset_index {
        CHAT_PRESET_INDEX => CHAT_SECTION_INDEX,
        LINT_PRESET_INDEX => LINT_SECTION_INDEX,
        _ => APP_SECTION_INDEX,
    }
}
