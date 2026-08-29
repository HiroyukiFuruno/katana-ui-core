use super::screen_state::StorybookScreenState;
use super::screen_state_search_control::SearchControlScreenAction;
use super::screen_state_segmented_toggle::SegmentedToggleScreenAction;
use super::search_box_screen_state::SearchBoxScreenAction;
use super::selection_screen_state::SelectionScreenAction;

impl StorybookScreenState {
    pub(in crate::visual) fn register_list_select(&mut self, index: usize) {
        self.action_count += 1;
        let update = self.list.select_row(index);
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_list_focus(&mut self) {
        self.action_count += 1;
        self.button_focused = true;
        let update = self.list.focus_row(1);
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_list_keyboard_next(&mut self) {
        if !self.button_focused {
            self.last_action = "list_keyboard_without_focus";
            self.last_event = "list_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let update = self.list.keyboard_next();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_list_scroll(&mut self) {
        self.action_count += 1;
        let update = self.list.scroll_virtual_range();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_selection_action(&mut self, action: SelectionScreenAction) {
        if matches!(
            action,
            SelectionScreenAction::ComboKeyboardSelect
                | SelectionScreenAction::SelectKeyboardSelect
        ) && !self.button_focused
        {
            self.last_action = "selection_keyboard_without_focus";
            self.last_event = "selection_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let update = self.selection.apply(action);
        if matches!(
            action,
            SelectionScreenAction::ComboFocus
                | SelectionScreenAction::SelectFocus
                | SelectionScreenAction::SelectionListFocus
        ) {
            self.button_focused = true;
        }
        if matches!(
            action,
            SelectionScreenAction::ComboHover
                | SelectionScreenAction::SelectHover
                | SelectionScreenAction::SelectionListHover
        ) {
            self.preview_hovered = true;
        }
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_search_box_action(&mut self, action: SearchBoxScreenAction) {
        if action == SearchBoxScreenAction::KeyboardSubmit && !self.button_focused {
            self.last_action = "search_keyboard_without_focus";
            self.last_event = "search_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let update = self.search_box.apply(action);
        if action == SearchBoxScreenAction::Focus {
            self.button_focused = true;
        }
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_search_control_action(
        &mut self,
        action: SearchControlScreenAction,
    ) {
        if action == SearchControlScreenAction::KeyboardNext && !self.button_focused {
            self.last_action = "search_control_keyboard_without_focus";
            self.last_event = "search_control_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let update = self.search_control.apply(action);
        if action == SearchControlScreenAction::Focus {
            self.button_focused = true;
        }
        if action == SearchControlScreenAction::Hover {
            self.preview_hovered = true;
        }
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_segmented_toggle_action(
        &mut self,
        action: SegmentedToggleScreenAction,
    ) {
        if action == SegmentedToggleScreenAction::KeyboardSelect && !self.button_focused {
            self.last_action = "segment_keyboard_without_focus";
            self.last_event = "segment_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let update = self.segmented_toggle.apply(action);
        if action == SegmentedToggleScreenAction::Focus {
            self.button_focused = true;
        }
        if action == SegmentedToggleScreenAction::Hover {
            self.preview_hovered = true;
        }
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_menu_focus(&mut self) {
        self.action_count += 1;
        self.button_focused = true;
        self.last_action = "menu_focus";
        self.last_event = "menu_focused";
        self.state_label = "focused=true";
    }

    pub(in crate::visual) fn register_menu_keyboard_open(&mut self) {
        if !self.button_focused {
            self.last_action = "menu_keyboard_without_focus";
            self.last_event = "menu_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        self.selection.select_open = true;
        self.last_action = "menu_keyboard_open";
        self.last_event = "menu_opened";
        self.state_label = "open=true";
    }

    pub(in crate::visual) fn register_menu_context_dismiss(&mut self) {
        self.action_count += 1;
        self.selection.select_open = false;
        self.last_action = "menu_context_dismiss";
        self.last_event = "menu_closed";
        self.state_label = "open=false";
    }

    pub(in crate::visual) fn register_menu_button_open(&mut self) {
        self.action_count += 1;
        let result = self.selection.apply_core_menu_button_open(true);
        self.selection.select_open = result.handled && result.after.open;
        self.last_action = "menu_button_open";
        self.last_event = "menu_button_opened";
        self.state_label = "open=true";
    }

    pub(in crate::visual) fn register_menu_button_close(&mut self) {
        self.action_count += 1;
        let result = self.selection.apply_core_menu_button_open(false);
        self.selection.select_open = result.handled && result.after.open;
        self.last_action = "menu_button_close";
        self.last_event = "menu_button_closed";
        self.state_label = "open=false";
    }

    pub(in crate::visual) fn register_menu_button_select(&mut self, index: usize) {
        self.action_count += 1;
        let result = self.selection.apply_core_menu_button_selected(index);
        if result.handled {
            self.selection.select_open = result.after.open;
            self.selection.select_selected_index = Some(result.after.selected_index);
        }
        self.last_action = "menu_button_select";
        self.last_event = "menu_button_item_selected";
        self.state_label = menu_button_selected_label(index);
    }

    pub(in crate::visual) fn register_menu_button_focus(&mut self, disabled: bool) {
        if disabled {
            self.last_action = "menu_button_focus_blocked";
            self.last_event = "menu_button_focus_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        self.button_focused = true;
        self.last_action = "menu_button_focus";
        self.last_event = "menu_button_focused";
        self.state_label = "focused=true";
    }

    pub(in crate::visual) fn register_menu_button_keyboard_open(&mut self, disabled: bool) {
        if disabled {
            self.last_action = "menu_button_keyboard_blocked";
            self.last_event = "menu_button_keyboard_ignored";
            self.state_label = "keyboard=false";
            return;
        }
        if !self.button_focused {
            self.last_action = "menu_button_keyboard_without_focus";
            self.last_event = "menu_button_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let result = self.selection.apply_core_menu_button_open(true);
        self.selection.select_open = result.handled && result.after.open;
        self.last_action = "menu_button_keyboard_open";
        self.last_event = "menu_button_opened";
        self.state_label = "open=true";
    }

    pub(in crate::visual) fn register_menu_button_context_open(&mut self, disabled: bool) {
        if disabled {
            self.last_action = "menu_button_context_blocked";
            self.last_event = "menu_button_context_ignored";
            self.state_label = "context_menu=false";
            return;
        }
        self.action_count += 1;
        let result = self.selection.apply_core_menu_button_open(true);
        self.selection.select_open = result.handled && result.after.open;
        self.last_action = "menu_button_context_open";
        self.last_event = "menu_button_opened";
        self.state_label = "open=true";
    }

    pub(in crate::visual) fn register_menu_button_disabled_trigger(&mut self) {
        self.last_action = "menu_button_disabled_trigger";
        self.last_event = "menu_button_disabled_ignored";
        self.state_label = "disabled=true";
        self.selection.select_open = false;
    }
}

fn menu_button_selected_label(index: usize) -> &'static str {
    match index {
        0 => "selected=new-file",
        1 => "selected=rename",
        _ => "selected=unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_and_selection_actions_cover_focus_hover_keyboard_and_reset_paths() {
        let mut state = StorybookScreenState::default();
        state.register_list_keyboard_next();
        assert_eq!(state.last_event, "list_keyboard_ignored");
        state.register_list_select(2);
        state.register_list_focus();
        state.register_list_keyboard_next();
        state.register_list_scroll();

        state.button_focused = false;
        state.register_selection_action(SelectionScreenAction::ComboKeyboardSelect);
        assert_eq!(state.last_event, "selection_keyboard_ignored");
        state.register_selection_action(SelectionScreenAction::SelectKeyboardSelect);
        assert_eq!(state.last_event, "selection_keyboard_ignored");

        let actions = [
            SelectionScreenAction::SelectStateRead,
            SelectionScreenAction::SelectOpen,
            SelectionScreenAction::SelectClose,
            SelectionScreenAction::SelectOption(2),
            SelectionScreenAction::SelectFocus,
            SelectionScreenAction::SelectHover,
            SelectionScreenAction::SelectKeyboardSelect,
            SelectionScreenAction::SelectScroll,
            SelectionScreenAction::SelectReset,
            SelectionScreenAction::ComboStateRead,
            SelectionScreenAction::ComboFilter,
            SelectionScreenAction::ComboOption(2),
            SelectionScreenAction::ComboFocus,
            SelectionScreenAction::ComboHover,
            SelectionScreenAction::ComboKeyboardSelect,
            SelectionScreenAction::ComboReset,
            SelectionScreenAction::SelectionListStateRead,
            SelectionScreenAction::SelectionListSelectRow(2),
            SelectionScreenAction::SelectionListMultiToggle(7),
            SelectionScreenAction::SelectionListFocus,
            SelectionScreenAction::SelectionListHover,
            SelectionScreenAction::SelectionListKeyboardNext,
            SelectionScreenAction::SelectionListScroll,
            SelectionScreenAction::SelectionListReset,
            SelectionScreenAction::SelectionListToggle(1),
        ];
        for action in actions {
            state.register_selection_action(action);
        }

        assert!(state.button_focused);
        assert!(state.preview_hovered);
        assert!(state.action_count >= actions.len());
    }

    #[test]
    fn search_and_segment_actions_cover_rejected_and_accepted_keyboard_paths() {
        let mut search_box = StorybookScreenState::default();
        search_box.register_search_box_action(SearchBoxScreenAction::KeyboardSubmit);
        assert_eq!(search_box.last_event, "search_keyboard_ignored");
        for action in [
            SearchBoxScreenAction::StateRead,
            SearchBoxScreenAction::TypeQuery,
            SearchBoxScreenAction::Submit,
            SearchBoxScreenAction::Clear,
            SearchBoxScreenAction::Focus,
            SearchBoxScreenAction::KeyboardSubmit,
            SearchBoxScreenAction::ToggleCase,
            SearchBoxScreenAction::ToggleRegex,
        ] {
            search_box.register_search_box_action(action);
        }
        assert!(search_box.button_focused);

        let mut search_control = StorybookScreenState::default();
        search_control.register_search_control_action(SearchControlScreenAction::KeyboardNext);
        assert_eq!(search_control.last_event, "search_control_keyboard_ignored");
        for action in [
            SearchControlScreenAction::Query,
            SearchControlScreenAction::ToggleRegex,
            SearchControlScreenAction::Focus,
            SearchControlScreenAction::Hover,
            SearchControlScreenAction::KeyboardNext,
        ] {
            search_control.register_search_control_action(action);
        }
        assert!(search_control.button_focused);
        assert!(search_control.preview_hovered);

        let mut segmented = StorybookScreenState::default();
        segmented.register_segmented_toggle_action(SegmentedToggleScreenAction::KeyboardSelect);
        assert_eq!(segmented.last_event, "segment_keyboard_ignored");
        for action in [
            SegmentedToggleScreenAction::Select,
            SegmentedToggleScreenAction::Focus,
            SegmentedToggleScreenAction::Hover,
            SegmentedToggleScreenAction::KeyboardSelect,
            SegmentedToggleScreenAction::DisabledSelect,
        ] {
            segmented.register_segmented_toggle_action(action);
        }
        assert!(segmented.button_focused);
        assert!(segmented.preview_hovered);
    }

    #[test]
    fn menu_actions_cover_focus_open_select_close_context_and_disabled_paths() {
        let mut state = StorybookScreenState::default();
        state.register_menu_keyboard_open();
        assert_eq!(state.last_event, "menu_keyboard_ignored");
        state.register_menu_focus();
        state.register_menu_keyboard_open();
        state.register_menu_context_dismiss();

        state.register_menu_button_focus(true);
        assert_eq!(state.last_event, "menu_button_focus_ignored");
        state.register_menu_button_keyboard_open(true);
        assert_eq!(state.last_event, "menu_button_keyboard_ignored");
        state.register_menu_button_context_open(true);
        assert_eq!(state.last_event, "menu_button_context_ignored");
        state.register_menu_button_disabled_trigger();

        state.button_focused = false;
        state.register_menu_button_keyboard_open(false);
        assert_eq!(state.state_label, "focused=false");
        state.register_menu_button_focus(false);
        state.register_menu_button_open();
        state.register_menu_button_select(0);
        state.register_menu_button_select(1);
        state.register_menu_button_select(9);
        state.register_menu_button_close();
        state.register_menu_button_keyboard_open(false);
        state.register_menu_button_context_open(false);

        assert!(state.button_focused);
        assert!(state.selection.select_open);
        assert_eq!(menu_button_selected_label(0), "selected=new-file");
        assert_eq!(menu_button_selected_label(1), "selected=rename");
        assert_eq!(menu_button_selected_label(2), "selected=unknown");
    }
}
