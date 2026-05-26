use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const CHAT_HISTORY_PRESET_INDEX: usize = 1;
const TOC_PRESET_INDEX: usize = 2;
const FLOATING_PRESET_INDEX: usize = 3;
const ICON_ONLY_PRESET_INDEX: usize = 4;
const EXPLORER_ROW_INDEX: usize = 0;
const CHAT_ROW_INDEX: usize = 1;
const TOC_ROW_INDEX: usize = 2;
const SETTINGS_PANEL_WIDTH: usize = 320;
const CHAT_HISTORY_PANEL_WIDTH: usize = 302;
const TOC_PANEL_WIDTH: usize = 206;
const FLOATING_PANEL_WIDTH: usize = 256;
const ICON_ONLY_PANEL_WIDTH: usize = 78;
const EXPLORER_PANEL_WIDTH: usize = 240;

pub(super) fn active_row_index(scenario: ScenarioContext<'_>) -> usize {
    if scenario.screen_state.has_widget_action() {
        return CHAT_ROW_INDEX;
    }
    match scenario.preset_index {
        CHAT_HISTORY_PRESET_INDEX | FLOATING_PRESET_INDEX => CHAT_ROW_INDEX,
        TOC_PRESET_INDEX => TOC_ROW_INDEX,
        _ => EXPLORER_ROW_INDEX,
    }
}

pub(super) fn panel_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.screen_state.has_settings_override() {
        return SETTINGS_PANEL_WIDTH;
    }
    match scenario.preset_index {
        CHAT_HISTORY_PRESET_INDEX => CHAT_HISTORY_PANEL_WIDTH,
        TOC_PRESET_INDEX => TOC_PANEL_WIDTH,
        FLOATING_PRESET_INDEX => FLOATING_PANEL_WIDTH,
        ICON_ONLY_PRESET_INDEX => ICON_ONLY_PANEL_WIDTH,
        _ => EXPLORER_PANEL_WIDTH,
    }
}

pub(super) fn panel_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() {
        return common::WARN;
    }
    palette.surface
}

pub(super) fn rail_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == ICON_ONLY_PRESET_INDEX {
        return palette.accent;
    }
    palette.panel
}

pub(super) fn row_fill(palette: &VisualPalette, active: usize, index: usize) -> u32 {
    if active == index {
        return palette.accent;
    }
    palette.panel
}

pub(super) fn handle_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action() {
        return palette.accent;
    }
    if scenario.preset_index == FLOATING_PRESET_INDEX {
        return common::TOKEN;
    }
    palette.border
}

pub(super) fn pin_fill(scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == FLOATING_PRESET_INDEX {
        return common::TOKEN;
    }
    if scenario.preset_index == ICON_ONLY_PRESET_INDEX {
        return common::PURPLE;
    }
    common::SUCCESS
}

pub(super) fn row_text(palette: &VisualPalette, active: usize, index: usize) -> u32 {
    if active == index {
        return palette.background;
    }
    palette.text
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        FLOATING_PRESET_INDEX => "mode=floating_overlay",
        ICON_ONLY_PRESET_INDEX => "mode=icon_only",
        TOC_PRESET_INDEX => "width=206",
        CHAT_HISTORY_PRESET_INDEX => "width=302",
        _ => "width=240",
    }
}
