use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const DEDUP_PRESET_INDEX: usize = 1;
const PAUSE_PRESET_INDEX: usize = 2;
const QUEUE_PRESET_INDEX: usize = 3;
const ACTION_PRESET_INDEX: usize = 4;

pub(super) fn position_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == DEDUP_PRESET_INDEX {
        return common::TOKEN;
    }
    palette.panel
}

pub(super) fn pause_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == PAUSE_PRESET_INDEX
        || scenario.screen_state.state_label == "toast_stack.paused=true"
    {
        return palette.accent;
    }
    palette.panel
}

pub(super) fn stack_panel_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() {
        return common::WARN;
    }
    palette.background
}

pub(super) fn top_toast_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action() {
        return palette.accent;
    }
    if scenario.preset_index == QUEUE_PRESET_INDEX {
        return common::DANGER;
    }
    palette.surface
}

pub(super) fn middle_toast_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == DEDUP_PRESET_INDEX {
        return common::TOKEN;
    }
    palette.panel
}

pub(super) fn lower_toast_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == QUEUE_PRESET_INDEX {
        return common::WARN;
    }
    palette.panel
}

pub(super) fn action_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == ACTION_PRESET_INDEX || scenario.screen_state.has_widget_action() {
        return palette.accent;
    }
    palette.panel
}

pub(super) fn queue_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == QUEUE_PRESET_INDEX {
        return common::DANGER;
    }
    palette.border
}

pub(super) fn position_text(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == DEDUP_PRESET_INDEX {
        return palette.background;
    }
    palette.text
}

pub(super) fn pause_text(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == PAUSE_PRESET_INDEX
        || scenario.screen_state.state_label == "toast_stack.paused=true"
    {
        return palette.background;
    }
    palette.text
}

pub(super) fn toast_text(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action() || scenario.preset_index == QUEUE_PRESET_INDEX {
        return palette.background;
    }
    palette.text
}

pub(super) fn action_text(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == ACTION_PRESET_INDEX || scenario.screen_state.has_widget_action() {
        return palette.background;
    }
    palette.muted
}

pub(super) fn position_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == DEDUP_PRESET_INDEX {
        return "dedup ById";
    }
    "BottomEnd"
}

pub(super) fn pause_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "toast_stack.paused=true" {
        return "paused by pointer/focus";
    }
    if scenario.preset_index == PAUSE_PRESET_INDEX {
        return "pause on hover";
    }
    "hover resumes timer"
}

pub(super) fn top_toast_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == QUEUE_PRESET_INDEX {
        return "Build failed";
    }
    "Saved"
}

pub(super) fn queue_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == QUEUE_PRESET_INDEX {
        return "queued=1 overflow";
    }
    "visible=2 queued=1"
}

pub(super) fn action_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_action == "none" {
        return "Undo";
    }
    scenario.screen_state.last_action
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return "visible=2 queued=1";
    }
    scenario.screen_state.state_label
}
