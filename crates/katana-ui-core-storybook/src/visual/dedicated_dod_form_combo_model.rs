use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::selection_control_metrics as sm;
use super::selection_screen_state::ComboBoxContractState;

const OPEN_PRESET_INDEX: usize = 1;
const SELECTED_PRESET_INDEX: usize = 2;
const VALUE_PRESET_INDEX: usize = 3;
const PLACEHOLDER_PRESET_INDEX: usize = 4;
const DISABLED_PRESET_INDEX: usize = 5;
const READONLY_PRESET_INDEX: usize = 6;
const INPUT_VALUE_PRESET_INDEX: usize = 7;
const FILTER_PRESET_INDEX: usize = 8;
const FREE_INPUT_PRESET_INDEX: usize = 9;
const KEYBOARD_PRESET_INDEX: usize = 10;
const PLACEMENT_PRESET_INDEX: usize = 11;
const HIGHLIGHT_PRESET_INDEX: usize = 12;
const LONG_LIST_PRESET_INDEX: usize = 13;
const DISMISS_PRESET_INDEX: usize = 14;
const FRAMED_PRESET_INDEX: usize = 15;
const TRIGGER_SUMMARY_PRESET_INDEX: usize = 16;
const SELECT_ACTION_PRESET_INDEX: usize = 17;
const INVALID_PRESET_INDEX: usize = 18;
const SELECTED_OPTION_INDEX: usize = 1;
const LONG_OPTION_LABELS: [&str; 6] = ["One", "Two", "Three", "Four", "Five", "Six"];
const DEFAULT_OPTION_LABELS: [&str; 2] = ["One", "Two"];
const FILTERED_OPTION_LABELS: [&str; 1] = ["Two"];
const STATUS_ROW_COUNT: usize = 3;

pub(super) fn input_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    let contract = combo_contract(scenario);
    if disabled(scenario) {
        return palette.panel;
    }
    if scenario.screen_state.is_button_focused() {
        return palette.selection;
    }
    if scenario.screen_state.preview_hovered {
        return palette.background;
    }
    if scenario.preset_index == FRAMED_PRESET_INDEX
        || contract.framed
        || scenario.preset_index == PLACEMENT_PRESET_INDEX
        || contract.placement_above
    {
        return palette.background;
    }
    palette.surface
}

pub(super) fn input_border(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.is_button_focused() {
        return palette.accent;
    }
    if scenario.screen_state.preview_hovered {
        return palette.hover_border;
    }
    if invalid(scenario) {
        return palette.accent;
    }
    if framed(scenario) {
        return palette.hover_border;
    }
    palette.border
}

pub(super) fn input_text_color(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if disabled(scenario)
        || readonly(scenario)
        || scenario.preset_index == PLACEHOLDER_PRESET_INDEX
        || combo_contract(scenario).placeholder_visible
    {
        return palette.muted;
    }
    palette.text
}

pub(super) fn input_value(scenario: ScenarioContext<'_>) -> &'static str {
    let contract = combo_contract(scenario);
    if scenario.preset_index == TRIGGER_SUMMARY_PRESET_INDEX || contract.trigger_summary {
        return "Two selected";
    }
    if disabled(scenario) {
        return "Disabled";
    }
    if readonly(scenario) {
        return "Readonly";
    }
    if scenario.preset_index == PLACEHOLDER_PRESET_INDEX || contract.placeholder_visible {
        return "Search commands...";
    }
    if selected_index(scenario) == Some(SELECTED_OPTION_INDEX) {
        return "Two";
    }
    if scenario.preset_index == VALUE_PRESET_INDEX || contract.value_applied {
        return "Two";
    }
    if scenario.preset_index == FREE_INPUT_PRESET_INDEX || contract.free_input {
        return "custom value";
    }
    if filtered(scenario) {
        return "tw";
    }
    "Type command"
}

pub(super) fn option_labels(scenario: ScenarioContext<'_>) -> &'static [&'static str] {
    if filtered(scenario) {
        return &FILTERED_OPTION_LABELS;
    }
    if long_list(scenario) {
        return &LONG_OPTION_LABELS;
    }
    &DEFAULT_OPTION_LABELS
}

pub(super) fn open(scenario: ScenarioContext<'_>) -> bool {
    scenario.screen_state.selection.combo_open
        || scenario.preset_index == OPEN_PRESET_INDEX
        || scenario.preset_index == FILTER_PRESET_INDEX
        || scenario.preset_index == KEYBOARD_PRESET_INDEX
        || scenario.preset_index == HIGHLIGHT_PRESET_INDEX
        || scenario.preset_index == LONG_LIST_PRESET_INDEX
}

pub(super) fn filtered(scenario: ScenarioContext<'_>) -> bool {
    scenario.screen_state.selection.combo_filtered
        || scenario.screen_state.selection.combo_contract.filter_result
        || scenario.screen_state.selection.combo_contract.input_value
        || scenario.preset_index == INPUT_VALUE_PRESET_INDEX
        || scenario.preset_index == FILTER_PRESET_INDEX
}

pub(super) fn option_count(scenario: ScenarioContext<'_>) -> usize {
    option_labels(scenario).len()
}

pub(super) fn status_rows(scenario: ScenarioContext<'_>) -> [&'static str; STATUS_ROW_COUNT] {
    [
        status_or_default(scenario.screen_state.last_action, action_status(scenario)),
        status_or_default(scenario.screen_state.last_event, "event ready"),
        status_or_default(scenario.screen_state.state_label, state_status(scenario)),
    ]
}

pub(super) fn framed(scenario: ScenarioContext<'_>) -> bool {
    scenario.preset_index == FRAMED_PRESET_INDEX || combo_contract(scenario).framed
}

pub(super) fn options_y(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == PLACEMENT_PRESET_INDEX || combo_contract(scenario).placement_above {
        return 0;
    }
    sm::COMBO_OPTIONS_Y
}

pub(super) fn highlighted_index(scenario: ScenarioContext<'_>) -> Option<usize> {
    if scenario.preset_index == HIGHLIGHT_PRESET_INDEX
        || scenario.preset_index == KEYBOARD_PRESET_INDEX
        || combo_contract(scenario).keyboard_navigation
    {
        return Some(SELECTED_OPTION_INDEX);
    }
    combo_contract(scenario).highlighted_index
}

fn selected_index(scenario: ScenarioContext<'_>) -> Option<usize> {
    if scenario
        .screen_state
        .selection
        .combo_selected_index
        .is_some()
    {
        return scenario.screen_state.selection.combo_selected_index;
    }
    if matches!(
        scenario.preset_index,
        SELECTED_PRESET_INDEX
            | VALUE_PRESET_INDEX
            | TRIGGER_SUMMARY_PRESET_INDEX
            | SELECT_ACTION_PRESET_INDEX
    ) {
        return Some(SELECTED_OPTION_INDEX);
    }
    None
}

fn status_or_default(value: &'static str, default_value: &'static str) -> &'static str {
    if matches!(value, "none" | "idle") {
        return default_value;
    }
    value
}

fn combo_contract(scenario: ScenarioContext<'_>) -> ComboBoxContractState {
    scenario.screen_state.selection.combo_contract
}

fn disabled(scenario: ScenarioContext<'_>) -> bool {
    scenario.preset_index == DISABLED_PRESET_INDEX || combo_contract(scenario).disabled
}

fn readonly(scenario: ScenarioContext<'_>) -> bool {
    scenario.preset_index == READONLY_PRESET_INDEX || combo_contract(scenario).readonly
}

fn invalid(scenario: ScenarioContext<'_>) -> bool {
    scenario.preset_index == INVALID_PRESET_INDEX || combo_contract(scenario).invalid
}

fn long_list(scenario: ScenarioContext<'_>) -> bool {
    scenario.preset_index == LONG_LIST_PRESET_INDEX
        || combo_contract(scenario).item_count > DEFAULT_OPTION_LABELS.len()
        || combo_contract(scenario).long_list
}

fn action_status(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == SELECT_ACTION_PRESET_INDEX || combo_contract(scenario).select_action
    {
        return "select callback";
    }
    if scenario.preset_index == DISMISS_PRESET_INDEX
        || combo_contract(scenario).outside_click_dismiss
    {
        return "dismiss ready";
    }
    "filter ready"
}

fn state_status(scenario: ScenarioContext<'_>) -> &'static str {
    if invalid(scenario) {
        return "invalid=true";
    }
    if disabled(scenario) {
        return "disabled=true";
    }
    if readonly(scenario) {
        return "readonly=true";
    }
    if scenario.preset_index == PLACEMENT_PRESET_INDEX || combo_contract(scenario).placement_above {
        return "placement=above";
    }
    if scenario.preset_index == FREE_INPUT_PRESET_INDEX || combo_contract(scenario).free_input {
        return "free_input=true";
    }
    "query=empty"
}
