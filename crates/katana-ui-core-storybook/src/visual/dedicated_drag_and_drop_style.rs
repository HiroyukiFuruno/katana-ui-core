use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const FILE_PRESET_INDEX: usize = 1;
const TAB_PRESET_INDEX: usize = 2;
const ATTACHMENT_PRESET_INDEX: usize = 3;
const KEYBOARD_PRESET_INDEX: usize = 4;

pub(super) fn source_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.drag_and_drop.is_dragging()
        || scenario.preset_index == KEYBOARD_PRESET_INDEX
    {
        return palette.accent;
    }
    palette.surface
}

pub(super) fn target_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override()
        || scenario.preset_index == FILE_PRESET_INDEX
        || scenario.preset_index == ATTACHMENT_PRESET_INDEX
    {
        return common::SUCCESS;
    }
    palette.panel
}

pub(super) fn indicator_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.drag_and_drop.committed() || scenario.preset_index == TAB_PRESET_INDEX
    {
        return palette.accent;
    }
    common::WARN
}

pub(super) fn rail_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == KEYBOARD_PRESET_INDEX {
        return common::PURPLE;
    }
    palette.surface
}

pub(super) fn payload_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        FILE_PRESET_INDEX => "3 files",
        TAB_PRESET_INDEX => "tab:settings",
        ATTACHMENT_PRESET_INDEX => "image.png",
        KEYBOARD_PRESET_INDEX => "Space+Arrow",
        _ => "item-02",
    }
}

pub(super) fn target_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.has_settings_override() {
        return "accept after";
    }
    match scenario.preset_index {
        FILE_PRESET_INDEX => "imports",
        TAB_PRESET_INDEX => "before tab",
        ATTACHMENT_PRESET_INDEX => "composer",
        KEYBOARD_PRESET_INDEX => "cancel target",
        _ => "after item-04",
    }
}

pub(super) fn mode_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == KEYBOARD_PRESET_INDEX {
        return "keyboard";
    }
    "pointer"
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        FILE_PRESET_INDEX => "copy=file",
        TAB_PRESET_INDEX => "indicator=before",
        ATTACHMENT_PRESET_INDEX => "inside drop",
        KEYBOARD_PRESET_INDEX => "cancelable",
        _ => "indicator=after",
    }
}
