use super::layout_metrics::LayoutRect;
use super::selection_control_metrics as sm;

const CONTROL_BUTTON_X: usize = sm::STATUS_X;
const CONTROL_BUTTON_Y: usize = 116;
const CONTROL_BUTTON_WIDTH: usize = 56;
const CONTROL_BUTTON_HEIGHT: usize = 20;
const CONTROL_BUTTON_GAP: usize = 8;

pub(super) fn combo_state_read_button_rect(x: usize, y: usize) -> LayoutRect {
    LayoutRect::new(
        x + CONTROL_BUTTON_X,
        y + CONTROL_BUTTON_Y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn combo_filter_button_rect(x: usize, y: usize) -> LayoutRect {
    let read = combo_state_read_button_rect(x, y);
    LayoutRect::new(
        read.right() + CONTROL_BUTTON_GAP,
        read.y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn combo_select_button_rect(x: usize, y: usize) -> LayoutRect {
    let read = combo_state_read_button_rect(x, y);
    LayoutRect::new(
        read.x,
        read.bottom() + CONTROL_BUTTON_GAP,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn combo_reset_button_rect(x: usize, y: usize) -> LayoutRect {
    let select = combo_select_button_rect(x, y);
    LayoutRect::new(
        select.right() + CONTROL_BUTTON_GAP,
        select.y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}
