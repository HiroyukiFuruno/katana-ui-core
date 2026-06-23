use super::dedicated_dod_form_choice_status as choice_status;
use super::layout_metrics::LayoutRect;

const CHOICE_ROW_X: usize = 18;
const CHOICE_ROW_WIDTH: usize = 244;
pub(super) const BINARY_CHOICE_AREA_HEIGHT: usize = 156;
pub(super) const CHOICE_ROW_HEIGHT: usize = 36;
const CHOICE_ROW_GAP: usize = 8;
pub(super) const CHOICE_LABEL_X: usize = 44;
const CHOICE_MARK_X: usize = 12;
const CHOICE_MARK_SIZE: usize = 20;
const CONTROL_BUTTON_Y: usize = 122;
const CONTROL_STATUS_Y: usize = choice_status::CHOICE_ROW_Y;
const CONTROL_HEIGHT: usize = 24;
const CONTROL_GAP: usize = 8;
pub(super) const CONTROL_TEXT_Y: usize = 4;
const CONTROL_STATE_X: usize = 280;
const CONTROL_STATE_WIDTH: usize = 160;
const CONTROL_BUTTON_WIDTH: usize = 68;
#[cfg(test)]
const MARK_TOP_OFFSET: usize = 8;
const MODERN_MARK_TOP_OFFSET: usize = 8;
const LABEL_RIGHT_INSET: usize = 6;

pub(super) fn row_rect(index: usize, x: usize, y: usize) -> LayoutRect {
    let row_y = y + choice_status::CHOICE_ROW_Y + index * (CHOICE_ROW_HEIGHT + CHOICE_ROW_GAP);
    LayoutRect::new(x + CHOICE_ROW_X, row_y, CHOICE_ROW_WIDTH, CHOICE_ROW_HEIGHT)
}

#[cfg(test)]
pub(super) fn checkbox_row_rect(index: usize, x: usize, y: usize) -> LayoutRect {
    row_rect(index, x, y)
}

pub(super) fn checkbox_mark_rect(index: usize, x: usize, y: usize) -> LayoutRect {
    let row = row_rect(index, x, y);
    LayoutRect::new(
        row.x + CHOICE_MARK_X,
        row.y + MODERN_MARK_TOP_OFFSET,
        CHOICE_MARK_SIZE,
        CHOICE_MARK_SIZE,
    )
}

pub(super) fn checkbox_label_rect(index: usize, x: usize, y: usize) -> LayoutRect {
    let row = row_rect(index, x, y);
    LayoutRect::new(
        row.x + CHOICE_LABEL_X,
        row.y,
        CHOICE_ROW_WIDTH - CHOICE_LABEL_X - LABEL_RIGHT_INSET,
        CHOICE_ROW_HEIGHT,
    )
}

pub(super) fn checkbox_state_read_button_rect(x: usize, y: usize) -> LayoutRect {
    LayoutRect::new(
        x + CHOICE_ROW_X,
        y + CONTROL_BUTTON_Y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_HEIGHT,
    )
}

pub(super) fn checkbox_toggle_button_rect(x: usize, y: usize) -> LayoutRect {
    let read = checkbox_state_read_button_rect(x, y);
    LayoutRect::new(
        read.right() + CONTROL_GAP,
        read.y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_HEIGHT,
    )
}

pub(super) fn checkbox_reset_button_rect(x: usize, y: usize) -> LayoutRect {
    let toggle = checkbox_toggle_button_rect(x, y);
    LayoutRect::new(
        toggle.right() + CONTROL_GAP,
        toggle.y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_HEIGHT,
    )
}

pub(super) fn checkbox_state_row_rect(x: usize, y: usize) -> LayoutRect {
    LayoutRect::new(
        x + CONTROL_STATE_X,
        y + CONTROL_STATUS_Y,
        CONTROL_STATE_WIDTH,
        CONTROL_HEIGHT,
    )
}

pub(super) fn checkbox_event_row_rect(x: usize, y: usize) -> LayoutRect {
    let state = checkbox_state_row_rect(x, y);
    LayoutRect::new(
        state.x,
        state.bottom() + CONTROL_GAP,
        state.width,
        state.height,
    )
}

pub(super) fn checkbox_log_row_rect(x: usize, y: usize) -> LayoutRect {
    let event = checkbox_event_row_rect(x, y);
    LayoutRect::new(
        event.x,
        event.bottom() + CONTROL_GAP,
        event.width,
        event.height,
    )
}

#[cfg(test)]
pub(super) fn radio_row_rect(index: usize, x: usize, y: usize) -> LayoutRect {
    row_rect(index, x, y)
}

#[cfg(test)]
pub(super) fn radio_mark_rect(index: usize, x: usize, y: usize) -> LayoutRect {
    let row = radio_row_rect(index, x, y);
    LayoutRect::new(
        row.x + CHOICE_MARK_X,
        row.y + MARK_TOP_OFFSET,
        CHOICE_MARK_SIZE,
        CHOICE_MARK_SIZE,
    )
}

#[cfg(test)]
pub(super) fn radio_label_rect(index: usize, x: usize, y: usize) -> LayoutRect {
    let row = radio_row_rect(index, x, y);
    LayoutRect::new(
        row.x + CHOICE_LABEL_X,
        row.y,
        CHOICE_ROW_WIDTH - CHOICE_LABEL_X - LABEL_RIGHT_INSET,
        CHOICE_ROW_HEIGHT,
    )
}

pub(super) fn radio_state_read_button_rect(x: usize, y: usize) -> LayoutRect {
    checkbox_state_read_button_rect(x, y)
}

pub(super) fn radio_select_button_rect(x: usize, y: usize) -> LayoutRect {
    checkbox_toggle_button_rect(x, y)
}

pub(super) fn radio_reset_button_rect(x: usize, y: usize) -> LayoutRect {
    checkbox_reset_button_rect(x, y)
}

pub(super) fn radio_state_row_rect(x: usize, y: usize) -> LayoutRect {
    checkbox_state_row_rect(x, y)
}

pub(super) fn radio_event_row_rect(x: usize, y: usize) -> LayoutRect {
    checkbox_event_row_rect(x, y)
}

pub(super) fn radio_log_row_rect(x: usize, y: usize) -> LayoutRect {
    checkbox_log_row_rect(x, y)
}
