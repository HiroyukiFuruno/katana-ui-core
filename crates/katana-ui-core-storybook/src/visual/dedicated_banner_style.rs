use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const VENDOR_PRESET_INDEX: usize = 1;
const ATTACHMENT_PRESET_INDEX: usize = 2;
const SUCCESS_PRESET_INDEX: usize = 3;
const DETAILS_PRESET_INDEX: usize = 4;
const TITLE_PRESET_INDEX: usize = 5;
const LEADING_ICON_PRESET_INDEX: usize = 6;
const PLACEMENT_PRESET_INDEX: usize = 7;

pub(super) fn banner_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() {
        return common::WARN;
    }
    if scenario.screen_state.has_widget_action() {
        return palette.accent;
    }
    if scenario.preset_index == TITLE_PRESET_INDEX {
        return common::TOKEN;
    }
    if scenario.preset_index == PLACEMENT_PRESET_INDEX {
        return palette.panel;
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
    if scenario.preset_index == PLACEMENT_PRESET_INDEX {
        return common::TOKEN;
    }
    palette.accent
}

pub(super) fn icon_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == SUCCESS_PRESET_INDEX {
        return common::SUCCESS;
    }
    if scenario.preset_index == LEADING_ICON_PRESET_INDEX {
        return common::PURPLE;
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
    if scenario.preset_index == PLACEMENT_PRESET_INDEX {
        return common::SUCCESS;
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
    if scenario.preset_index == TITLE_PRESET_INDEX {
        return "Optional title visible";
    }
    if scenario.preset_index == SUCCESS_PRESET_INDEX {
        return "Publish complete";
    }
    if scenario.preset_index == VENDOR_PRESET_INDEX {
        return "Adapter disconnected";
    }
    "Format result"
}

pub(super) fn body_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == PLACEMENT_PRESET_INDEX {
        return "Placement hint: sticky";
    }
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
    if scenario.preset_index == TITLE_PRESET_INDEX {
        return "title=Some(...)";
    }
    if scenario.preset_index == LEADING_ICON_PRESET_INDEX {
        return "leading_icon=external";
    }
    if scenario.preset_index == PLACEMENT_PRESET_INDEX {
        return "placement=Sticky";
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

pub(super) fn icon_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == LEADING_ICON_PRESET_INDEX {
        return "i";
    }
    "!"
}
