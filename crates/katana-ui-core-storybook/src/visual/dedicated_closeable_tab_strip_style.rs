use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const OVERFLOW_PRESET_INDEX: usize = 1;
const PINNED_PRESET_INDEX: usize = 2;
const GROUPS_PRESET_INDEX: usize = 3;
const DIRTY_PRESET_INDEX: usize = 4;
const DRAGGING_PRESET_INDEX: usize = 5;
const DEFAULT_TAB_INDEX: usize = 0;
const PINNED_TAB_INDEX: usize = 1;
const GROUPS_TAB_INDEX: usize = 2;
const DIRTY_TAB_INDEX: usize = 3;
const DRAGGING_TAB_INDEX: usize = 4;

pub(super) fn active_index(scenario: ScenarioContext<'_>) -> usize {
    if scenario.screen_state.state_label == "tabs.active=settings" {
        return DIRTY_TAB_INDEX;
    }
    if scenario.screen_state.has_widget_action() {
        return DIRTY_TAB_INDEX;
    }
    match scenario.preset_index {
        PINNED_PRESET_INDEX => PINNED_TAB_INDEX,
        GROUPS_PRESET_INDEX => GROUPS_TAB_INDEX,
        DIRTY_PRESET_INDEX => DIRTY_TAB_INDEX,
        DRAGGING_PRESET_INDEX => DRAGGING_TAB_INDEX,
        _ => DEFAULT_TAB_INDEX,
    }
}

pub(super) fn strip_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() {
        return common::WARN;
    }
    if scenario.preset_index == OVERFLOW_PRESET_INDEX {
        return common::TOKEN;
    }
    palette.surface
}

pub(super) fn tab_fill(palette: &VisualPalette, active: usize, index: usize) -> u32 {
    if active == index {
        return palette.accent;
    }
    palette.panel
}

pub(super) fn overflow_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == OVERFLOW_PRESET_INDEX {
        return common::TOKEN;
    }
    palette.panel
}

pub(super) fn dirty_fill(scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == DIRTY_PRESET_INDEX {
        return common::DANGER;
    }
    common::SUCCESS
}

pub(super) fn theme_line_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == DRAGGING_PRESET_INDEX {
        return common::PURPLE;
    }
    palette.accent
}

pub(super) fn tab_text(palette: &VisualPalette, active: usize, index: usize) -> u32 {
    if active == index {
        return palette.background;
    }
    palette.text
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return "active=inbox tabs=6";
    }
    scenario.screen_state.state_label
}
