use super::canvas::Canvas;
use super::dedicated_context_menu_metrics as cm;
use super::dedicated_dod_common::{self as common, Rect};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::text::TextRenderer;

pub(super) fn draw_anchor_surface(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    preset_index: usize,
    x: usize,
    y: usize,
) {
    match preset_index {
        cm::PRESET_EXPLORER_EMPTY => draw_explorer_empty_area(canvas, text, palette, x, y),
        cm::PRESET_TAB_BAR => draw_tab_bar(canvas, text, palette, x, y),
        cm::PRESET_MESSAGE_ROW => draw_message_row(canvas, text, palette, x, y),
        _ => draw_editor_surface(canvas, text, palette, x, y),
    }
}

fn draw_editor_surface(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    common::fill(
        canvas,
        Rect::new(
            x + cm::EDITOR_X,
            y + cm::EDITOR_Y,
            cm::EDITOR_WIDTH,
            cm::EDITOR_HEIGHT,
        ),
        palette.code_background,
    );
    text.draw(
        canvas,
        "fn main()",
        x + cm::EDITOR_TEXT_X,
        y + cm::EDITOR_TEXT_Y,
        m::FONT_8,
        palette.muted,
    );
    draw_pointer(
        canvas,
        palette,
        x + cm::EDITOR_POINTER_X,
        y + cm::EDITOR_POINTER_Y,
    );
}

fn draw_explorer_empty_area(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    let rect = Rect::new(
        x + cm::EXPLORER_X,
        y + cm::EXPLORER_Y,
        cm::EXPLORER_WIDTH,
        cm::EXPLORER_HEIGHT,
    );
    common::fill(canvas, rect, palette.surface);
    common::outline(canvas, palette, rect);
    text.draw(
        canvas,
        "empty area",
        x + cm::EXPLORER_TEXT_X,
        y + cm::EXPLORER_TEXT_Y,
        m::FONT_8,
        palette.muted,
    );
    draw_pointer(
        canvas,
        palette,
        x + cm::EXPLORER_POINTER_X,
        y + cm::EXPLORER_POINTER_Y,
    );
}

fn draw_tab_bar(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    common::fill(
        canvas,
        Rect::new(
            x + cm::TAB_BAR_X,
            y + cm::TAB_BAR_Y,
            cm::TAB_BAR_WIDTH,
            cm::TAB_BAR_HEIGHT,
        ),
        palette.code_background,
    );
    common::fill(
        canvas,
        Rect::new(
            x + cm::TAB_ITEM_X,
            y + cm::TAB_ITEM_Y,
            cm::TAB_ITEM_WIDTH,
            cm::TAB_ITEM_HEIGHT,
        ),
        palette.surface,
    );
    text.draw(
        canvas,
        "tab.rs",
        x + cm::TAB_TEXT_X,
        y + cm::TAB_TEXT_Y,
        m::FONT_7,
        palette.text,
    );
    draw_pointer(
        canvas,
        palette,
        x + cm::TAB_POINTER_X,
        y + cm::TAB_POINTER_Y,
    );
}

fn draw_message_row(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    let rect = Rect::new(
        x + cm::MESSAGE_X,
        y + cm::MESSAGE_Y,
        cm::MESSAGE_WIDTH,
        cm::MESSAGE_HEIGHT,
    );
    common::fill(canvas, rect, palette.surface);
    common::outline(canvas, palette, rect);
    text.draw(
        canvas,
        "message row",
        x + cm::MESSAGE_TEXT_X,
        y + cm::MESSAGE_TEXT_Y,
        m::FONT_8,
        palette.text,
    );
    text.draw(
        canvas,
        "reply / copy",
        x + cm::MESSAGE_TEXT_X,
        y + cm::MESSAGE_SUBTEXT_Y,
        m::FONT_7,
        palette.muted,
    );
    draw_pointer(
        canvas,
        palette,
        x + cm::MESSAGE_POINTER_X,
        y + cm::MESSAGE_POINTER_Y,
    );
}

fn draw_pointer(canvas: &mut Canvas, palette: &VisualPalette, x: usize, y: usize) {
    common::fill(
        canvas,
        Rect::new(x, y, cm::POINTER_WIDTH, cm::POINTER_HEIGHT),
        palette.accent,
    );
    common::fill(
        canvas,
        Rect::new(
            x + cm::POINTER_VERTICAL_X_OFFSET,
            y - cm::POINTER_VERTICAL_Y_OFFSET,
            cm::POINTER_VERTICAL_WIDTH,
            cm::POINTER_VERTICAL_HEIGHT,
        ),
        palette.accent,
    );
}
