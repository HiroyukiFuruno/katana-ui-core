use super::layout_metrics::LayoutRect;
use super::selection_control_metrics as sm;

const CONTROL_BUTTON_X: usize = sm::STATUS_X;
const CONTROL_BUTTON_Y: usize = 116;
const CONTROL_BUTTON_WIDTH: usize = 56;
const CONTROL_BUTTON_HEIGHT: usize = 20;
const CONTROL_BUTTON_GAP: usize = 8;

pub(super) fn select_state_read_button_rect(x: usize, y: usize) -> LayoutRect {
    LayoutRect::new(
        x + CONTROL_BUTTON_X,
        y + CONTROL_BUTTON_Y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn select_open_button_rect(x: usize, y: usize) -> LayoutRect {
    let read = select_state_read_button_rect(x, y);
    LayoutRect::new(
        read.right() + CONTROL_BUTTON_GAP,
        read.y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn select_close_button_rect(x: usize, y: usize) -> LayoutRect {
    let read = select_state_read_button_rect(x, y);
    LayoutRect::new(
        read.x,
        read.bottom() + CONTROL_BUTTON_GAP,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn select_reset_button_rect(x: usize, y: usize) -> LayoutRect {
    let close = select_close_button_rect(x, y);
    LayoutRect::new(
        close.right() + CONTROL_BUTTON_GAP,
        close.y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}
