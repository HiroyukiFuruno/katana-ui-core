use super::super::dedicated_dod_form_input_live_layout::search_inline_clear_rect;
use super::super::text::TextBox;
use super::{Canvas, TextRenderer, VisualPalette, m};

pub(super) fn draw_clear_action(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    hovered: bool,
) {
    let rect = search_inline_clear_rect(x, y);
    let border = if hovered {
        palette.hover_border
    } else {
        palette.border
    };
    canvas.fill_rect(rect.x, rect.y, rect.width, rect.height, palette.panel);
    canvas.stroke_rect(rect.x, rect.y, rect.width, rect.height, border);
    text.draw_in_box(
        canvas,
        "x",
        TextBox::centered(rect.x, rect.y, rect.width, rect.height),
        m::FONT_8,
        palette.text,
    );
}
