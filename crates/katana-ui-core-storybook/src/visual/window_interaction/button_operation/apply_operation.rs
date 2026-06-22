use super::{SelectionScreenAction, StorybookButtonOperation, StorybookWindowState};
use crate::visual::search_box_screen_state::SearchBoxScreenAction;

pub(super) fn apply(operation: StorybookButtonOperation, state: &mut StorybookWindowState) -> bool {
    match operation {
        StorybookButtonOperation::LightTheme => state.theme_id = "light",
        StorybookButtonOperation::DarkTheme => state.theme_id = "dark",
        StorybookButtonOperation::Preset(index) => state.select_preset(index),
        StorybookButtonOperation::PreviewButton => state
            .screen_state
            .register_button_click(state.selected_page),
        StorybookButtonOperation::PreviewComponent => {
            apply_preview_component_action(state);
        }
        StorybookButtonOperation::ButtonOption(control) => {
            state.select_preset(crate::visual::button_options::preset_index_for_control(
                control,
            ));
            state.screen_state.register_button_option(control);
        }
        StorybookButtonOperation::PanelOption(control) => {
            state.screen_state.register_panel_option(control)
        }
        StorybookButtonOperation::PanelChild(panel) => {
            state.screen_state.register_panel_active_child(panel)
        }
        StorybookButtonOperation::ColorPicker(action) => {
            state.screen_state.register_color_picker_action(action)
        }
        StorybookButtonOperation::DiagnosticsList(action) => {
            state.screen_state.register_diagnostics_list_action(action)
        }
        StorybookButtonOperation::DragAndDrop(action) => {
            state.screen_state.register_drag_and_drop_action(action)
        }
        StorybookButtonOperation::DynamicArrayEditor(action) => {
            state
                .screen_state
                .register_dynamic_array_editor_action(action);
        }
        StorybookButtonOperation::Layout(action) => {
            state.screen_state.register_layout_action(action)
        }
        StorybookButtonOperation::PanelResize => state.screen_state.register_panel_resize(),
        StorybookButtonOperation::ScrollArea(action) => {
            state.screen_state.register_scroll_area_action(action)
        }
        StorybookButtonOperation::SettingsList(action) => {
            state.screen_state.register_settings_list_action(action)
        }
        StorybookButtonOperation::SplitPane(action) => {
            state.screen_state.register_split_pane_action(action)
        }
        StorybookButtonOperation::ThemeTokens(action) => {
            state.screen_state.register_theme_tokens_action(action)
        }
        StorybookButtonOperation::SettingsOption {
            option,
            preset_index,
        } => apply_settings_option(state, option, preset_index),
        StorybookButtonOperation::SelectionControl(action) => {
            state.screen_state.register_selection_action(action)
        }
        StorybookButtonOperation::CheckboxStateRead => {
            if !disabled_preview_action_is_blocked(state) {
                state.screen_state.register_checkbox_state_read();
            }
        }
        StorybookButtonOperation::CheckboxToggle(index) => {
            apply_checkbox_write(state, CheckboxWrite::Toggle(index))
        }
        StorybookButtonOperation::CheckboxToggleFocused => {
            apply_checkbox_write(state, CheckboxWrite::ToggleFocused)
        }
        StorybookButtonOperation::CheckboxReset => {
            apply_checkbox_write(state, CheckboxWrite::Reset)
        }
        StorybookButtonOperation::RadioStateRead => state.screen_state.register_radio_state_read(),
        StorybookButtonOperation::RadioSelect => apply_radio_write(state, RadioWrite::Select),
        StorybookButtonOperation::RadioSelectIndex(index) => {
            apply_radio_write(state, RadioWrite::SelectIndex(index))
        }
        StorybookButtonOperation::RadioReset => apply_radio_write(state, RadioWrite::Reset),
        StorybookButtonOperation::ComboStateRead => state
            .screen_state
            .register_selection_action(SelectionScreenAction::ComboStateRead),
        StorybookButtonOperation::ComboFilter => state
            .screen_state
            .register_selection_action(SelectionScreenAction::ComboFilter),
        StorybookButtonOperation::ComboSelect => state
            .screen_state
            .register_selection_action(SelectionScreenAction::ComboOption(1)),
        StorybookButtonOperation::ComboReset => state
            .screen_state
            .register_selection_action(SelectionScreenAction::ComboReset),
        StorybookButtonOperation::SearchStateRead => {
            apply_search(state, SearchBoxScreenAction::StateRead)
        }
        StorybookButtonOperation::SearchTypeQuery => {
            apply_search(state, SearchBoxScreenAction::TypeQuery)
        }
        StorybookButtonOperation::SearchSubmit => {
            apply_search(state, SearchBoxScreenAction::Submit)
        }
        StorybookButtonOperation::SearchClear => apply_search(state, SearchBoxScreenAction::Clear),
        StorybookButtonOperation::SearchCaseToggle => {
            apply_search(state, SearchBoxScreenAction::ToggleCase)
        }
        StorybookButtonOperation::SearchRegexToggle => {
            apply_search(state, SearchBoxScreenAction::ToggleRegex)
        }
        StorybookButtonOperation::MenuOpen => apply_menu_open(state),
        StorybookButtonOperation::MenuClose => apply_menu_close(state),
        StorybookButtonOperation::MenuSelect(index) => apply_menu_select(state, index),
        StorybookButtonOperation::MenuDisabledItem => {}
        StorybookButtonOperation::MenuShortcutActivation => apply_menu_shortcut(state),
        StorybookButtonOperation::MenuButtonOpen => state.screen_state.register_menu_button_open(),
        StorybookButtonOperation::MenuButtonClose => {
            state.screen_state.register_menu_button_close();
        }
        StorybookButtonOperation::MenuButtonSelect(index) => {
            state.screen_state.register_menu_button_select(index);
        }
        StorybookButtonOperation::MenuButtonDisabledTrigger => {
            state.screen_state.register_menu_button_disabled_trigger();
        }
        StorybookButtonOperation::StatusBarSegment(index) => {
            state.screen_state.register_status_bar_segment_click(index);
        }
        StorybookButtonOperation::ToolbarActionButton(index) => apply_toolbar_action(state, index),
        StorybookButtonOperation::TabsControl(action) => {
            state.screen_state.register_tabs_action(action)
        }
        StorybookButtonOperation::TabsPinIcon { tab_id } => {
            state.screen_state.register_tabs_pin_icon_unpin(&tab_id)
        }
        StorybookButtonOperation::CloseableTabStripSelect { tab_id } => state
            .screen_state
            .register_closeable_tab_strip_select(&tab_id),
        StorybookButtonOperation::TreeViewPointer {
            pointer_x,
            pointer_y,
        } => state
            .screen_state
            .register_tree_view_pointer_click(pointer_x, pointer_y),
        StorybookButtonOperation::BreadcrumbSelection(index) => {
            state.screen_state.register_breadcrumb_click(index)
        }
        StorybookButtonOperation::TextInputFocus {
            initial_value,
            readonly,
        } => {
            let instance = component_instance(state);
            state
                .screen_state
                .register_text_input_focus_for(instance, initial_value, readonly);
        }
        StorybookButtonOperation::TextInputClearAction {
            initial_value,
            readonly,
        } => {
            let instance = component_instance(state);
            state.screen_state.register_text_input_clear_action_for(
                instance,
                initial_value,
                readonly,
            );
        }
        StorybookButtonOperation::TextInputIconButton => {
            state.screen_state.register_text_input_icon_button()
        }
        StorybookButtonOperation::TextAreaFocus { readonly, disabled } => {
            let instance = component_instance(state);
            state
                .screen_state
                .register_text_area_focus_for(instance, readonly, disabled);
        }
        StorybookButtonOperation::TextAreaClearAction { readonly, disabled } => {
            let instance = component_instance(state);
            state
                .screen_state
                .register_text_area_clear_action_for(instance, readonly, disabled);
        }
        StorybookButtonOperation::TextAreaIconButton => {
            state.screen_state.register_text_area_icon_button()
        }
    }
    true
}

fn component_instance(state: &StorybookWindowState) -> &'static str {
    super::super::component_instance_id_for_page(state.selected_page, state.selected_instance_id)
}

enum CheckboxWrite {
    Toggle(usize),
    ToggleFocused,
    Reset,
}

enum RadioWrite {
    Select,
    SelectIndex(usize),
    Reset,
}

fn apply_preview_component_action(state: &mut StorybookWindowState) {
    if disabled_preview_action_is_blocked(state) {
        return;
    }
    if state.selected_page == "modal" {
        state.screen_state.register_modal_escape_close();
        return;
    }
    state
        .screen_state
        .register_preview_action(state.selected_page);
}

fn apply_checkbox_write(state: &mut StorybookWindowState, write: CheckboxWrite) {
    if disabled_preview_action_is_blocked(state) {
        return;
    }
    match write {
        CheckboxWrite::Toggle(index) => state.screen_state.register_checkbox_toggle_at(index),
        CheckboxWrite::ToggleFocused => state.screen_state.register_checkbox_toggle(),
        CheckboxWrite::Reset => state.screen_state.register_checkbox_reset(),
    }
}

fn apply_radio_write(state: &mut StorybookWindowState, write: RadioWrite) {
    if disabled_preview_action_is_blocked(state) {
        return;
    }
    match write {
        RadioWrite::Select => state.screen_state.register_radio_select(),
        RadioWrite::SelectIndex(index) => state.screen_state.register_radio_select_index(index),
        RadioWrite::Reset => state.screen_state.register_radio_reset(),
    }
}

fn disabled_preview_action_is_blocked(state: &StorybookWindowState) -> bool {
    const DISABLED_PRESET_INDEX: usize = 2;
    const CHIP_DISABLED_PRESET_INDEX: usize = 8;
    match state.selected_page {
        "checkbox" => {
            state.screen_state.is_checkbox_disabled() || state.preset_index == DISABLED_PRESET_INDEX
        }
        "radio" => state.screen_state.is_radio_disabled(),
        "chip" => state.preset_index == CHIP_DISABLED_PRESET_INDEX,
        "toggle" | "segmented-toggle" => state.preset_index == DISABLED_PRESET_INDEX,
        _ => false,
    }
}

fn apply_settings_option(
    state: &mut StorybookWindowState,
    option: crate::visual::storybook_ui_option_contract::StorybookUiOptionContract,
    preset_index: Option<usize>,
) {
    if let Some(index) = preset_index {
        state.select_preset(index);
    }
    if state.selected_page == "theme-tokens" && option.setting == "theme.id" {
        state.theme_id = option.after;
    }
    let instance = component_instance(state);
    state
        .screen_state
        .register_settings_contract_change_for_instance(state.selected_page, instance, option);
}

fn apply_search(state: &mut StorybookWindowState, action: SearchBoxScreenAction) {
    state.screen_state.register_search_box_action(action);
}

#[path = "apply_operation_menu.rs"]
mod menu;
use menu::{apply_menu_close, apply_menu_open, apply_menu_select, apply_menu_shortcut};

fn apply_toolbar_action(state: &mut StorybookWindowState, index: usize) {
    if super::toolbar_operation::is_action_disabled(state.preset_index, index) {
        return;
    }
    state.screen_state.register_toolbar_action_button(index);
}

fn menu_selected_label(index: usize) -> &'static str {
    match index {
        0 => "selected=open",
        1 => "selected=close",
        _ => "selected=unknown",
    }
}
