use super::screen_state::StorybookScreenState;
use super::window_interaction::color_picker_operation::ColorPickerAction;
use super::window_interaction::color_picker_update::ColorPickerUpdate;
use super::window_interaction::command_palette_state::{
    CommandPaletteStoryAction, CommandPaletteUpdate,
};
use super::window_interaction::diagnostics_list_operation::DiagnosticsListStoryAction;
use super::window_interaction::diagnostics_list_update::DiagnosticsListUpdate;
use super::window_interaction::drag_and_drop_operation::{DragAndDropAction, DragAndDropUpdate};
use super::window_interaction::dynamic_array_editor_operation::{
    DynamicArrayEditorAction, DynamicArrayUpdate,
};
use super::window_interaction::layout_operation::{LayoutStoryAction, LayoutStoryUpdate};
use super::window_interaction::scroll_area_operation::{
    ScrollAreaStoryAction, ScrollAreaStoryUpdate,
};
use super::window_interaction::settings_list_operation::SettingsListStoryAction;
use super::window_interaction::settings_list_update::SettingsListUpdate;
use super::window_interaction::split_pane_operation::{SplitPaneStoryAction, SplitPaneStoryUpdate};
use super::window_interaction::theme_tokens_operation::ThemeTokensStoryAction;

impl StorybookScreenState {
    pub(in crate::visual) fn register_command_palette_action(
        &mut self,
        action: CommandPaletteStoryAction,
    ) {
        if matches!(
            action,
            CommandPaletteStoryAction::KeyboardExecute | CommandPaletteStoryAction::KeyboardClose
        ) && !self.button_focused
        {
            self.last_action = "command_palette_keyboard_without_focus";
            self.last_event = "command_palette_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let update = self.command_palette.apply_action(action);
        if action == CommandPaletteStoryAction::Focus {
            self.button_focused = true;
        }
        if action == CommandPaletteStoryAction::Hover {
            self.preview_hovered = true;
        }
        self.apply_command_palette_update(update);
        self.last_setting = "command_palette.interaction";
        self.last_setting_value = self.command_palette.callback_action();
    }

    pub(in crate::visual) fn register_color_picker_action(&mut self, action: ColorPickerAction) {
        let update = self.color_picker.apply_action(action);
        if update.count_action {
            self.action_count += 1;
        }
        if action == ColorPickerAction::Focus {
            self.button_focused = true;
        }
        if action == ColorPickerAction::Hover {
            self.preview_hovered = true;
        }
        self.apply_color_picker_update(update);
        self.last_setting = update.setting;
        self.last_setting_value = self.color_picker.callback_action();
    }

    pub(in crate::visual) fn register_dynamic_array_editor_action(
        &mut self,
        action: DynamicArrayEditorAction,
    ) {
        if action == DynamicArrayEditorAction::KeyboardEdit && !self.button_focused {
            self.last_action = "array_keyboard_without_focus";
            self.last_event = "array_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let update = self.dynamic_array_editor.apply_action(action);
        if action == DynamicArrayEditorAction::Focus {
            self.button_focused = true;
        }
        if action == DynamicArrayEditorAction::Hover {
            self.preview_hovered = true;
        }
        self.apply_dynamic_array_editor_update(update);
        self.last_setting = "array.items";
        self.last_setting_value = self.dynamic_array_editor.callback_event();
    }

    pub(in crate::visual) fn register_settings_list_action(
        &mut self,
        action: SettingsListStoryAction,
    ) {
        if action == SettingsListStoryAction::KeyboardNext && !self.button_focused {
            self.last_action = "settings_keyboard_without_focus";
            self.last_event = "settings_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let update = self.settings_list.apply_action(action);
        if action == SettingsListStoryAction::FocusField {
            self.button_focused = true;
        }
        if action == SettingsListStoryAction::HoverField {
            self.preview_hovered = true;
        }
        self.apply_settings_list_update(update);
        self.last_setting = update.setting;
        self.last_setting_value = self.settings_list.callback_action();
    }

    pub(in crate::visual) fn register_diagnostics_list_action(
        &mut self,
        action: DiagnosticsListStoryAction,
    ) {
        if action == DiagnosticsListStoryAction::KeyboardNavigate && !self.button_focused {
            self.last_action = "diagnostic_keyboard_without_focus";
            self.last_event = "diagnostic_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let update = self.diagnostics_list.apply_action(action);
        if action == DiagnosticsListStoryAction::FocusList {
            self.button_focused = true;
        }
        if action == DiagnosticsListStoryAction::HoverItem {
            self.preview_hovered = true;
        }
        self.apply_diagnostics_list_update(update);
        self.last_setting = update.setting;
        self.last_setting_value = self.diagnostics_list.callback_action();
    }

    pub(in crate::visual) fn register_drag_and_drop_action(&mut self, action: DragAndDropAction) {
        self.action_count += 1;
        let update = self.drag_and_drop.apply_action(action);
        self.apply_drag_and_drop_update(update);
        self.last_setting = "drag.session";
        self.last_setting_value = self.state_label;
    }

    pub(in crate::visual) fn register_layout_action(&mut self, action: LayoutStoryAction) {
        self.action_count += 1;
        let update = self.layout.apply_action(action);
        if matches!(
            action,
            LayoutStoryAction::RowFocus
                | LayoutStoryAction::ColumnFocus
                | LayoutStoryAction::StackFocus
                | LayoutStoryAction::GridFocus
                | LayoutStoryAction::AlignCenterFocus
        ) {
            self.button_focused = true;
        }
        if matches!(
            action,
            LayoutStoryAction::RowHover
                | LayoutStoryAction::ColumnHover
                | LayoutStoryAction::StackHover
                | LayoutStoryAction::GridHover
                | LayoutStoryAction::AlignCenterHover
        ) {
            self.preview_hovered = true;
        }
        self.apply_layout_update(update);
        self.last_setting = "layout.interaction";
        self.last_setting_value = self.layout.callback();
    }

    pub(in crate::visual) fn register_scroll_area_action(&mut self, action: ScrollAreaStoryAction) {
        if action == ScrollAreaStoryAction::Keyboard && !self.button_focused {
            self.last_action = "scroll_area_keyboard_without_focus";
            self.last_event = "scroll_area_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let update = self.scroll_area.apply_action(action);
        if action == ScrollAreaStoryAction::Focus {
            self.button_focused = true;
        }
        if action == ScrollAreaStoryAction::Hover {
            self.preview_hovered = true;
        }
        self.apply_scroll_area_update(update);
        self.last_setting = "scroll_area.interaction";
        self.last_setting_value = self.scroll_area.callback();
    }

    pub(in crate::visual) fn register_split_pane_action(&mut self, action: SplitPaneStoryAction) {
        if action == SplitPaneStoryAction::Keyboard && !self.button_focused {
            self.last_action = "split_pane_keyboard_without_focus";
            self.last_event = "split_pane_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let update = self.split_pane.apply_action(action);
        if action == SplitPaneStoryAction::Focus {
            self.button_focused = true;
        }
        if action == SplitPaneStoryAction::Hover {
            self.preview_hovered = true;
        }
        self.apply_split_pane_update(update);
        self.last_setting = "split_pane.interaction";
        self.last_setting_value = self.split_pane.callback();
    }

    pub(in crate::visual) fn register_theme_tokens_action(
        &mut self,
        action: ThemeTokensStoryAction,
    ) {
        if action == ThemeTokensStoryAction::Keyboard && !self.button_focused {
            self.last_action = "theme_token_keyboard_without_focus";
            self.last_event = "theme_token_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let update = self.theme_tokens.apply_action(action);
        if action == ThemeTokensStoryAction::Focus {
            self.button_focused = true;
        }
        if action == ThemeTokensStoryAction::Hover {
            self.preview_hovered = true;
        }
        self.apply_theme_tokens_update(update);
        self.last_setting = "theme_tokens.interaction";
        self.last_setting_value = self.theme_tokens.callback();
    }

    pub(in crate::visual) fn apply_dynamic_array_editor_update(
        &mut self,
        update: DynamicArrayUpdate,
    ) {
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn apply_color_picker_update(&mut self, update: ColorPickerUpdate) {
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn apply_command_palette_update(&mut self, update: CommandPaletteUpdate) {
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn apply_settings_list_update(&mut self, update: SettingsListUpdate) {
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn apply_diagnostics_list_update(
        &mut self,
        update: DiagnosticsListUpdate,
    ) {
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn apply_drag_and_drop_update(&mut self, update: DragAndDropUpdate) {
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn apply_layout_update(&mut self, update: LayoutStoryUpdate) {
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn apply_scroll_area_update(&mut self, update: ScrollAreaStoryUpdate) {
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn apply_split_pane_update(&mut self, update: SplitPaneStoryUpdate) {
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }
}
