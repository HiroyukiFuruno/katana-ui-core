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
    SelectOpen,
    SelectOption(usize),
    ComboFilter,
    ComboOption(usize),
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
            SelectionScreenAction::SelectOpen => self.open_select(),
            SelectionScreenAction::SelectOption(index) => self.select_option(index),
            SelectionScreenAction::ComboFilter => self.filter_combo(),
            SelectionScreenAction::ComboOption(index) => self.select_combo_option(index),
            SelectionScreenAction::SelectionListToggle(index) => self.toggle_selection_list(index),
        }
    }

    fn open_select(&mut self) -> SelectionScreenUpdate {
        self.select_open = true;
        SelectionScreenUpdate::new("select_open", "select_opened", "open=true")
    }

    fn select_option(&mut self, index: usize) -> SelectionScreenUpdate {
        self.select_open = false;
        self.select_selected_index = Some(index);
        SelectionScreenUpdate::new("select_option", "select_changed", select_state(index))
    }

    fn filter_combo(&mut self) -> SelectionScreenUpdate {
        self.combo_open = true;
        self.combo_filtered = true;
        SelectionScreenUpdate::new("combo_filter", "combo_filtered", "query=tw")
    }

    fn select_combo_option(&mut self, index: usize) -> SelectionScreenUpdate {
        self.combo_open = false;
        self.combo_selected_index = Some(index);
        SelectionScreenUpdate::new("combo_select", "combo_selected", combo_state(index))
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

fn combo_state(index: usize) -> &'static str {
    match index {
        COMBO_TWO_INDEX => "selected=two",
        _ => "selected=one",
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
