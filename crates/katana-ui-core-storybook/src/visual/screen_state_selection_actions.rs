use super::{
    CollapsiblePanelStoryAction, SideMenuScreenAction, StorybookScreenState,
    VirtualizationStoryAction,
};

impl StorybookScreenState {
    pub(in crate::visual) fn register_virtualization_action(
        &mut self,
        action: VirtualizationStoryAction,
    ) {
        if action == VirtualizationStoryAction::KeyboardFocus && !self.button_focused {
            self.last_action = "virtualized_keyboard_without_focus";
            self.last_event = "virtualized_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let update = self.virtualization.apply(action);
        if action == VirtualizationStoryAction::Focus {
            self.button_focused = true;
        }
        if action == VirtualizationStoryAction::Hover {
            self.preview_hovered = true;
        }
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }
}

impl StorybookScreenState {
    pub(in crate::visual) fn register_collapsible_panel_action(
        &mut self,
        action: CollapsiblePanelStoryAction,
    ) {
        if action == CollapsiblePanelStoryAction::KeyboardToggle && !self.button_focused {
            self.last_action = "collapsible_panel_keyboard_without_focus";
            self.last_event = "collapsible_panel_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let update = self.collapsible_panel.apply(action);
        if action == CollapsiblePanelStoryAction::Focus {
            self.button_focused = true;
        }
        if action == CollapsiblePanelStoryAction::Hover {
            self.preview_hovered = true;
        }
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }
}

impl StorybookScreenState {
    pub(in crate::visual) fn register_shortcut_cheatsheet_preview(&mut self) {
        self.action_count += 1;
        let update = self.shortcut_cheatsheet.select_format();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_shortcut_cheatsheet_focus(&mut self) {
        self.action_count += 1;
        self.button_focused = true;
        let update = self.shortcut_cheatsheet.focus();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_shortcut_cheatsheet_hover(&mut self) {
        self.action_count += 1;
        self.preview_hovered = true;
        let update = self.shortcut_cheatsheet.hover();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_shortcut_cheatsheet_keyboard_select(&mut self) {
        if !self.button_focused {
            self.last_action = "shortcut_cheatsheet_keyboard_without_focus";
            self.last_event = "shortcut_cheatsheet_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let update = self.shortcut_cheatsheet.keyboard_select();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_shortcut_cheatsheet_scroll(&mut self) {
        self.action_count += 1;
        let update = self.shortcut_cheatsheet.scroll_results();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }
}

impl StorybookScreenState {
    pub(in crate::visual) fn register_shortcut_combo_preview(&mut self) {
        self.action_count += 1;
        let update = self.runtime_structured.shortcut_combo.preview_platform();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_shortcut_combo_focus(&mut self) {
        self.action_count += 1;
        self.button_focused = true;
        let update = self.runtime_structured.shortcut_combo.focus();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_shortcut_combo_hover(&mut self) {
        self.action_count += 1;
        self.preview_hovered = true;
        let update = self.runtime_structured.shortcut_combo.hover();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_shortcut_combo_keyboard_preview(&mut self) {
        if !self.button_focused {
            self.last_action = "shortcut_combo_keyboard_without_focus";
            self.last_event = "shortcut_combo_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.register_shortcut_combo_preview();
    }
}

impl StorybookScreenState {
    pub(in crate::visual) fn register_side_menu_action(&mut self, action: SideMenuScreenAction) {
        if action == SideMenuScreenAction::KeyboardNext && !self.button_focused {
            self.last_action = "side_menu_keyboard_without_focus";
            self.last_event = "side_menu_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let update = self.side_menu.apply(action);
        if action == SideMenuScreenAction::Focus {
            self.button_focused = true;
        }
        if action == SideMenuScreenAction::Hover {
            self.preview_hovered = true;
        }
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }
}
