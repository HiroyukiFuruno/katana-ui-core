use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const OVERFLOW_PRESET_INDEX: usize = 0;
const SPLIT_PRESET_INDEX: usize = 1;
const DISPLAY_PRESET_INDEX: usize = 2;
const DENSITY_PRESET_INDEX: usize = 3;
const ACCELERATOR_PRESET_INDEX: usize = 4;

pub(super) fn bar_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() {
        return common::WARN;
    }
    if scenario.screen_state.has_widget_action() {
        return palette.accent;
    }
    palette.surface
}

pub(super) fn action_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == DISPLAY_PRESET_INDEX || scenario.screen_state.has_widget_action() {
        return palette.accent;
    }
    palette.panel
}

pub(super) fn split_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == SPLIT_PRESET_INDEX {
        return palette.accent;
    }
    palette.panel
}

pub(super) fn more_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == OVERFLOW_PRESET_INDEX {
        return common::TOKEN;
    }
    palette.panel
}

pub(super) fn accelerator_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == ACCELERATOR_PRESET_INDEX {
        return palette.accent;
    }
    palette.panel
}

pub(super) fn density_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == DENSITY_PRESET_INDEX {
        return common::PURPLE;
    }
    palette.panel
}

pub(super) fn action_text(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == DISPLAY_PRESET_INDEX || scenario.screen_state.has_widget_action() {
        return palette.background;
    }
    palette.text
}

pub(super) fn accelerator_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == ACCELERATOR_PRESET_INDEX {
        return "Cmd+F";
    }
    "Cmd+S"
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return "overflow=menu split=false";
    }
    scenario.screen_state.state_label
}
