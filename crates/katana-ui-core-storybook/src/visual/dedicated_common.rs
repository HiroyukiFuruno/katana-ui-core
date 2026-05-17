use super::canvas::Canvas;
use super::palette::VisualPalette;
use super::text::TextRenderer;

pub(super) const HINT_WIDTH: usize = 154;
pub(super) const HINT_HEIGHT: usize = 24;
pub(super) const TEXT_X: usize = 10;
pub(super) const TEXT_Y: usize = 6;
pub(super) const TEXT_SIZE: f32 = 10.0;
pub(super) const SMALL_MARK: usize = 8;
pub(super) const MEDIUM_MARK: usize = 18;
pub(super) const LARGE_MARK: usize = 46;
pub(super) const BAR_HEIGHT: usize = 6;
pub(super) const BAR_Y: usize = 9;
pub(super) const SUCCESS: u32 = 0x6a9955;
pub(super) const WARNING: u32 = 0xd7ba7d;
pub(super) const DANGER: u32 = 0xf44747;

pub(super) fn filled(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    label: &str,
    color: u32,
) {
    canvas.fill_rect(x, y, HINT_WIDTH, HINT_HEIGHT, color);
    text.draw(
        canvas,
        label,
        x + TEXT_X,
        y + TEXT_Y,
        TEXT_SIZE,
        palette.background,
    );
}

pub(super) fn outlined(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    label: &str,
) {
    canvas.stroke_rect(x, y, HINT_WIDTH, HINT_HEIGHT, palette.border);
    text.draw(
        canvas,
        label,
        x + TEXT_X,
        y + TEXT_Y,
        TEXT_SIZE,
        palette.text,
    );
}
