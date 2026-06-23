use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const SELECTION_PRESET_INDEX: usize = 1;
const EMPTY_PRESET_INDEX: usize = 2;
const THEME_PRESET_INDEX: usize = 3;
const VIRTUALIZATION_PRESET_INDEX: usize = 4;
const FIRST_ROW_INDEX: usize = 0;
const SECOND_ROW_INDEX: usize = 1;
const THIRD_ROW_INDEX: usize = 2;
const ROW_COUNT_LABEL: &str = "rows=3";
const EMPTY_LABEL: &str = "empty";
const VIRTUALIZED_LABEL: &str = "virtual=48/200";

pub(super) fn active_row_index(scenario: ScenarioContext<'_>) -> usize {
    if let Some(index) = scenario.screen_state.list.selected_index {
        return index.min(THIRD_ROW_INDEX);
    }
    if let Some(index) = scenario.screen_state.list.focused_index {
        return index.min(THIRD_ROW_INDEX);
    }
    if scenario.preset_index == SELECTION_PRESET_INDEX {
        return SECOND_ROW_INDEX;
    }
    if scenario.screen_state.list.scrolled || scenario.preset_index == VIRTUALIZATION_PRESET_INDEX {
        return THIRD_ROW_INDEX;
    }
    FIRST_ROW_INDEX
}

pub(super) fn list_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.preview_hovered {
        return palette.panel;
    }
    if scenario.screen_state.is_button_focused() {
        return common::TOKEN;
    }
    if scenario.screen_state.has_settings_override() {
        return common::WARN;
    }
    palette.surface
}

pub(super) fn row_fill(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    index: usize,
) -> u32 {
    if scenario.preset_index == EMPTY_PRESET_INDEX {
        return palette.surface;
    }
    if active_row_index(scenario) == index {
        return palette.accent;
    }
    palette.panel
}

pub(super) fn scrollbar_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.list.scrolled || scenario.preset_index == VIRTUALIZATION_PRESET_INDEX {
        return common::TOKEN;
    }
    palette.border
}

pub(super) fn theme_line_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.preview_hovered {
        return common::WARN;
    }
    if scenario.preset_index == THEME_PRESET_INDEX {
        return common::PURPLE;
    }
    palette.accent
}

pub(super) fn row_text(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    index: usize,
) -> u32 {
    if scenario.preset_index == EMPTY_PRESET_INDEX {
        return palette.muted;
    }
    if active_row_index(scenario) == index {
        return palette.background;
    }
    palette.text
}

pub(super) fn row_label(scenario: ScenarioContext<'_>, index: usize) -> &'static str {
    if scenario.preset_index == EMPTY_PRESET_INDEX {
        return EMPTY_LABEL;
    }
    match index {
        FIRST_ROW_INDEX => "Row 1",
        SECOND_ROW_INDEX => "Row 2",
        _ => "Row 3",
    }
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        EMPTY_PRESET_INDEX => EMPTY_LABEL,
        VIRTUALIZATION_PRESET_INDEX => VIRTUALIZED_LABEL,
        _ => ROW_COUNT_LABEL,
    }
}
