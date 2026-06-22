use super::{StorybookWindowState, menu_selected_label};

pub(super) fn apply_menu_open(state: &mut StorybookWindowState) {
    state.screen_state.action_count += 1;
    state.screen_state.selection.select_open = true;
    state.screen_state.last_action = "menu_open";
    state.screen_state.last_event = "menu_opened";
    state.screen_state.state_label = "open=true";
}

pub(super) fn apply_menu_close(state: &mut StorybookWindowState) {
    state.screen_state.action_count += 1;
    state.screen_state.selection.select_open = false;
    state.screen_state.last_action = "menu_close";
    state.screen_state.last_event = "menu_closed";
    state.screen_state.state_label = "open=false";
}

pub(super) fn apply_menu_select(state: &mut StorybookWindowState, index: usize) {
    state.screen_state.action_count += 1;
    state.screen_state.selection.select_open = false;
    state.screen_state.selection.select_selected_index = Some(index);
    state.screen_state.last_action = "menu_select";
    state.screen_state.last_event = "menu_item_selected";
    state.screen_state.state_label = menu_selected_label(index);
}

pub(super) fn apply_menu_shortcut(state: &mut StorybookWindowState) {
    state.screen_state.action_count += 1;
    state.screen_state.selection.select_open = false;
    state.screen_state.selection.select_selected_index = Some(0);
    state.screen_state.last_action = "menu_shortcut_activate";
    state.screen_state.last_event = "menu_item_selected";
    state.screen_state.state_label = "shortcut=Cmd+O selected=open";
}
