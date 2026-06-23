use super::dedicated_dod_common as common;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const CHAT_PRESET_INDEX: usize = 1;
const LINT_PRESET_INDEX: usize = 2;
const DIRTY_PRESET_INDEX: usize = 3;
const QUERY_PRESET_INDEX: usize = 4;
const RESET_PRESET_INDEX: usize = 5;
const LABEL_PRESET_INDEX: usize = 6;
const SECTION_LABEL_PRESET_INDEX: usize = 7;
const SECTION_DESCRIPTION_PRESET_INDEX: usize = 8;
const SECTION_ICON_PRESET_INDEX: usize = 9;
const FIELD_COUNT_PRESET_INDEX: usize = 10;
const SECTION_FOOTER_PRESET_INDEX: usize = 11;
const SECTION_COLLAPSE_PRESET_INDEX: usize = 12;
const DEFAULT_COLLAPSED_PRESET_INDEX: usize = 13;
const FIELD_LABEL_PRESET_INDEX: usize = 14;
const FIELD_DESCRIPTION_PRESET_INDEX: usize = 15;
const CONTROL_OPTIONS_PRESET_INDEX: usize = 16;
const CUSTOM_CONTROL_PRESET_INDEX: usize = 17;
const SET_VALUE_PRESET_INDEX: usize = 18;
const APP_SECTION_INDEX: usize = 0;
const CHAT_SECTION_INDEX: usize = 1;
const LINT_SECTION_INDEX: usize = 2;

pub(super) fn section_fill(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    section: usize,
) -> u32 {
    if scenario.preset_index == DEFAULT_COLLAPSED_PRESET_INDEX {
        return palette.background;
    }
    if matches!(
        scenario.preset_index,
        LABEL_PRESET_INDEX
            | SECTION_LABEL_PRESET_INDEX
            | SECTION_DESCRIPTION_PRESET_INDEX
            | SECTION_ICON_PRESET_INDEX
            | FIELD_COUNT_PRESET_INDEX
            | SECTION_FOOTER_PRESET_INDEX
            | SECTION_COLLAPSE_PRESET_INDEX
    ) && section == active_section(scenario)
    {
        return match scenario.preset_index {
            SECTION_DESCRIPTION_PRESET_INDEX | SECTION_FOOTER_PRESET_INDEX => common::PURPLE,
            SECTION_ICON_PRESET_INDEX | SECTION_COLLAPSE_PRESET_INDEX => common::WARN,
            FIELD_COUNT_PRESET_INDEX => common::SUCCESS,
            _ => common::TOKEN,
        };
    }
    if section == active_section(scenario) {
        return palette.surface;
    }
    palette.panel
}

pub(super) fn control_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.last_action == "settings_keyboard_next" {
        return common::PURPLE;
    }
    if scenario.screen_state.settings_list.hovered {
        return common::TOKEN;
    }
    if scenario.screen_state.settings_list.focused {
        return common::SUCCESS;
    }
    if scenario.screen_state.has_widget_action()
        || scenario.screen_state.has_settings_override()
        || scenario.preset_index == DIRTY_PRESET_INDEX
    {
        return common::WARN;
    }
    if scenario.preset_index == CONTROL_OPTIONS_PRESET_INDEX {
        return common::TOKEN;
    }
    if scenario.preset_index == CUSTOM_CONTROL_PRESET_INDEX {
        return common::PURPLE;
    }
    if scenario.preset_index == RESET_PRESET_INDEX {
        return common::SUCCESS;
    }
    palette.accent
}

pub(super) fn query_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.settings_list.scroll_offset > 0 {
        return common::WARN;
    }
    if matches!(
        scenario.preset_index,
        QUERY_PRESET_INDEX | LABEL_PRESET_INDEX | FIELD_DESCRIPTION_PRESET_INDEX
    ) || scenario.screen_state.has_settings_override()
    {
        return common::TOKEN;
    }
    palette.panel
}

pub(super) fn dirty_marker_fill(scenario: ScenarioContext<'_>) -> u32 {
    if matches!(
        scenario.preset_index,
        DIRTY_PRESET_INDEX | SECTION_COLLAPSE_PRESET_INDEX | SET_VALUE_PRESET_INDEX
    ) || scenario.screen_state.has_widget_action()
    {
        return common::WARN;
    }
    common::SUCCESS
}

pub(super) fn section_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == LABEL_PRESET_INDEX {
        return "Workspace settings";
    }
    if scenario.preset_index == SECTION_LABEL_PRESET_INDEX {
        return "Editor";
    }
    match active_section(scenario) {
        CHAT_SECTION_INDEX => "Chat settings",
        LINT_SECTION_INDEX => "Lint settings",
        _ => "App settings",
    }
}

pub(super) fn field_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.has_settings_override() {
        return "query=font";
    }
    match scenario.preset_index {
        CHAT_PRESET_INDEX => "Model: GPT-5 Codex",
        LINT_PRESET_INDEX => "Severity: warning",
        DIRTY_PRESET_INDEX => "Font size: dirty",
        QUERY_PRESET_INDEX => "filter: format",
        RESET_PRESET_INDEX => "Font size: default",
        SECTION_DESCRIPTION_PRESET_INDEX => "section description",
        SECTION_ICON_PRESET_INDEX => "icon=gear",
        FIELD_COUNT_PRESET_INDEX => "Fields: 5",
        SECTION_FOOTER_PRESET_INDEX => "footer=policy",
        SECTION_COLLAPSE_PRESET_INDEX => "collapsible=true",
        DEFAULT_COLLAPSED_PRESET_INDEX => "collapsed=true",
        FIELD_LABEL_PRESET_INDEX => "Font size",
        FIELD_DESCRIPTION_PRESET_INDEX => "field description",
        CONTROL_OPTIONS_PRESET_INDEX => "options=4",
        CUSTOM_CONTROL_PRESET_INDEX => "Custom action",
        SET_VALUE_PRESET_INDEX => "value=changed",
        _ => "Format on save",
    }
}

pub(super) fn status_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        DIRTY_PRESET_INDEX => "dirty=true",
        QUERY_PRESET_INDEX => "query=format",
        RESET_PRESET_INDEX => "reset=ready",
        LABEL_PRESET_INDEX => "label=Workspace",
        SECTION_LABEL_PRESET_INDEX => "section=Editor",
        SECTION_DESCRIPTION_PRESET_INDEX => "section desc",
        SECTION_ICON_PRESET_INDEX => "section icon",
        FIELD_COUNT_PRESET_INDEX => "fields=5",
        SECTION_FOOTER_PRESET_INDEX => "footer=policy",
        SECTION_COLLAPSE_PRESET_INDEX => "collapsible=true",
        DEFAULT_COLLAPSED_PRESET_INDEX => "default collapsed",
        FIELD_LABEL_PRESET_INDEX => "field=Font size",
        FIELD_DESCRIPTION_PRESET_INDEX => "field desc",
        CONTROL_OPTIONS_PRESET_INDEX => "options=4",
        CUSTOM_CONTROL_PRESET_INDEX => "custom control",
        SET_VALUE_PRESET_INDEX => "set_value=true",
        _ => "sections=3",
    }
}

fn active_section(scenario: ScenarioContext<'_>) -> usize {
    match scenario.preset_index {
        CHAT_PRESET_INDEX => CHAT_SECTION_INDEX,
        LINT_PRESET_INDEX
        | FIELD_COUNT_PRESET_INDEX
        | DEFAULT_COLLAPSED_PRESET_INDEX
        | SECTION_COLLAPSE_PRESET_INDEX => LINT_SECTION_INDEX,
        SECTION_LABEL_PRESET_INDEX
        | SECTION_DESCRIPTION_PRESET_INDEX
        | SECTION_ICON_PRESET_INDEX => CHAT_SECTION_INDEX,
        _ => APP_SECTION_INDEX,
    }
}
