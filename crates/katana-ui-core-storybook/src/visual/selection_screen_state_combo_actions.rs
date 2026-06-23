use super::super::selection_screen_state::{SelectionScreenState, SelectionScreenUpdate};
use super::super::selection_screen_state_labels::{combo_read_state, combo_state};

pub(super) fn filter_combo(state: &mut SelectionScreenState) -> SelectionScreenUpdate {
    let result = state.apply_core_combo_filter("tw");
    state.combo_open = result.opened;
    state.combo_filtered = result.filtered;
    SelectionScreenUpdate::new("combo_filter", "combo_filtered", "query=tw")
}

pub(super) fn read_combo_state(state: &mut SelectionScreenState) -> SelectionScreenUpdate {
    SelectionScreenUpdate::new(
        "combo_state_read",
        "combo_state_read",
        combo_read_state(
            state.combo_open,
            state.combo_filtered,
            state.combo_selected_index,
        ),
    )
}

pub(super) fn select_combo_option(
    state: &mut SelectionScreenState,
    index: usize,
) -> SelectionScreenUpdate {
    let result = state.apply_core_combo_selected(index);
    if result.handled {
        state.combo_open = result.after.open;
        state.combo_selected_index = Some(result.after.selected_index);
    }
    SelectionScreenUpdate::new("combo_select", "combo_selected", combo_state(index))
}

pub(super) fn focus_combo(state: &mut SelectionScreenState) -> SelectionScreenUpdate {
    let result = state.apply_core_combo_focus();
    SelectionScreenUpdate::new(
        "combo_focus",
        "combo_focused",
        if result.handled && result.after.focused {
            "focus=true"
        } else {
            "focus=false"
        },
    )
}

pub(super) fn hover_combo(state: &mut SelectionScreenState) -> SelectionScreenUpdate {
    let result = state.apply_core_combo_hover();
    SelectionScreenUpdate::new(
        "combo_hover",
        "combo_hovered",
        if result.handled && result.after.hovered {
            "hover=true"
        } else {
            "hover=false"
        },
    )
}

pub(super) fn keyboard_select_combo(state: &mut SelectionScreenState) -> SelectionScreenUpdate {
    let result = state.apply_core_combo_selected(1);
    if result.handled {
        state.combo_open = result.after.open;
        state.combo_selected_index = Some(result.after.selected_index);
    }
    SelectionScreenUpdate::new("combo_keyboard_select", "combo_selected", combo_state(1))
}

pub(super) fn reset_combo(state: &mut SelectionScreenState) -> SelectionScreenUpdate {
    state.combo_open = false;
    state.combo_filtered = false;
    state.combo_selected_index = None;
    state.combo_contract = Default::default();
    SelectionScreenUpdate::new("combo_reset", "combo_reset", "query=empty selected=none")
}
