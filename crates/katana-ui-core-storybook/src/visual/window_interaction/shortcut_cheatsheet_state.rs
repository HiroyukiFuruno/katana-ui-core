use super::shortcut_cheatsheet_fixture::{
    DEFAULT_GROUP_COUNT, DEFAULT_ITEM_COUNT, EXPANDED_GROUP_COUNT, EXPANDED_ITEM_COUNT,
    FILTERED_RESULT_COUNT, FORMAT_ID, QUERY_CATEGORY, assert_query_event, assert_selected_event,
    cheatsheet_with_group_count, cheatsheet_with_item_count, default_cheatsheet,
};
use katana_ui_core::molecule::{
    ShortcutCheatsheet, ShortcutCheatsheetAction, ShortcutCheatsheetLayout,
};

const MAX_SCROLL_OFFSET: usize = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::visual) struct ShortcutCheatsheetScreenState {
    cheatsheet: ShortcutCheatsheet,
    option_state: ShortcutCheatsheetOptionState,
    callback_action: &'static str,
    pub(in crate::visual) focused: bool,
    pub(in crate::visual) hovered: bool,
    pub(in crate::visual) scroll_offset: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) struct ShortcutCheatsheetOptionState {
    pub(in crate::visual) label_editor_keys: bool,
    pub(in crate::visual) group_count: usize,
    pub(in crate::visual) group_title_navigation: bool,
    pub(in crate::visual) item_count: usize,
    pub(in crate::visual) item_combo_command_shift_p: bool,
    pub(in crate::visual) group_layout_one_column: bool,
    pub(in crate::visual) query_category: bool,
    pub(in crate::visual) selected_format: bool,
    pub(in crate::visual) result_count: usize,
}

impl Default for ShortcutCheatsheetOptionState {
    fn default() -> Self {
        Self {
            label_editor_keys: false,
            group_count: DEFAULT_GROUP_COUNT,
            group_title_navigation: false,
            item_count: DEFAULT_ITEM_COUNT,
            item_combo_command_shift_p: false,
            group_layout_one_column: false,
            query_category: false,
            selected_format: false,
            result_count: DEFAULT_ITEM_COUNT,
        }
    }
}

impl Default for ShortcutCheatsheetScreenState {
    fn default() -> Self {
        Self {
            cheatsheet: default_cheatsheet(),
            option_state: ShortcutCheatsheetOptionState::default(),
            callback_action: "none",
            focused: false,
            hovered: false,
            scroll_offset: 0,
        }
    }
}

impl ShortcutCheatsheetScreenState {
    pub(in crate::visual) fn select_format(&mut self) -> ShortcutCheatsheetUpdate {
        let event = self
            .cheatsheet
            .apply_action(ShortcutCheatsheetAction::SelectShortcut(
                FORMAT_ID.to_string(),
            ));
        assert_selected_event(event);
        self.option_state.selected_format = true;
        self.callback_action = "shortcut_cheatsheet_selected";
        ShortcutCheatsheetUpdate::new(
            "shortcut_filter_select",
            "shortcut_selected",
            "selected=format",
        )
    }

    pub(in crate::visual) fn focus(&mut self) -> ShortcutCheatsheetUpdate {
        self.focused = true;
        ShortcutCheatsheetUpdate::new("shortcut_cheatsheet_focus", "focus", "focus=true")
    }

    pub(in crate::visual) fn hover(&mut self) -> ShortcutCheatsheetUpdate {
        self.hovered = true;
        ShortcutCheatsheetUpdate::new("shortcut_cheatsheet_hover", "hover_start", "hover=true")
    }

    pub(in crate::visual) fn keyboard_select(&mut self) -> ShortcutCheatsheetUpdate {
        self.select_format()
    }

    pub(in crate::visual) fn scroll_results(&mut self) -> ShortcutCheatsheetUpdate {
        self.scroll_offset = (self.scroll_offset + 1).min(MAX_SCROLL_OFFSET);
        ShortcutCheatsheetUpdate::new(
            "shortcut_cheatsheet_scroll",
            "scroll_by",
            self.scroll_label(),
        )
    }

    pub(in crate::visual) fn apply_option(&mut self, setting: &str) {
        match setting {
            "shortcut_cheatsheet.label" => self.apply_label(),
            "shortcut_cheatsheet.groups" => self.apply_groups(),
            "shortcut_cheatsheet.group_title" => self.apply_group_title(),
            "shortcut_cheatsheet.items" => self.apply_items(),
            "shortcut_cheatsheet.item_combo" => self.apply_item_combo(),
            "shortcut_cheatsheet.group_layout" => self.apply_group_layout(),
            "shortcut_cheatsheet.query" => self.apply_query(),
            "shortcut_cheatsheet.selected" => self.apply_selected(),
            "shortcut_cheatsheet.result_count" => self.apply_result_count(),
            _ => {}
        }
    }

    #[cfg(test)]
    pub(in crate::visual) const fn option_state(&self) -> ShortcutCheatsheetOptionState {
        self.option_state
    }

    #[cfg(test)]
    pub(in crate::visual) fn visible_item_count(&self) -> usize {
        self.cheatsheet.visible_items().len()
    }

    #[cfg(test)]
    pub(in crate::visual) const fn callback_action(&self) -> &'static str {
        self.callback_action
    }

    fn apply_label(&mut self) {
        self.option_state.label_editor_keys = true;
        self.callback_action = "shortcut_cheatsheet_label";
    }

    fn apply_groups(&mut self) {
        self.cheatsheet = cheatsheet_with_group_count(EXPANDED_GROUP_COUNT);
        self.option_state.group_count = EXPANDED_GROUP_COUNT;
        self.callback_action = "shortcut_cheatsheet_groups";
    }

    fn apply_group_title(&mut self) {
        self.option_state.group_title_navigation = true;
        self.callback_action = "shortcut_cheatsheet_group_title";
    }

    fn apply_items(&mut self) {
        self.cheatsheet = cheatsheet_with_item_count(EXPANDED_ITEM_COUNT);
        self.option_state.item_count = EXPANDED_ITEM_COUNT;
        self.callback_action = "shortcut_cheatsheet_items";
    }

    fn apply_item_combo(&mut self) {
        self.option_state.item_combo_command_shift_p = true;
        self.callback_action = "shortcut_cheatsheet_item_combo";
    }

    fn apply_group_layout(&mut self) {
        self.cheatsheet = default_cheatsheet().group_layout(ShortcutCheatsheetLayout::OneColumn);
        self.option_state.group_layout_one_column = true;
        self.callback_action = "shortcut_cheatsheet_group_layout";
    }

    fn apply_query(&mut self) {
        let event = self
            .cheatsheet
            .apply_action(ShortcutCheatsheetAction::SetQuery(
                QUERY_CATEGORY.to_string(),
            ));
        assert_query_event(event);
        self.option_state.query_category = true;
        self.callback_action = "shortcut_cheatsheet_query";
    }

    fn apply_selected(&mut self) {
        let event = self
            .cheatsheet
            .apply_action(ShortcutCheatsheetAction::SelectShortcut(
                FORMAT_ID.to_string(),
            ));
        assert_selected_event(event);
        self.option_state.selected_format = true;
        self.callback_action = "shortcut_cheatsheet_selected";
    }

    fn apply_result_count(&mut self) {
        let event = self
            .cheatsheet
            .apply_action(ShortcutCheatsheetAction::SetQuery(
                QUERY_CATEGORY.to_string(),
            ));
        assert_query_event(event);
        self.option_state.result_count = FILTERED_RESULT_COUNT;
        self.callback_action = "shortcut_cheatsheet_result_count";
    }

    fn scroll_label(&self) -> &'static str {
        match self.scroll_offset {
            0 => "scroll=0",
            1 => "scroll=1",
            2 => "scroll=2",
            _ => "scroll=3",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) struct ShortcutCheatsheetUpdate {
    pub(in crate::visual) action: &'static str,
    pub(in crate::visual) event: &'static str,
    pub(in crate::visual) state: &'static str,
}

impl ShortcutCheatsheetUpdate {
    const fn new(action: &'static str, event: &'static str, state: &'static str) -> Self {
        Self {
            action,
            event,
            state,
        }
    }
}
