#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) struct SelectionScreenState {
    pub(super) select_open: bool,
    pub(super) select_selected_index: Option<usize>,
    pub(super) select_focused: bool,
    pub(super) select_hovered: bool,
    pub(super) select_scroll_offset: usize,
    pub(super) combo_open: bool,
    pub(super) combo_filtered: bool,
    pub(super) combo_selected_index: Option<usize>,
    pub(super) combo_contract: ComboBoxContractState,
    pub(super) selection_list_selected_index: Option<usize>,
    pub(super) selection_list_multi_mask: u8,
    pub(super) selection_list_focus_index: Option<usize>,
    pub(super) selection_list_hovered: bool,
    pub(super) selection_list_scroll_offset: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) struct ComboBoxContractState {
    pub(super) item_count: usize,
    pub(super) value_applied: bool,
    pub(super) placeholder_visible: bool,
    pub(super) disabled: bool,
    pub(super) readonly: bool,
    pub(super) input_value: bool,
    pub(super) filter_result: bool,
    pub(super) free_input: bool,
    pub(super) keyboard_navigation: bool,
    pub(super) placement_above: bool,
    pub(super) highlighted_index: Option<usize>,
    pub(super) long_list: bool,
    pub(super) outside_click_dismiss: bool,
    pub(super) framed: bool,
    pub(super) trigger_summary: bool,
    pub(super) select_action: bool,
    pub(super) invalid: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SelectionScreenAction {
    SelectStateRead,
    SelectOpen,
    SelectClose,
    SelectOption(usize),
    SelectFocus,
    SelectHover,
    SelectKeyboardSelect,
    SelectScroll,
    SelectReset,
    ComboStateRead,
    ComboFilter,
    ComboOption(usize),
    ComboFocus,
    ComboHover,
    ComboKeyboardSelect,
    ComboReset,
    SelectionListStateRead,
    SelectionListSelectRow(usize),
    SelectionListMultiToggle(usize),
    SelectionListFocus,
    SelectionListHover,
    SelectionListKeyboardNext,
    SelectionListScroll,
    SelectionListReset,
    SelectionListToggle(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct SelectionScreenUpdate {
    pub(super) action: &'static str,
    pub(super) event: &'static str,
    pub(super) state: &'static str,
}

impl SelectionScreenUpdate {
    pub(super) const fn new(
        action: &'static str,
        event: &'static str,
        state: &'static str,
    ) -> Self {
        Self {
            action,
            event,
            state,
        }
    }
}
