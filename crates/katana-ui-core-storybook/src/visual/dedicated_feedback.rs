use super::canvas::Canvas;
use super::dedicated_common::{
    BAR_HEIGHT, BAR_Y, LARGE_MARK, MEDIUM_MARK, SUCCESS, TEXT_X, TEXT_Y, filled, outlined,
};
use super::palette::VisualPalette;
use super::text::TextRenderer;
use katana_ui_core::render_model::UiNode;

const BAR_WIDTH: usize = 112;
const FILL_MIN: usize = 12;
const FULL_PERCENT: usize = 100;

pub(super) fn badge(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    label: &str,
) {
    filled(canvas, text, palette, x, y, label, SUCCESS);
}

pub(super) fn progress(
    canvas: &mut Canvas,
    text: &TextRenderer,
    node: &UiNode,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    label: &str,
) {
    outlined(canvas, text, palette, x, y, label);
    let fill = usize::from(node.props().progress_percent).max(FILL_MIN) * BAR_WIDTH / FULL_PERCENT;
    canvas.fill_rect(x + TEXT_X, y + BAR_Y, fill, BAR_HEIGHT, palette.accent);
}

pub(super) fn overlay(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    label: &str,
) {
    outlined(canvas, text, palette, x, y, label);
    canvas.stroke_rect(
        x + TEXT_X,
        y + TEXT_Y,
        LARGE_MARK,
        MEDIUM_MARK,
        palette.accent,
    );
}
