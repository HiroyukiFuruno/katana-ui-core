use super::canvas::Canvas;
use super::dedicated_common::{
    BAR_Y, HINT_HEIGHT, HINT_WIDTH, LARGE_MARK, MEDIUM_MARK, SMALL_MARK, TEXT_SIZE, TEXT_X, TEXT_Y,
    filled, outlined,
};
use super::palette::VisualPalette;
use super::text::TextRenderer;

pub(super) fn button(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    label: &str,
) {
    filled(canvas, text, palette, x, y, label, palette.accent);
}

pub(super) fn outlined_control(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    label: &str,
) {
    outlined(canvas, text, palette, x, y, label);
}

pub(super) fn structured(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    label: &str,
) {
    outlined(canvas, text, palette, x, y, label);
    canvas.fill_rect(
        x + HINT_WIDTH - MEDIUM_MARK,
        y + BAR_Y,
        SMALL_MARK,
        SMALL_MARK,
        palette.accent,
    );
}

pub(super) fn toggle(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    label: &str,
) {
    outlined(canvas, text, palette, x, y, label);
    canvas.fill_rect(
        x + HINT_WIDTH - MEDIUM_MARK,
        y + TEXT_Y,
        SMALL_MARK,
        SMALL_MARK,
        palette.accent,
    );
}

pub(super) fn fallback(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    canvas.fill_rect(x, y, LARGE_MARK, HINT_HEIGHT, palette.surface);
    text.draw(
        canvas,
        "node",
        x + TEXT_X,
        y + TEXT_Y,
        TEXT_SIZE,
        palette.text,
    );
}
