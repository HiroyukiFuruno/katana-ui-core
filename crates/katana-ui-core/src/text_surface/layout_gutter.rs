use super::layout_model::TextSurfaceLayout;

impl TextSurfaceLayout {
    #[must_use]
    pub(super) fn logical_row_for_byte_offset(&self, byte_offset: usize) -> Option<usize> {
        if byte_offset > self.text.len() || !self.text.is_char_boundary(byte_offset) {
            return None;
        }
        if self.graphemes.is_empty() {
            return None;
        }
        let glyph = self
            .graphemes
            .iter()
            .find(|grapheme| grapheme.byte_start <= byte_offset && byte_offset < grapheme.byte_end)
            .or_else(|| (byte_offset == self.text.len()).then_some(self.graphemes.last()?))?;
        self.logical_row_at(glyph.bounds.y)
    }

    fn logical_row_at(&self, y: i32) -> Option<usize> {
        self.lines
            .iter()
            .find(|line| {
                y >= line.bounds.y && y < line.bounds.y.saturating_add_unsigned(line.bounds.height)
            })
            .map(|line| line.logical_row)
    }
}
