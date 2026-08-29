use super::render_context::ScenarioContext;

const INPUT_IME_PRESET_INDEX: usize = 1;
const INPUT_READONLY_PRESET_INDEX: usize = 2;
const INPUT_PLACEHOLDER_PRESET_INDEX: usize = 3;
const INPUT_RESERVED_SLOT_PRESET_INDEX: usize = 4;
const INPUT_LEADING_ICON_PRESET_INDEX: usize = 5;
const INPUT_ICON_BUTTONS_PRESET_INDEX: usize = 6;
const INPUT_INVALID_PRESET_INDEX: usize = 7;
const INPUT_THEME_PRESET_INDEX: usize = 8;
const INPUT_DISABLED_PRESET_INDEX: usize = 9;
const INPUT_FONT_ROLE_PRESET_INDEX: usize = 10;
const INPUT_TRAILING_SLOT_PRESET_INDEX: usize = 11;
const INPUT_CLEAR_ACTION_PRESET_INDEX: usize = 12;
const INPUT_SUBMIT_ENTER_PRESET_INDEX: usize = 13;
const INPUT_EMOJI_PRESET_INDEX: usize = 14;
const SEARCH_SUBMIT_PRESET_INDEX: usize = 1;
const SEARCH_REGEX_PRESET_INDEX: usize = 2;
const SEARCH_THEME_PRESET_INDEX: usize = 3;

pub(super) fn input_value(scenario: ScenarioContext<'_>) -> &str {
    if scenario
        .screen_state
        .text_input_uses_live_value_for(scenario.selected_instance_id)
    {
        return scenario
            .screen_state
            .text_input_value_for(scenario.selected_instance_id);
    }
    if scenario.screen_state.has_widget_action() {
        return "typed 日本語 🔷";
    }
    input_static_value_for_preset(scenario.preset_index)
}

pub(super) fn input_static_value_for_preset(preset_index: usize) -> &'static str {
    match preset_index {
        INPUT_IME_PRESET_INDEX => "composing にほんご",
        INPUT_READONLY_PRESET_INDEX => "readonly value",
        INPUT_PLACEHOLDER_PRESET_INDEX => "",
        INPUT_RESERVED_SLOT_PRESET_INDEX => "reserved slot",
        INPUT_LEADING_ICON_PRESET_INDEX => "search term",
        INPUT_ICON_BUTTONS_PRESET_INDEX => "file.rs",
        INPUT_INVALID_PRESET_INDEX => "invalid@example",
        INPUT_THEME_PRESET_INDEX => "theme input bg",
        INPUT_DISABLED_PRESET_INDEX => "disabled input",
        INPUT_FONT_ROLE_PRESET_INDEX => "font role monospace",
        INPUT_TRAILING_SLOT_PRESET_INDEX => "trailing slot",
        INPUT_CLEAR_ACTION_PRESET_INDEX => "clear action ready",
        INPUT_SUBMIT_ENTER_PRESET_INDEX => "submit on enter",
        INPUT_EMOJI_PRESET_INDEX => "emoji disabled",
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

#[cfg(test)]
mod tests {
    use super::search_value;
    use crate::visual::render_context::ScenarioContext;
    use crate::visual::screen_state::StorybookScreenState;

    #[test]
    fn cleared_search_value_is_empty() {
        let mut state = StorybookScreenState::default();
        state.search_box.cleared = true;

        assert_eq!(
            "",
            search_value(ScenarioContext::for_test("search-box", 0, &state))
        );
    }
}
