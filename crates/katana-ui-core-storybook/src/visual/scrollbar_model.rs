use super::layout_metrics::LayoutRect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ScrollbarModel {
    pub(super) track: LayoutRect,
    pub(super) thumb_height: usize,
    pub(super) max_offset: usize,
}

impl ScrollbarModel {
    pub(super) const fn new(track: LayoutRect, thumb_height: usize, max_offset: usize) -> Self {
        Self {
            track,
            thumb_height,
            max_offset,
        }
    }

    pub(super) fn thumb_rect(self, offset: usize) -> LayoutRect {
        LayoutRect::new(
            self.track.x,
            self.thumb_y(offset),
            self.track.width,
            self.thumb_height,
        )
    }

    pub(super) fn horizontal_thumb_rect(self, offset: usize) -> LayoutRect {
        LayoutRect::new(
            self.thumb_x(offset),
            self.track.y,
            self.thumb_height,
            self.track.height,
        )
    }

    pub(super) fn thumb_y(self, offset: usize) -> usize {
        let movable = self.track.height.saturating_sub(self.thumb_height);
        if self.max_offset == 0 {
            return self.track.y;
        }
        self.track.y + movable.saturating_mul(offset.min(self.max_offset)) / self.max_offset
    }

    pub(super) fn thumb_x(self, offset: usize) -> usize {
        let movable = self.track.width.saturating_sub(self.thumb_height);
        if self.max_offset == 0 {
            return self.track.x;
        }
        self.track.x + movable.saturating_mul(offset.min(self.max_offset)) / self.max_offset
    }

    pub(super) fn offset_from_thumb_y(self, y: usize) -> usize {
        let movable = self.track.height.saturating_sub(self.thumb_height);
        if movable == 0 {
            return 0;
        }
        let relative = y.saturating_sub(self.track.y).min(movable);
        relative * self.max_offset / movable
    }

    pub(super) fn offset_from_thumb_x(self, x: usize) -> usize {
        let movable = self.track.width.saturating_sub(self.thumb_height);
        if movable == 0 {
            return 0;
        }
        let relative = x.saturating_sub(self.track.x).min(movable);
        relative * self.max_offset / movable
    }
}
