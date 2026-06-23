use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const CLICK_PRESET_INDEX: usize = 1;
const OVERFLOW_PRESET_INDEX: usize = 2;
const THEME_PRESET_INDEX: usize = 3;
const FILE_CRUMB_INDEX: usize = 2;

pub(super) fn active_index(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == CLICK_PRESET_INDEX {
        return FILE_CRUMB_INDEX;
    }
    scenario.screen_state.breadcrumb_selected_index
}

pub(super) fn bar_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() {
        return common::WARN;
    }
    if scenario.screen_state.has_widget_action() {
        return palette.accent;
    }
    palette.surface
}

pub(super) fn crumb_fill(palette: &VisualPalette, active: usize, index: usize) -> u32 {
    if active == index {
        return palette.surface;
    }
    palette.panel
}

pub(super) fn overflow_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == OVERFLOW_PRESET_INDEX {
        return palette.accent;
    }
    palette.panel
}

pub(super) fn theme_line_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == THEME_PRESET_INDEX {
        return common::TOKEN;
    }
    palette.accent
}

pub(super) fn crumb_text(palette: &VisualPalette, active: usize, index: usize) -> u32 {
    if active == index {
        return palette.text;
    }
    palette.muted
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return breadcrumb_state_label(scenario.screen_state.breadcrumb_selected_index);
    }
    scenario.screen_state.state_label
}

fn breadcrumb_state_label(index: usize) -> &'static str {
    match index {
        0 => "route=0",
        1 => "route=1",
        _ => "route=2",
    }
}
