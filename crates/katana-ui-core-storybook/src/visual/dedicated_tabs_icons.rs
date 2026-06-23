use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Rect};
use super::dedicated_tabs_metrics::{
    PIN_CROSS_Y_OFFSET, PIN_HEAD_WIDTH, PIN_HEAD_X_OFFSET, PIN_ICON_SIZE, PIN_STEM_HEIGHT,
    PIN_STEM_WIDTH, PIN_STEM_X_OFFSET,
};

pub(super) fn draw_pin_icon(canvas: &mut Canvas, x: usize, y: usize, color: u32) {
    common::fill(
        canvas,
        Rect::new(x + PIN_HEAD_X_OFFSET, y, PIN_HEAD_WIDTH, PIN_ICON_SIZE),
        color,
    );
    common::fill(
        canvas,
        Rect::new(x, y + PIN_CROSS_Y_OFFSET, PIN_ICON_SIZE, PIN_HEAD_WIDTH),
        color,
    );
    common::fill(
        canvas,
        Rect::new(
            x + PIN_STEM_X_OFFSET,
            y + PIN_ICON_SIZE - PIN_STEM_WIDTH,
            PIN_STEM_WIDTH,
            PIN_STEM_HEIGHT,
        ),
        color,
    );
}
