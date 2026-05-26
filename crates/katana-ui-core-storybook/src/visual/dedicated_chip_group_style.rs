use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const OVERFLOW_PRESET_INDEX: usize = 1;
const SCROLL_PRESET_INDEX: usize = 2;
const REORDER_PRESET_INDEX: usize = 3;
const FIRST_CHIP_INDEX: usize = 0;
const SECOND_CHIP_INDEX: usize = 1;

pub(super) fn group_fill(palette: &VisualPalette) -> u32 {
    palette.surface
}

pub(super) fn chip_fill(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    index: usize,
) -> u32 {
    if scenario.preset_index == REORDER_PRESET_INDEX && index == SECOND_CHIP_INDEX {
        return common::WARN;
    }
    if scenario.preset_index == OVERFLOW_PRESET_INDEX && index != FIRST_CHIP_INDEX {
        return palette.panel;
    }
    if scenario.screen_state.has_settings_override() && index == FIRST_CHIP_INDEX {
        return common::TOKEN;
    }
    palette.accent
}

pub(super) fn overflow_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action() || scenario.preset_index == OVERFLOW_PRESET_INDEX {
        return common::PURPLE;
    }
    palette.panel
}

pub(super) fn scroll_thumb_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == SCROLL_PRESET_INDEX {
        return common::TOKEN;
    }
    palette.border
}

pub(super) fn reorder_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == REORDER_PRESET_INDEX {
        return common::WARN;
    }
    palette.border
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        OVERFLOW_PRESET_INDEX => "overflow=menu",
        SCROLL_PRESET_INDEX => "overflow=scroll",
        REORDER_PRESET_INDEX => "reorder=true",
        _ => "wrap=true",
    }
}

pub(super) fn overflow_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.has_widget_action() {
        return "open";
    }
    if scenario.preset_index == OVERFLOW_PRESET_INDEX {
        return "+2";
    }
    "more"
}
