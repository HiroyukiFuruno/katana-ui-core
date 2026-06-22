use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const CLICK_PRESET_INDEX: usize = 1;
const NESTED_CONTROLS_PRESET_INDEX: usize = 2;
const THEME_BORDER_PRESET_INDEX: usize = 3;
const SLOTS_PRESET_INDEX: usize = 0;
const LABEL_PRESET_INDEX: usize = 4;
const HEADER_PRESET_INDEX: usize = 5;
const FOOTER_PRESET_INDEX: usize = 6;
const PADDING_PRESET_INDEX: usize = 7;
const HEADER_COLOR: u32 = 0x20242c;
const FIRST_SLOT_INDEX: usize = 0;
const SECOND_SLOT_INDEX: usize = 1;
const THIRD_SLOT_INDEX: usize = 2;

pub(super) fn card_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.preview_hovered {
        return palette.panel;
    }
    if scenario.screen_state.is_button_focused() {
        return common::TOKEN;
    }
    if scenario.screen_state.has_widget_action() {
        return palette.accent;
    }
    if scenario.screen_state.has_settings_override() {
        return common::WARN;
    }
    palette.surface
}

pub(super) fn header_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == THEME_BORDER_PRESET_INDEX {
        return palette.accent;
    }
    if scenario.preset_index == HEADER_PRESET_INDEX {
        return common::TOKEN;
    }
    HEADER_COLOR
}

pub(super) fn active_slot_index(scenario: ScenarioContext<'_>) -> usize {
    if scenario.screen_state.has_widget_action() || scenario.preset_index == CLICK_PRESET_INDEX {
        return SECOND_SLOT_INDEX;
    }
    if scenario.preset_index == NESTED_CONTROLS_PRESET_INDEX {
        return THIRD_SLOT_INDEX;
    }
    if scenario.preset_index == FOOTER_PRESET_INDEX {
        return THIRD_SLOT_INDEX;
    }
    if scenario.preset_index == LABEL_PRESET_INDEX {
        return SECOND_SLOT_INDEX;
    }
    FIRST_SLOT_INDEX
}

pub(super) fn slot_fill(palette: &VisualPalette, active: usize, index: usize) -> u32 {
    if active == index {
        return palette.accent;
    }
    palette.panel
}

pub(super) fn badge_fill(scenario: ScenarioContext<'_>) -> u32 {
    match scenario.preset_index {
        CLICK_PRESET_INDEX => common::TOKEN,
        NESTED_CONTROLS_PRESET_INDEX => common::PURPLE,
        THEME_BORDER_PRESET_INDEX => common::WARN,
        LABEL_PRESET_INDEX => common::TOKEN,
        HEADER_PRESET_INDEX => common::PURPLE,
        FOOTER_PRESET_INDEX => common::DANGER,
        PADDING_PRESET_INDEX => common::WARN,
        _ => common::SUCCESS,
    }
}

pub(super) fn border_line_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.preview_hovered {
        return common::WARN;
    }
    if matches!(
        scenario.preset_index,
        THEME_BORDER_PRESET_INDEX | PADDING_PRESET_INDEX
    ) {
        return common::TOKEN;
    }
    palette.accent
}

pub(super) fn slot_text(palette: &VisualPalette, active: usize, index: usize) -> u32 {
    if active == index {
        return palette.background;
    }
    palette.text
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        CLICK_PRESET_INDEX => "click=ready",
        NESTED_CONTROLS_PRESET_INDEX => "nested=2",
        THEME_BORDER_PRESET_INDEX => "theme=border",
        LABEL_PRESET_INDEX => "label=summary",
        HEADER_PRESET_INDEX => "header=custom",
        FOOTER_PRESET_INDEX => "footer=visible",
        PADDING_PRESET_INDEX => "padding=large",
        SLOTS_PRESET_INDEX => "slots=3",
        _ => "state=ready",
    }
}
