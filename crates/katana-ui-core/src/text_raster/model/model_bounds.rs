use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlatformTextGraphemeBounds {
    pub byte_start: usize,
    pub byte_end: usize,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl PlatformTextGraphemeBounds {
    #[must_use]
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformTextGraphemeRange {
    pub byte_start: usize,
    pub byte_end: usize,
}

impl PlatformTextGraphemeRange {
    #[must_use]
    pub fn ranges(text: &str) -> Vec<Self> {
        text.grapheme_indices(true)
            .map(|(byte_start, grapheme)| Self {
                byte_start,
                byte_end: byte_start + grapheme.len(),
            })
            .collect()
    }

    #[must_use]
    pub fn previous(text: &str, byte_offset: usize) -> Option<Self> {
        let byte_offset = clamp_to_char_boundary(text, byte_offset);
        Self::ranges(text)
            .into_iter()
            .rev()
            .find(|range| range.byte_end <= byte_offset)
    }

    #[must_use]
    pub fn next(text: &str, byte_offset: usize) -> Option<Self> {
        let byte_offset = clamp_to_char_boundary(text, byte_offset);
        Self::ranges(text)
            .into_iter()
            .find(|range| range.byte_start >= byte_offset)
    }
}

fn clamp_to_char_boundary(text: &str, byte_offset: usize) -> usize {
    let mut boundary = byte_offset.min(text.len());
    while boundary > 0 && !text.is_char_boundary(boundary) {
        boundary -= 1;
    }
    boundary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_clamps_an_interior_utf8_offset_to_the_previous_boundary() {
        let text = "a⭐b";
        let next = PlatformTextGraphemeRange::next(text, 2).expect("star grapheme");
        assert_eq!(next.byte_start, 1);
    }
}
