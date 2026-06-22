use super::super::StorybookWindowState;
use super::{
    CHIP_DISABLED_PRESET_INDEX, CLICK_OFFSET, DISABLED_PRESET_INDEX, structured_operation,
};
use crate::visual::{
    button_options, dedicated_breadcrumb, dedicated_chip, dedicated_context_menu_popup,
    dedicated_dod_form_binary_choice_live, dedicated_dod_molecule_menu, dedicated_menu_button,
    preview_detail,
};

pub(super) fn focus_at(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if state.selected_page == "checkbox" {
        let component = preview_detail::component_action_hit_rect("checkbox");
        for index in 0..2 {
            if dedicated_dod_form_binary_choice_live::row_rect(index, component.x, component.y)
                .contains(x, y)
            {
                state
                    .screen_state
                    .register_checkbox_focus_at(index, state.preset_index == DISABLED_PRESET_INDEX);
                return true;
            }
        }
        return false;
    }
    if state.selected_page == "radio" {
        let component = preview_detail::component_action_hit_rect("radio");
        if !dedicated_dod_form_binary_choice_live::row_rect(0, component.x, component.y)
            .contains(x, y)
        {
            return false;
        }
        state.screen_state.register_radio_focus();
        return true;
    }
    if state.selected_page == "toggle" {
        let component = preview_detail::component_action_hit_rect("toggle");
        if !component.contains(x, y) {
            return false;
        }
        state
            .screen_state
            .register_toggle_focus(state.preset_index == DISABLED_PRESET_INDEX);
        return true;
    }
    if state.selected_page == "slide-control" {
        let component = preview_detail::component_action_hit_rect("slide-control");
        if !component.contains(x, y) {
            return false;
        }
        state.screen_state.register_slide_focus();
        return true;
    }
    if state.selected_page == "card" {
        let component = preview_detail::component_action_hit_rect("card");
        if !component.contains(x, y) {
            return false;
        }
        state.screen_state.register_card_focus();
        return true;
    }
    if state.selected_page == "list" {
        let component = preview_detail::component_action_hit_rect("list");
        if !component.contains(x, y) {
            return false;
        }
        state.screen_state.register_list_focus();
        return true;
    }
    if state.selected_page == "breadcrumb" {
        let component = preview_detail::component_action_hit_rect("breadcrumb");
        if !dedicated_breadcrumb::file_crumb_rect(component.x, component.y).contains(x, y) {
            return false;
        }
        state.screen_state.register_breadcrumb_focus(2);
        return true;
    }
    if state.selected_page == "context-menu" {
        let component = preview_detail::component_action_hit_rect("context-menu");
        if !dedicated_context_menu_popup::insert_row_rect(component.x, component.y).contains(x, y) {
            return false;
        }
        state.screen_state.register_context_menu_focus();
        return true;
    }
    if state.selected_page == "menu" {
        let component = preview_detail::component_action_hit_rect("menu");
        if !dedicated_dod_molecule_menu::first_row_rect(component).contains(x, y) {
            return false;
        }
        state.screen_state.register_menu_focus();
        return true;
    }
    if state.selected_page == "menu-button" {
        let component = preview_detail::component_action_hit_rect("menu-button");
        if !dedicated_menu_button::trigger_rect(component).contains(x, y) {
            return false;
        }
        state
            .screen_state
            .register_menu_button_focus(state.preset_index == DISABLED_PRESET_INDEX);
        return true;
    }
    if state.selected_page == "toast-stack-manager" {
        let component = preview_detail::component_action_hit_rect("toast-stack-manager");
        if !component.contains(x, y) {
            return false;
        }
        state.screen_state.register_toast_stack_focus_pause();
        return true;
    }
    if state.selected_page == "tooltip" {
        let anchor = crate::visual::dedicated_tooltip::anchor_hit_rect(state.preset_index);
        if !anchor.contains(x, y) {
            return false;
        }
        state.screen_state.register_tooltip_focus_open();
        return true;
    }
    if state.selected_page == "modal" {
        let component = preview_detail::component_action_hit_rect("modal");
        if !component.contains(x, y) {
            return false;
        }
        return state.screen_state.register_modal_focus_trap();
    }
    if state.selected_page == "modal-overlay" {
        let component = preview_detail::component_action_hit_rect("modal-overlay");
        if !component.contains(x, y) {
            return false;
        }
        state.screen_state.register_modal_overlay_focus();
        return true;
    }
    if state.selected_page == "toolbar" {
        let component = preview_detail::component_action_hit_rect("toolbar");
        if !component.contains(x, y) {
            return false;
        }
        state.screen_state.register_toolbar_focus();
        return true;
    }
    if state.selected_page == "form-field" {
        let component = preview_detail::component_action_hit_rect("form-field");
        if !component.contains(x, y) {
            return false;
        }
        state.screen_state.register_form_field_focus_link();
        return true;
    }
    if state.selected_page == "accordion" {
        let component = preview_detail::component_action_hit_rect("accordion");
        if !component.contains(x, y) {
            return false;
        }
        state.screen_state.register_accordion_focus();
        return true;
    }
    if state.selected_page == "code-diff" {
        let component = preview_detail::component_action_hit_rect("code-diff");
        if !component.contains(x, y) {
            return false;
        }
        state.screen_state.register_code_diff_focus();
        return true;
    }
    if state.selected_page == "color-picker-rgba" {
        let component = preview_detail::component_action_hit_rect("color-picker-rgba");
        if !component.contains(x, y) {
            return false;
        }
        state.screen_state.register_color_picker_action(
            crate::visual::window_interaction::color_picker_operation::ColorPickerAction::Focus,
        );
        return true;
    }
    if state.selected_page == "combo-box" {
        let component = preview_detail::component_action_hit_rect("combo-box");
        if !component.contains(x, y) {
            return false;
        }
        state.screen_state.register_selection_action(
            crate::visual::selection_screen_state::SelectionScreenAction::ComboFocus,
        );
        return true;
    }
    if structured_operation::focus_at(state, x, y) {
        return true;
    }
    if state.selected_page == "chip" {
        let component = preview_detail::component_action_hit_rect("chip");
        let chip_x = component.x + dedicated_chip::CHIP_X;
        let chip_y = component.y + dedicated_chip::CHIP_Y;
        if x < chip_x
            || y < chip_y
            || x >= chip_x + dedicated_chip::CHIP_WIDTH
            || y >= chip_y + dedicated_chip::CHIP_HEIGHT + CLICK_OFFSET
        {
            return false;
        }
        state
            .screen_state
            .register_chip_focus(state.preset_index == CHIP_DISABLED_PRESET_INDEX);
        return true;
    }
    if !button_options::is_button_page(state.selected_page) {
        return false;
    }
    if !preview_detail::button_action_hit_rect(state.selected_page).contains(x, y) {
        return false;
    }
    state.screen_state.register_button_focus();
    true
}
