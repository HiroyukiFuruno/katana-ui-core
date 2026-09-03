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

#[cfg(test)]
mod tests {
    use super::{Canvas, draw_text_selection_highlight};

    #[test]
    fn selection_highlight_requires_both_points_and_a_selected_text_run() {
        let mut canvas = Canvas::new(32, 16, 0x101010);
        assert!(!draw_text_selection_highlight(
            &mut canvas,
            None,
            Some((20, 8)),
            0x4499ff,
        ));
        assert!(!draw_text_selection_highlight(
            &mut canvas,
            Some((4, 4)),
            None,
            0x4499ff,
        ));
        assert!(!draw_text_selection_highlight(
            &mut canvas,
            Some((4, 4)),
            Some((20, 8)),
            0x4499ff,
        ));

        canvas.record_text_run("selected", 4, 4, 20, 8);
        assert!(draw_text_selection_highlight(
            &mut canvas,
            Some((5, 5)),
            Some((20, 5)),
            0x4499ff,
        ));
        assert!(canvas.pixels().iter().any(|pixel| *pixel != 0x101010));
    }
}
