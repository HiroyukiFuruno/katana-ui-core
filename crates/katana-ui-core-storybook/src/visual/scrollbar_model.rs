use super::layout_metrics::LayoutRect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ScrollbarModel {
    pub(super) track: LayoutRect,
    viewport_length: usize,
    max_offset: usize,
    min_thumb_length: usize,
}

impl ScrollbarModel {
    pub(super) fn vertical(
        track: LayoutRect,
        viewport_height: usize,
        content_height: usize,
        min_thumb_length: usize,
    ) -> Self {
        Self::vertical_from_max_offset(
            track,
            viewport_height,
            content_height.saturating_sub(viewport_height),
            min_thumb_length,
        )
    }

    #[cfg(test)]
    pub(super) fn horizontal(
        track: LayoutRect,
        viewport_width: usize,
        content_width: usize,
        min_thumb_length: usize,
    ) -> Self {
        Self::horizontal_from_max_offset(
            track,
            viewport_width,
            content_width.saturating_sub(viewport_width),
            min_thumb_length,
        )
    }

    pub(super) fn vertical_from_max_offset(
        track: LayoutRect,
        viewport_height: usize,
        max_offset: usize,
        min_thumb_length: usize,
    ) -> Self {
        Self::new(track, viewport_height, max_offset, min_thumb_length)
    }

    #[cfg(test)]
    pub(super) fn horizontal_from_max_offset(
        track: LayoutRect,
        viewport_width: usize,
        max_offset: usize,
        min_thumb_length: usize,
    ) -> Self {
        Self::new(track, viewport_width, max_offset, min_thumb_length)
    }

    fn new(
        track: LayoutRect,
        viewport_length: usize,
        max_offset: usize,
        min_thumb_length: usize,
    ) -> Self {
        Self {
            track,
            viewport_length,
            max_offset,
            min_thumb_length,
        }
    }

    pub(super) fn thumb_rect(self, offset: usize) -> LayoutRect {
        LayoutRect::new(
            self.track.x,
            self.thumb_y(offset),
            self.track.width,
            self.thumb_length(self.track.height),
        )
    }

    #[cfg(test)]
    pub(super) fn horizontal_thumb_rect(self, offset: usize) -> LayoutRect {
        LayoutRect::new(
            self.thumb_x(offset),
            self.track.y,
            self.thumb_length(self.track.width),
            self.track.height,
        )
    }

    pub(super) fn thumb_y(self, offset: usize) -> usize {
        let movable = self
            .track
            .height
            .saturating_sub(self.thumb_length(self.track.height));
        let max_offset = self.max_offset();
        if max_offset == 0 {
            return self.track.y;
        }
        self.track.y + movable.saturating_mul(offset.min(max_offset)) / max_offset
    }

    #[cfg(test)]
    pub(super) fn thumb_x(self, offset: usize) -> usize {
        let movable = self
            .track
            .width
            .saturating_sub(self.thumb_length(self.track.width));
        let max_offset = self.max_offset();
        if max_offset == 0 {
            return self.track.x;
        }
        self.track.x + movable.saturating_mul(offset.min(max_offset)) / max_offset
    }

    pub(super) fn offset_from_thumb_y(self, y: usize) -> usize {
        let movable = self
            .track
            .height
            .saturating_sub(self.thumb_length(self.track.height));
        if movable == 0 {
            return 0;
        }
        let relative = y.saturating_sub(self.track.y).min(movable);
        relative * self.max_offset() / movable
    }

    #[cfg(test)]
    pub(super) fn offset_from_thumb_x(self, x: usize) -> usize {
        let movable = self
            .track
            .width
            .saturating_sub(self.thumb_length(self.track.width));
        if movable == 0 {
            return 0;
        }
        let relative = x.saturating_sub(self.track.x).min(movable);
        relative * self.max_offset() / movable
    }

    pub(super) fn max_offset(self) -> usize {
        self.max_offset
    }

    fn thumb_length(self, track_length: usize) -> usize {
        thumb_length(
            track_length,
            self.viewport_length,
            self.content_length(),
            self.min_thumb_length,
        )
    }

    fn content_length(self) -> usize {
        self.viewport_length.saturating_add(self.max_offset)
    }
}

fn thumb_length(
    track_length: usize,
    viewport_length: usize,
    content_length: usize,
    min_thumb_length: usize,
) -> usize {
    if track_length == 0 || content_length == 0 || content_length <= viewport_length {
        return track_length;
    }
    track_length
        .saturating_mul(viewport_length)
        .checked_div(content_length)
        .unwrap_or(track_length)
        .max(min_thumb_length)
        .min(track_length)
}

#[cfg(test)]
#[path = "scrollbar_model_tests.rs"]
mod tests;
