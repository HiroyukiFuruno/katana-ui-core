use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const SELECT_PRESET_INDEX: usize = 1;
const COLLAPSE_PRESET_INDEX: usize = 2;
const THEME_PRESET_INDEX: usize = 3;
const FIRST_ROW_INDEX: usize = 0;
const SECOND_ROW_INDEX: usize = 1;

pub(super) fn active_row_index(scenario: ScenarioContext<'_>) -> usize {
    if let Some(index) = scenario.screen_state.side_menu.selected_index {
        return index.min(SECOND_ROW_INDEX);
    }
    if scenario.screen_state.has_widget_action() || scenario.preset_index == SELECT_PRESET_INDEX {
        return SECOND_ROW_INDEX;
    }
    FIRST_ROW_INDEX
}

pub(super) fn row_fill(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    active: usize,
    index: usize,
) -> u32 {
    if scenario.screen_state.has_settings_override() {
        return common::WARN;
    }
    if scenario.screen_state.side_menu.hovered && index == FIRST_ROW_INDEX {
        return common::TOKEN;
    }
    if scenario.screen_state.side_menu.focus_index == Some(index) {
        return common::SUCCESS;
    }
    if active == index {
        return palette.accent;
    }
    palette.panel
}

pub(super) fn collapse_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.side_menu.hover_expansion {
        return common::TOKEN;
    }
    if scenario.preset_index == COLLAPSE_PRESET_INDEX {
        return common::PURPLE;
    }
    palette.panel
}

pub(super) fn theme_line_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.side_menu.scroll_offset > 0 {
        return common::WARN;
    }
    if scenario.preset_index == THEME_PRESET_INDEX {
        return common::TOKEN;
    }
    palette.accent
}

pub(super) fn row_text(palette: &VisualPalette, active: usize, index: usize) -> u32 {
    if active == index {
        return palette.background;
    }
    palette.text
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return "route=0";
    }
    scenario.screen_state.state_label
}
