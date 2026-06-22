use super::canvas::Canvas;
use super::text_selection::{TextSelection, selected_text_run_rects};

const TEXT_SELECTION_ALPHA: u8 = 96;

pub(super) fn draw_text_selection_highlight(
    canvas: &mut Canvas,
    start: Option<(usize, usize)>,
    end: Option<(usize, usize)>,
    color: u32,
) -> bool {
    let Some(start) = start else {
        return false;
    };
    let Some(end) = end else {
        return false;
    };
    let rects = selected_text_run_rects(canvas.text_runs(), TextSelection::drag(start, end));
    if rects.is_empty() {
        return false;
    }
    for rect in rects {
        canvas.blend_rect(
            rect.x,
            rect.y,
            rect.width,
            rect.height,
            color,
            TEXT_SELECTION_ALPHA,
        );
    }
    true
}
