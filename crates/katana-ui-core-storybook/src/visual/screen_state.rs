use super::button_options::{StorybookButtonOptionControl, StorybookButtonOptions};
use super::interaction_spec::StorybookInteractionSpec;
use super::selection_screen_state::{SelectionScreenAction, SelectionScreenState};
use katana_ui_core::atom;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
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
    pub(super) hovered_summary_index: Option<usize>,
    pub(super) selection: SelectionScreenState,
    pub(super) checkbox_state: UiComponentState,
    pub(super) radio_state: UiComponentState,
}

impl Default for StorybookScreenState {
    fn default() -> Self {
        Self {
            action_count: 0,
            settings_revision: 0,
            last_action: "none",
            last_event: "none",
            last_setting: "none",
            last_setting_value: "none",
            state_label: "idle",
            button_options: StorybookButtonOptions::default(),
            button_pressed: false,
            preview_hovered: false,
            hovered_summary_index: None,
            selection: SelectionScreenState::default(),
            checkbox_state: default_checkbox_state(),
            radio_state: default_radio_state(),
        }
    }
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
        self.state_label = checkbox_state_label(self.checkbox_state.checked, self.checkbox_state.checked);
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

    pub(super) fn set_preview_hovered(&mut self, hovered: bool) -> bool {
        if self.preview_hovered == hovered {
            return false;
        }
        self.preview_hovered = hovered;
        true
    }

    pub(super) fn set_hovered_summary_index(&mut self, index: Option<usize>) -> bool {
        if self.hovered_summary_index == index {
            return false;
        }
        self.hovered_summary_index = index;
        true
    }

    pub(super) fn register_context_menu(&mut self, page: &str) {
        if page != "tree-view" && page != "context-menu" {
            return;
        }
        self.action_count += 1;
        if page == "tree-view" {
            self.last_action = "tree_context_menu";
            self.last_event = "tree_context_opened";
            self.last_setting = "empty_area_context_menu";
            self.last_setting_value = "visible";
            self.state_label = "context_menu=open";
            return;
        }
        let spec = StorybookInteractionSpec::for_page(page);
        self.last_action = spec.action;
        self.last_event = spec.event;
        self.last_setting = spec.option;
        self.last_setting_value = spec.after;
        self.state_label = spec.state;
    }

    pub(super) fn register_selection_action(&mut self, action: SelectionScreenAction) {
        self.action_count += 1;
        let update = self.selection.apply(action);
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
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
}

fn default_checkbox_state() -> UiComponentState {
    atom::Checkbox::new("Storybook Checkbox").state_snapshot()
}

fn default_radio_state() -> UiComponentState {
    atom::Radio::new("Storybook Radio").selected(false).state_snapshot()
}

fn apply_checkbox_checked_state(before: &UiComponentState, checked: bool) -> UiComponentState {
    let mut checkbox = atom::Checkbox::new("Storybook Checkbox").set_state(before.clone());
    let _result = checkbox.apply_action(&UiAction::checkbox_checked(before.state_id.clone(), checked));
    checkbox.state_snapshot()
}

fn apply_radio_selected_state(before: &UiComponentState, selected: bool) -> UiComponentState {
    let mut radio = atom::Radio::new("Storybook Radio").set_state(before.clone());
    if !selected {
        radio = radio.selected(false);
    }
    if selected {
        let _result = radio.apply_action(&UiAction::radio_selected(before.state_id.clone()));
    }
    radio.state_snapshot()
}

fn checkbox_state_label(before: bool, after: bool) -> &'static str {
    match (before, after) {
        (false, true) => "before=false after=true",
        (true, false) => "before=true after=false",
        (true, true) => "before=true after=true",
        (false, false) => "before=false after=false",
    }
}

fn radio_state_label(before: bool, after: bool) -> &'static str {
    match (before, after) {
        (false, true) => "before=false after=true",
        (true, false) => "before=true after=false",
        (true, true) => "before=true after=true",
        (false, false) => "before=false after=false",
    }
}

fn format_setting_action(option: &str) -> &'static str {
    match option {
        "theme_id" => "settings_theme_id",
        "text.role" => "settings_text_role",
        "icon.svg_source" => "settings_icon_svg_source",
        "interaction.open" => "settings_interaction_open",
        "interaction.selected_index" => "settings_selected_index",
        "interaction.value" => "settings_interaction_value",
        "color_swatch.selected_color" => "settings_color_value",
        "layout.align" => "settings_layout_align",
        "context_menu.anchor" => "settings_context_menu_anchor",
        _ => "settings_option_changed",
    }
}

fn format_setting_event(page: &str) -> &'static str {
    match page {
        "theme-tokens" => "theme_settings_changed",
        "text" => "text_settings_changed",
        "icon" => "icon_settings_changed",
        "button" | "text-button" | "svg-button" | "icon-text-button" => "button_settings_changed",
        "text-input" | "search-box" => "input_settings_changed",
        "color-swatch" | "color-picker-rgba" => "color_settings_changed",
        "tree-view" => "tree_settings_changed",
        "context-menu" => "context_menu_settings_changed",
        _ => "component_settings_changed",
    }
}
