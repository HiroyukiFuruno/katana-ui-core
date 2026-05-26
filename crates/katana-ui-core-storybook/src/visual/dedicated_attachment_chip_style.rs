use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const IMAGE_PRESET_INDEX: usize = 1;
const URL_PRESET_INDEX: usize = 2;
const UPLOADING_PRESET_INDEX: usize = 3;
const ERROR_PRESET_INDEX: usize = 4;
const PROGRESS_UPLOAD_WIDTH: usize = 70;
const PROGRESS_COMPLETE_WIDTH: usize = 118;
const PROGRESS_ERROR_WIDTH: usize = 44;

pub(super) fn attachment_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action() || scenario.screen_state.has_settings_override() {
        return palette.surface;
    }
    palette.surface
}

pub(super) fn kind_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    match scenario.preset_index {
        IMAGE_PRESET_INDEX => common::PURPLE,
        URL_PRESET_INDEX => common::TOKEN,
        _ => palette.accent,
    }
}

pub(super) fn progress_fill(scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action() || scenario.screen_state.has_settings_override() {
        return common::DANGER;
    }
    match scenario.preset_index {
        ERROR_PRESET_INDEX => common::DANGER,
        UPLOADING_PRESET_INDEX => common::WARN,
        _ => common::SUCCESS,
    }
}

pub(super) fn progress_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.screen_state.has_widget_action() || scenario.screen_state.has_settings_override() {
        return PROGRESS_ERROR_WIDTH;
    }
    match scenario.preset_index {
        UPLOADING_PRESET_INDEX => PROGRESS_UPLOAD_WIDTH,
        ERROR_PRESET_INDEX => PROGRESS_ERROR_WIDTH,
        _ => PROGRESS_COMPLETE_WIDTH,
    }
}

pub(super) fn retry_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action()
        || scenario.screen_state.has_settings_override()
        || scenario.preset_index == ERROR_PRESET_INDEX
    {
        return common::DANGER;
    }
    palette.panel
}

pub(super) fn status_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        IMAGE_PRESET_INDEX => "kind=image",
        URL_PRESET_INDEX => "kind=url",
        UPLOADING_PRESET_INDEX => "status=uploading",
        ERROR_PRESET_INDEX => "status=error",
        _ => "kind=file",
    }
}

pub(super) fn file_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        IMAGE_PRESET_INDEX => "screenshot.png",
        URL_PRESET_INDEX => "https://katana",
        _ => "design.md",
    }
}

pub(super) fn retry_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.has_widget_action()
        || scenario.screen_state.has_settings_override()
        || scenario.preset_index == ERROR_PRESET_INDEX
    {
        return "retry";
    }
    "open"
}
