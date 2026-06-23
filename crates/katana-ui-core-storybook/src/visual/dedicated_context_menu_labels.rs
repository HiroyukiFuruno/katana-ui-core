use super::dedicated_context_menu_metrics as cm;
use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

pub(super) fn preset_title(index: usize) -> &'static str {
    match index {
        cm::PRESET_EXPLORER_EMPTY => "ContextMenu / explorer 空領域",
        cm::PRESET_TAB_BAR => "ContextMenu / tab bar",
        cm::PRESET_MESSAGE_ROW => "ContextMenu / message 行",
        cm::PRESET_ICON_SHORTCUT => "ContextMenu / leading icon + shortcut",
        _ => "ContextMenu / 編集器右クリック",
    }
}

pub(super) fn preset_row_label(index: usize) -> &'static str {
    match index {
        cm::PRESET_EXPLORER_EMPTY => "New file",
        cm::PRESET_TAB_BAR => "Close tab",
        cm::PRESET_MESSAGE_ROW => "Copy message",
        cm::PRESET_ICON_SHORTCUT => "Icon Copy",
        _ => "Delete",
    }
}

pub(super) fn preset_shortcut(index: usize) -> &'static str {
    if index == cm::PRESET_ICON_SHORTCUT {
        "Cmd+C"
    } else {
        ""
    }
}

pub(super) fn action_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_action == "none" {
        return "action:open";
    }
    scenario.screen_state.last_action
}

pub(super) fn event_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_event == "none" {
        return "event:opened";
    }
    scenario.screen_state.last_event
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return "state:open";
    }
    scenario.screen_state.state_label
}

pub(super) fn marker_color(index: usize, palette: &VisualPalette) -> u32 {
    match index {
        cm::MARKER_ACTION_INDEX => common::SUCCESS,
        cm::MARKER_EVENT_INDEX => common::TOKEN,
        cm::MARKER_STATE_INDEX => palette.accent,
        _ => common::PURPLE,
    }
}
