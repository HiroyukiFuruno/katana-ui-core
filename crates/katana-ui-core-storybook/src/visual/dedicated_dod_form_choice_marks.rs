use super::canvas::Canvas;
use super::dedicated_dod_form_choice_status as choice_status;
use super::palette::VisualPalette;

pub(super) const CHOICE_MARK_SIZE: usize = 20;

const CHOICE_ROW_X: usize = 18;
const CHOICE_ROW_HEIGHT: usize = 36;
const CHOICE_ROW_GAP: usize = 8;
const CHOICE_MARK_X: usize = 12;
const CHECKBOX_RADIUS: usize = 7;
const MARK_BORDER_INSET: usize = 1;
const RADIO_DOT_INSET: usize = 5;
const RADIO_DOT_SIZE: usize = 10;
const MARK_ROW_Y_OFFSET: usize = 8;
const CHECK_GLYPH_SEGMENTS: [(usize, usize, usize, usize); 9] = [
    (4, 9, 2, 2),
    (5, 10, 2, 2),
    (6, 11, 2, 2),
    (7, 10, 2, 2),
    (8, 9, 2, 2),
    (9, 8, 2, 2),
    (10, 7, 2, 2),
    (11, 6, 2, 2),
    (12, 5, 3, 2),
];

pub(super) fn draw_checkbox_mark(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    index: usize,
    active: bool,
) {
    let (mark_x, mark_y) = mark_origin(x, y, index);
    let fill = if active {
        palette.accent
    } else {
        palette.panel
    };
    draw_outer_mark(canvas, palette, mark_x, mark_y, CHECKBOX_RADIUS, active);
    canvas.fill_round_rect(
        mark_x + MARK_BORDER_INSET,
        mark_y + MARK_BORDER_INSET,
        CHOICE_MARK_SIZE - MARK_BORDER_INSET * 2,
        CHOICE_MARK_SIZE - MARK_BORDER_INSET * 2,
        CHECKBOX_RADIUS.saturating_sub(MARK_BORDER_INSET),
        fill,
    );
    if active {
        draw_check_glyph(canvas, palette, mark_x, mark_y);
    }
}

pub(super) fn draw_radio_mark(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    index: usize,
    active: bool,
) {
    let (mark_x, mark_y) = mark_origin(x, y, index);
    draw_outer_mark(
        canvas,
        palette,
        mark_x,
        mark_y,
        CHOICE_MARK_SIZE / 2,
        active,
    );
    canvas.fill_round_rect(
        mark_x + MARK_BORDER_INSET,
        mark_y + MARK_BORDER_INSET,
        CHOICE_MARK_SIZE - MARK_BORDER_INSET * 2,
        CHOICE_MARK_SIZE - MARK_BORDER_INSET * 2,
        (CHOICE_MARK_SIZE - MARK_BORDER_INSET * 2) / 2,
        palette.panel,
    );
    if active {
        canvas.fill_round_rect(
            mark_x + RADIO_DOT_INSET,
            mark_y + RADIO_DOT_INSET,
            RADIO_DOT_SIZE,
            RADIO_DOT_SIZE,
            RADIO_DOT_SIZE / 2,
            palette.accent,
        );
    }
}

fn draw_outer_mark(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    mark_x: usize,
    mark_y: usize,
    radius: usize,
    active: bool,
) {
    let border = if active {
        palette.accent
    } else {
        palette.border
    };
    canvas.fill_round_rect(
        mark_x,
        mark_y,
        CHOICE_MARK_SIZE,
        CHOICE_MARK_SIZE,
        radius,
        border,
    );
}

fn draw_check_glyph(canvas: &mut Canvas, palette: &VisualPalette, mark_x: usize, mark_y: usize) {
    for (x, y, width, height) in CHECK_GLYPH_SEGMENTS {
        canvas.fill_rect(
            mark_x + x,
            mark_y + y,
            width,
            height,
            palette.accent_foreground,
        );
    }
}

fn mark_origin(x: usize, y: usize, index: usize) -> (usize, usize) {
    let mark_x = x + CHOICE_ROW_X + CHOICE_MARK_X;
    let mark_y = y
        + choice_status::CHOICE_ROW_Y
        + index * (CHOICE_ROW_HEIGHT + CHOICE_ROW_GAP)
        + MARK_ROW_Y_OFFSET;
    (mark_x, mark_y)
}

#[cfg(test)]
mod tests {
    use super::{Canvas, VisualPalette, draw_checkbox_mark};

    const BACKGROUND: u32 = 0x000000;
    const CUSTOM_ACCENT_FOREGROUND: u32 = 0x123456;
    const MARK_X: usize = 10;
    const MARK_Y: usize = 10;

    #[test]
    fn checkbox_checked_glyph_uses_accent_foreground_token() {
        let palette = VisualPalette {
            background: BACKGROUND,
            surface: 0x202020,
            panel: 0x242424,
            code_background: 0x282828,
            border: 0x303030,
            hover_border: 0x404040,
            text: 0xffffff,
            muted: 0x808080,
            accent: 0x0055aa,
            accent_foreground: CUSTOM_ACCENT_FOREGROUND,
            selection: 0x334455,
        };
        let mut canvas = Canvas::new(80, 80, BACKGROUND);

        draw_checkbox_mark(&mut canvas, &palette, MARK_X, MARK_Y, 0, true);

        assert!(
            canvas.pixels().contains(&CUSTOM_ACCENT_FOREGROUND),
            "checked glyph must use palette.accent_foreground instead of a fixed literal"
        );
    }
}
