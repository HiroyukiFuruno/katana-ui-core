const LIGHT_OPTION_INDEX: usize = 1;
const DARK_OPTION_INDEX: usize = 2;
const SYSTEM_OPTION_INDEX: usize = 3;
const COMBO_TWO_INDEX: usize = 1;
const SELECTION_LIST_ZERO_INDEX: usize = 0;
const SELECTION_LIST_ONE_INDEX: usize = 1;
const SELECTION_LIST_TWO_INDEX: usize = 2;
const SELECTION_LIST_THREE_INDEX: usize = 3;
const SELECTION_LIST_MULTI_MASK: u8 = 0b1111;
const SELECTION_LIST_SECOND_MASK: u8 = 0b0010;
const SELECTION_LIST_SECOND_THIRD_MASK: u8 = 0b0110;

pub(super) fn select_state(index: usize) -> &'static str {
    match index {
        LIGHT_OPTION_INDEX => "selected=light",
        DARK_OPTION_INDEX => "selected=dark",
        SYSTEM_OPTION_INDEX => "selected=system",
        _ => "selected=none",
    }
}

pub(super) fn select_read_state(is_open: bool, selected_index: Option<usize>) -> &'static str {
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

pub(super) fn combo_state(index: usize) -> &'static str {
    match index {
        COMBO_TWO_INDEX => "selected=two",
        _ => "selected=one",
    }
}

pub(super) fn combo_read_state(
    is_open: bool,
    is_filtered: bool,
    selected_index: Option<usize>,
) -> &'static str {
    match (is_open, is_filtered, selected_index) {
        (true, true, Some(COMBO_TWO_INDEX)) => "open=true query=tw selected=two",
        (true, true, _) => "open=true query=tw selected=none",
        (false, false, Some(COMBO_TWO_INDEX)) => "open=false query=empty selected=two",
        (false, false, _) => "open=false query=empty selected=none",
        _ => "open=false query=empty selected=none",
    }
}

pub(super) fn selection_list_state(
    single: Option<usize>,
    multi_mask: u8,
    focus: Option<usize>,
) -> &'static str {
    match (single, multi_mask & SELECTION_LIST_MULTI_MASK, focus) {
        (None, 0, None) => "single=none multi=none focus=none",
        (Some(SELECTION_LIST_ZERO_INDEX), 0, Some(SELECTION_LIST_ZERO_INDEX)) => {
            "single=0 multi=none focus=0"
        }
        (Some(SELECTION_LIST_ONE_INDEX), 0, Some(SELECTION_LIST_ONE_INDEX)) => {
            "single=1 multi=none focus=1"
        }
        (Some(SELECTION_LIST_TWO_INDEX), 0, Some(SELECTION_LIST_TWO_INDEX)) => {
            "single=2 multi=none focus=2"
        }
        (Some(SELECTION_LIST_THREE_INDEX), 0, Some(SELECTION_LIST_THREE_INDEX)) => {
            "single=3 multi=none focus=3"
        }
        (
            Some(SELECTION_LIST_ONE_INDEX),
            SELECTION_LIST_SECOND_MASK,
            Some(SELECTION_LIST_ONE_INDEX),
        ) => "single=1 multi=1 focus=1",
        (
            Some(SELECTION_LIST_ONE_INDEX),
            SELECTION_LIST_SECOND_THIRD_MASK,
            Some(SELECTION_LIST_TWO_INDEX),
        ) => "single=1 multi=1,2 focus=2",
        (
            Some(SELECTION_LIST_TWO_INDEX),
            SELECTION_LIST_SECOND_THIRD_MASK,
            Some(SELECTION_LIST_TWO_INDEX),
        ) => "single=2 multi=1,2 focus=2",
        (
            Some(SELECTION_LIST_TWO_INDEX),
            SELECTION_LIST_SECOND_THIRD_MASK,
            Some(SELECTION_LIST_THREE_INDEX),
        ) => "single=2 multi=1,2 focus=3",
        (
            Some(SELECTION_LIST_THREE_INDEX),
            SELECTION_LIST_SECOND_THIRD_MASK,
            Some(SELECTION_LIST_THREE_INDEX),
        ) => "single=3 multi=1,2 focus=3",
        (
            Some(SELECTION_LIST_THREE_INDEX),
            SELECTION_LIST_SECOND_THIRD_MASK,
            Some(SELECTION_LIST_ZERO_INDEX),
        ) => "single=3 multi=1,2 focus=0",
        (
            Some(SELECTION_LIST_ZERO_INDEX),
            SELECTION_LIST_SECOND_THIRD_MASK,
            Some(SELECTION_LIST_ZERO_INDEX),
        ) => "single=0 multi=1,2 focus=0",
        (
            Some(SELECTION_LIST_TWO_INDEX),
            SELECTION_LIST_SECOND_MASK,
            Some(SELECTION_LIST_TWO_INDEX),
        ) => "single=2 multi=1 focus=2",
        (
            Some(SELECTION_LIST_THREE_INDEX),
            SELECTION_LIST_SECOND_MASK,
            Some(SELECTION_LIST_THREE_INDEX),
        ) => "single=3 multi=1 focus=3",
        (
            Some(SELECTION_LIST_ZERO_INDEX),
            SELECTION_LIST_SECOND_MASK,
            Some(SELECTION_LIST_ZERO_INDEX),
        ) => "single=0 multi=1 focus=0",
        _ => "single=none multi=none focus=none",
    }
}
