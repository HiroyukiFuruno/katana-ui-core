use super::common;
use super::m;
use crate::visual::layout_metrics::LayoutRect;

pub(in crate::visual) fn scrollbar_drag_rect(origin_x: usize, origin_y: usize) -> LayoutRect {
    LayoutRect::new(
        origin_x + super::SCROLLBAR_X,
        origin_y + super::SCROLLBAR_Y,
        super::SCROLLBAR_WIDTH,
        super::SCROLLBAR_HEIGHT,
    )
}

pub(in crate::visual) fn frame_rect(origin_x: usize, origin_y: usize) -> LayoutRect {
    LayoutRect::new(
        origin_x,
        origin_y,
        common::AREA_WIDTH,
        super::SCROLL_AREA_FRAME_HEIGHT,
    )
}

#[cfg(test)]
pub(in crate::visual) fn viewport_rect(origin_x: usize, origin_y: usize) -> LayoutRect {
    LayoutRect::new(
        origin_x + super::VIEWPORT_X,
        origin_y + super::VIEWPORT_Y,
        super::VIEWPORT_WIDTH,
        super::VIEWPORT_HEIGHT,
    )
}

pub(in crate::visual) fn content_clip_rect(origin_x: usize, origin_y: usize) -> LayoutRect {
    LayoutRect::new(
        origin_x + super::CONTENT_X,
        origin_y + super::CONTENT_Y,
        super::CONTENT_WIDTH,
        super::CONTENT_HEIGHT,
    )
}

pub(in crate::visual) fn resize_handle_rect(origin_x: usize, origin_y: usize) -> LayoutRect {
    LayoutRect::new(
        origin_x + super::VIEWPORT_X + super::VIEWPORT_WIDTH - m::PX_14,
        origin_y + super::VIEWPORT_Y + super::VIEWPORT_HEIGHT - m::PX_14,
        m::PX_12,
        m::PX_12,
    )
}

#[cfg(test)]
pub(in crate::visual) fn status_rects(
    origin_x: usize,
    origin_y: usize,
) -> [LayoutRect; super::STATUS_LABEL_COUNT] {
    [
        status_rect(origin_x, origin_y, 0),
        status_rect(origin_x, origin_y, 1),
        status_rect(origin_x, origin_y, 2),
    ]
}

#[cfg(test)]
fn status_rect(origin_x: usize, origin_y: usize, index: usize) -> LayoutRect {
    LayoutRect::new(
        origin_x + super::VIEWPORT_X + index * (super::STATUS_WIDTH + super::STATUS_GAP),
        origin_y + super::STATUS_Y,
        super::STATUS_WIDTH,
        super::STATUS_HEIGHT,
    )
}
