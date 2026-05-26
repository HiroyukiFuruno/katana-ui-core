use super::layout_metrics::{CONTENT_HEIGHT, LayoutRect};
use super::render::{VIEWPORT_HEIGHT, WIDTH};
use super::scrollbar_model::ScrollbarModel;

const SCROLLBAR_TRACK_WIDTH: usize = 6;
const SCROLLBAR_TRACK_RIGHT_INSET: usize = 10;
const SCROLLBAR_TRACK_Y: usize = 12;
const SCROLLBAR_TRACK_BOTTOM_INSET: usize = 12;
const SCROLLBAR_THUMB_MIN_HEIGHT: usize = 64;

pub(super) fn track_rect() -> LayoutRect {
    LayoutRect::new(
        WIDTH - SCROLLBAR_TRACK_RIGHT_INSET - SCROLLBAR_TRACK_WIDTH,
        SCROLLBAR_TRACK_Y,
        SCROLLBAR_TRACK_WIDTH,
        VIEWPORT_HEIGHT - SCROLLBAR_TRACK_Y - SCROLLBAR_TRACK_BOTTOM_INSET,
    )
}

pub(super) fn thumb_rect(scroll_y: usize) -> LayoutRect {
    model().thumb_rect(scroll_y)
}

fn model() -> ScrollbarModel {
    ScrollbarModel::vertical(
        track_rect(),
        VIEWPORT_HEIGHT,
        CONTENT_HEIGHT,
        SCROLLBAR_THUMB_MIN_HEIGHT,
    )
}

#[cfg(test)]
mod tests {
    use super::{SCROLLBAR_THUMB_MIN_HEIGHT, thumb_rect, track_rect};
    use crate::visual::layout_metrics::{CONTENT_HEIGHT, MAX_SCROLL_Y};
    use crate::visual::render::VIEWPORT_HEIGHT;

    #[test]
    fn root_visible_scrollbar_thumb_follows_content_metrics() {
        let track = track_rect();
        let thumb = thumb_rect(0);
        let expected_height = (track.height * VIEWPORT_HEIGHT / CONTENT_HEIGHT)
            .max(SCROLLBAR_THUMB_MIN_HEIGHT)
            .min(track.height);

        assert_eq!(expected_height, thumb.height);
    }

    #[test]
    fn root_visible_scrollbar_thumb_reaches_track_edges() {
        let track = track_rect();
        let top = thumb_rect(0);
        let bottom = thumb_rect(MAX_SCROLL_Y);

        assert_eq!(track.x, top.x);
        assert_eq!(track.x, bottom.x);
        assert_eq!(track.y, top.y);
        assert!(bottom.y > top.y);
        assert_eq!(track.bottom(), bottom.bottom());
    }
}
