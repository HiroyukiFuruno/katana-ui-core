use super::StorybookWindowState;
use crate::visual::button_options;

#[path = "clickable_operation/focus_operation.rs"]
mod focus_operation;
#[path = "clickable_operation/structured_operation.rs"]
mod structured_operation;

const DISABLED_PRESET_INDEX: usize = 2;
const CHIP_DISABLED_PRESET_INDEX: usize = 8;
const CLICK_OFFSET: usize = 4;

pub(super) fn focus_at(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    focus_operation::focus_at(state, x, y)
}

pub(super) fn keyboard_activate(state: &mut StorybookWindowState) -> bool {
    if state.selected_page == "checkbox" {
        state
            .screen_state
            .register_checkbox_keyboard_toggle(state.preset_index == DISABLED_PRESET_INDEX);
        return true;
    }
    if state.selected_page == "radio" {
        state.screen_state.register_radio_keyboard_select();
        return true;
    }
    if state.selected_page == "toggle" {
        state
            .screen_state
            .register_toggle_keyboard_toggle(state.preset_index == DISABLED_PRESET_INDEX);
        return true;
    }
    if state.selected_page == "slide-control" {
        state.screen_state.register_slide_keyboard_increment();
        return true;
    }
    if state.selected_page == "card" {
        state.screen_state.register_card_keyboard_activation();
        return true;
    }
    if state.selected_page == "list" {
        state.screen_state.register_list_keyboard_next();
        return true;
    }
    if state.selected_page == "breadcrumb" {
        state.screen_state.register_breadcrumb_keyboard_next();
        return true;
    }
    if state.selected_page == "context-menu" {
        state.screen_state.register_context_menu_keyboard_select();
        return true;
    }
    if state.selected_page == "menu" {
        state.screen_state.register_menu_keyboard_open();
        return true;
    }
    if state.selected_page == "menu-button" {
        state
            .screen_state
            .register_menu_button_keyboard_open(state.preset_index == DISABLED_PRESET_INDEX);
        return true;
    }
    if state.selected_page == "modal" {
        return state.screen_state.register_modal_keyboard_escape();
    }
    if state.selected_page == "modal-overlay" {
        state.screen_state.register_modal_overlay_keyboard_escape();
        return true;
    }
    if state.selected_page == "toolbar" {
        state.screen_state.register_toolbar_keyboard_activation();
        return true;
    }
    if state.selected_page == "chip" {
        state
            .screen_state
            .register_chip_keyboard_dismiss(state.preset_index == CHIP_DISABLED_PRESET_INDEX);
        return true;
    }
    if state.selected_page == "accordion" {
        state.screen_state.register_accordion_keyboard_toggle();
        return true;
    }
    if state.selected_page == "code-diff" {
        state.screen_state.register_code_diff_keyboard_expand();
        return true;
    }
    if state.selected_page == "combo-box" {
        state.screen_state.register_selection_action(
            crate::visual::selection_screen_state::SelectionScreenAction::ComboKeyboardSelect,
        );
        return true;
    }
    if structured_operation::keyboard_activate(state) {
        return true;
    }
    if !button_options::is_button_page(state.selected_page) {
        return false;
    }
    state
        .screen_state
        .register_button_keyboard_activation(state.selected_page);
    true
}
