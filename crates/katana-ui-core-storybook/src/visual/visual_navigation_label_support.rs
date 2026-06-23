use super::Canvas;
use super::visual_navigation_support::pixel_at;

pub(super) struct InkVerticalBounds {
    pub(super) top: usize,
    pub(super) bottom: usize,
}

pub(super) fn ink_vertical_bounds_in_rect(
    canvas: &Canvas,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    background: u32,
) -> Option<InkVerticalBounds> {
    let mut top = y + height;
    let mut bottom = y;
    for current_y in y..y + height {
        for current_x in x..x + width {
            if pixel_at(canvas, current_x, current_y) == Some(background) {
                continue;
            }
            top = top.min(current_y);
            bottom = bottom.max(current_y);
        }
    }
    if top > bottom {
        return None;
    }
    Some(InkVerticalBounds { top, bottom })
}

pub(super) fn count_text_antialias_pixels(
    canvas: &Canvas,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    background: u32,
    text: u32,
) -> usize {
    let mut count = 0;
    for current_y in y..y + height {
        for current_x in x..x + width {
            let Some(pixel) = pixel_at(canvas, current_x, current_y) else {
                continue;
            };
            if pixel != background && pixel != text {
                count += 1;
            }
        }
    }
    count
}
