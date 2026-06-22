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
