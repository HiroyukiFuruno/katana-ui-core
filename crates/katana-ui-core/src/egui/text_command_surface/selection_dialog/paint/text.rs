use super::TextCommandSurfaceStyle;
use crate::render_model::{UiTextSpan, UiTextSpanStyle};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Copy)]
pub(super) struct DialogText<'a> {
    pub(super) value: &'a str,
    pub(super) highlights: &'a [(usize, usize)],
}

pub(super) fn styled_spans(
    text: DialogText<'_>,
    style: &TextCommandSurfaceStyle,
) -> Vec<UiTextSpan> {
    let base = UiTextSpanStyle {
        color_rgba: style.text_raster.fallback_color_rgba,
        ..Default::default()
    };
    if text.highlights.is_empty() {
        return UiTextSpan::emoji_marked_spans(text.value, base);
    }
    text.value
        .grapheme_indices(true)
        .flat_map(|(offset, grapheme)| {
            let highlighted = text
                .highlights
                .iter()
                .any(|&(start, end)| offset >= start && offset < end);
            UiTextSpan::emoji_marked_spans(
                grapheme,
                UiTextSpanStyle {
                    bold: highlighted,
                    highlight: highlighted,
                    color_rgba: if highlighted {
                        style.text_paint.caret_rgba
                    } else {
                        base.color_rgba
                    },
                    ..base
                },
            )
        })
        .collect()
}
