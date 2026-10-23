use super::layout_metrics::LayoutRect;
use super::text_selection_types::SelectableTextRun;
use katana_ui_core::render_model::UiRect;
use katana_ui_core::text_selection::{UiTextGlyphBox, UiTextLineBox, UiTextSelectionModel};
use unicode_segmentation::UnicodeSegmentation;

impl SelectableTextRun {
    #[must_use]
    pub(super) fn new(text: &str, x: usize, y: usize, width: usize, height: usize) -> Self {
        Self::new_in_logical_line(text, x, y, width, height, None)
    }

    #[must_use]
    pub(super) fn new_in_logical_line(
        text: &str,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        logical_line: Option<u64>,
    ) -> Self {
        Self::new_in_logical_line_with_separator(text, x, y, width, height, logical_line, "")
    }

    #[must_use]
    pub(super) fn new_in_logical_line_with_separator(
        text: &str,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        logical_line: Option<u64>,
        separator_before: &str,
    ) -> Self {
        let grapheme_count = text.graphemes(true).count().max(1);
        let glyph_width = (width.max(1) / grapheme_count).max(1) as u32;
        let model = UiTextSelectionModel::from_monospace_text(
            text,
            x as i32,
            y as i32,
            glyph_width,
            height.max(1) as u32,
        );
        Self {
            text: text.to_string(),
            rect: LayoutRect::new(x, y, width, height),
            model,
            logical_line,
            visual_line: None,
            separator_before: separator_before.to_owned(),
            resolved_bidi_rtl: None,
        }
    }

    #[must_use]
    #[cfg(test)]
    pub(super) fn with_glyph_widths(
        text: &str,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        glyph_widths: &[usize],
    ) -> Self {
        Self::with_glyph_widths_in_logical_line(text, x, y, width, height, glyph_widths, None)
    }

    #[must_use]
    #[cfg(test)]
    pub(super) fn with_glyph_widths_in_logical_line(
        text: &str,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        glyph_widths: &[usize],
        logical_line: Option<u64>,
    ) -> Self {
        let mut glyph_x = 0usize;
        let glyph_positions = glyph_widths
            .iter()
            .map(|width| {
                let position = (glyph_x, (*width).max(1));
                glyph_x = glyph_x.saturating_add(position.1);
                position
            })
            .collect::<Vec<_>>();
        Self::with_glyph_positions_in_logical_line(
            text,
            x,
            y,
            width,
            height,
            &glyph_positions,
            logical_line,
        )
    }

    #[must_use]
    #[cfg(test)]
    pub(super) fn with_glyph_positions(
        text: &str,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        glyph_positions: &[(usize, usize)],
    ) -> Self {
        Self::with_glyph_positions_in_logical_line(text, x, y, width, height, glyph_positions, None)
    }

    #[must_use]
    #[cfg(test)]
    pub(super) fn with_glyph_positions_in_logical_line(
        text: &str,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        glyph_positions: &[(usize, usize)],
        logical_line: Option<u64>,
    ) -> Self {
        Self::with_glyph_positions_in_logical_line_with_bidi_direction(
            text,
            x,
            y,
            width,
            height,
            glyph_positions,
            logical_line,
            None,
        )
    }

    #[must_use]
    pub(super) fn with_glyph_positions_in_logical_line_with_bidi_direction(
        text: &str,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        glyph_positions: &[(usize, usize)],
        logical_line: Option<u64>,
        resolved_bidi_rtl: Option<bool>,
    ) -> Self {
        Self::with_glyph_positions_in_logical_line_with_bidi_direction_and_separator(
            text,
            x,
            y,
            width,
            height,
            glyph_positions,
            logical_line,
            resolved_bidi_rtl,
            "",
        )
    }

    #[must_use]
    pub(super) fn with_glyph_positions_in_logical_line_with_bidi_direction_and_separator(
        text: &str,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        glyph_positions: &[(usize, usize)],
        logical_line: Option<u64>,
        resolved_bidi_rtl: Option<bool>,
        separator_before: &str,
    ) -> Self {
        let mut glyphs = Vec::new();
        let mut byte_offset = 0usize;
        let mut fallback_x = 0usize;
        for (index, grapheme) in text.graphemes(true).enumerate() {
            let byte_start = byte_offset;
            byte_offset += grapheme.len();
            let (glyph_offset, glyph_width) = glyph_positions
                .get(index)
                .copied()
                .unwrap_or((fallback_x, 1));
            fallback_x = fallback_x.max(glyph_offset.saturating_add(glyph_width.max(1)));
            glyphs.push(
                UiTextGlyphBox::new(
                    index,
                    byte_start..byte_offset,
                    UiRect::new(
                        x.saturating_add(glyph_offset) as i32,
                        y as i32,
                        glyph_width.max(1) as u32,
                        height.max(1) as u32,
                    ),
                    y as i32 + height.max(1) as i32,
                )
                .with_text(grapheme),
            );
        }
        let lines = vec![UiTextLineBox::new(0..text.len(), glyphs)];
        let model = match resolved_bidi_rtl {
            Some(direction) => {
                UiTextSelectionModel::with_resolved_bidi_direction(text, lines, direction)
            }
            None => UiTextSelectionModel::new(text, lines),
        };
        Self {
            text: text.to_string(),
            rect: LayoutRect::new(x, y, width, height),
            model,
            logical_line,
            visual_line: None,
            separator_before: separator_before.to_owned(),
            resolved_bidi_rtl,
        }
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    pub(in crate::raster_host) fn rect(&self) -> LayoutRect {
        self.rect
    }

    #[must_use]
    pub(in crate::raster_host) const fn resolved_bidi_rtl(&self) -> Option<bool> {
        self.resolved_bidi_rtl
    }

    #[must_use]
    pub fn x(&self) -> usize {
        self.rect.x
    }

    #[must_use]
    pub fn y(&self) -> usize {
        self.rect.y
    }

    #[must_use]
    pub fn width(&self) -> usize {
        self.rect.width
    }

    #[must_use]
    pub fn height(&self) -> usize {
        self.rect.height
    }

    #[must_use]
    pub fn right(&self) -> usize {
        self.rect.right()
    }

    #[must_use]
    pub fn bottom(&self) -> usize {
        self.rect.bottom()
    }

    #[must_use]
    pub(in crate::raster_host) fn model(&self) -> &UiTextSelectionModel {
        &self.model
    }

    #[must_use]
    pub(in crate::raster_host) const fn logical_line(&self) -> Option<u64> {
        self.logical_line
    }

    #[must_use]
    pub(in crate::raster_host) fn separator_before(&self) -> &str {
        &self.separator_before
    }
}

#[cfg(test)]
mod tests {
    use super::SelectableTextRun;

    #[test]
    fn resolved_outer_bidi_direction_controls_single_fragment_hit_testing() {
        let run = SelectableTextRun::with_glyph_positions_in_logical_line_with_bidi_direction(
            "a",
            10,
            0,
            10,
            10,
            &[(0, 10)],
            None,
            Some(true),
        );

        assert_eq!(1, run.model().point_to_caret(11, 5));
        assert_eq!(0, run.model().point_to_caret(19, 5));
    }

    #[test]
    fn resolved_direction_participates_in_run_equality() {
        let ltr = SelectableTextRun::with_glyph_positions_in_logical_line_with_bidi_direction(
            "a",
            10,
            0,
            10,
            10,
            &[(0, 10)],
            None,
            Some(false),
        );
        let rtl = SelectableTextRun::with_glyph_positions_in_logical_line_with_bidi_direction(
            "a",
            10,
            0,
            10,
            10,
            &[(0, 10)],
            None,
            Some(true),
        );

        assert_ne!(ltr, rtl);
    }
}
