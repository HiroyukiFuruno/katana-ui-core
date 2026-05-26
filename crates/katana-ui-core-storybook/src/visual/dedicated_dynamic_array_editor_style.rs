use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const ADD_REMOVE_PRESET_INDEX: usize = 1;
const REORDER_PRESET_INDEX: usize = 2;
const THEME_PRESET_INDEX: usize = 3;
const ACTIVE_ROW_INDEX: usize = 1;

pub(super) fn row_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>, row: usize) -> u32 {
    if scenario.screen_state.has_widget_action()
        || scenario.screen_state.has_settings_override()
        || scenario.preset_index == THEME_PRESET_INDEX && row == ACTIVE_ROW_INDEX
    {
        return palette.accent;
    }
    palette.surface
}

pub(super) fn control_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == ADD_REMOVE_PRESET_INDEX {
        return common::SUCCESS;
    }
    palette.panel
}

pub(super) fn reorder_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == REORDER_PRESET_INDEX {
        return common::WARN;
    }
    palette.border
}

pub(super) fn row_label(scenario: ScenarioContext<'_>, row: usize) -> &'static str {
    if scenario.screen_state.has_settings_override() {
        return "array item updated";
    }
    match row {
        0 => "Item 1",
        1 => active_label(scenario),
        _ => "Item 3",
    }
}

pub(super) fn status_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        ADD_REMOVE_PRESET_INDEX => "add/remove",
        REORDER_PRESET_INDEX => "reorder=true",
        THEME_PRESET_INDEX => "theme=accent",
        _ => "rows=3",
    }
}

fn active_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        ADD_REMOVE_PRESET_INDEX => "New item",
        REORDER_PRESET_INDEX => "Drag item",
        THEME_PRESET_INDEX => "Accent row",
        _ => "Item 2",
    }
}
