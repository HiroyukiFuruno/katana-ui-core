use super::layout_metrics::LayoutRect;

pub(super) const FIELD_X: usize = 18;
pub(super) const FIELD_Y: usize = 36;
pub(super) const FIELD_WIDTH: usize = 210;
pub(super) const FIELD_HEIGHT: usize = 34;
pub(super) const FIELD_BORDER_WIDTH: usize = 1;
pub(super) const FIELD_TEXT_MARGIN_LEFT: usize = 2;
pub(super) const FIELD_TEXT_PADDING: usize = FIELD_BORDER_WIDTH + FIELD_TEXT_MARGIN_LEFT;
pub(super) const FIELD_TEXT_X: usize = FIELD_X + FIELD_TEXT_PADDING;
pub(super) const FIELD_TEXT_X_WITH_LEADING_SLOT: usize = 43;
#[cfg(test)]
pub(super) const FIELD_TEXT_CLIP_WIDTH: usize =
    FIELD_WIDTH - FIELD_TEXT_PADDING - FIELD_TEXT_PADDING;
pub(super) const FIELD_ICON_X: usize = 28;
pub(super) const FIELD_ICON_Y: usize = 47;
pub(super) const FIELD_CURSOR_WIDTH: usize = 2;
pub(super) const FIELD_CURSOR_HEIGHT: usize = 18;
pub(super) const FIELD_TRAILING_BUTTON_COUNT: usize = 3;
pub(super) const FIELD_TRAILING_BUTTON_SIZE: usize = 20;
pub(super) const FIELD_TRAILING_BUTTON_GAP: usize = 4;
pub(super) const FIELD_TRAILING_BUTTON_INSET: usize = 6;
pub(super) const CLEAR_X: usize = 208;
pub(super) const CLEAR_Y: usize = 46;
pub(super) const CLEAR_SIZE: usize = 14;
pub(super) const SEARCH_ICON_STEM_OFFSET: usize = 4;
pub(super) const STATUS_TEXT_X: usize = 7;
pub(super) const STATUS_TEXT_Y: usize = 6;
pub(super) const CHIP_LABEL_COUNT: usize = 3;
pub(super) const CHIP_WIDTH: usize = 68;
pub(super) const CHIP_HEIGHT: usize = 18;
pub(super) const CONTROL_BUTTON_X: usize = 246;
pub(super) const CONTROL_BUTTON_Y: usize = 116;
pub(super) const CONTROL_BUTTON_WIDTH: usize = 56;
pub(super) const CONTROL_BUTTON_HEIGHT: usize = 20;
pub(super) const CONTROL_BUTTON_GAP: usize = 8;
pub(super) const TEXT_AREA_Y: usize = 32;
pub(super) const TEXT_AREA_WIDTH: usize = 236;
pub(super) const TEXT_AREA_HEIGHT: usize = 92;
pub(super) const TEXT_AREA_LINE_X: usize = 30;
pub(super) const TEXT_AREA_LINE_FIRST_Y: usize = 54;
pub(super) const TEXT_AREA_LINE_STEP: usize = 18;
const STATUS_X: usize = 246;
const STATUS_Y: usize = 36;
const STATUS_WIDTH: usize = 150;
const STATUS_HEIGHT: usize = 20;
const STATUS_GAP: usize = 8;
const STATUS_ROW_COUNT: usize = 3;
const CHIP_Y: usize = 84;
const CHIP_GAP: usize = 8;
#[cfg(test)]
const TEXT_AREA_STATUS_X: usize = 272;
#[cfg(test)]
const TEXT_AREA_STATUS_WIDTH: usize = 96;

pub(super) fn search_state_read_button_rect(x: usize, y: usize) -> LayoutRect {
    LayoutRect::new(
        x + CONTROL_BUTTON_X,
        y + CONTROL_BUTTON_Y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn text_input_field_rect(x: usize, y: usize) -> LayoutRect {
    LayoutRect::new(x + FIELD_X, y + FIELD_Y, FIELD_WIDTH, FIELD_HEIGHT)
}

pub(super) fn search_field_rect(x: usize, y: usize) -> LayoutRect {
    text_input_field_rect(x, y)
}

pub(super) fn text_input_text_x(x: usize, leading_slot_reserved: bool) -> usize {
    if leading_slot_reserved {
        return x + FIELD_TEXT_X_WITH_LEADING_SLOT;
    }
    x + FIELD_TEXT_X
}

pub(super) fn text_input_text_clip_width(
    leading_slot_reserved: bool,
    trailing_icon_buttons: bool,
) -> usize {
    let text_x = if leading_slot_reserved {
        FIELD_TEXT_X_WITH_LEADING_SLOT
    } else {
        FIELD_TEXT_X
    };
    let trailing_width = if trailing_icon_buttons {
        FIELD_TRAILING_BUTTON_COUNT * FIELD_TRAILING_BUTTON_SIZE
            + (FIELD_TRAILING_BUTTON_COUNT - 1) * FIELD_TRAILING_BUTTON_GAP
            + FIELD_TRAILING_BUTTON_INSET
    } else {
        FIELD_TEXT_PADDING
    };
    FIELD_X
        .saturating_add(FIELD_WIDTH)
        .saturating_sub(text_x)
        .saturating_sub(trailing_width)
}

pub(super) fn text_input_trailing_icon_button_rects(
    x: usize,
    y: usize,
) -> [LayoutRect; FIELD_TRAILING_BUTTON_COUNT] {
    let total_width = FIELD_TRAILING_BUTTON_COUNT * FIELD_TRAILING_BUTTON_SIZE
        + (FIELD_TRAILING_BUTTON_COUNT - 1) * FIELD_TRAILING_BUTTON_GAP;
    let left = x + FIELD_X + FIELD_WIDTH - FIELD_TRAILING_BUTTON_INSET - total_width;
    let top = y + FIELD_Y + (FIELD_HEIGHT - FIELD_TRAILING_BUTTON_SIZE) / 2;
    [
        trailing_icon_button_rect(left, top, 0),
        trailing_icon_button_rect(left, top, 1),
        trailing_icon_button_rect(left, top, 2),
    ]
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

pub(super) fn text_input_status_rects(x: usize, y: usize) -> [LayoutRect; STATUS_ROW_COUNT] {
    [
        status_rect(x, y, 0),
        status_rect(x, y, 1),
        status_rect(x, y, 2),
    ]
}

pub(super) fn text_input_chip_rects(x: usize, y: usize) -> [LayoutRect; CHIP_LABEL_COUNT] {
    [chip_rect(x, y, 0), chip_rect(x, y, 1), chip_rect(x, y, 2)]
}

pub(super) fn text_area_rect(x: usize, y: usize) -> LayoutRect {
    LayoutRect::new(
        x + FIELD_X,
        y + TEXT_AREA_Y,
        TEXT_AREA_WIDTH,
        TEXT_AREA_HEIGHT,
    )
}

#[cfg(test)]
pub(super) fn text_area_status_rects(x: usize, y: usize) -> [LayoutRect; STATUS_ROW_COUNT] {
    [
        text_area_status_rect(x, y, 0),
        text_area_status_rect(x, y, 1),
        text_area_status_rect(x, y, 2),
    ]
}

fn status_rect(x: usize, y: usize, index: usize) -> LayoutRect {
    LayoutRect::new(
        x + STATUS_X,
        y + STATUS_Y + index * (STATUS_HEIGHT + STATUS_GAP),
        STATUS_WIDTH,
        STATUS_HEIGHT,
    )
}

fn chip_rect(x: usize, y: usize, index: usize) -> LayoutRect {
    LayoutRect::new(
        x + FIELD_X + index * (CHIP_WIDTH + CHIP_GAP),
        y + CHIP_Y,
        CHIP_WIDTH,
        CHIP_HEIGHT,
    )
}

#[cfg(test)]
fn text_area_status_rect(x: usize, y: usize, index: usize) -> LayoutRect {
    LayoutRect::new(
        x + TEXT_AREA_STATUS_X,
        y + STATUS_Y + index * (STATUS_HEIGHT + STATUS_GAP),
        TEXT_AREA_STATUS_WIDTH,
        STATUS_HEIGHT,
    )
}

fn trailing_icon_button_rect(left: usize, top: usize, index: usize) -> LayoutRect {
    LayoutRect::new(
        left + index * (FIELD_TRAILING_BUTTON_SIZE + FIELD_TRAILING_BUTTON_GAP),
        top,
        FIELD_TRAILING_BUTTON_SIZE,
        FIELD_TRAILING_BUTTON_SIZE,
    )
}
