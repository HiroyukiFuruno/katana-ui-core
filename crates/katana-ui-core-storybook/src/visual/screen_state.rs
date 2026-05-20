use super::button_options::{StorybookButtonOptionControl, StorybookButtonOptions};
use super::interaction_spec::StorybookInteractionSpec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct StorybookScreenState {
    pub(super) action_count: usize,
    pub(super) settings_revision: usize,
    pub(super) last_action: &'static str,
    pub(super) last_event: &'static str,
    pub(super) last_setting: &'static str,
    pub(super) last_setting_value: &'static str,
    pub(super) state_label: &'static str,
    pub(super) button_options: StorybookButtonOptions,
    pub(super) preview_hovered: bool,
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
            preview_hovered: false,
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
        let spec = StorybookInteractionSpec::for_page(page);
        self.last_action = spec.action;
        self.last_event = spec.event;
        self.state_label = spec.state;
    }

    pub(super) fn register_preview_action(&mut self, page: &str) {
        self.action_count += 1;
        let spec = StorybookInteractionSpec::for_page(page);
        self.last_action = spec.action;
        self.last_event = spec.event;
        self.state_label = spec.state;
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

    pub(super) fn has_widget_action(self) -> bool {
        self.action_count > 0
    }

    pub(super) fn has_settings_override(self) -> bool {
        self.settings_revision % 2 == 1
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
