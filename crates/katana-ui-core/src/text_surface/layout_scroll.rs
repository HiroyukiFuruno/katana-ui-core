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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_model::UiRect;
    use crate::text_surface::TextSurfaceGraphemeBox;

    #[test]
    fn visible_rows_and_bounds_cover_expected_geometry() {
        let layout = TextSurfaceLayout::from_grapheme_boxes(
            "layout-scroll",
            UiRect::new(0, 0, 24, 40),
            "a\nb",
            vec![
                TextSurfaceGraphemeBox {
                    grapheme_index: 0,
                    byte_start: 0,
                    byte_end: 1,
                    bounds: UiRect::new(0, 0, 6, 20),
                },
                TextSurfaceGraphemeBox {
                    grapheme_index: 1,
                    byte_start: 1,
                    byte_end: 2,
                    bounds: UiRect::new(0, 0, 6, 20),
                },
                TextSurfaceGraphemeBox {
                    grapheme_index: 2,
                    byte_start: 2,
                    byte_end: 3,
                    bounds: UiRect::new(0, 20, 6, 20),
                },
            ],
        );

        assert_eq!(
            vec![0],
            layout.visible_logical_rows(UiRect::new(0, 0, 24, 10))
        );
        assert_eq!(
            vec![1],
            layout.visible_logical_rows(UiRect::new(0, 20, 24, 5))
        );
        assert_eq!(Some(UiRect::new(0, 0, 6, 20)), layout.line_bounds(0));
        assert_eq!(None, layout.line_bounds(9));
    }

    #[test]
    fn byte_offset_and_range_boundaries_validate_char_boundaries() {
        let layout = TextSurfaceLayout::from_grapheme_boxes(
            "layout-scroll-boundaries",
            UiRect::new(0, 0, 24, 40),
            "ab",
            vec![
                TextSurfaceGraphemeBox {
                    grapheme_index: 0,
                    byte_start: 0,
                    byte_end: 1,
                    bounds: UiRect::new(0, 0, 6, 20),
                },
                TextSurfaceGraphemeBox {
                    grapheme_index: 1,
                    byte_start: 1,
                    byte_end: 2,
                    bounds: UiRect::new(6, 0, 6, 20),
                },
            ],
        );

        assert_eq!(
            Some(UiRect::new(0, 0, 6, 20)),
            layout.bounds_for_byte_offset(0)
        );
        assert_eq!(
            Some(UiRect::new(6, 0, 6, 20)),
            layout.bounds_for_byte_offset(2)
        );
        assert_eq!(
            Some(UiRect::new(0, 0, 12, 20)),
            layout.bounds_for_byte_range(0, 2)
        );
        assert_eq!(
            Some(UiRect::new(6, 0, 6, 20)),
            layout.bounds_for_byte_range(1, 1)
        );
        assert_eq!(None, layout.bounds_for_byte_range(2, 1));
        assert_eq!(None, layout.bounds_for_byte_range(0, 3));
    }
}
