use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const RESULTS_PRESET_INDEX: usize = 1;
const SLASH_PRESET_INDEX: usize = 2;
const DISABLED_PRESET_INDEX: usize = 3;
const VIRTUAL_PRESET_INDEX: usize = 4;
const VIRTUALIZATION_PRESET_INDEX: usize = 5;
const ROW_HIGHLIGHT_INDEX: usize = 0;
const ROW_DISABLED_INDEX: usize = 2;

pub(super) fn search_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.last_action == "command_palette_keyboard_close" {
        return common::DANGER;
    }
    if scenario.screen_state.has_settings_override() {
        return common::TOKEN;
    }
    palette.surface
}

pub(super) fn row_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>, row: usize) -> u32 {
    if scenario.screen_state.last_action == "command_palette_keyboard_execute" && row == 1 {
        return common::SUCCESS;
    }
    if scenario.screen_state.last_action == "command_palette_keyboard_close"
        && row == ROW_HIGHLIGHT_INDEX
    {
        return common::DANGER;
    }
    if row == ROW_HIGHLIGHT_INDEX
        && (scenario.screen_state.has_widget_action()
            || scenario.screen_state.has_settings_override()
            || scenario.preset_index == RESULTS_PRESET_INDEX)
    {
        return palette.accent;
    }
    if row == ROW_DISABLED_INDEX && scenario.preset_index == DISABLED_PRESET_INDEX {
        return common::DANGER;
    }
    palette.surface
}

pub(super) fn shortcut_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if is_virtual_preset(scenario) {
        return common::WARN;
    }
    palette.panel
}

pub(super) fn query_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.has_settings_override() {
        return "theme";
    }
    match scenario.preset_index {
        SLASH_PRESET_INDEX => "/format",
        VIRTUAL_PRESET_INDEX | VIRTUALIZATION_PRESET_INDEX => "open 50",
        _ => "open",
    }
}

pub(super) fn row_label(scenario: ScenarioContext<'_>, row: usize) -> &'static str {
    if scenario.preset_index == SLASH_PRESET_INDEX {
        return slash_row_label(row);
    }
    match row {
        ROW_HIGHLIGHT_INDEX => "Open File",
        1 => "Format Document",
        _ => "Locked command",
    }
}

pub(super) fn status_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        RESULTS_PRESET_INDEX => "highlighted=0",
        SLASH_PRESET_INDEX => "launcher=slash",
        DISABLED_PRESET_INDEX => "disabled=readonly",
        VIRTUAL_PRESET_INDEX => "rows=50",
        VIRTUALIZATION_PRESET_INDEX => "virtual window",
        _ => "open=true",
    }
}

fn is_virtual_preset(scenario: ScenarioContext<'_>) -> bool {
    matches!(
        scenario.preset_index,
        VIRTUAL_PRESET_INDEX | VIRTUALIZATION_PRESET_INDEX
    )
}

fn slash_row_label(row: usize) -> &'static str {
    match row {
        ROW_HIGHLIGHT_INDEX => "/open",
        1 => "/format",
        _ => "/theme",
    }
}
