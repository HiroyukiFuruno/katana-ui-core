use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const SWITCH_PRESET_INDEX: usize = 1;
const OVERFLOW_PRESET_INDEX: usize = 2;
const THEME_PRESET_INDEX: usize = 3;
const FIRST_TAB_INDEX: usize = 0;
const SECOND_TAB_INDEX: usize = 1;

pub(super) fn active_index(scenario: ScenarioContext<'_>) -> usize {
    if scenario.screen_state.has_widget_action() || scenario.preset_index == SWITCH_PRESET_INDEX {
        return SECOND_TAB_INDEX;
    }
    FIRST_TAB_INDEX
}

pub(super) fn panel_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() {
        return common::WARN;
    }
    if scenario.screen_state.has_widget_action() {
        return palette.accent;
    }
    palette.surface
}

pub(super) fn tab_fill(palette: &VisualPalette, active: usize, index: usize) -> u32 {
    if active == index {
        return palette.surface;
    }
    palette.panel
}

pub(super) fn overflow_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == OVERFLOW_PRESET_INDEX {
        return palette.accent;
    }
    palette.panel
}

pub(super) fn theme_line_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == THEME_PRESET_INDEX {
        return common::TOKEN;
    }
    palette.accent
}

pub(super) fn tab_text(palette: &VisualPalette, active: usize, index: usize) -> u32 {
    if active == index {
        return palette.text;
    }
    palette.muted
}

pub(super) fn panel_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.has_widget_action() || scenario.preset_index == SWITCH_PRESET_INDEX {
        return "Output panel selected";
    }
    if scenario.preset_index == OVERFLOW_PRESET_INDEX {
        return "Overflow menu exposes hidden tab";
    }
    if scenario.preset_index == THEME_PRESET_INDEX {
        return "Theme line follows accent token";
    }
    "Preview panel selected"
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return "tab=0";
    }
    scenario.screen_state.state_label
}
