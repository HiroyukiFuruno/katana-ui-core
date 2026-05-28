use crate::visual::dedicated_dod_form_input_live_layout::{
    FIELD_CURSOR_HEIGHT, FIELD_CURSOR_WIDTH, FIELD_HEIGHT, FIELD_Y, TEXT_AREA_LINE_FIRST_Y,
    TEXT_AREA_LINE_STEP, TEXT_AREA_LINE_X, TEXT_AREA_WIDTH, text_area_rect,
};
use crate::visual::dedicated_dod_metrics as m;
use crate::visual::layout_metrics::LayoutRect;

#[cfg(test)]
use crate::visual::dedicated_dod_form_input_live_layout::{FIELD_TEXT_CLIP_WIDTH, FIELD_TEXT_X};

const MAX_VISIBLE_TEXT_AREA_LINE_INDEX: usize = 3;
const TEXT_AREA_CARET_RIGHT_PADDING: usize = 42;

#[cfg(test)]
pub(in crate::visual) fn text_input_caret_rect(
    x: usize,
    y: usize,
    value_width: usize,
) -> LayoutRect {
    text_input_caret_rect_with_layout(x + FIELD_TEXT_X, y, FIELD_TEXT_CLIP_WIDTH, value_width)
}

pub(in crate::visual) fn text_input_caret_rect_with_layout(
    text_x: usize,
    y: usize,
    clip_width: usize,
    value_width: usize,
) -> LayoutRect {
    let max_offset = clip_width.saturating_sub(FIELD_CURSOR_WIDTH);
    LayoutRect::new(
        text_x + value_width.min(max_offset),
        y + FIELD_Y + (FIELD_HEIGHT - FIELD_CURSOR_HEIGHT) / m::PX_2,
        FIELD_CURSOR_WIDTH,
        FIELD_CURSOR_HEIGHT,
    )
}

pub(in crate::visual) fn text_area_caret_rect(
    x: usize,
    y: usize,
    value_width: usize,
    line_index: usize,
) -> LayoutRect {
    let rect = text_area_rect(x, y);
    let max_offset = TEXT_AREA_WIDTH.saturating_sub(TEXT_AREA_CARET_RIGHT_PADDING);
    LayoutRect::new(
        x + TEXT_AREA_LINE_X + value_width.min(max_offset),
        y + TEXT_AREA_LINE_FIRST_Y
            + line_index.min(MAX_VISIBLE_TEXT_AREA_LINE_INDEX) * TEXT_AREA_LINE_STEP,
        FIELD_CURSOR_WIDTH,
        FIELD_CURSOR_HEIGHT,
    )
    .clamped_to(rect)
}

trait CaretRectClamp {
    fn clamped_to(self, bounds: LayoutRect) -> Self;
}

impl CaretRectClamp for LayoutRect {
    fn clamped_to(self, bounds: LayoutRect) -> Self {
        Self::new(
            self.x.min(bounds.right().saturating_sub(self.width)),
            self.y.min(bounds.bottom().saturating_sub(self.height)),
            self.width,
            self.height,
        )
    }
}
