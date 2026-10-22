#[cfg(test)]
use super::RichTextStyle;
use super::text_runtime::{RichTextDrawRequest, default_line_height};
use super::{Canvas, RichTextBidiContext, RichTextLineSpan, TextRenderer};

impl TextRenderer {
    pub(in crate::raster_host) fn measure_rich_span_layout_with_bidi_context(
        &self,
        source_spans: &[RichTextLineSpan],
        bidi_context: Option<&RichTextBidiContext>,
        resolved_rtl: bool,
    ) -> (Vec<(usize, usize)>, usize) {
        let source_text = source_spans
            .iter()
            .map(|span| span.text.as_str())
            .collect::<String>();
        let spans = Self::rich_raster_spans(source_spans, bidi_context);
        let font = self.font_with_size(
            source_spans
                .first()
                .map(|span| span.style.size)
                .unwrap_or(self.font.size),
        );
        let line_height_px = source_spans
            .iter()
            .map(|span| default_line_height(span.style.size))
            .fold(default_line_height(font.size), f32::max);
        let Some(raster) = self.rasterize(spans, font, 1.0, line_height_px) else {
            return fallback_rich_span_layout(self, source_spans);
        };
        let prefix_len = bidi_context.map_or(0, |context| context.prefix.len());
        let layout = source_span_layout(
            source_spans,
            &raster.grapheme_bounds,
            prefix_len,
            resolved_rtl,
        );
        let width =
            super::text_runtime_paint::text_run_selection_width(&source_text, &raster, prefix_len);
        (layout, width)
    }

    #[cfg(test)]
    pub(crate) fn draw_rich_line_signed(
        &self,
        canvas: &mut Canvas,
        spans: &[RichTextLineSpan],
        x: isize,
        y: usize,
    ) {
        self.draw_rich_line_signed_with_optional_bidi_context(canvas, spans, None, x, y, None);
    }

    pub(crate) fn draw_rich_line_signed_with_resolved_direction(
        &self,
        canvas: &mut Canvas,
        spans: &[RichTextLineSpan],
        x: isize,
        y: usize,
        resolved_rtl: Option<bool>,
    ) {
        self.draw_rich_line_signed_with_optional_bidi_context(
            canvas,
            spans,
            None,
            x,
            y,
            resolved_rtl,
        );
    }

    pub(crate) fn draw_rich_line_signed_with_bidi_context_and_resolved_direction(
        &self,
        canvas: &mut Canvas,
        spans: &[RichTextLineSpan],
        bidi_context: &RichTextBidiContext,
        x: isize,
        y: usize,
        resolved_rtl: Option<bool>,
    ) {
        self.draw_rich_line_signed_with_optional_bidi_context(
            canvas,
            spans,
            Some(bidi_context),
            x,
            y,
            resolved_rtl,
        );
    }

    fn draw_rich_line_signed_with_optional_bidi_context(
        &self,
        canvas: &mut Canvas,
        spans: &[RichTextLineSpan],
        bidi_context: Option<&RichTextBidiContext>,
        x: isize,
        y: usize,
        resolved_rtl: Option<bool>,
    ) {
        let raster_vertical_scale = spans
            .iter()
            .map(|span| span.style.raster_vertical_scale)
            .fold(1.0_f32, f32::max);
        let font = self.font_with_size(
            spans
                .first()
                .map(|span| span.style.size)
                .unwrap_or(self.font.size),
        );
        let line_height_px = spans
            .iter()
            .map(|span| default_line_height(span.style.size))
            .fold(default_line_height(font.size), f32::max);
        self.draw_rich_request(
            canvas,
            spans,
            bidi_context,
            RichTextDrawRequest {
                x,
                paint_origin_y: y as f32,
                text_run_origin_y: y as f32,
                scale_factor: canvas.scale_factor(),
                raster_vertical_scale,
                font,
                line_height_px,
                resolved_rtl,
            },
        );
    }

    #[cfg(test)]
    pub(crate) fn measure_width_rich_with_bidi_context(
        &self,
        text: &str,
        style: RichTextStyle,
        bidi_context: &RichTextBidiContext,
    ) -> usize {
        let span = self.rich_line_span(text, style);
        let spans = Self::rich_raster_spans(&[span], Some(bidi_context));
        self.rasterize(
            spans,
            self.font_with_size(style.size),
            1.0,
            default_line_height(style.size),
        )
        .map_or_else(
            || self.measure_width_rich(text, style),
            |raster| {
                super::text_runtime_paint::text_run_selection_width(
                    text,
                    &raster,
                    bidi_context.prefix.len(),
                )
            },
        )
    }
}

fn source_span_layout(
    source_spans: &[RichTextLineSpan],
    bounds: &[katana_ui_core::text_raster::PlatformTextGraphemeBounds],
    prefix_len: usize,
    resolved_rtl: bool,
) -> Vec<(usize, usize)> {
    let ranges = source_span_ranges(source_spans);
    let mut geometry = vec![None; source_spans.len()];
    for bound in bounds {
        let Some(source_start) = bound.byte_start.checked_sub(prefix_len) else {
            continue;
        };
        let Some(source_end) = bound.byte_end.checked_sub(prefix_len) else {
            continue;
        };
        let overlaps = ranges
            .iter()
            .enumerate()
            .filter_map(|(index, range)| {
                let start = range.start.max(source_start);
                let end = range.end.min(source_end);
                (start < end).then(|| (index, end - start))
            })
            .collect::<Vec<_>>();
        assign_bound_geometry(&mut geometry, &overlaps, bound, resolved_rtl);
    }
    let mut layout = geometry
        .into_iter()
        .map(|range: Option<(usize, usize)>| {
            range.map_or((0, 0), |(start, end)| (start, end.saturating_sub(start)))
        })
        .collect::<Vec<_>>();
    remove_visual_overlaps(&mut layout);
    layout
}

fn remove_visual_overlaps(layout: &mut [(usize, usize)]) {
    let mut visual_order = layout
        .iter()
        .enumerate()
        .filter(|(_, (_, width))| *width > 0)
        .map(|(index, (start, _))| (index, *start))
        .collect::<Vec<_>>();
    visual_order.sort_by_key(|(_, start)| *start);
    let mut right = 0usize;
    for (index, _) in visual_order {
        let (start, width) = layout[index];
        let end = start.saturating_add(width);
        let clipped_start = start.max(right);
        layout[index] = (clipped_start, end.saturating_sub(clipped_start));
        right = right.max(end);
    }
}

fn source_span_ranges(source_spans: &[RichTextLineSpan]) -> Vec<std::ops::Range<usize>> {
    let mut cursor = 0usize;
    source_spans
        .iter()
        .map(|span| {
            let start = cursor;
            cursor = cursor.saturating_add(span.text.len());
            start..cursor
        })
        .collect()
}

fn assign_bound_geometry(
    geometry: &mut [Option<(usize, usize)>],
    overlaps: &[(usize, usize)],
    bound: &katana_ui_core::text_raster::PlatformTextGraphemeBounds,
    resolved_rtl: bool,
) {
    let start = super::text_runtime_paint::logical_origin(bound.x);
    let width = super::text_runtime_paint::logical_bound_extent(bound.width);
    let total_source_width = overlaps.iter().map(|(_, width)| *width).sum::<usize>();
    if total_source_width == 0 {
        return;
    }
    let mut consumed_source_width = 0usize;
    for (index, source_width) in overlaps {
        let before = width.saturating_mul(consumed_source_width) / total_source_width;
        consumed_source_width = consumed_source_width.saturating_add(*source_width);
        let after = width.saturating_mul(consumed_source_width) / total_source_width;
        let (part_start, part_end) = if resolved_rtl {
            (
                start.saturating_add(width.saturating_sub(after)),
                start.saturating_add(width.saturating_sub(before)),
            )
        } else {
            (start.saturating_add(before), start.saturating_add(after))
        };
        geometry[*index] = Some(match geometry[*index] {
            Some((current_start, current_end)) => {
                (current_start.min(part_start), current_end.max(part_end))
            }
            None => (part_start, part_end),
        });
    }
}

fn fallback_rich_span_layout(
    renderer: &TextRenderer,
    source_spans: &[RichTextLineSpan],
) -> (Vec<(usize, usize)>, usize) {
    let mut cursor = 0usize;
    let layout = source_spans
        .iter()
        .map(|span| {
            let width = renderer.measure_width_rich(&span.text, span.style).max(1);
            let position = (cursor, width);
            cursor = cursor.saturating_add(width);
            position
        })
        .collect();
    (layout, cursor.max(1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::facade::UiCoreFacade;
    use katana_ui_core::text_raster::{
        PlatformTextFaceSelection, PlatformTextGraphemeBounds, PlatformTextRasterConfig,
    };
    use katana_ui_core::theme::ThemeSnapshot;
    use std::path::PathBuf;

    fn renderer_without_color_emoji() -> TextRenderer {
        let config = PlatformTextRasterConfig {
            emoji_candidates: vec![PathBuf::from("/kuc-test-font-catalog/missing-emoji.ttf")],
            emoji_candidate_sha256: Vec::new(),
            ..PlatformTextRasterConfig::default()
        };
        let facade = UiCoreFacade::new(ThemeSnapshot::dark());
        TextRenderer::load_with_text_raster_config(
            &facade,
            "body",
            config,
            PlatformTextFaceSelection::System,
        )
    }

    #[test]
    fn bidi_measurement_falls_back_when_color_emoji_rasterization_is_unavailable() {
        let renderer = renderer_without_color_emoji();
        let style = RichTextStyle::new(14.0, 0xffffff).emoji(true);
        let context = RichTextBidiContext::default();

        assert_eq!(
            renderer.measure_width_rich("🔥", style),
            renderer.measure_width_rich_with_bidi_context("🔥", style, &context),
        );
    }

    #[test]
    fn combined_grapheme_sidecar_partitions_selection_geometry() {
        let style = RichTextStyle::new(14.0, 0);
        let spans = vec![
            RichTextLineSpan {
                text: "e".to_owned(),
                style,
            },
            RichTextLineSpan {
                text: "\u{0301}".to_owned(),
                style,
            },
        ];
        let bounds = vec![grapheme_bounds(0, 3, 10.0, 10.0)];

        let layout = source_span_layout(&spans, &bounds, 0, false);

        assert_eq!(vec![(10, 3), (13, 7)], layout);
        assert_eq!(20, layout[1].0 + layout[1].1);
    }

    #[test]
    fn zwj_sidecar_partitions_rtl_selection_geometry() {
        let style = RichTextStyle::new(14.0, 0);
        let spans = vec![
            RichTextLineSpan {
                text: "\u{1f469}".to_owned(),
                style,
            },
            RichTextLineSpan {
                text: "\u{200d}".to_owned(),
                style,
            },
            RichTextLineSpan {
                text: "\u{1f4bb}".to_owned(),
                style,
            },
        ];
        let bounds = vec![grapheme_bounds(0, 11, 40.0, 11.0)];

        let layout = source_span_layout(&spans, &bounds, 0, true);

        assert_eq!(vec![(47, 4), (44, 3), (40, 4)], layout);
    }

    #[test]
    fn one_pixel_combined_grapheme_assigns_only_one_ltr_fragment() {
        let layout = source_span_layout(
            &combined_grapheme_spans(),
            &[grapheme_bounds(0, 3, 10.0, 1.0)],
            0,
            false,
        );

        assert_eq!(vec![(10, 0), (10, 1)], layout);
        assert_eq!(1, layout.iter().filter(|(_, width)| *width > 0).count());
    }

    #[test]
    fn one_pixel_combined_grapheme_assigns_only_one_rtl_fragment() {
        let layout = source_span_layout(
            &combined_grapheme_spans(),
            &[grapheme_bounds(0, 3, 10.0, 1.0)],
            0,
            true,
        );

        assert_eq!(vec![(11, 0), (10, 1)], layout);
        assert_eq!(1, layout.iter().filter(|(_, width)| *width > 0).count());
    }

    fn combined_grapheme_spans() -> Vec<RichTextLineSpan> {
        let style = RichTextStyle::new(14.0, 0);
        vec![
            RichTextLineSpan {
                text: "e".to_owned(),
                style,
            },
            RichTextLineSpan {
                text: "\u{0301}".to_owned(),
                style,
            },
        ]
    }

    fn grapheme_bounds(
        byte_start: usize,
        byte_end: usize,
        x: f32,
        width: f32,
    ) -> PlatformTextGraphemeBounds {
        PlatformTextGraphemeBounds {
            byte_start,
            byte_end,
            x,
            y: 0.0,
            width,
            height: 1.0,
        }
    }
}
