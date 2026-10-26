use super::layout_model::{TextSurfaceGraphemeBox, TextSurfaceLayout, TextSurfaceLineBox};
use crate::render_model::UiRect;
use crate::text_selection::{UiTextGlyphBox, UiTextLineBox, UiTextSelectionModel};
use std::collections::BTreeMap;

impl TextSurfaceLayout {
    pub(super) fn selection_model(
        text: &str,
        graphemes: &[TextSurfaceGraphemeBox],
        line_boxes: &[TextSurfaceLineBox],
    ) -> UiTextSelectionModel {
        let mut rows = BTreeMap::<i32, Vec<UiTextGlyphBox>>::new();
        for value in graphemes {
            let glyph = UiTextGlyphBox::new(
                value.grapheme_index,
                value.byte_start..value.byte_end,
                value.bounds,
                value.bounds.y.saturating_add(value.bounds.height as i32),
            )
            .with_text(
                text.get(value.byte_start..value.byte_end)
                    .unwrap_or_default(),
            );
            rows.entry(value.bounds.y).or_default().push(glyph);
        }
        if let Some(line) = line_boxes.last()
            && !rows.contains_key(&line.bounds.y)
        {
            let bounds = UiRect::new(line.bounds.x, line.bounds.y, 1, line.bounds.height);
            let caret = UiTextGlyphBox::new(
                graphemes.len(),
                text.len()..text.len(),
                bounds,
                bounds.y.saturating_add_unsigned(bounds.height),
            )
            .with_text("");
            rows.insert(bounds.y, vec![caret]);
        }
        let lines = rows
            .into_values()
            .map(|glyphs| {
                let byte_start = glyphs.first().map_or(0, |glyph| glyph.byte_range.start);
                let byte_end = glyphs
                    .last()
                    .map_or(byte_start, |glyph| glyph.byte_range.end);
                UiTextLineBox::new(byte_start..byte_end, glyphs)
            })
            .collect();
        UiTextSelectionModel::new(text, lines)
    }
}
