use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::text::TextRenderer;

const PANEL_X: usize = 26;
const PANEL_Y: usize = 26;
const PANEL_WIDTH: usize = 210;
const PANEL_HEIGHT: usize = 86;
const ROW_X: usize = PANEL_X + 10;
const FIRST_ROW_Y: usize = PANEL_Y + 12;
const ROW_WIDTH: usize = PANEL_WIDTH - 20;
const ROW_HEIGHT: usize = 22;
const ROW_GAP: usize = 8;
const ROW_TEXT_X_OFFSET: usize = 10;
const ROW_TEXT_Y_OFFSET: usize = 7;

pub(super) fn menu(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    draw_menu_panel(canvas, text, palette, x, y, "Menu panel", ["Open", "Close"]);
}

pub(super) fn menu_button(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    draw_menu_panel(
        canvas,
        text,
        palette,
        x,
        y,
        "Menu button panel",
        ["Trigger", "Open"],
    );
}

pub(super) fn side_menu(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    draw_menu_panel(
        canvas,
        text,
        palette,
        x,
        y,
        "Side menu panel",
        ["Files", "Settings"],
    );
}

fn draw_menu_panel(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    title: &'static str,
    rows: [&'static str; 2],
) {
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        title,
        &[
            Block::outlined(PANEL_X, PANEL_Y, PANEL_WIDTH, PANEL_HEIGHT, palette.panel),
            Block::outlined(ROW_X, FIRST_ROW_Y, ROW_WIDTH, ROW_HEIGHT, palette.surface),
            Block::outlined(
                ROW_X,
                FIRST_ROW_Y + ROW_HEIGHT + ROW_GAP,
                ROW_WIDTH,
                ROW_HEIGHT,
                palette.surface,
            ),
        ],
        &[
            TextSpec::new(
                ROW_X + ROW_TEXT_X_OFFSET,
                FIRST_ROW_Y + ROW_TEXT_Y_OFFSET,
                m::FONT_8,
                palette.text,
                rows[0],
            ),
            TextSpec::new(
                ROW_X + ROW_TEXT_X_OFFSET,
                FIRST_ROW_Y + ROW_HEIGHT + ROW_GAP + ROW_TEXT_Y_OFFSET,
                m::FONT_8,
                palette.muted,
                rows[1],
            ),
        ],
    );
}
