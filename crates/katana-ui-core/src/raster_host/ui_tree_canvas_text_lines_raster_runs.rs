use super::bidi_context::bidi_context_for_fragment;
use super::{
    SpanTextRenderers, UiTextSpan, UiTreeTextMetrics, span_part_width_with_bidi_context,
    span_visible_part_bounds_with_edges,
};
use crate::raster_host::text::RichTextBidiContext;
use std::ops::Range;
use unicode_bidi::BidiInfo;

pub(in crate::raster_host) struct PositionedSpan {
    pub(in crate::raster_host) line_span_index: usize,
    pub(in crate::raster_host) span: UiTextSpan,
    pub(in crate::raster_host) x: isize,
    pub(in crate::raster_host) width: usize,
    source_range: Range<usize>,
    source_span_range: Range<usize>,
    pub(super) bidi_context: Option<RichTextBidiContext>,
}

impl PositionedSpan {
    pub(in crate::raster_host) fn visible_part_bounds(
        &self,
        renderers: SpanTextRenderers<'_>,
        metrics: UiTreeTextMetrics,
        preserve_whitespace: bool,
    ) -> (usize, usize) {
        if self.bidi_context.is_some()
            && (self.span.style.inline_code || self.span.style.monospace)
            && !self
                .span
                .text
                .chars()
                .next()
                .is_some_and(char::is_whitespace)
            && !self
                .span
                .text
                .chars()
                .next_back()
                .is_some_and(char::is_whitespace)
        {
            return (0, self.width);
        }
        span_visible_part_bounds_with_edges(
            renderers,
            &self.span,
            metrics,
            preserve_whitespace,
            self.source_range.start == self.source_span_range.start,
            self.source_range.end == self.source_span_range.end,
        )
    }
}

pub(in crate::raster_host) fn visual_span_positions(
    renderers: SpanTextRenderers<'_>,
    line: &[UiTextSpan],
    metrics: UiTreeTextMetrics,
    preserve_whitespace: bool,
    x: isize,
) -> Option<Vec<PositionedSpan>> {
    if !renderers.has_independent_code_raster_config() {
        return None;
    }
    if !line.iter().any(is_code) || !line.iter().any(|span| !is_code(span)) {
        return None;
    }
    let text = line
        .iter()
        .map(|span| span.text.as_str())
        .collect::<String>();
    let bidi = BidiInfo::new(&text, None);
    let paragraph = bidi.paragraphs.first()?;
    let (levels, visual_ranges) = bidi.visual_runs(paragraph, paragraph.range.clone());
    let reorders_spans = visual_ranges
        .windows(2)
        .any(|ranges| ranges[1].start < ranges[0].start)
        || visual_ranges.iter().any(|range| {
            levels
                .get(range.start)
                .is_some_and(unicode_bidi::Level::is_rtl)
        });
    if !reorders_spans {
        return None;
    }
    let mut positioned = Vec::new();
    let mut cursor = x;
    for visual_range in visual_ranges {
        let level = levels.get(visual_range.start)?;
        let mut fragments = Vec::new();
        let mut source_start = 0;
        for (line_span_index, source_span) in line.iter().enumerate() {
            let source_end = source_start + source_span.text.len();
            let source_span_range = source_start..source_end;
            if let Some(fragment_range) = intersection(&source_span_range, &visual_range) {
                let source_fragment_range =
                    fragment_slice_range(&text, &source_span.text, &fragment_range, source_start)?;
                let mut span = source_span.clone();
                span.text = source_span.text[source_fragment_range].to_owned();
                fragments.push(PositionedSpan {
                    line_span_index,
                    span,
                    x,
                    width: 0,
                    bidi_context: bidi_context_for_fragment(&text, &levels, &fragment_range),
                    source_range: fragment_range,
                    source_span_range,
                });
            }
            source_start = source_end;
        }
        if level.is_rtl() {
            fragments.reverse();
        }
        for fragment in &mut fragments {
            fragment.x = cursor;
            fragment.width = span_part_width_with_bidi_context(
                renderers,
                &fragment.span,
                metrics,
                preserve_whitespace,
                fragment.bidi_context.as_ref(),
            );
            cursor = cursor.saturating_add(fragment.width as isize);
        }
        positioned.extend(fragments);
    }
    positioned.sort_by_key(|fragment| fragment.source_range.start);
    (!positioned.is_empty()).then_some(positioned)
}

fn intersection(left: &Range<usize>, right: &Range<usize>) -> Option<Range<usize>> {
    let start = left.start.max(right.start);
    let end = left.end.min(right.end);
    (start < end).then_some(start..end)
}

fn fragment_slice_range(
    text: &str,
    source_text: &str,
    fragment_range: &Range<usize>,
    source_start: usize,
) -> Option<Range<usize>> {
    let start = fragment_range.start.checked_sub(source_start)?;
    let end = fragment_range.end.checked_sub(source_start)?;
    (text.is_char_boundary(fragment_range.start)
        && text.is_char_boundary(fragment_range.end)
        && source_text.is_char_boundary(start)
        && source_text.is_char_boundary(end))
    .then_some(start..end)
}

pub(super) fn is_code(span: &UiTextSpan) -> bool {
    span.style.monospace || span.style.inline_code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fragment_slices_require_global_and_local_utf8_boundaries() {
        assert_eq!(Some(0..2), fragment_slice_range("abc", "bc", &(1..3), 1),);
        assert_eq!(
            None,
            fragment_slice_range("あ", "あ", &(1..3), 0),
            "the global range must start at a character boundary",
        );
        assert_eq!(
            None,
            fragment_slice_range("abc", "あ", &(1..3), 1),
            "the source span range must end at a character boundary",
        );
        assert_eq!(
            None,
            fragment_slice_range("abc", "bc", &(0..1), 1),
            "the fragment must remain within its source span",
        );
    }
}
