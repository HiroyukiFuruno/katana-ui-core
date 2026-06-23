use super::layout_metrics::LayoutRect;
use super::selection_control_metrics as sm;

const CONTROL_BUTTON_X: usize = sm::STATUS_X;
const CONTROL_BUTTON_Y: usize = 116;
const CONTROL_BUTTON_WIDTH: usize = 56;
const CONTROL_BUTTON_HEIGHT: usize = 20;
const CONTROL_BUTTON_GAP: usize = 8;

pub(super) fn selection_list_state_read_button_rect(x: usize, y: usize) -> LayoutRect {
    LayoutRect::new(
        x + CONTROL_BUTTON_X,
        y + CONTROL_BUTTON_Y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn selection_list_select_row_button_rect(x: usize, y: usize) -> LayoutRect {
    let state_read = selection_list_state_read_button_rect(x, y);
    LayoutRect::new(
        state_read.right() + CONTROL_BUTTON_GAP,
        state_read.y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn selection_list_multi_toggle_button_rect(x: usize, y: usize) -> LayoutRect {
    let state_read = selection_list_state_read_button_rect(x, y);
    LayoutRect::new(
        state_read.x,
        state_read.bottom() + CONTROL_BUTTON_GAP,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn selection_list_keyboard_next_button_rect(x: usize, y: usize) -> LayoutRect {
    let multi = selection_list_multi_toggle_button_rect(x, y);
    LayoutRect::new(
        multi.x,
        multi.bottom() + CONTROL_BUTTON_GAP,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn selection_list_reset_button_rect(x: usize, y: usize) -> LayoutRect {
    let multi = selection_list_multi_toggle_button_rect(x, y);
    LayoutRect::new(
        multi.right() + CONTROL_BUTTON_GAP,
        multi.y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}
