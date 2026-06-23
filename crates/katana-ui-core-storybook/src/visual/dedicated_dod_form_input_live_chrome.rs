use super::{
    CLEAR_SIZE, CLEAR_X, CLEAR_Y, Canvas, FIELD_ICON_X, FIELD_ICON_Y, SEARCH_ICON_STEM_OFFSET,
    VisualPalette, common, m,
};

pub(super) fn draw_search_icon(canvas: &mut Canvas, palette: &VisualPalette, x: usize, y: usize) {
    canvas.fill_rect(
        x + FIELD_ICON_X,
        y + FIELD_ICON_Y,
        m::PX_10,
        m::PX_2,
        palette.accent,
    );
    canvas.fill_rect(
        x + FIELD_ICON_X + SEARCH_ICON_STEM_OFFSET,
        y + FIELD_ICON_Y - SEARCH_ICON_STEM_OFFSET,
        m::PX_2,
        m::PX_10,
        palette.accent,
    );
}

pub(super) fn draw_clear_button(canvas: &mut Canvas, x: usize, y: usize) {
    canvas.fill_rect(
        x + CLEAR_X,
        y + CLEAR_Y,
        CLEAR_SIZE,
        CLEAR_SIZE,
        common::DANGER,
    );
}
