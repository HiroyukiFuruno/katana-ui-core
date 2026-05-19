use super::canvas::Canvas;
use super::dedicated_dod_form_choice_status as choice_status;
use super::palette::VisualPalette;

pub(super) const CHOICE_MARK_SIZE: usize = 12;

const CHOICE_ROW_X: usize = 18;
const CHOICE_ROW_HEIGHT: usize = 22;
const CHOICE_ROW_GAP: usize = 10;
const CHOICE_MARK_X: usize = 10;
const CHECKBOX_RADIUS: usize = 3;
const MARK_BORDER_INSET: usize = 1;
const CHECK_MARK_FIRST_X: usize = 3;
const CHECK_MARK_FIRST_Y: usize = 6;
const CHECK_MARK_FIRST_WIDTH: usize = 3;
const CHECK_MARK_HEIGHT: usize = 2;
const CHECK_MARK_SECOND_X: usize = 5;
const CHECK_MARK_SECOND_Y: usize = 4;
const CHECK_MARK_SECOND_WIDTH: usize = 5;
const RADIO_DOT_INSET: usize = 3;
const RADIO_DOT_SIZE: usize = 6;
const MARK_ROW_Y_OFFSET: usize = 5;

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
    draw_outer_mark(canvas, palette, mark_x, mark_y, CHECKBOX_RADIUS);
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
    draw_outer_mark(canvas, palette, mark_x, mark_y, CHOICE_MARK_SIZE / 2);
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
) {
    canvas.fill_round_rect(
        mark_x,
        mark_y,
        CHOICE_MARK_SIZE,
        CHOICE_MARK_SIZE,
        radius,
        palette.border,
    );
}

fn draw_check_glyph(canvas: &mut Canvas, palette: &VisualPalette, mark_x: usize, mark_y: usize) {
    canvas.fill_rect(
        mark_x + CHECK_MARK_FIRST_X,
        mark_y + CHECK_MARK_FIRST_Y,
        CHECK_MARK_FIRST_WIDTH,
        CHECK_MARK_HEIGHT,
        palette.background,
    );
    canvas.fill_rect(
        mark_x + CHECK_MARK_SECOND_X,
        mark_y + CHECK_MARK_SECOND_Y,
        CHECK_MARK_SECOND_WIDTH,
        CHECK_MARK_HEIGHT,
        palette.background,
    );
}

fn mark_origin(x: usize, y: usize, index: usize) -> (usize, usize) {
    let mark_x = x + CHOICE_ROW_X + CHOICE_MARK_X;
    let mark_y = y
        + choice_status::CHOICE_ROW_Y
        + index * (CHOICE_ROW_HEIGHT + CHOICE_ROW_GAP)
        + MARK_ROW_Y_OFFSET;
    (mark_x, mark_y)
}
