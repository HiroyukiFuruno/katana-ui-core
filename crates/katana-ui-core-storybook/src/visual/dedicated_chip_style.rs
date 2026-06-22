use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const LEADING_ICON_PRESET_INDEX: usize = 1;
const TRAILING_ICON_PRESET_INDEX: usize = 2;
const VARIANT_PRESET_INDEX: usize = 3;
const TONE_MATRIX_PRESET_INDEX: usize = 4;
const SIZE_PRESET_INDEX: usize = 5;
const INTERACTIVE_PRESET_INDEX: usize = 6;
const SELECTED_PRESET_INDEX: usize = 7;
const DISABLED_PRESET_INDEX: usize = 8;
const DISMISS_PRESET_INDEX: usize = 9;
const A11Y_PRESET_INDEX: usize = 10;
const FOCUSED_PRESET_INDEX: usize = 11;
const PRIMARY_TONE_INDEX: usize = 0;
const WARNING_TONE_INDEX: usize = 1;

pub(super) fn chip_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.last_action == "chip_dismiss" {
        return common::WARN;
    }
    if scenario.screen_state.has_widget_action() {
        return common::DANGER;
    }
    if scenario.screen_state.preview_hovered {
        return palette.selection;
    }
    if scenario.screen_state.has_settings_override() {
        return common::WARN;
    }
    if scenario.preset_index == LEADING_ICON_PRESET_INDEX {
        return common::SUCCESS;
    }
    if scenario.preset_index == TRAILING_ICON_PRESET_INDEX {
        return palette.accent;
    }
    if scenario.preset_index == VARIANT_PRESET_INDEX {
        return common::PURPLE;
    }
    if scenario.preset_index == SELECTED_PRESET_INDEX {
        return palette.selection;
    }
    if scenario.preset_index == SIZE_PRESET_INDEX {
        return common::PURPLE;
    }
    if scenario.preset_index == DISABLED_PRESET_INDEX {
        return common::WARN;
    }
    palette.surface
}

pub(super) fn icon_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    match scenario.preset_index {
        LEADING_ICON_PRESET_INDEX => common::SUCCESS,
        TRAILING_ICON_PRESET_INDEX => common::DANGER,
        A11Y_PRESET_INDEX => common::PURPLE,
        _ => palette.accent,
    }
}

pub(super) fn focus_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.is_button_focused() {
        return palette.accent;
    }
    if matches!(
        scenario.preset_index,
        SELECTED_PRESET_INDEX | INTERACTIVE_PRESET_INDEX | FOCUSED_PRESET_INDEX
    ) {
        return palette.accent;
    }
    if scenario.screen_state.has_widget_action() || scenario.screen_state.has_settings_override() {
        return common::TOKEN;
    }
    palette.border
}

pub(super) fn dismiss_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.last_action == "chip_dismiss" {
        return common::SUCCESS;
    }
    if scenario.screen_state.has_widget_action() || scenario.preset_index == DISMISS_PRESET_INDEX {
        return common::DANGER;
    }
    palette.panel
}

pub(super) fn tone_fill(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    index: usize,
) -> u32 {
    if scenario.preset_index != TONE_MATRIX_PRESET_INDEX {
        return palette.panel;
    }
    match index {
        PRIMARY_TONE_INDEX => common::SUCCESS,
        WARNING_TONE_INDEX => common::WARN,
        _ => common::PURPLE,
    }
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        LEADING_ICON_PRESET_INDEX => "leading=tag",
        TRAILING_ICON_PRESET_INDEX => "trailing=close",
        DISMISS_PRESET_INDEX => "dismiss=ready",
        SELECTED_PRESET_INDEX => "selected=true",
        VARIANT_PRESET_INDEX => "variant=filled",
        TONE_MATRIX_PRESET_INDEX => "tone=matrix",
        SIZE_PRESET_INDEX => "size=large",
        INTERACTIVE_PRESET_INDEX => "interactive=true",
        DISABLED_PRESET_INDEX => "disabled=true",
        A11Y_PRESET_INDEX => "a11y=filter",
        FOCUSED_PRESET_INDEX => "focused=true",
        _ => "filter=active",
    }
}

pub(super) fn dismiss_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.has_widget_action() {
        return "dismissed";
    }
    if scenario.preset_index == DISMISS_PRESET_INDEX {
        return "backspace";
    }
    "dismiss"
}
