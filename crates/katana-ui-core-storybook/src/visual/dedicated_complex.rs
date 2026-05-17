use super::canvas::Canvas;
use super::dedicated_common::{
    BAR_HEIGHT, BAR_Y, DANGER, HINT_WIDTH, LARGE_MARK, MEDIUM_MARK, SMALL_MARK, SUCCESS, TEXT_X,
    TEXT_Y, WARNING, outlined,
};
use super::palette::VisualPalette;
use super::text::TextRenderer;

pub(super) fn diff(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    label: &str,
) {
    outlined(canvas, text, palette, x, y, label);
    canvas.fill_rect(x + TEXT_X, y + TEXT_Y, LARGE_MARK, BAR_HEIGHT, DANGER);
    canvas.fill_rect(
        x + TEXT_X,
        y + TEXT_Y + BAR_Y,
        LARGE_MARK,
        BAR_HEIGHT,
        SUCCESS,
    );
}

pub(super) fn color_picker(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    label: &str,
) {
    outlined(canvas, text, palette, x, y, label);
    canvas.fill_rect(
        x + HINT_WIDTH - LARGE_MARK,
        y + TEXT_Y,
        MEDIUM_MARK,
        MEDIUM_MARK,
        palette.accent,
    );
    canvas.fill_rect(
        x + HINT_WIDTH - MEDIUM_MARK,
        y + TEXT_Y,
        SMALL_MARK,
        MEDIUM_MARK,
        WARNING,
    );
}
