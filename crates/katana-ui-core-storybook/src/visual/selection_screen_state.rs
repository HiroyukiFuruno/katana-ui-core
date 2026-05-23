const LIGHT_OPTION_INDEX: usize = 1;
const DARK_OPTION_INDEX: usize = 2;
const SYSTEM_OPTION_INDEX: usize = 3;
const FOURTH_LIST_INDEX: usize = 3;
const COMBO_TWO_INDEX: usize = 1;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) struct SelectionScreenState {
    pub(super) select_open: bool,
    pub(super) select_selected_index: Option<usize>,
    pub(super) combo_open: bool,
    pub(super) combo_filtered: bool,
    pub(super) combo_selected_index: Option<usize>,
    pub(super) selection_list_selected_index: Option<usize>,
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
            SelectionScreenAction::SelectionListToggle(index) => self.toggle_selection_list(index),
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

    fn toggle_selection_list(&mut self, index: usize) -> SelectionScreenUpdate {
        self.selection_list_selected_index = Some(index);
        SelectionScreenUpdate::new("selection_toggle", "selection_changed", list_state(index))
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

fn select_state(index: usize) -> &'static str {
    match index {
        LIGHT_OPTION_INDEX => "selected=light",
        DARK_OPTION_INDEX => "selected=dark",
        SYSTEM_OPTION_INDEX => "selected=system",
        _ => "selected=none",
    }
}

fn select_read_state(is_open: bool, selected_index: Option<usize>) -> &'static str {
    match (is_open, selected_index) {
        (true, Some(LIGHT_OPTION_INDEX)) => "open=true selected=light",
        (true, Some(DARK_OPTION_INDEX)) => "open=true selected=dark",
        (true, Some(SYSTEM_OPTION_INDEX)) => "open=true selected=system",
        (true, _) => "open=true selected=none",
        (false, Some(LIGHT_OPTION_INDEX)) => "open=false selected=light",
        (false, Some(DARK_OPTION_INDEX)) => "open=false selected=dark",
        (false, Some(SYSTEM_OPTION_INDEX)) => "open=false selected=system",
        (false, _) => "open=false selected=none",
    }
}

fn combo_state(index: usize) -> &'static str {
    match index {
        COMBO_TWO_INDEX => "selected=two",
        _ => "selected=one",
    }
}

fn combo_read_state(is_open: bool, is_filtered: bool, selected_index: Option<usize>) -> &'static str {
    match (is_open, is_filtered, selected_index) {
        (true, true, Some(COMBO_TWO_INDEX)) => "open=true query=tw selected=two",
        (true, true, _) => "open=true query=tw selected=none",
        (false, false, Some(COMBO_TWO_INDEX)) => "open=false query=empty selected=two",
        (false, false, _) => "open=false query=empty selected=none",
        _ => "open=false query=empty selected=none",
    }
}

fn list_state(index: usize) -> &'static str {
    match index {
        0 => "selected=0",
        1 => "selected=1",
        2 => "selected=2",
        FOURTH_LIST_INDEX => "selected=3",
        _ => "selected=none",
    }
}
