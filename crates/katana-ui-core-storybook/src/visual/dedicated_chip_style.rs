use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const DISMISS_PRESET_INDEX: usize = 1;
const SELECTED_PRESET_INDEX: usize = 2;
const TONE_MATRIX_PRESET_INDEX: usize = 3;
const PRIMARY_TONE_INDEX: usize = 0;
const WARNING_TONE_INDEX: usize = 1;

pub(super) fn chip_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action() {
        return common::DANGER;
    }
    if scenario.screen_state.has_settings_override() {
        return common::WARN;
    }
    if scenario.preset_index == DISMISS_PRESET_INDEX {
        return palette.accent;
    }
    if scenario.preset_index == SELECTED_PRESET_INDEX {
        return palette.selection;
    }
    palette.surface
}

pub(super) fn focus_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == SELECTED_PRESET_INDEX {
        return palette.accent;
    }
    if scenario.screen_state.has_widget_action() || scenario.screen_state.has_settings_override() {
        return common::TOKEN;
    }
    palette.border
}

pub(super) fn dismiss_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action() || scenario.preset_index == DISMISS_PRESET_INDEX {
        return common::DANGER;
    }
    palette.panel
}

pub(super) fn tone_fill(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    index: usize,
) -> u32 {
    if scenario.preset_index != TONE_MATRIX_PRESET_INDEX {
        return palette.panel;
    }
    match index {
        PRIMARY_TONE_INDEX => common::SUCCESS,
        WARNING_TONE_INDEX => common::WARN,
        _ => common::PURPLE,
    }
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        DISMISS_PRESET_INDEX => "dismiss=ready",
        SELECTED_PRESET_INDEX => "selected=true",
        TONE_MATRIX_PRESET_INDEX => "tone=matrix",
        _ => "filter=active",
    }
}

pub(super) fn dismiss_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.has_widget_action() {
        return "dismissed";
    }
    if scenario.preset_index == DISMISS_PRESET_INDEX {
        return "backspace";
    }
    "dismiss"
}
