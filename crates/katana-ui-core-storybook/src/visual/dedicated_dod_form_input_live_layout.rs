use super::layout_metrics::LayoutRect;

const FIELD_X: usize = 18;
const FIELD_Y: usize = 36;
const FIELD_WIDTH: usize = 210;
const FIELD_HEIGHT: usize = 34;
const CLEAR_X: usize = 208;
const CLEAR_Y: usize = 46;
const CLEAR_SIZE: usize = 14;
const CONTROL_BUTTON_X: usize = 246;
const CONTROL_BUTTON_Y: usize = 116;
const CONTROL_BUTTON_WIDTH: usize = 56;
const CONTROL_BUTTON_HEIGHT: usize = 20;
const CONTROL_BUTTON_GAP: usize = 8;

pub(super) fn search_state_read_button_rect(x: usize, y: usize) -> LayoutRect {
    LayoutRect::new(
        x + CONTROL_BUTTON_X,
        y + CONTROL_BUTTON_Y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn search_field_rect(x: usize, y: usize) -> LayoutRect {
    LayoutRect::new(x + FIELD_X, y + FIELD_Y, FIELD_WIDTH, FIELD_HEIGHT)
}

pub(super) fn search_inline_clear_rect(x: usize, y: usize) -> LayoutRect {
    LayoutRect::new(x + CLEAR_X, y + CLEAR_Y, CLEAR_SIZE, CLEAR_SIZE)
}

pub(super) fn search_type_query_button_rect(x: usize, y: usize) -> LayoutRect {
    let read = search_state_read_button_rect(x, y);
    LayoutRect::new(
        read.right() + CONTROL_BUTTON_GAP,
        read.y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn search_submit_button_rect(x: usize, y: usize) -> LayoutRect {
    let read = search_state_read_button_rect(x, y);
    LayoutRect::new(
        read.x,
        read.bottom() + CONTROL_BUTTON_GAP,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn search_clear_button_rect(x: usize, y: usize) -> LayoutRect {
    let submit = search_submit_button_rect(x, y);
    LayoutRect::new(
        submit.right() + CONTROL_BUTTON_GAP,
        submit.y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn search_case_toggle_button_rect(x: usize, y: usize) -> LayoutRect {
    let submit = search_submit_button_rect(x, y);
    LayoutRect::new(
        submit.x,
        submit.bottom() + CONTROL_BUTTON_GAP,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn search_regex_toggle_button_rect(x: usize, y: usize) -> LayoutRect {
    let case = search_case_toggle_button_rect(x, y);
    LayoutRect::new(
        case.right() + CONTROL_BUTTON_GAP,
        case.y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}
