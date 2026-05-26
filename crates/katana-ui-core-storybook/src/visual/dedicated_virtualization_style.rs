use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const VARIABLE_ROWS_PRESET_INDEX: usize = 1;
const FOCUSED_SENTINEL_PRESET_INDEX: usize = 2;
const MEASURED_CORRECTION_PRESET_INDEX: usize = 3;
const FIRST_ROW_INDEX: usize = 0;
const SECOND_ROW_INDEX: usize = 1;
const THIRD_ROW_INDEX: usize = 2;
const FIXED_LABEL: &str = "fixed rows";
const VARIABLE_LABEL: &str = "variable rows";
const FOCUSED_LABEL: &str = "focused=42";
const MEASURED_LABEL: &str = "measured=+8";

pub(super) fn viewport_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() {
        return common::WARN;
    }
    palette.surface
}

pub(super) fn active_row_index(scenario: ScenarioContext<'_>) -> usize {
    if scenario.screen_state.has_widget_action()
        || scenario.preset_index == FOCUSED_SENTINEL_PRESET_INDEX
    {
        return SECOND_ROW_INDEX;
    }
    if scenario.preset_index == MEASURED_CORRECTION_PRESET_INDEX {
        return THIRD_ROW_INDEX;
    }
    FIRST_ROW_INDEX
}

pub(super) fn row_fill(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    index: usize,
) -> u32 {
    if active_row_index(scenario) == index {
        return palette.accent;
    }
    if scenario.preset_index == VARIABLE_ROWS_PRESET_INDEX && index == THIRD_ROW_INDEX {
        return common::PURPLE;
    }
    palette.panel
}

pub(super) fn correction_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == MEASURED_CORRECTION_PRESET_INDEX {
        return common::TOKEN;
    }
    palette.border
}

pub(super) fn row_text(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    index: usize,
) -> u32 {
    if active_row_index(scenario) == index {
        return palette.background;
    }
    palette.text
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        VARIABLE_ROWS_PRESET_INDEX => VARIABLE_LABEL,
        FOCUSED_SENTINEL_PRESET_INDEX => FOCUSED_LABEL,
        MEASURED_CORRECTION_PRESET_INDEX => MEASURED_LABEL,
        _ => FIXED_LABEL,
    }
}
