use super::storybook_ui_option_contract::StorybookUiOptionContract;

pub(super) const BUTTON_OPTIONS: [StorybookUiOptionContract; 15] = [
    StorybookUiOptionContract::new("visible", "true", "false"),
    StorybookUiOptionContract::new("disabled", "false", "true"),
    StorybookUiOptionContract::new("focusable", "true", "false"),
    StorybookUiOptionContract::new("width", "auto", "160px"),
    StorybookUiOptionContract::new("height", "auto", "28px"),
    StorybookUiOptionContract::new("border", "visible", "hidden"),
    StorybookUiOptionContract::new("label", "Save changes", "保存する"),
    StorybookUiOptionContract::new("tab-index", "0", "1"),
    StorybookUiOptionContract::new("z-index", "auto", "10"),
    StorybookUiOptionContract::new("button.command", "save", "open"),
    StorybookUiOptionContract::new("button.keyboard_activation", "true", "false"),
    StorybookUiOptionContract::new("button.icon_position", "leading", "trailing"),
    StorybookUiOptionContract::new("button.layout_preset", "page", "dense"),
    StorybookUiOptionContract::new("button.svg_source", "default-svg", "custom-svg"),
    StorybookUiOptionContract::new("button.aria_label", "Svg action", "Close panel"),
];

pub(super) const INPUT_OPTIONS: [StorybookUiOptionContract; 15] = [
    StorybookUiOptionContract::new("interaction.value", "typed", "typed 日本語"),
    StorybookUiOptionContract::new("readonly", "false", "true"),
    StorybookUiOptionContract::new("placeholder", "visible", "hidden"),
    StorybookUiOptionContract::new("text_entry.leading_slot_reserved", "false", "true"),
    StorybookUiOptionContract::new("text_entry.leading_slot.icon", "none", "search-svg"),
    StorybookUiOptionContract::new("text_entry.trailing_icon_buttons", "none", "callbacks"),
    StorybookUiOptionContract::new("validation", "valid", "invalid"),
    StorybookUiOptionContract::new("ime", "idle", "composition"),
    StorybookUiOptionContract::new("theme.input_bg", "surface", "light"),
    StorybookUiOptionContract::new("disabled", "false", "true"),
    StorybookUiOptionContract::new("font_role", "body", "monospace"),
    StorybookUiOptionContract::new("text_entry.trailing_slot_reserved", "false", "true"),
    StorybookUiOptionContract::new("text_entry.clear_action", "none", "visible"),
    StorybookUiOptionContract::new("text_entry.submit_on_enter", "false", "true"),
    StorybookUiOptionContract::new("text_entry.emoji_enabled", "true", "false"),
];

pub(super) const SEARCH_BOX_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("text_entry.value", "query", "typed query"),
    StorybookUiOptionContract::new("text_entry.submit_on_enter", "false", "true"),
    StorybookUiOptionContract::new("text_entry.clear_button", "visible", "cleared"),
    StorybookUiOptionContract::new("text_entry.regex_case", "false/false", "true/true"),
];

pub(super) const SEARCH_CONTROL_STRIP_OPTIONS: [StorybookUiOptionContract; 7] = [
    StorybookUiOptionContract::new("search_control.query", "head", "heading"),
    StorybookUiOptionContract::new("search_control.match_case", "false", "true"),
    StorybookUiOptionContract::new("search_control.whole_word", "false", "true"),
    StorybookUiOptionContract::new("search_control.use_regex", "false", "true"),
    StorybookUiOptionContract::new("search_control.replace_mode", "Visible", "Disabled"),
    StorybookUiOptionContract::new("search_control.result_count", "12", "0"),
    StorybookUiOptionContract::new("search_control.active_index", "Some(2)", "None"),
];

pub(super) const COMBO_BOX_OPTIONS: [StorybookUiOptionContract; 19] = [
    StorybookUiOptionContract::new("combo.items", "2", "6"),
    StorybookUiOptionContract::new("interaction.open", "false", "true"),
    StorybookUiOptionContract::new("interaction.selected_index", "none", "1"),
    StorybookUiOptionContract::new("interaction.value", "empty", "two"),
    StorybookUiOptionContract::new("placeholder", "hidden", "visible"),
    StorybookUiOptionContract::new("disabled", "false", "true"),
    StorybookUiOptionContract::new("readonly", "false", "true"),
    StorybookUiOptionContract::new("combo.input_value", "empty", "tw"),
    StorybookUiOptionContract::new("combo.filter_result", "all", "filtered"),
    StorybookUiOptionContract::new("combo.free_input", "false", "true"),
    StorybookUiOptionContract::new("combo.keyboard_navigation", "idle", "active"),
    StorybookUiOptionContract::new("combo.placement", "below", "above"),
    StorybookUiOptionContract::new("combo.highlighted_index", "none", "1"),
    StorybookUiOptionContract::new("combo.long_list", "false", "true"),
    StorybookUiOptionContract::new("combo.outside_click_dismiss", "false", "true"),
    StorybookUiOptionContract::new("combo.framed", "false", "true"),
    StorybookUiOptionContract::new("combo.trigger_summary", "label", "selected summary"),
    StorybookUiOptionContract::new("combo.select_action", "none", "callback"),
    StorybookUiOptionContract::new("validation", "valid", "invalid"),
];
