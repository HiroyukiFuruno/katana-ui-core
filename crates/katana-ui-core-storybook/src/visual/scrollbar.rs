use super::canvas::Canvas;
use super::layout_metrics::{CONTENT_HEIGHT, LayoutRect, MAX_SCROLL_Y};
use super::palette::VisualPalette;
use super::render::{VIEWPORT_HEIGHT, WIDTH};

const SCROLLBAR_TRACK_WIDTH: usize = 6;
const SCROLLBAR_TRACK_RIGHT_INSET: usize = 10;
const SCROLLBAR_TRACK_Y: usize = 12;
const SCROLLBAR_TRACK_BOTTOM_INSET: usize = 12;
const SCROLLBAR_THUMB_MIN_HEIGHT: usize = 64;
const SCROLLBAR_RADIUS: usize = 3;

pub(super) fn draw(canvas: &mut Canvas, palette: &VisualPalette, scroll_y: usize) {
    let track = track_rect();
    canvas.fill_round_rect(
        track.x,
        track.y,
        track.width,
        track.height,
        SCROLLBAR_RADIUS,
        palette.code_background,
    );
    let thumb = thumb_rect(scroll_y);
    canvas.fill_round_rect(
        thumb.x,
        thumb.y,
        thumb.width,
        thumb.height,
        SCROLLBAR_RADIUS,
        palette.accent,
    );
}

pub(super) fn track_rect() -> LayoutRect {
    LayoutRect::new(
        WIDTH - SCROLLBAR_TRACK_RIGHT_INSET - SCROLLBAR_TRACK_WIDTH,
        SCROLLBAR_TRACK_Y,
        SCROLLBAR_TRACK_WIDTH,
        VIEWPORT_HEIGHT - SCROLLBAR_TRACK_Y - SCROLLBAR_TRACK_BOTTOM_INSET,
    )
}

pub(super) fn thumb_rect(scroll_y: usize) -> LayoutRect {
    let track = track_rect();
    let thumb_height = (VIEWPORT_HEIGHT * track.height / CONTENT_HEIGHT)
        .max(SCROLLBAR_THUMB_MIN_HEIGHT)
        .min(track.height);
    let travel = track.height.saturating_sub(thumb_height);
    let offset = travel
        .saturating_mul(scroll_y.min(MAX_SCROLL_Y))
        .checked_div(MAX_SCROLL_Y)
        .unwrap_or(0);
    LayoutRect::new(track.x, track.y + offset, track.width, thumb_height)
}

#[cfg(test)]
mod tests {
    use super::{thumb_rect, track_rect};
    use crate::visual::layout_metrics::MAX_SCROLL_Y;

    #[test]
    fn thumb_moves_inside_track_when_scrolled() {
        let track = track_rect();
        let top = thumb_rect(0);
        let bottom = thumb_rect(MAX_SCROLL_Y);

        assert_eq!(track.x, top.x);
        assert_eq!(track.x, bottom.x);
        assert!(bottom.y > top.y);
        assert!(bottom.bottom() <= track.bottom());
    }
}
