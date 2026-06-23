use super::{StorybookButtonOperation, StorybookWindowState};
use crate::visual::dedicated_dod_molecule_menu as menu;
use crate::visual::dedicated_menu_button;
use crate::visual::preview_detail;

const SHORTCUT_PRESET_INDEX: usize = 1;
const DISABLED_PRESET_INDEX: usize = 2;
const OPEN_INDEX: usize = 0;
const CLOSE_INDEX: usize = 1;

pub(super) fn operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    if state.selected_page == "menu-button" {
        return menu_button_operation_at(state, x, y);
    }
    if state.selected_page != "menu" {
        return None;
    }
    let component = preview_detail::component_action_hit_rect(state.selected_page);
    if menu::first_row_rect(component).contains(x, y) {
        return first_row_operation(state);
    }
    if menu::second_row_rect(component).contains(x, y) {
        return second_row_operation(state);
    }
    None
}

fn menu_button_operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    let component = preview_detail::component_action_hit_rect(state.selected_page);
    if dedicated_menu_button::trigger_rect(component).contains(x, y) {
        if state.preset_index == DISABLED_PRESET_INDEX {
            return Some(StorybookButtonOperation::MenuButtonDisabledTrigger);
        }
        return Some(StorybookButtonOperation::MenuButtonOpen);
    }
    if !state.screen_state.selection.select_open {
        return None;
    }
    if dedicated_menu_button::first_item_rect(component).contains(x, y) {
        return Some(StorybookButtonOperation::MenuButtonSelect(OPEN_INDEX));
    }
    if dedicated_menu_button::second_item_rect(component).contains(x, y) {
        return Some(StorybookButtonOperation::MenuButtonClose);
    }
    None
}

fn first_row_operation(state: &StorybookWindowState) -> Option<StorybookButtonOperation> {
    if state.preset_index == SHORTCUT_PRESET_INDEX {
        return Some(StorybookButtonOperation::MenuShortcutActivation);
    }
    if state.screen_state.selection.select_open {
        return Some(StorybookButtonOperation::MenuSelect(OPEN_INDEX));
    }
    Some(StorybookButtonOperation::MenuOpen)
}

fn second_row_operation(state: &StorybookWindowState) -> Option<StorybookButtonOperation> {
    if state.preset_index == DISABLED_PRESET_INDEX {
        return Some(StorybookButtonOperation::MenuDisabledItem);
    }
    if state.screen_state.selection.select_open {
        return Some(StorybookButtonOperation::MenuSelect(CLOSE_INDEX));
    }
    Some(StorybookButtonOperation::MenuClose)
}
