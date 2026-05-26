use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const CATEGORY_PRESET_INDEX: usize = 1;
const TWO_COLUMN_PRESET_INDEX: usize = 2;
const ONE_COLUMN_PRESET_INDEX: usize = 3;
const SELECT_PRESET_INDEX: usize = 4;

pub(super) fn filter_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override()
        || scenario.preset_index == CATEGORY_PRESET_INDEX
    {
        return common::WARN;
    }
    palette.surface
}

pub(super) fn primary_row_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action() || scenario.preset_index == SELECT_PRESET_INDEX {
        return palette.accent;
    }
    palette.surface
}

pub(super) fn secondary_panel_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    match scenario.preset_index {
        TWO_COLUMN_PRESET_INDEX => palette.accent,
        ONE_COLUMN_PRESET_INDEX => palette.background,
        _ => palette.panel,
    }
}

pub(super) fn layout_rail_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == ONE_COLUMN_PRESET_INDEX {
        return common::WARN;
    }
    palette.accent
}

pub(super) fn filter_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.has_settings_override()
        || scenario.preset_index == CATEGORY_PRESET_INDEX
    {
        return "カテゴリ";
    }
    "format"
}

pub(super) fn layout_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.has_settings_override()
        || scenario.preset_index == ONE_COLUMN_PRESET_INDEX
    {
        return "one column";
    }
    "two column"
}

pub(super) fn result_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == CATEGORY_PRESET_INDEX {
        return "1 category";
    }
    if scenario.preset_index == ONE_COLUMN_PRESET_INDEX {
        return "stacked results";
    }
    "2 commands"
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        CATEGORY_PRESET_INDEX => "query=カテゴリ",
        TWO_COLUMN_PRESET_INDEX => "layout=two",
        ONE_COLUMN_PRESET_INDEX => "layout=one",
        SELECT_PRESET_INDEX => "selected=format",
        _ => "query=format",
    }
}
