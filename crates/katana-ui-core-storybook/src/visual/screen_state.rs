use super::button_options::{StorybookButtonOptionControl, StorybookButtonOptions};
use super::interaction_spec::StorybookInteractionSpec;
use super::panel_screen_state::{
    PanelChildKey, PanelOptionControl, PanelScreenState, PanelScreenUpdate,
};
use super::screen_state_forms::{
    apply_checkbox_checked_state, apply_radio_selected_state, checkbox_state_label,
    radio_state_label,
};
use super::screen_state_settings::{format_setting_action, format_setting_event};
use super::screen_state_tabs::TabsScreenState;
use super::search_box_screen_state::SearchBoxScreenState;
use super::selection_screen_state::SelectionScreenState;
use super::text_input_screen_state::TextInputStateStore;
use katana_ui_core::state::UiComponentState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct StorybookScreenState {
    pub(super) action_count: usize,
    pub(super) settings_revision: usize,
    pub(super) last_action: &'static str,
    pub(super) last_event: &'static str,
    pub(super) last_setting: &'static str,
    pub(super) last_setting_value: &'static str,
    pub(super) state_label: &'static str,
    pub(super) button_options: StorybookButtonOptions,
    pub(super) button_pressed: bool,
    pub(super) preview_hovered: bool,
    pub(super) hovered_text_input_icon_button_index: Option<usize>,
    pub(super) hovered_summary_index: Option<usize>,
    pub(super) selection: SelectionScreenState,
    pub(super) search_box: SearchBoxScreenState,
    pub(super) tabs: TabsScreenState,
    pub(super) panel: PanelScreenState,
    pub(super) checkbox_state: UiComponentState,
    pub(super) radio_state: UiComponentState,
    pub(super) text_inputs: TextInputStateStore,
    pub(super) text_area_value: String,
    pub(super) text_area_focused: bool,
    pub(super) text_area_uses_live_value: bool,
    pub(super) text_area_caret_visible: bool,
    pub(super) text_area_wrap_enabled: bool,
    pub(super) text_area_resize_enabled: bool,
    pub(super) text_area_vertical_scroll_enabled: bool,
    pub(super) text_area_horizontal_scroll_enabled: bool,
    pub(super) text_area_vertical_scrollbar_visible: bool,
    pub(super) text_area_horizontal_scrollbar_visible: bool,
    pub(super) text_area_scroll_offset: usize,
    pub(super) text_area_scroll_x_offset: usize,
    pub(super) text_area_resize_width_delta: usize,
    pub(super) text_area_resize_height_delta: usize,
}

impl StorybookScreenState {
    pub(super) fn register_button_click(&mut self, page: &str) {
        if self.button_options.disabled {
            self.last_action = "button_press_blocked";
            self.last_event = "button_disabled_ignored";
            self.state_label = "disabled=true";
            return;
        }
        self.action_count += 1;
        self.button_pressed = true;
        let spec = StorybookInteractionSpec::for_page(page);
        self.last_action = spec.action;
        self.last_event = spec.event;
        self.state_label = spec.state;
    }

    pub(super) fn register_preview_action(&mut self, page: &str) {
        if page == "checkbox" {
            self.register_checkbox_toggle();
            return;
        }
        if page == "radio" {
            self.register_radio_select();
            return;
        }
        if page == "panel" {
            self.action_count += 1;
            let update = self.panel.apply_preview_action();
            self.apply_panel_update(update);
            return;
        }
        if page == "tabs" {
            self.register_tabs_preview_action();
            return;
        }
        self.action_count += 1;
        let spec = StorybookInteractionSpec::for_page(page);
        self.last_action = spec.action;
        self.last_event = spec.event;
        self.state_label = spec.state;
    }

    pub(super) fn register_checkbox_state_read(&mut self) {
        self.action_count += 1;
        self.last_action = "checkbox_state_read";
        self.last_event = "checked_read";
        self.state_label =
            checkbox_state_label(self.checkbox_state.checked, self.checkbox_state.checked);
    }

    pub(super) fn register_checkbox_toggle(&mut self) {
        self.action_count += 1;
        let before = self.checkbox_state.checked;
        self.checkbox_state = apply_checkbox_checked_state(&self.checkbox_state, !before);
        self.last_action = "checkbox_toggle";
        self.last_event = "checked_changed";
        self.state_label = checkbox_state_label(before, self.checkbox_state.checked);
    }

    pub(super) fn register_checkbox_reset(&mut self) {
        self.action_count += 1;
        let before = self.checkbox_state.checked;
        self.checkbox_state = apply_checkbox_checked_state(&self.checkbox_state, false);
        self.last_action = "checkbox_reset";
        self.last_event = "checked_changed";
        self.state_label = checkbox_state_label(before, self.checkbox_state.checked);
    }

    pub(super) fn register_radio_state_read(&mut self) {
        self.action_count += 1;
        self.last_action = "radio_state_read";
        self.last_event = "selected_read";
        self.state_label = radio_state_label(self.radio_state.checked, self.radio_state.checked);
    }

    pub(super) fn register_radio_select(&mut self) {
        self.action_count += 1;
        let before = self.radio_state.checked;
        self.radio_state = apply_radio_selected_state(&self.radio_state, true);
        self.last_action = "radio_select";
        self.last_event = "radio_selected";
        self.state_label = radio_state_label(before, self.radio_state.checked);
    }

    pub(super) fn register_radio_reset(&mut self) {
        self.action_count += 1;
        let before = self.radio_state.checked;
        self.radio_state = apply_radio_selected_state(&self.radio_state, false);
        self.last_action = "radio_reset";
        self.last_event = "radio_selected";
        self.state_label = radio_state_label(before, self.radio_state.checked);
    }

    pub(super) fn register_settings_change(&mut self, page: &str) {
        if page == "panel" {
            self.settings_revision += 1;
            let update = self
                .panel
                .apply_option(PanelOptionControl::ScrollbarVisible(false));
            self.apply_panel_update(update);
            return;
        }
        if page == "text-area" {
            self.register_text_area_resize_toggle();
            return;
        }
        if page == "tabs" {
            self.register_tabs_setting_change();
            return;
        }
        self.settings_revision += 1;
        let spec = StorybookInteractionSpec::for_page(page);
        self.last_action = format_setting_action(spec.option);
        self.last_event = format_setting_event(page);
        self.last_setting = spec.option;
        self.last_setting_value = spec.after;
        self.state_label = spec.state;
    }

    pub(super) fn register_button_option(&mut self, control: StorybookButtonOptionControl) {
        self.settings_revision += 1;
        self.button_options.toggle(control);
        self.last_action = "button_option_apply";
        self.last_event = "button_option_changed";
        self.last_setting = control.setting_name();
        self.last_setting_value = control.setting_value(self.button_options);
        self.state_label = control.state_label(self.button_options);
    }

    pub(super) fn register_panel_option(&mut self, control: PanelOptionControl) {
        self.settings_revision += 1;
        let update = self.panel.apply_option(control);
        self.apply_panel_update(update);
    }

    pub(super) fn register_panel_active_child(&mut self, panel: PanelChildKey) {
        self.action_count += 1;
        let update = self
            .panel
            .apply_option(PanelOptionControl::ActivePanel(panel));
        self.apply_panel_update(update);
    }

    pub(super) fn set_preview_hovered(&mut self, hovered: bool) -> bool {
        if self.preview_hovered == hovered {
            return false;
        }
        self.preview_hovered = hovered;
        true
    }

    pub(super) fn set_hovered_text_input_icon_button_index(
        &mut self,
        index: Option<usize>,
    ) -> bool {
        if self.hovered_text_input_icon_button_index == index {
            return false;
        }
        self.hovered_text_input_icon_button_index = index;
        true
    }

    pub(super) fn set_hovered_summary_index(&mut self, index: Option<usize>) -> bool {
        if self.hovered_summary_index == index {
            return false;
        }
        self.hovered_summary_index = index;
        true
    }

    pub(super) fn has_widget_action(&self) -> bool {
        self.action_count > 0
    }

    pub(super) const fn is_button_pressed(&self) -> bool {
        self.button_pressed
    }

    pub(super) fn release_button_press(&mut self) -> bool {
        if !self.button_pressed {
            return false;
        }
        self.button_pressed = false;
        self.state_label = "pressed=false";
        true
    }

    pub(super) fn has_settings_override(&self) -> bool {
        self.settings_revision % 2 == 1
    }

    pub(super) fn scroll_panel_vertical(&mut self, panel: PanelChildKey, delta_y: f32) -> bool {
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

    pub(super) fn scroll_panel_horizontal(&mut self, panel: PanelChildKey, delta_x: f32) -> bool {
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

    pub(super) const fn is_checkbox_checked(&self) -> bool {
        self.checkbox_state.checked
    }

    pub(super) const fn is_radio_selected(&self) -> bool {
        self.radio_state.checked
    }

    #[cfg(test)]
    pub(super) fn checkbox_state_snapshot(&self) -> &UiComponentState {
        &self.checkbox_state
    }

    #[cfg(test)]
    pub(super) fn radio_state_snapshot(&self) -> &UiComponentState {
        &self.radio_state
    }

    fn apply_panel_update(&mut self, update: PanelScreenUpdate) {
        self.last_action = update.action;
        self.last_event = update.event;
        self.last_setting = update.setting;
        self.last_setting_value = update.value;
        self.state_label = update.state;
    }
}
