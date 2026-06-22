use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const OVERFLOW_PRESET_INDEX: usize = 0;
const SPLIT_PRESET_INDEX: usize = 1;
const DISPLAY_PRESET_INDEX: usize = 2;
const DENSITY_PRESET_INDEX: usize = 3;
const ACCELERATOR_PRESET_INDEX: usize = 4;
const CONTEXT_ANCHOR_PRESET_INDEX: usize = 5;
const ACTION_PRIORITY_PRESET_INDEX: usize = 6;
const ACTION_ACCELERATOR_PRESET_INDEX: usize = 7;
const ACTION_SPLIT_PRESET_INDEX: usize = 8;
const ACTION_GROUP_PRESET_INDEX: usize = 9;
const ACTION_TOOLTIP_PRESET_INDEX: usize = 10;
const ACTION_A11Y_PRESET_INDEX: usize = 11;
const ACTION_DISABLED_PRESET_INDEX: usize = 12;
const GROUP_LABEL_PRESET_INDEX: usize = 13;
const GROUP_DIVIDER_PRESET_INDEX: usize = 14;
const SPLIT_DISABLED_PRESET_INDEX: usize = 15;
const SPLIT_TOOLTIP_PRESET_INDEX: usize = 16;
const SPLIT_A11Y_PRESET_INDEX: usize = 17;

pub(super) fn bar_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() {
        return common::WARN;
    }
    if scenario.screen_state.has_widget_action() {
        return palette.accent;
    }
    if matches!(
        scenario.preset_index,
        CONTEXT_ANCHOR_PRESET_INDEX | ACTION_GROUP_PRESET_INDEX | GROUP_LABEL_PRESET_INDEX
    ) {
        return common::TOKEN;
    }
    if scenario.preset_index == GROUP_DIVIDER_PRESET_INDEX {
        return common::PURPLE;
    }
    palette.surface
}

pub(super) fn action_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == ACTION_DISABLED_PRESET_INDEX {
        return common::WARN;
    }
    if matches!(
        scenario.preset_index,
        DISPLAY_PRESET_INDEX
            | ACTION_PRIORITY_PRESET_INDEX
            | ACTION_TOOLTIP_PRESET_INDEX
            | ACTION_A11Y_PRESET_INDEX
    ) || scenario.screen_state.has_widget_action()
    {
        return palette.accent;
    }
    palette.panel
}

pub(super) fn split_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == SPLIT_DISABLED_PRESET_INDEX {
        return common::WARN;
    }
    if matches!(
        scenario.preset_index,
        SPLIT_PRESET_INDEX | ACTION_SPLIT_PRESET_INDEX | SPLIT_TOOLTIP_PRESET_INDEX
    ) {
        return palette.accent;
    }
    if scenario.preset_index == SPLIT_A11Y_PRESET_INDEX {
        return common::TOKEN;
    }
    palette.panel
}

pub(super) fn more_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == OVERFLOW_PRESET_INDEX {
        return common::TOKEN;
    }
    palette.panel
}

pub(super) fn accelerator_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if matches!(
        scenario.preset_index,
        ACCELERATOR_PRESET_INDEX | ACTION_ACCELERATOR_PRESET_INDEX
    ) {
        return palette.accent;
    }
    palette.panel
}

pub(super) fn density_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == DENSITY_PRESET_INDEX {
        return common::PURPLE;
    }
    palette.panel
}

pub(super) fn action_text(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == DISPLAY_PRESET_INDEX || scenario.screen_state.has_widget_action() {
        return palette.background;
    }
    palette.text
}

pub(super) fn accelerator_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == ACTION_ACCELERATOR_PRESET_INDEX {
        return "Alt+P";
    }
    if scenario.preset_index == ACCELERATOR_PRESET_INDEX {
        return "Cmd+F";
    }
    "Cmd+S"
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    if scenario.preset_index == CONTEXT_ANCHOR_PRESET_INDEX {
        return "anchor=pointer";
    }
    if scenario.preset_index == ACTION_PRIORITY_PRESET_INDEX {
        return "priority=90";
    }
    if scenario.preset_index == ACTION_ACCELERATOR_PRESET_INDEX {
        return "accelerator=Alt+P";
    }
    if scenario.preset_index == ACTION_SPLIT_PRESET_INDEX {
        return "action split=menu";
    }
    if scenario.preset_index == ACTION_GROUP_PRESET_INDEX {
        return "action group=edit";
    }
    if scenario.preset_index == ACTION_TOOLTIP_PRESET_INDEX {
        return "tooltip=Save file";
    }
    if scenario.preset_index == ACTION_A11Y_PRESET_INDEX {
        return "a11y=Save file";
    }
    if scenario.preset_index == ACTION_DISABLED_PRESET_INDEX {
        return "action disabled";
    }
    if scenario.preset_index == GROUP_LABEL_PRESET_INDEX {
        return "group=File actions";
    }
    if scenario.preset_index == GROUP_DIVIDER_PRESET_INDEX {
        return "divider=false";
    }
    if scenario.preset_index == SPLIT_DISABLED_PRESET_INDEX {
        return "split disabled";
    }
    if scenario.preset_index == SPLIT_TOOLTIP_PRESET_INDEX {
        return "split tooltip";
    }
    if scenario.preset_index == SPLIT_A11Y_PRESET_INDEX {
        return "split a11y";
    }
    "overflow=menu split=false"
}
