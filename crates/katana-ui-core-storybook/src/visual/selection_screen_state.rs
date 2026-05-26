use super::selection_screen_state_labels::{
    combo_read_state, combo_state, select_read_state, select_state, selection_list_state,
};

const FOURTH_LIST_INDEX: usize = 3;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) struct SelectionScreenState {
    pub(super) select_open: bool,
    pub(super) select_selected_index: Option<usize>,
    pub(super) combo_open: bool,
    pub(super) combo_filtered: bool,
    pub(super) combo_selected_index: Option<usize>,
    pub(super) selection_list_selected_index: Option<usize>,
    pub(super) selection_list_multi_mask: u8,
    pub(super) selection_list_focus_index: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SelectionScreenAction {
    SelectStateRead,
    SelectOpen,
    SelectClose,
    SelectOption(usize),
    SelectReset,
    ComboStateRead,
    ComboFilter,
    ComboOption(usize),
    ComboReset,
    SelectionListStateRead,
    SelectionListSelectRow(usize),
    SelectionListMultiToggle(usize),
    SelectionListKeyboardNext,
    SelectionListReset,
    SelectionListToggle(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct SelectionScreenUpdate {
    pub(super) action: &'static str,
    pub(super) event: &'static str,
    pub(super) state: &'static str,
}

impl SelectionScreenState {
    pub(super) fn apply(&mut self, action: SelectionScreenAction) -> SelectionScreenUpdate {
        match action {
            SelectionScreenAction::SelectStateRead => self.read_select_state(),
            SelectionScreenAction::SelectOpen => self.open_select(),
            SelectionScreenAction::SelectClose => self.close_select(),
            SelectionScreenAction::SelectOption(index) => self.select_option(index),
            SelectionScreenAction::SelectReset => self.reset_select(),
            SelectionScreenAction::ComboStateRead => self.read_combo_state(),
            SelectionScreenAction::ComboFilter => self.filter_combo(),
            SelectionScreenAction::ComboOption(index) => self.select_combo_option(index),
            SelectionScreenAction::ComboReset => self.reset_combo(),
            SelectionScreenAction::SelectionListStateRead => self.read_selection_list_state(),
            SelectionScreenAction::SelectionListSelectRow(index) => {
                self.select_selection_list_row(index)
            }
            SelectionScreenAction::SelectionListMultiToggle(index) => {
                self.toggle_selection_list_multi(index)
            }
            SelectionScreenAction::SelectionListKeyboardNext => self.selection_list_keyboard_next(),
            SelectionScreenAction::SelectionListReset => self.reset_selection_list(),
            SelectionScreenAction::SelectionListToggle(index) => {
                self.select_selection_list_row(index)
            }
        }
    }

    fn read_select_state(&mut self) -> SelectionScreenUpdate {
        SelectionScreenUpdate::new(
            "select_state_read",
            "select_state_read",
            select_read_state(self.select_open, self.select_selected_index),
        )
    }

    fn open_select(&mut self) -> SelectionScreenUpdate {
        self.select_open = true;
        SelectionScreenUpdate::new("select_open", "select_opened", "open=true")
    }

    fn close_select(&mut self) -> SelectionScreenUpdate {
        self.select_open = false;
        SelectionScreenUpdate::new("select_close", "select_closed", "open=false")
    }

    fn select_option(&mut self, index: usize) -> SelectionScreenUpdate {
        self.select_open = false;
        self.select_selected_index = Some(index);
        SelectionScreenUpdate::new("select_option", "select_changed", select_state(index))
    }

    fn reset_select(&mut self) -> SelectionScreenUpdate {
        self.select_open = false;
        self.select_selected_index = None;
        SelectionScreenUpdate::new("select_reset", "select_reset", "selected=none")
    }

    fn filter_combo(&mut self) -> SelectionScreenUpdate {
        self.combo_open = true;
        self.combo_filtered = true;
        SelectionScreenUpdate::new("combo_filter", "combo_filtered", "query=tw")
    }

    fn read_combo_state(&mut self) -> SelectionScreenUpdate {
        SelectionScreenUpdate::new(
            "combo_state_read",
            "combo_state_read",
            combo_read_state(
                self.combo_open,
                self.combo_filtered,
                self.combo_selected_index,
            ),
        )
    }

    fn select_combo_option(&mut self, index: usize) -> SelectionScreenUpdate {
        self.combo_open = false;
        self.combo_selected_index = Some(index);
        SelectionScreenUpdate::new("combo_select", "combo_selected", combo_state(index))
    }

    fn reset_combo(&mut self) -> SelectionScreenUpdate {
        self.combo_open = false;
        self.combo_filtered = false;
        self.combo_selected_index = None;
        SelectionScreenUpdate::new("combo_reset", "combo_reset", "query=empty selected=none")
    }

    fn read_selection_list_state(&mut self) -> SelectionScreenUpdate {
        SelectionScreenUpdate::new(
            "selection_list_state_read",
            "selection_list_state_read",
            selection_list_state(
                self.selection_list_selected_index,
                self.selection_list_multi_mask,
                self.selection_list_focus_index,
            ),
        )
    }

    fn select_selection_list_row(&mut self, index: usize) -> SelectionScreenUpdate {
        self.selection_list_selected_index = Some(index);
        self.selection_list_focus_index = Some(index);
        SelectionScreenUpdate::new(
            "selection_list_select_row",
            "selection_list_changed",
            selection_list_state(
                self.selection_list_selected_index,
                self.selection_list_multi_mask,
                self.selection_list_focus_index,
            ),
        )
    }

    fn toggle_selection_list_multi(&mut self, index: usize) -> SelectionScreenUpdate {
        let bit = 1u8 << index.min(FOURTH_LIST_INDEX);
        self.selection_list_multi_mask ^= bit;
        self.selection_list_focus_index = Some(index.min(FOURTH_LIST_INDEX));
        SelectionScreenUpdate::new(
            "selection_list_multi_toggle",
            "selection_list_multi_changed",
            selection_list_state(
                self.selection_list_selected_index,
                self.selection_list_multi_mask,
                self.selection_list_focus_index,
            ),
        )
    }

    fn selection_list_keyboard_next(&mut self) -> SelectionScreenUpdate {
        let next = match self.selection_list_focus_index {
            Some(index) if index < FOURTH_LIST_INDEX => index + 1,
            _ => 0,
        };
        self.selection_list_focus_index = Some(next);
        self.selection_list_selected_index = Some(next);
        SelectionScreenUpdate::new(
            "selection_list_keyboard_next",
            "selection_list_keyboard_moved",
            selection_list_state(
                self.selection_list_selected_index,
                self.selection_list_multi_mask,
                self.selection_list_focus_index,
            ),
        )
    }

    fn reset_selection_list(&mut self) -> SelectionScreenUpdate {
        self.selection_list_selected_index = None;
        self.selection_list_multi_mask = 0;
        self.selection_list_focus_index = None;
        SelectionScreenUpdate::new(
            "selection_list_reset",
            "selection_list_reset",
            "single=none multi=none focus=none",
        )
    }
}

impl SelectionScreenUpdate {
    const fn new(action: &'static str, event: &'static str, state: &'static str) -> Self {
        Self {
            action,
            event,
            state,
        }
    }
}
