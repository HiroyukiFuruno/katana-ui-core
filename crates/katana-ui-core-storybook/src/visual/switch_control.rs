use super::canvas::Canvas;
use super::palette::VisualPalette;

const BORDER_INSET: usize = 1;
const KNOB_INSET: usize = 3;

pub(super) fn draw_switch(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    enabled: bool,
) {
    let fill = if enabled {
        palette.accent
    } else {
        palette.surface
    };
    let radius = height / 2;
    canvas.fill_round_rect(x, y, width, height, radius, palette.border);
    draw_track_fill(canvas, x, y, width, height, radius, fill);
    let knob = height.saturating_sub(KNOB_INSET * 2);
    let knob_x = if enabled {
        x + width.saturating_sub(knob + KNOB_INSET)
    } else {
        x + KNOB_INSET
    };
    canvas.fill_round_rect(
        knob_x,
        y + KNOB_INSET,
        knob,
        knob,
        knob / 2,
        palette.background,
    );
}

fn draw_track_fill(
    canvas: &mut Canvas,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    radius: usize,
    fill: u32,
) {
    let inner_width = width.saturating_sub(BORDER_INSET * 2);
    let inner_height = height.saturating_sub(BORDER_INSET * 2);
    if inner_width == 0 || inner_height == 0 {
        return;
    }
    canvas.fill_round_rect(
        x + BORDER_INSET,
        y + BORDER_INSET,
        inner_width,
        inner_height,
        radius.saturating_sub(BORDER_INSET),
        fill,
    );
}
