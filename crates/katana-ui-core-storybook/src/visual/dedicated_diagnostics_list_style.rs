use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const EDITOR_PRESET_INDEX: usize = 1;
const TOOL_PRESET_INDEX: usize = 2;
const EMPTY_PRESET_INDEX: usize = 3;
const LOADING_PRESET_INDEX: usize = 4;
const BULK_PRESET_INDEX: usize = 5;
const VIRTUAL_PRESET_INDEX: usize = 6;
const ROW_ERROR_INDEX: usize = 0;
const ROW_WARNING_INDEX: usize = 1;
const ROW_TOOL_INDEX: usize = 2;
const DEFAULT_RANGE_WIDTH: usize = 64;
const VIRTUAL_RANGE_WIDTH: usize = 132;

pub(super) fn row_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>, row: usize) -> u32 {
    if scenario.preset_index == EMPTY_PRESET_INDEX || scenario.preset_index == LOADING_PRESET_INDEX
    {
        return palette.panel;
    }
    if scenario.screen_state.diagnostics_list.focused && row == ROW_ERROR_INDEX {
        return palette.accent;
    }
    if scenario.screen_state.diagnostics_list.hovered && row == ROW_ERROR_INDEX {
        return common::TOKEN;
    }
    if scenario.screen_state.diagnostics_list.selected_item() && row == ROW_ERROR_INDEX {
        return common::SUCCESS;
    }
    if scenario.screen_state.has_settings_override() && row == ROW_ERROR_INDEX {
        return common::DANGER;
    }
    if scenario.screen_state.has_widget_action() && row == ROW_WARNING_INDEX {
        return common::SUCCESS;
    }
    palette.surface
}

pub(super) fn severity_fill(scenario: ScenarioContext<'_>, row: usize) -> u32 {
    if scenario.preset_index == TOOL_PRESET_INDEX && row == ROW_TOOL_INDEX {
        return common::TOKEN;
    }
    match row {
        ROW_ERROR_INDEX => common::DANGER,
        ROW_WARNING_INDEX => common::WARN,
        _ => common::SUCCESS,
    }
}

pub(super) fn preview_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action()
        || scenario.screen_state.has_settings_override()
        || scenario.preset_index == BULK_PRESET_INDEX
    {
        return palette.accent;
    }
    palette.panel
}

pub(super) fn range_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.screen_state.diagnostics_list.scroll_retained() {
        return VIRTUAL_RANGE_WIDTH;
    }
    if scenario.preset_index == VIRTUAL_PRESET_INDEX {
        return VIRTUAL_RANGE_WIDTH;
    }
    DEFAULT_RANGE_WIDTH
}

pub(super) fn header_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.diagnostics_list.keyboard_navigated() {
        return "jump requested";
    }
    if scenario.screen_state.diagnostics_list.focused {
        return "focus selected";
    }
    if scenario.screen_state.diagnostics_list.hovered {
        return "hover row";
    }
    if scenario.screen_state.has_settings_override() {
        return "group=source";
    }
    match scenario.preset_index {
        EDITOR_PRESET_INDEX => "editor inline",
        TOOL_PRESET_INDEX => "tool result",
        EMPTY_PRESET_INDEX => "empty",
        LOADING_PRESET_INDEX => "loading",
        BULK_PRESET_INDEX => "bulk fix",
        VIRTUAL_PRESET_INDEX => "virtual range",
        _ => "lint result",
    }
}

pub(super) fn row_label(scenario: ScenarioContext<'_>, row: usize) -> &'static str {
    if scenario.preset_index == EMPTY_PRESET_INDEX {
        return "No diagnostics";
    }
    if scenario.preset_index == LOADING_PRESET_INDEX {
        return "Loading diagnostics";
    }
    match row {
        ROW_ERROR_INDEX => "E Missing semicolon",
        ROW_WARNING_INDEX => "W Unused import",
        _ => "T cargo fmt",
    }
}

pub(super) fn status_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        EMPTY_PRESET_INDEX => "items=0",
        LOADING_PRESET_INDEX => "loading=true",
        BULK_PRESET_INDEX => "bulk=open",
        VIRTUAL_PRESET_INDEX => "range=8..18",
        _ => "preview=false",
    }
}
