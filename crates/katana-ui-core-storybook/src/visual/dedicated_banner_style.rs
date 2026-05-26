use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const VENDOR_PRESET_INDEX: usize = 1;
const ATTACHMENT_PRESET_INDEX: usize = 2;
const SUCCESS_PRESET_INDEX: usize = 3;
const DETAILS_PRESET_INDEX: usize = 4;

pub(super) fn banner_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() {
        return common::WARN;
    }
    if scenario.screen_state.has_widget_action() {
        return palette.accent;
    }
    palette.surface
}

pub(super) fn severity_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == SUCCESS_PRESET_INDEX {
        return common::SUCCESS;
    }
    if scenario.preset_index == VENDOR_PRESET_INDEX {
        return common::DANGER;
    }
    if scenario.preset_index == ATTACHMENT_PRESET_INDEX {
        return common::PURPLE;
    }
    palette.accent
}

pub(super) fn icon_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == SUCCESS_PRESET_INDEX {
        return common::SUCCESS;
    }
    palette.panel
}

pub(super) fn action_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == SUCCESS_PRESET_INDEX || scenario.screen_state.has_widget_action() {
        return palette.accent;
    }
    palette.panel
}

pub(super) fn dismiss_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == VENDOR_PRESET_INDEX {
        return common::DANGER;
    }
    palette.panel
}

pub(super) fn details_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == DETAILS_PRESET_INDEX || scenario.screen_state.has_widget_action() {
        return palette.panel;
    }
    palette.background
}

pub(super) fn banner_text(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action() || scenario.screen_state.has_settings_override() {
        return palette.background;
    }
    palette.text
}

pub(super) fn action_text(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == SUCCESS_PRESET_INDEX || scenario.screen_state.has_widget_action() {
        return palette.background;
    }
    palette.muted
}

pub(super) fn title_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == SUCCESS_PRESET_INDEX {
        return "Publish complete";
    }
    if scenario.preset_index == VENDOR_PRESET_INDEX {
        return "Vendor disconnected";
    }
    "Format result"
}

pub(super) fn body_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == ATTACHMENT_PRESET_INDEX {
        return "Attachment exceeds size";
    }
    if scenario.preset_index == SUCCESS_PRESET_INDEX {
        return "3 files saved";
    }
    "Formatter changed 3 files."
}

pub(super) fn details_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == DETAILS_PRESET_INDEX || scenario.screen_state.has_widget_action() {
        return "src/lib.rs, src/panel.rs, tests/storybook.rs";
    }
    "details collapsed"
}

pub(super) fn action_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_action == "none" {
        return "Open diff";
    }
    scenario.screen_state.last_action
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return "details_open=false";
    }
    scenario.screen_state.state_label
}
