use super::{
    CollapsiblePanelStoryAction, SideMenuScreenAction, StorybookScreenState,
    StorybookUiOptionContract, VirtualizationStoryAction, format_setting_action,
    format_setting_event, layout_operation, semantic_setting_state,
};
#[cfg(test)]
use crate::visual::window_interaction;

impl StorybookScreenState {
    #[cfg(test)]
    pub(in crate::visual) fn register_settings_contract_change(
        &mut self,
        page: &str,
        option: StorybookUiOptionContract,
    ) {
        let instance = window_interaction::component_instance_id_for_page(
            page,
            window_interaction::DEFAULT_INSTANCE_ID,
        );
        self.register_settings_contract_change_for_instance(page, instance, option);
    }

    pub(in crate::visual) fn register_settings_contract_change_for_instance(
        &mut self,
        page: &str,
        instance: &'static str,
        option: StorybookUiOptionContract,
    ) {
        if page == "panel" && self.register_panel_contract_setting(option) {
            return;
        }
        if page == "closeable-tab-strip"
            && self.register_closeable_tab_strip_contract_setting(option)
        {
            return;
        }
        if page == "tabs" && self.register_tabs_contract_setting(option) {
            return;
        }
        if page == "dynamic-array-editor" {
            self.settings_revision += 1;
            let update = self.dynamic_array_editor.apply_option(option.setting);
            self.apply_dynamic_array_editor_update(update);
            self.last_setting = option.setting;
            self.last_setting_value = option.after;
            return;
        }
        if page == "breadcrumb" && self.apply_breadcrumb_contract_setting(option) {
            return;
        }
        if page == "drag-and-drop" {
            self.settings_revision += 1;
            let update = self.drag_and_drop.apply_option(option.setting);
            self.apply_drag_and_drop_update(update);
            self.last_setting = option.setting;
            self.last_setting_value = option.after;
            return;
        }
        if layout_operation::is_live_layout_page(page)
            && let Some(update) = self.layout.apply_option(page, option.setting)
        {
            self.settings_revision += 1;
            self.apply_layout_update(update);
            self.last_setting = option.setting;
            self.last_setting_value = option.after;
            return;
        }
        if page == "form-field" && self.register_form_field_contract_setting(option) {
            return;
        }
        if page == "toggle" && self.register_toggle_contract_setting(option) {
            return;
        }
        if self.register_binary_choice_contract_setting(page, option) {
            return;
        }
        if page == "progress-bar" && self.register_progress_bar_contract_setting(option) {
            return;
        }
        if is_runtime_structured_page(page) {
            self.runtime_structured.apply_option(page, option.setting);
        }
        if page == "text-area" {
            self.apply_text_area_contract_option_for(instance, option);
        }
        if page == "text-input" {
            self.apply_text_input_contract_option_for(instance, option);
        }
        if page == "color-picker-rgba" {
            self.color_picker.apply_option(option.setting);
        }
        if page == "command-palette" {
            self.command_palette.apply_option(option.setting);
        }
        if page == "collapsible-panel" {
            self.collapsible_panel
                .apply(CollapsiblePanelStoryAction::Resize);
        }
        if page == "virtualization" {
            self.virtualization.apply(VirtualizationStoryAction::Scroll);
        }
        if page == "diagnostics-list" {
            self.diagnostics_list.apply_option(option.setting);
        }
        if page == "settings-list" {
            self.settings_list.apply_option(option.setting);
        }
        if page == "shortcut-cheatsheet" {
            self.shortcut_cheatsheet.apply_option(option.setting);
        }
        if page == "combo-box" {
            self.selection.apply_combo_contract_option(option);
        }
        if page == "side-menu" && option.setting == "side_menu.hover_expansion" {
            self.side_menu.apply(SideMenuScreenAction::HoverExpansion);
        }
        self.settings_revision += 1;
        self.last_action = format_setting_action(option.setting);
        self.last_event = format_setting_event(page);
        self.last_setting = option.setting;
        self.last_setting_value = option.after;
        self.state_label = semantic_setting_state(page, option);
    }
}

fn is_runtime_structured_page(page: &str) -> bool {
    matches!(
        page,
        "shortcut-combo"
            | "skeleton-cluster"
            | "motion"
            | "window-control-button-group"
            | "startup-state-panel"
            | "attachment-chip"
            | "chip-group"
            | "accordion"
    )
}
