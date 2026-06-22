use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const CHAT_USAGE_PRESET_INDEX: usize = 1;
const LINTER_PRESET_INDEX: usize = 2;
const PROGRESS_PRESET_INDEX: usize = 3;
const POPOVER_PRESET_INDEX: usize = 4;
const MESSAGE_PRESET_INDEX: usize = 5;
const SEVERITY_PRESET_INDEX: usize = 6;
const DISMISS_PRESET_INDEX: usize = 7;
const A11Y_PRESET_INDEX: usize = 8;
const PROGRESS_COMPACT_WIDTH: usize = 80;
const PROGRESS_FULL_WIDTH: usize = 128;

pub(super) fn segment_fill(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    index: usize,
) -> u32 {
    if let Some(active_index) = scenario.screen_state.status_bar_open_segment_index {
        return if index == active_index {
            palette.accent
        } else {
            palette.surface
        };
    }
    if scenario.screen_state.status_bar_hovered_segment_index == Some(index) {
        return common::TOKEN;
    }
    if scenario.screen_state.status_bar_focused_segment_index == Some(index) {
        return common::SUCCESS;
    }
    if scenario.screen_state.has_widget_action() || scenario.screen_state.has_settings_override() {
        return palette.accent;
    }
    if scenario.preset_index == LINTER_PRESET_INDEX && index == 1 {
        return common::WARN;
    }
    if scenario.preset_index == SEVERITY_PRESET_INDEX && index == 0 {
        return common::WARN;
    }
    if scenario.preset_index == POPOVER_PRESET_INDEX && index == 0 {
        return common::TOKEN;
    }
    if scenario.preset_index == DISMISS_PRESET_INDEX && index == 2 {
        return common::DANGER;
    }
    if scenario.preset_index == A11Y_PRESET_INDEX && index == 1 {
        return common::PURPLE;
    }
    palette.surface
}

pub(super) fn progress_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == PROGRESS_PRESET_INDEX {
        return common::SUCCESS;
    }
    if scenario.preset_index == DISMISS_PRESET_INDEX {
        return common::DANGER;
    }
    palette.accent
}

pub(super) fn progress_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == PROGRESS_PRESET_INDEX {
        return PROGRESS_FULL_WIDTH;
    }
    PROGRESS_COMPACT_WIDTH
}

pub(super) fn center_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        CHAT_USAGE_PRESET_INDEX => "tokens 42%",
        LINTER_PRESET_INDEX => "2 warnings",
        PROGRESS_PRESET_INDEX => "indexing 72%",
        POPOVER_PRESET_INDEX => "main ahead",
        MESSAGE_PRESET_INDEX => "Ready",
        SEVERITY_PRESET_INDEX => "warning",
        DISMISS_PRESET_INDEX => "dismiss",
        A11Y_PRESET_INDEX => "Diagnostics summary",
        _ => "Ln 12, Col 4",
    }
}

pub(super) fn status_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        CHAT_USAGE_PRESET_INDEX => "mode=chat",
        LINTER_PRESET_INDEX => "lint=warning",
        PROGRESS_PRESET_INDEX => "progress=72",
        POPOVER_PRESET_INDEX => "popover=branch",
        MESSAGE_PRESET_INDEX => "message=Ready",
        SEVERITY_PRESET_INDEX => "severity=Warning",
        DISMISS_PRESET_INDEX => "dismiss=available",
        A11Y_PRESET_INDEX => "a11y=custom",
        _ => "segments=4",
    }
}
