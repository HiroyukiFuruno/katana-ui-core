use super::canvas::Canvas;
use super::dedicated_common::{
    BAR_Y, HINT_HEIGHT, HINT_WIDTH, MEDIUM_MARK, SMALL_MARK, SUCCESS, TEXT_SIZE, TEXT_X, TEXT_Y,
    outlined,
};
use super::palette::VisualPalette;
use super::text::TextRenderer;

const SELECTION_MARK_Y_OFFSET: usize = 1;
const SPACER_LABEL_X: usize = 38;
const KEY_CAP_LABEL_X: usize = 36;
const DOT_GROUP_X: usize = 38;
const DOT_OFFSETS: [usize; 3] = [0, 12, 24];
const SLIDE_TRACK_X: usize = 48;
const SLIDE_TRACK_WIDTH: usize = 36;
const SLIDE_THUMB_X: usize = 30;
const HAIRLINE: usize = 1;

pub(super) fn icon_button(
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

pub(super) fn selection_control(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    label: &str,
) {
    outlined(canvas, text, palette, x, y, label);
    canvas.stroke_rect(
        x + HINT_WIDTH - MEDIUM_MARK,
        y + BAR_Y - SELECTION_MARK_Y_OFFSET,
        SMALL_MARK,
        SMALL_MARK,
        palette.accent,
    );
}

pub(super) fn divider(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    label: &str,
) {
    text.draw(
        canvas,
        label,
        x + TEXT_X,
        y + TEXT_Y,
        TEXT_SIZE,
        palette.text,
    );
    canvas.fill_rect(
        x,
        y + HINT_HEIGHT - BAR_Y,
        HINT_WIDTH,
        HAIRLINE,
        palette.border,
    );
}

pub(super) fn spacer(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    label: &str,
) {
    canvas.stroke_rect(x, y, HINT_WIDTH, HINT_HEIGHT, palette.border);
    canvas.fill_rect(x + TEXT_X, y + BAR_Y, MEDIUM_MARK, BAR_Y, palette.panel);
    text.draw(
        canvas,
        label,
        x + SPACER_LABEL_X,
        y + TEXT_Y,
        TEXT_SIZE,
        palette.muted,
    );
}

pub(super) fn key_cap(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    label: &str,
) {
    canvas.fill_rect(x, y, MEDIUM_MARK + SMALL_MARK, HINT_HEIGHT, palette.panel);
    canvas.stroke_rect(x, y, MEDIUM_MARK + SMALL_MARK, HINT_HEIGHT, palette.border);
    text.draw(
        canvas,
        label,
        x + KEY_CAP_LABEL_X,
        y + TEXT_Y,
        TEXT_SIZE,
        palette.text,
    );
}

pub(super) fn loading_dots(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    label: &str,
) {
    outlined(canvas, text, palette, x, y, label);
    for offset in DOT_OFFSETS {
        canvas.fill_rect(
            x + HINT_WIDTH - DOT_GROUP_X + offset,
            y + BAR_Y,
            SMALL_MARK,
            SMALL_MARK,
            palette.accent,
        );
    }
}

pub(super) fn spinner(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    label: &str,
) {
    outlined(canvas, text, palette, x, y, label);
    canvas.stroke_rect(
        x + HINT_WIDTH - MEDIUM_MARK,
        y + TEXT_Y,
        MEDIUM_MARK,
        MEDIUM_MARK,
        palette.accent,
    );
}

pub(super) fn color_swatch(
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
        MEDIUM_MARK,
        MEDIUM_MARK,
        SUCCESS,
    );
}

pub(super) fn slide_control(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    label: &str,
) {
    outlined(canvas, text, palette, x, y, label);
    canvas.fill_rect(
        x + HINT_WIDTH - SLIDE_TRACK_X,
        y + BAR_Y,
        SLIDE_TRACK_WIDTH,
        BAR_Y,
        palette.border,
    );
    canvas.fill_rect(
        x + HINT_WIDTH - SLIDE_THUMB_X,
        y + TEXT_Y,
        SMALL_MARK,
        MEDIUM_MARK,
        palette.accent,
    );
}
