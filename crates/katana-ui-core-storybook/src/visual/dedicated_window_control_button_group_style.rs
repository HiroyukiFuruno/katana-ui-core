use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const WINDOWS_PRESET_INDEX: usize = 1;
const LINUX_PRESET_INDEX: usize = 2;
const FULLSCREEN_PRESET_INDEX: usize = 3;
const CLOSE_ONLY_PRESET_INDEX: usize = 4;
const LEADING_BUTTON_OFFSET: usize = 18;
const TRAILING_BUTTON_OFFSET: usize = 344;
const COMPACT_BUTTON_SIZE: usize = 14;
const TALL_BUTTON_SIZE: usize = 18;

pub(super) fn chrome_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == FULLSCREEN_PRESET_INDEX {
        return common::PURPLE;
    }
    palette.panel
}

pub(super) fn close_fill(scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action()
        || scenario.screen_state.has_settings_override()
        || scenario.preset_index == CLOSE_ONLY_PRESET_INDEX
    {
        return common::DANGER;
    }
    common::WARN
}

pub(super) fn minimize_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == CLOSE_ONLY_PRESET_INDEX {
        return palette.panel;
    }
    common::WARN
}

pub(super) fn maximize_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == CLOSE_ONLY_PRESET_INDEX {
        return palette.panel;
    }
    common::SUCCESS
}

pub(super) fn leading_offset(scenario: ScenarioContext<'_>) -> usize {
    if scenario.screen_state.has_settings_override()
        || scenario.preset_index == WINDOWS_PRESET_INDEX
        || scenario.preset_index == CLOSE_ONLY_PRESET_INDEX
    {
        return TRAILING_BUTTON_OFFSET;
    }
    LEADING_BUTTON_OFFSET
}

pub(super) fn button_size(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == LINUX_PRESET_INDEX {
        return TALL_BUTTON_SIZE;
    }
    COMPACT_BUTTON_SIZE
}

pub(super) fn platform_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        WINDOWS_PRESET_INDEX => "Windows trailing",
        LINUX_PRESET_INDEX => "Linux hover",
        FULLSCREEN_PRESET_INDEX => "fullscreen hover",
        CLOSE_ONLY_PRESET_INDEX => "close only",
        _ => "macOS leading",
    }
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        WINDOWS_PRESET_INDEX => "pressed=Maximize",
        LINUX_PRESET_INDEX => "visibility=Hover",
        FULLSCREEN_PRESET_INDEX => "fullscreen=true",
        CLOSE_ONLY_PRESET_INDEX => "pressed=Close",
        _ => "pressed=Close",
    }
}
