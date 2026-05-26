use super::render_context::ScenarioContext;

const INPUT_IME_PRESET_INDEX: usize = 1;
const INPUT_INVALID_PRESET_INDEX: usize = 2;
const INPUT_THEME_PRESET_INDEX: usize = 3;
const SEARCH_SUBMIT_PRESET_INDEX: usize = 1;
const SEARCH_REGEX_PRESET_INDEX: usize = 2;
const SEARCH_THEME_PRESET_INDEX: usize = 3;

pub(super) fn input_value(scenario: ScenarioContext<'_>) -> &str {
    if scenario.screen_state.text_input_uses_live_value() {
        return scenario.screen_state.text_input_value();
    }
    if scenario.screen_state.has_widget_action() {
        return "typed 日本語 🔷";
    }
    match scenario.preset_index {
        INPUT_IME_PRESET_INDEX => "composing にほんご",
        INPUT_INVALID_PRESET_INDEX => "invalid@example",
        INPUT_THEME_PRESET_INDEX => "theme input bg",
        _ => "日本語 value 🔷",
    }
}

pub(super) fn search_value(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.search_box.cleared {
        return "";
    }
    if scenario.screen_state.search_box.typed {
        return "typed query";
    }
    match scenario.preset_index {
        SEARCH_SUBMIT_PRESET_INDEX => "submit ready",
        SEARCH_REGEX_PRESET_INDEX => "regex: TODO|FIXME",
        SEARCH_THEME_PRESET_INDEX => "theme clear",
        _ => "query",
    }
}

pub(super) fn status_action(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_action == "none" {
        return "action ready";
    }
    scenario.screen_state.last_action
}

pub(super) fn status_event(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_event == "none" {
        return "event ready";
    }
    scenario.screen_state.last_event
}

pub(super) fn status_state(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_action == "none" {
        return "value=query case=false regex=false";
    }
    scenario.screen_state.state_label
}
