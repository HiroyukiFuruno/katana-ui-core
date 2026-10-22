use super::bidi_context::bidi_context_for_fragment;
use super::{
    SpanTextRenderers, UiTextSpan, UiTreeTextMetrics, span_visible_part_bounds_with_edges,
};
use crate::raster_host::text::{RichTextBidiContext, RichTextLineSpan};
use crate::raster_host::ui_tree_canvas_text_line_width::span_rich_text_style;
use std::ops::Range;
use unicode_bidi::BidiInfo;

pub(in crate::raster_host) struct PositionedSpan {
    pub(in crate::raster_host) line_span_index: usize,
    pub(in crate::raster_host) shaping_run_index: usize,
    pub(in crate::raster_host) shaping_run_x: isize,
    pub(in crate::raster_host) span: UiTextSpan,
    pub(in crate::raster_host) x: isize,
    pub(in crate::raster_host) width: usize,
    source_range: Range<usize>,
    source_span_range: Range<usize>,
    pub(super) bidi_context: Option<RichTextBidiContext>,
    pub(in crate::raster_host) resolved_rtl: bool,
}

impl PositionedSpan {
    pub(in crate::raster_host) fn visible_part_bounds(
        &self,
        renderers: SpanTextRenderers<'_>,
        metrics: UiTreeTextMetrics,
        preserve_whitespace: bool,
    ) -> (usize, usize) {
        if !self
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
    if bidi.paragraphs.is_empty() {
        return None;
    }
    let mut positioned = Vec::new();
    let mut cursor = x;
    let mut reorders_spans = false;
    let mut shaping_run_index = 0usize;
    for paragraph in &bidi.paragraphs {
        let (levels, visual_ranges) = bidi.visual_runs(paragraph, paragraph.range.clone());
        reorders_spans |= visual_ranges
            .windows(2)
            .any(|ranges| ranges[1].start < ranges[0].start)
            || visual_ranges.iter().any(|range| {
                levels
                    .get(range.start)
                    .is_some_and(unicode_bidi::Level::is_rtl)
            });
        for visual_range in visual_ranges {
            let level = levels.get(visual_range.start)?;
            let mut fragments = Vec::new();
            let mut source_start = 0;
            for (line_span_index, source_span) in line.iter().enumerate() {
                let source_end = source_start + source_span.text.len();
                let source_span_range = source_start..source_end;
                if let Some(fragment_range) = intersection(&source_span_range, &visual_range) {
                    let source_fragment_range = fragment_slice_range(
                        &text,
                        &source_span.text,
                        &fragment_range,
                        source_start,
                    )?;
                    let mut span = source_span.clone();
                    span.text = source_span.text[source_fragment_range].to_owned();
                    fragments.push(PositionedSpan {
                        line_span_index,
                        shaping_run_index: 0,
                        shaping_run_x: x,
                        span,
                        x,
                        width: 0,
                        bidi_context: None,
                        resolved_rtl: level.is_rtl(),
                        source_range: fragment_range,
                        source_span_range,
                    });
                }
                source_start = source_end;
            }
            fragments.sort_by_key(|fragment| fragment.source_range.start);
            let mut groups = split_shaping_groups(fragments, renderers, &text);
            if level.is_rtl() {
                groups.reverse();
            }
            for mut group in groups {
                let shaping_run_x = cursor;
                let renderer = renderers.for_span(&group[0].span);
                let group_range = group.first()?.source_range.start..group.last()?.source_range.end;
                let bidi_context = bidi_context_for_fragment(&text, &levels, &group_range);
                let rich_spans = group
                    .iter()
                    .map(|fragment| {
                        renderer.rich_line_span(
                            fragment.span.text.clone(),
                            span_rich_text_style(&fragment.span, metrics, 0),
                        )
                    })
                    .collect::<Vec<RichTextLineSpan>>();
                let (layout, group_width) = renderer.measure_rich_span_layout_with_bidi_context(
                    &rich_spans,
                    bidi_context.as_ref(),
                    level.is_rtl(),
                );
                for (fragment, (offset, width)) in group.iter_mut().zip(layout) {
                    fragment.x = cursor.saturating_add(offset as isize);
                    fragment.width = width;
                    fragment.shaping_run_x = shaping_run_x;
                    fragment.bidi_context = bidi_context.clone();
                    fragment.shaping_run_index = shaping_run_index;
                    fragment.resolved_rtl = level.is_rtl();
                }
                cursor = cursor.saturating_add(group_width as isize);
                positioned.extend(group);
                shaping_run_index = shaping_run_index.saturating_add(1);
            }
        }
    }
    if !reorders_spans {
        return None;
    }
    positioned.sort_by_key(|fragment| fragment.source_range.start);
    (!positioned.is_empty()).then_some(positioned)
}

fn split_shaping_groups(
    fragments: Vec<PositionedSpan>,
    renderers: SpanTextRenderers<'_>,
    source_text: &str,
) -> Vec<Vec<PositionedSpan>> {
    let mut groups: Vec<Vec<PositionedSpan>> = Vec::new();
    for fragment in fragments {
        let renderer = renderers.for_span(&fragment.span);
        if let Some(group) = groups.last_mut()
            && group.last().is_some_and(|previous: &PositionedSpan| {
                std::ptr::eq(renderers.for_span(&previous.span), renderer)
                    && can_share_shaping_request(source_text, previous, &fragment)
            })
        {
            group.push(fragment);
        } else {
            groups.push(vec![fragment]);
        }
    }
    groups
}

fn can_share_shaping_request(
    _source_text: &str,
    previous: &PositionedSpan,
    next: &PositionedSpan,
) -> bool {
    previous.source_range.end == next.source_range.start
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
    use crate::raster_host::text::TextRenderer;
    use katana_ui_core::facade::UiCoreFacade;
    use katana_ui_core::render_model::{UiNode, UiNodeKind, UiTextSpanStyle};
    use katana_ui_core::text_raster::{PlatformTextFaceSelection, PlatformTextRasterConfig};
    use katana_ui_core::theme::ThemeSnapshot;

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

    #[test]
    fn split_grapheme_fragments_share_a_shaping_group() {
        let facade = UiCoreFacade::new(ThemeSnapshot::dark());
        let renderer = TextRenderer::load(&facade, "body");
        let fragments = vec![
            positioned_fragment(0, "e", 0..1),
            positioned_fragment(1, "\u{0301}", 1..3),
        ];

        let groups = split_shaping_groups(
            fragments,
            SpanTextRenderers::new(&renderer, &renderer),
            "e\u{0301}",
        );

        assert_eq!(1, groups.len());
        assert_eq!(2, groups[0].len());
    }

    #[test]
    fn zwj_fragments_share_a_shaping_group() {
        let facade = UiCoreFacade::new(ThemeSnapshot::dark());
        let renderer = TextRenderer::load(&facade, "body");
        let fragments = vec![
            positioned_fragment(0, "\u{1f469}", 0..4),
            positioned_fragment(1, "\u{200d}", 4..7),
            positioned_fragment(2, "\u{1f4bb}", 7..11),
        ];

        let groups = split_shaping_groups(
            fragments,
            SpanTextRenderers::new(&renderer, &renderer),
            "\u{1f469}\u{200d}\u{1f4bb}",
        );

        assert_eq!(1, groups.len());
        assert_eq!(3, groups[0].len());
    }

    #[test]
    fn combined_grapheme_fragments_keep_individual_visual_geometry() {
        let facade = UiCoreFacade::new(ThemeSnapshot::dark());
        let renderer = TextRenderer::load(&facade, "body");
        let code_renderer = TextRenderer::load(&facade, "code").with_raster_config(
            PlatformTextRasterConfig::default(),
            PlatformTextFaceSelection::CandidateChain,
        );
        let line = vec![
            UiTextSpan::plain("\u{05d0}"),
            UiTextSpan {
                text: "e".to_owned(),
                style: UiTextSpanStyle {
                    underline: true,
                    highlight: true,
                    ..UiTextSpanStyle::default()
                },
                link_target: String::new(),
            },
            UiTextSpan {
                text: "\u{0301}".to_owned(),
                style: UiTextSpanStyle::default(),
                link_target: "https://example.test/combining".to_owned(),
            },
            UiTextSpan {
                text: "x".to_owned(),
                style: UiTextSpanStyle {
                    inline_code: true,
                    ..UiTextSpanStyle::default()
                },
                link_target: String::new(),
            },
        ];
        let metrics = UiTreeTextMetrics::for_node(&UiNode::new(UiNodeKind::Text, "fixture"));

        let positioned = visual_span_positions(
            SpanTextRenderers::new(&renderer, &code_renderer),
            &line,
            metrics,
            0,
        )
        .expect("mixed bidi/code line must use visual span geometry");
        let base = positioned
            .iter()
            .find(|span| span.line_span_index == 1)
            .expect("base fragment geometry");
        let combining = positioned
            .iter()
            .find(|span| span.line_span_index == 2)
            .expect("combining fragment geometry");

        assert_eq!(base.shaping_run_index, combining.shaping_run_index);
        assert!(base.width > 0 && combining.width > 0);
        assert!(
            base.x.saturating_add(base.width as isize) <= combining.x
                || combining.x.saturating_add(combining.width as isize) <= base.x,
            "styled fragments inside one grapheme must retain distinct decoration and link ranges"
        );
    }

    #[test]
    fn neutral_inline_code_group_keeps_rtl_context_when_body_shares_its_visual_range() {
        let facade = UiCoreFacade::new(ThemeSnapshot::dark());
        let body_renderer = TextRenderer::load(&facade, "body");
        let code_renderer = TextRenderer::load(&facade, "code").with_raster_config(
            PlatformTextRasterConfig::default(),
            PlatformTextFaceSelection::CandidateChain,
        );
        let line = vec![
            UiTextSpan::plain("אבג "),
            UiTextSpan {
                text: "(".to_owned(),
                style: UiTextSpanStyle {
                    inline_code: true,
                    ..UiTextSpanStyle::default()
                },
                link_target: String::new(),
            },
        ];
        let metrics = UiTreeTextMetrics::for_node(&UiNode::new(UiNodeKind::Text, "fixture"));

        let positioned = visual_span_positions(
            SpanTextRenderers::new(&body_renderer, &code_renderer),
            &line,
            metrics,
            0,
        )
        .expect("mixed bidi/code line must use visual span geometry");
        let code = positioned
            .iter()
            .find(|span| span.line_span_index == 1)
            .expect("inline-code fragment geometry");

        assert_eq!(
            Some("\u{2067}".to_owned()),
            code.bidi_context
                .as_ref()
                .map(|context| context.prefix.clone())
        );
        assert_eq!(
            Some("\u{2069}".to_owned()),
            code.bidi_context
                .as_ref()
                .map(|context| context.suffix.clone())
        );
        assert!(
            code.resolved_rtl,
            "the full visual run must retain its RTL level"
        );
    }

    fn positioned_fragment(
        line_span_index: usize,
        text: &str,
        source_range: Range<usize>,
    ) -> PositionedSpan {
        PositionedSpan {
            line_span_index,
            shaping_run_index: 0,
            shaping_run_x: 0,
            span: UiTextSpan::plain(text),
            x: 0,
            width: 0,
            bidi_context: None,
            resolved_rtl: false,
            source_span_range: source_range.clone(),
            source_range,
        }
    }
}
