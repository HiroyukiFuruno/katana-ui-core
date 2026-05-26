use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const WINDOWS_PRESET_INDEX: usize = 1;
const LINUX_PRESET_INDEX: usize = 2;
const SEPARATOR_PRESET_INDEX: usize = 3;
const A11Y_PRESET_INDEX: usize = 4;

pub(super) fn key_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action()
        || scenario.screen_state.has_settings_override()
        || scenario.preset_index == A11Y_PRESET_INDEX
    {
        return palette.accent;
    }
    palette.surface
}

pub(super) fn separator_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == SEPARATOR_PRESET_INDEX {
        return common::WARN;
    }
    palette.panel
}

pub(super) fn platform_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.has_settings_override() {
        return "MacOS";
    }
    match scenario.preset_index {
        WINDOWS_PRESET_INDEX => "Windows",
        LINUX_PRESET_INDEX => "Linux",
        _ => "macOS",
    }
}

pub(super) fn modifier_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        WINDOWS_PRESET_INDEX => "Ctrl",
        LINUX_PRESET_INDEX => "Super",
        _ => "Cmd",
    }
}

pub(super) fn separator_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == SEPARATOR_PRESET_INDEX {
        return "none";
    }
    "+"
}

pub(super) fn status_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        WINDOWS_PRESET_INDEX => "platform=windows",
        LINUX_PRESET_INDEX => "platform=linux",
        SEPARATOR_PRESET_INDEX => "separator=none",
        A11Y_PRESET_INDEX => "a11y=label",
        _ => "combo=Command+K",
    }
}
