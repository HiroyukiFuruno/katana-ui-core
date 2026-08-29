use super::layout_model::TextSurfaceLayout;
use crate::render_model::UiRect;

impl TextSurfaceLayout {
    /// Returns the logical rows whose KUC layout bounds intersect `viewport`.
    #[must_use]
    pub fn visible_logical_rows(&self, viewport: UiRect) -> Vec<usize> {
        self.lines
            .iter()
            .filter_map(|line| Self::intersects(line.bounds, viewport).then_some(line.logical_row))
            .collect()
    }

    #[must_use]
    pub fn line_bounds(&self, logical_row: usize) -> Option<UiRect> {
        self.lines
            .iter()
            .find(|line| line.logical_row == logical_row)
            .map(|line| line.bounds)
    }

    #[must_use]
    pub fn bounds_for_byte_offset(&self, byte_offset: usize) -> Option<UiRect> {
        if byte_offset > self.text.len() || !self.text.is_char_boundary(byte_offset) {
            return None;
        }
        self.graphemes
            .iter()
            .find(|grapheme| grapheme.byte_start <= byte_offset && byte_offset < grapheme.byte_end)
            .or_else(|| {
                self.graphemes
                    .last()
                    .filter(|_| byte_offset == self.text.len())
            })
            .map(|grapheme| grapheme.bounds)
    }

    #[must_use]
    pub fn bounds_for_byte_range(&self, byte_start: usize, byte_end: usize) -> Option<UiRect> {
        if byte_start > byte_end
            || byte_end > self.text.len()
            || !self.text.is_char_boundary(byte_start)
            || !self.text.is_char_boundary(byte_end)
        {
            return None;
        }
        if byte_start == byte_end {
            return self.bounds_for_byte_offset(byte_start);
        }
        self.graphemes
            .iter()
            .filter(|grapheme| grapheme.byte_end > byte_start && grapheme.byte_start < byte_end)
            .map(|grapheme| grapheme.bounds)
            .reduce(TextSurfaceLayout::union)
    }
}
