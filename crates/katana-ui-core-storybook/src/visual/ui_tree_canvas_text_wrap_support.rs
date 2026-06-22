use super::canvas::Canvas;
use super::ui_tree_canvas_types::UiTreeRenderArea;

pub(super) fn range_has_content(
    canvas: &Canvas,
    background: u32,
    start_y: usize,
    end_y: usize,
) -> bool {
    (start_y..end_y)
        .any(|y| (0..canvas.width()).any(|x| canvas.pixels()[y * canvas.width() + x] != background))
}

pub(super) fn rect_has_content(
    canvas: &Canvas,
    background: u32,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
) -> bool {
    (start_y..end_y)
        .any(|y| (start_x..end_x).any(|x| canvas.pixels()[y * canvas.width() + x] != background))
}

pub(super) fn diff_in_rect(
    left: &Canvas,
    right: &Canvas,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
) -> usize {
    (start_y..end_y)
        .map(|y| {
            (start_x..end_x)
                .filter(|x| {
                    left.pixels()[y * left.width() + *x] != right.pixels()[y * right.width() + *x]
                })
                .count()
        })
        .sum()
}

pub(super) fn render_area(width: usize, height: usize) -> UiTreeRenderArea {
    UiTreeRenderArea {
        x: 0,
        y: 0,
        width,
        height,
        scroll_y: 0.0,
    }
}

pub(super) fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    if x >= canvas.width() || y >= canvas.height() {
        return None;
    }
    Some(canvas.pixels()[y * canvas.width() + x])
}
