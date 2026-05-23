use super::canvas::Canvas;
use super::layout_metrics::NAV_ROW_HEIGHT;
use super::palette::VisualPalette;

const DISCLOSURE_X: usize = 22;
const DISCLOSURE_SIZE: usize = 7;

pub(super) fn draw_disclosure(canvas: &mut Canvas, palette: &VisualPalette, open: bool, y: usize) {
    let top = centered_icon_y(y, DISCLOSURE_SIZE);
    if open {
        draw_open_disclosure(canvas, palette, top);
        return;
    }
    draw_closed_disclosure(canvas, palette, top);
}

fn draw_open_disclosure(canvas: &mut Canvas, palette: &VisualPalette, top: usize) {
    for offset in 0..DISCLOSURE_SIZE / 2 {
        canvas.set(DISCLOSURE_X + offset, top + offset, palette.text);
        canvas.set(
            DISCLOSURE_X + DISCLOSURE_SIZE - offset - 1,
            top + offset,
            palette.text,
        );
    }
    canvas.set(
        DISCLOSURE_X + DISCLOSURE_SIZE / 2,
        top + DISCLOSURE_SIZE / 2,
        palette.text,
    );
}

fn draw_closed_disclosure(canvas: &mut Canvas, palette: &VisualPalette, top: usize) {
    for offset in 0..DISCLOSURE_SIZE / 2 {
        canvas.set(DISCLOSURE_X + offset, top + offset, palette.text);
        canvas.set(
            DISCLOSURE_X + offset,
            top + DISCLOSURE_SIZE - offset - 1,
            palette.text,
        );
    }
    canvas.set(
        DISCLOSURE_X + DISCLOSURE_SIZE / 2,
        top + DISCLOSURE_SIZE / 2,
        palette.text,
    );
}

fn centered_icon_y(row_y: usize, icon_size: usize) -> usize {
    row_y + (NAV_ROW_HEIGHT - icon_size) / 2
}
