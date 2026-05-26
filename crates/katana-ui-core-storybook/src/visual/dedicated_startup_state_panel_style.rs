use super::dedicated_dod_common as common;
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const SESSION_PRESET_INDEX: usize = 1;
const UPDATE_PRESET_INDEX: usize = 2;
const ERROR_PRESET_INDEX: usize = 3;

pub(super) fn status_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action()
        || scenario.screen_state.has_settings_override()
        || scenario.preset_index == ERROR_PRESET_INDEX
    {
        return common::DANGER;
    }
    if scenario.preset_index == UPDATE_PRESET_INDEX {
        return common::WARN;
    }
    palette.surface
}

pub(super) fn progress_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.screen_state.has_settings_override() {
        return m::PX_230;
    }
    match scenario.preset_index {
        SESSION_PRESET_INDEX => m::PX_104,
        UPDATE_PRESET_INDEX => m::PX_230,
        ERROR_PRESET_INDEX => m::PX_278,
        _ => m::PX_52,
    }
}

pub(super) fn action_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action() || scenario.preset_index == ERROR_PRESET_INDEX {
        return palette.accent;
    }
    palette.panel
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        SESSION_PRESET_INDEX => "session init",
        UPDATE_PRESET_INDEX => "progress=64",
        ERROR_PRESET_INDEX => "retry=true",
        _ => "idle=v0.1.0",
    }
}

pub(super) fn headline_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.has_settings_override() {
        return "Workspace failed";
    }
    match scenario.preset_index {
        SESSION_PRESET_INDEX => "Preparing session",
        UPDATE_PRESET_INDEX => "Installing update",
        ERROR_PRESET_INDEX => "Workspace failed",
        _ => "Ready to start",
    }
}
