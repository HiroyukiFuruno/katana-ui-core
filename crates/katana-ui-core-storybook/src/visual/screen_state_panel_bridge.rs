use super::panel_screen_state::{PanelChildKey, PanelOptionControl, PanelScreenUpdate};
use super::screen_state::StorybookScreenState;
use super::storybook_ui_option_contract::StorybookUiOptionContract;

impl StorybookScreenState {
    pub(in crate::visual) fn register_panel_option(&mut self, control: PanelOptionControl) {
        self.settings_revision += 1;
        let update = self.panel.apply_option(control);
        self.apply_panel_update(update);
    }

    pub(in crate::visual) fn register_panel_active_child(&mut self, panel: PanelChildKey) {
        self.action_count += 1;
        let update = self
            .panel
            .apply_option(PanelOptionControl::ActivePanel(panel));
        self.apply_panel_update(update);
    }

    pub(in crate::visual) fn register_panel_hover(&mut self) {
        self.action_count += 1;
        self.preview_hovered = true;
        let update = self.panel.apply_hover();
        self.apply_panel_update(update);
    }

    pub(in crate::visual) fn register_panel_focus(&mut self) {
        self.action_count += 1;
        self.button_focused = true;
        let update = self.panel.apply_focus();
        self.apply_panel_update(update);
    }

    pub(in crate::visual) fn register_panel_keyboard_scroll(&mut self) {
        self.action_count += 1;
        self.button_focused = true;
        let update = self.panel.apply_keyboard_scroll();
        self.apply_panel_update(update);
    }

    pub(in crate::visual) fn register_panel_resize(&mut self) {
        self.action_count += 1;
        let update = self.panel.apply_resize();
        self.apply_panel_update(update);
    }

    pub(in crate::visual) fn scroll_panel_vertical(
        &mut self,
        panel: PanelChildKey,
        delta_y: f32,
    ) -> bool {
        if !self.panel.scroll_vertical(panel, delta_y) {
            return false;
        }
        self.action_count += 1;
        self.last_action = "panel_wheel_y";
        self.last_event = "panel_scroll_changed";
        self.last_setting = "panel.vertical_scroll";
        self.last_setting_value = "wheel";
        self.state_label = "panel_scroll_y=changed";
        true
    }

    pub(in crate::visual) fn scroll_panel_horizontal(
        &mut self,
        panel: PanelChildKey,
        delta_x: f32,
    ) -> bool {
        if !self.panel.scroll_horizontal(panel, delta_x) {
            return false;
        }
        self.action_count += 1;
        self.last_action = "panel_wheel_x";
        self.last_event = "panel_scroll_changed";
        self.last_setting = "panel.horizontal_scroll";
        self.last_setting_value = "wheel";
        self.state_label = "panel_scroll_x=changed";
        true
    }

    pub(in crate::visual) fn apply_panel_update(&mut self, update: PanelScreenUpdate) {
        self.last_action = update.action;
        self.last_event = update.event;
        self.last_setting = update.setting;
        self.last_setting_value = update.value;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_panel_contract_setting(
        &mut self,
        option: StorybookUiOptionContract,
    ) -> bool {
        let control = match option.setting {
            "active_panel" => PanelOptionControl::ActivePanel(PanelChildKey::Details),
            "scrollbar_visibility" => PanelOptionControl::ScrollbarVisible(false),
            _ => return false,
        };
        self.settings_revision += 1;
        let update = self.panel.apply_option(control);
        self.apply_panel_update(update);
        self.last_setting = option.setting;
        self.last_setting_value = option.after;
        true
    }
}
