use super::ScenarioContext;
use super::dedicated_dod_form_input_live_text_area_chrome as chrome;
use super::dedicated_dod_form_input_live_text_area_slots as slots;

const TEXT_AREA_LINE_COUNT: usize = 4;
const TEXT_AREA_STATUS_ROW_COUNT: usize = 3;
const NEWLINE_PRESET_INDEX: usize = 1;
const WRAP_PRESET_INDEX: usize = 2;
const VALUE_PRESET_INDEX: usize = 13;
const PLACEHOLDER_PRESET_INDEX: usize = 14;
const FONT_ROLE_PRESET_INDEX: usize = 15;
const DISABLED_PRESET_INDEX: usize = 16;
const READONLY_PRESET_INDEX: usize = 17;
const INVALID_PRESET_INDEX: usize = 18;
const MIN_ROWS_PRESET_INDEX: usize = 19;
const MAX_ROWS_PRESET_INDEX: usize = 20;
const IME_PRESET_INDEX: usize = 21;
const LEADING_SLOT_RESERVED_PRESET_INDEX: usize = 22;
const TRAILING_SLOT_RESERVED_PRESET_INDEX: usize = 23;

pub(super) fn line_count(
    preset_index: usize,
    screen_state: &crate::visual::screen_state::StorybookScreenState,
    instance: &'static str,
) -> usize {
    if screen_state.text_area_uses_live_value_for(instance) {
        return screen_state
            .text_area_value_for(instance)
            .lines()
            .count()
            .max(1);
    }
    static_rows(preset_index).len()
}

pub(super) fn content_lines(scenario: ScenarioContext<'_>) -> Vec<String> {
    if scenario
        .screen_state
        .text_area_uses_live_value_for(scenario.selected_instance_id)
    {
        return scenario
            .screen_state
            .text_area_value_for(scenario.selected_instance_id)
            .lines()
            .map(str::to_string)
            .collect();
    }
    static_rows(scenario.preset_index)
        .iter()
        .map(|line| (*line).to_string())
        .collect()
}

pub(super) fn visible_lines(lines: Vec<String>, offset: usize) -> [String; TEXT_AREA_LINE_COUNT] {
    let mut rows = [String::new(), String::new(), String::new(), String::new()];
    for (index, line) in lines
        .into_iter()
        .skip(offset)
        .take(TEXT_AREA_LINE_COUNT)
        .enumerate()
    {
        rows[index] = line;
    }
    rows
}

pub(super) fn status_rows(
    scenario: ScenarioContext<'_>,
) -> [&'static str; TEXT_AREA_STATUS_ROW_COUNT] {
    if matches!(
        scenario.screen_state.last_event,
        "text_area_scroll_changed" | "text_area_resized"
    ) {
        return [
            scenario.screen_state.last_action,
            scenario.screen_state.last_event,
            scenario.screen_state.state_label,
        ];
    }
    match scenario.preset_index {
        chrome::RESIZE_PRESET_INDEX => ["resize=true", "default false", "option on"],
        chrome::VERTICAL_SCROLL_PRESET_INDEX => ["scroll-y on", "bar hidden", "wrap=true"],
        chrome::HORIZONTAL_SCROLL_PRESET_INDEX => ["scroll-x on", "bar hidden", "wrap=false"],
        chrome::TAB_BEHAVIOR_PRESET_INDEX => ["tab behavior", "MoveFocus", "InsertTab"],
        chrome::VERTICAL_SCROLLBAR_PRESET_INDEX => ["scroll-y on", "bar visible", "display=true"],
        chrome::HORIZONTAL_SCROLLBAR_PRESET_INDEX => ["scroll-x on", "bar visible", "display=true"],
        slots::TEXT_AREA_LEADING_SVG_PRESET_INDEX => ["leading svg", "source=external", "slot"],
        slots::TEXT_AREA_TRAILING_BUTTONS_PRESET_INDEX => {
            ["trailing buttons", "callbacks", "svg icons"]
        }
        slots::TEXT_AREA_CLEAR_ACTION_PRESET_INDEX => ["clear action", "UiClearAction", "visible"],
        VALUE_PRESET_INDEX => ["value", "typed", "state"],
        PLACEHOLDER_PRESET_INDEX => ["placeholder", "visible", "empty"],
        FONT_ROLE_PRESET_INDEX => ["font role", "monospace", "token"],
        DISABLED_PRESET_INDEX => ["disabled", "true", "blocked"],
        READONLY_PRESET_INDEX => ["readonly", "true", "blocked"],
        INVALID_PRESET_INDEX => ["invalid", "true", "danger"],
        MIN_ROWS_PRESET_INDEX => ["min rows", "3", "layout"],
        MAX_ROWS_PRESET_INDEX => ["max rows", "8", "layout"],
        IME_PRESET_INDEX => ["ime", "disabled", "composition"],
        LEADING_SLOT_RESERVED_PRESET_INDEX => ["leading slot", "reserved", "no icon"],
        TRAILING_SLOT_RESERVED_PRESET_INDEX => ["trailing slot", "reserved", "no button"],
        _ if scenario
            .screen_state
            .text_area_uses_live_value_for(scenario.selected_instance_id) =>
        {
            [
                scenario.screen_state.last_action,
                scenario.screen_state.last_event,
                scenario.screen_state.state_label,
            ]
        }
        _ => ["wrap=true", "resize=false", "scroll=false"],
    }
}

fn static_rows(preset_index: usize) -> &'static [&'static str] {
    match preset_index {
        NEWLINE_PRESET_INDEX => &["newline key", "Shift+Enter", "Enter submit", "multi-line"],
        WRAP_PRESET_INDEX => &["長文 line 1", "line 2 wraps", "line 3 keeps", "line 4"],
        chrome::RESIZE_PRESET_INDEX => &[
            "resize option",
            "default false",
            "handle visible",
            "corner grip",
        ],
        chrome::AUTO_GROW_PRESET_INDEX => {
            &["auto grow", "rows 2 -> 4", "resize event", "scroll=false"]
        }
        chrome::VERTICAL_SCROLL_PRESET_INDEX => &[
            "line 01", "line 02", "line 03", "line 04", "line 05", "line 06", "line 07", "line 08",
        ],
        chrome::HORIZONTAL_SCROLL_PRESET_INDEX => &[
            "long unwrapped line keeps horizontal scroll",
            "wrap=false",
            "scroll-x enabled",
            "bar hidden",
        ],
        chrome::TAB_BEHAVIOR_PRESET_INDEX => &[
            "tab behavior",
            "MoveFocus",
            "InsertTab option",
            "focus event",
        ],
        chrome::VERTICAL_SCROLLBAR_PRESET_INDEX => &[
            "line 01", "line 02", "line 03", "line 04", "line 05", "line 06", "line 07", "line 08",
        ],
        chrome::HORIZONTAL_SCROLLBAR_PRESET_INDEX => &[
            "long unwrapped line with visible horizontal scrollbar",
            "scroll-x enabled",
            "display=true",
            "wrap=false",
        ],
        slots::TEXT_AREA_LEADING_SVG_PRESET_INDEX => &[
            "leading svg slot",
            "caller svg source",
            "reserved left area",
            "text offset",
        ],
        slots::TEXT_AREA_TRAILING_BUTTONS_PRESET_INDEX => &[
            "trailing icon callbacks",
            "clear callback",
            "format callback",
            "caller svg buttons",
        ],
        slots::TEXT_AREA_CLEAR_ACTION_PRESET_INDEX => &[
            "clear action",
            "UiClearActionSpec",
            "callback ready",
            "textarea entry props",
        ],
        VALUE_PRESET_INDEX => &["value option", "typed text", "state sync", "action value"],
        PLACEHOLDER_PRESET_INDEX => &[
            "placeholder",
            "empty value",
            "hint visible",
            "input contract",
        ],
        FONT_ROLE_PRESET_INDEX => &["font role", "monospace", "body text", "token change"],
        DISABLED_PRESET_INDEX => &["disabled", "read only paint", "event blocked", "muted"],
        READONLY_PRESET_INDEX => &["readonly", "focus allowed", "mutation blocked", "caret off"],
        INVALID_PRESET_INDEX => &["invalid", "danger border", "error text", "validation"],
        MIN_ROWS_PRESET_INDEX => &["min rows", "2 -> 3", "height lower bound", "layout"],
        MAX_ROWS_PRESET_INDEX => &["max rows", "6 -> 8", "height upper bound", "layout"],
        IME_PRESET_INDEX => &["ime", "composition off", "jp input", "event"],
        LEADING_SLOT_RESERVED_PRESET_INDEX => {
            &["leading slot", "reserve space", "no icon", "text offset"]
        }
        TRAILING_SLOT_RESERVED_PRESET_INDEX => {
            &["trailing slot", "reserve space", "no button", "clip width"]
        }
        _ => &["chat composer", "English", "日本語 🔷", "Cmd+Enter"],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visual::screen_state::StorybookScreenState;

    #[test]
    fn line_count_uses_the_selected_instance_live_value() {
        let mut state = StorybookScreenState::default();
        state.set_text_area_value_for_test("textarea.secondary", "one\ntwo\nthree");

        assert_eq!(3, line_count(0, &state, "textarea.secondary"));
        state.set_text_area_value_for_test("textarea.secondary", "");
        assert_eq!(1, line_count(0, &state, "textarea.secondary"));
    }
}
