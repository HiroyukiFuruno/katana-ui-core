use super::dedicated_dod_common as common;
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const MESSAGE_PRESET_INDEX: usize = 1;
const CARD_PRESET_INDEX: usize = 2;
const PARAGRAPH_PRESET_INDEX: usize = 3;
const CODE_PRESET_INDEX: usize = 4;
const IMAGE_PRESET_INDEX: usize = 5;

pub(super) fn media_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action()
        || scenario.screen_state.has_settings_override()
        || scenario.preset_index == CARD_PRESET_INDEX
    {
        return palette.accent;
    }
    if scenario.preset_index == IMAGE_PRESET_INDEX {
        return common::TOKEN;
    }
    palette.surface
}

pub(super) fn line_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    match scenario.preset_index {
        MESSAGE_PRESET_INDEX => common::SUCCESS,
        CODE_PRESET_INDEX => common::PURPLE,
        _ => palette.surface,
    }
}

pub(super) fn secondary_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == PARAGRAPH_PRESET_INDEX {
        return common::WARN;
    }
    palette.panel
}

pub(super) fn media_width(scenario: ScenarioContext<'_>) -> usize {
    match scenario.preset_index {
        CARD_PRESET_INDEX => m::PX_188,
        IMAGE_PRESET_INDEX => m::PX_118,
        _ => m::PX_64,
    }
}

pub(super) fn line_width(scenario: ScenarioContext<'_>) -> usize {
    match scenario.preset_index {
        MESSAGE_PRESET_INDEX => m::PX_214,
        PARAGRAPH_PRESET_INDEX => m::PX_258,
        CODE_PRESET_INDEX => m::PX_176,
        _ => m::PX_198,
    }
}

pub(super) fn preset_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.has_settings_override() {
        return "Card";
    }
    match scenario.preset_index {
        MESSAGE_PRESET_INDEX => "Message",
        CARD_PRESET_INDEX => "Card",
        PARAGRAPH_PRESET_INDEX => "Paragraph",
        CODE_PRESET_INDEX => "CodeBlock",
        IMAGE_PRESET_INDEX => "ImageCard",
        _ => "ListRow",
    }
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        MESSAGE_PRESET_INDEX => "items=3",
        CARD_PRESET_INDEX => "card=loading",
        PARAGRAPH_PRESET_INDEX => "paragraph=4",
        CODE_PRESET_INDEX => "code=block",
        IMAGE_PRESET_INDEX => "image=card",
        _ => "items=2",
    }
}
