use super::super::canvas::Canvas;
use super::super::canvas_text_selection_visual_line::resolve_text_selection_line_id;
use super::super::text::TextRenderer;
use super::super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::super::ui_tree_canvas_text_metrics::UiTreeTextMetrics;
use super::super::ui_tree_canvas_text_role::UiTreeTextRoleRenderer;
use super::super::ui_tree_canvas_types::UiTreeRenderArea;
use katana_ui_core::render_model::{UiNode, UiTextSpan};

#[path = "ui_tree_canvas_text_lines_bidi_context.rs"]
mod bidi_context;
#[path = "ui_tree_canvas_text_lines_decoration.rs"]
mod line_decoration;
#[path = "ui_tree_canvas_text_lines_plain.rs"]
mod plain;
#[path = "ui_tree_canvas_text_lines_raster_run_draw.rs"]
mod raster_run_draw;
#[path = "ui_tree_canvas_text_lines_raster_runs.rs"]
mod raster_runs;
#[path = "ui_tree_canvas_text_lines_rich_span.rs"]
mod rich_span;
#[path = "ui_tree_canvas_text_lines_span_geometry.rs"]
mod span_geometry;
#[path = "ui_tree_canvas_text_span_style.rs"]
mod span_style;
#[path = "ui_tree_canvas_text_wrap.rs"]
pub(in crate::raster_host) mod text_wrap;
#[path = "ui_tree_canvas_text_wrap_state.rs"]
mod wrap_state;

use crate::raster_host::ui_tree_canvas_text_line_width::{
    SpanTextRenderers, preserves_whitespace, span_line_width, span_part_width,
    span_visible_part_bounds, span_visible_part_bounds_with_edges,
};
use line_decoration::decoration_y;
pub(super) use line_decoration::underline_y_offset;
use plain::draw_plain_at_logical_y as draw_plain_lines_at_logical_y;
use raster_run_draw::RasterRun;
pub(in crate::raster_host) use raster_runs::visual_span_positions;
use rich_span::rich_line_span;
use span_geometry::push_span_geometry;
use span_style::{draw_span_background, span_color};
use text_wrap::UiTreeTextWrap;

const UNDERLINE_HEIGHT_BOTTOM_PADDING: usize = 3;

pub(super) struct UiTreeTextLines;

#[derive(Clone, Copy)]
pub(super) struct UiTreeTextLineContext<'a> {
    pub(super) renderer: &'a TextRenderer,
    pub(super) code_renderer: &'a TextRenderer,
    pub(super) node: &'a UiNode,
    pub(super) area: UiTreeRenderArea,
    pub(super) palette: UiTreeCanvasPalette,
    pub(super) metrics: UiTreeTextMetrics,
}

impl UiTreeTextLines {
    #[cfg(test)]
    pub(super) fn draw_plain(
        canvas: &mut Canvas,
        context: UiTreeTextLineContext<'_>,
        origin_x: usize,
        x: usize,
        y: usize,
    ) {
        Self::draw_plain_at_logical_y(canvas, context, origin_x, x, y as f32);
    }

    pub(super) fn draw_plain_at_logical_y(
        canvas: &mut Canvas,
        context: UiTreeTextLineContext<'_>,
        origin_x: usize,
        x: usize,
        y: f32,
    ) {
        draw_plain_lines_at_logical_y(canvas, context, origin_x, x, y);
    }

    #[cfg(test)]
    pub(super) fn draw_spans(
        canvas: &mut Canvas,
        context: UiTreeTextLineContext<'_>,
        x: usize,
        y: usize,
    ) {
        Self::draw_spans_at_logical_y(canvas, context, x, y as f32);
    }

    pub(super) fn draw_spans_at_logical_y(
        canvas: &mut Canvas,
        context: UiTreeTextLineContext<'_>,
        x: usize,
        y: f32,
    ) {
        let lines = UiTreeTextWrap::span_lines_with_metadata(
            SpanTextRenderers::new(context.renderer, context.code_renderer),
            context.node,
            x,
            context.area,
            context.metrics,
        );
        let preserve_whitespace = preserves_whitespace(context.node);
        let renderers = SpanTextRenderers::new(context.renderer, context.code_renderer);
        let mut logical_line = None;
        for (line_index, line) in lines.iter().enumerate() {
            let logical_line_id =
                resolve_text_selection_line_id(&mut logical_line, line.separator_is_explicit);
            let Some(line_box_top) = visible_line_y(line_index, y, context.area, context.metrics)
            else {
                continue;
            };
            let line_bold = UiTreeTextRoleRenderer::line_bold(context.node, line_index);
            let bold_line_storage;
            let render_line = if line_bold {
                bold_line_storage = line
                    .content
                    .iter()
                    .cloned()
                    .map(force_span_bold)
                    .collect::<Vec<_>>();
                bold_line_storage.as_slice()
            } else {
                line.content.as_slice()
            };
            let line_width =
                span_line_width(renderers, render_line, context.metrics, preserve_whitespace);
            let line_x = UiTreeTextRoleRenderer::line_x(
                context.node,
                x,
                x,
                context.area,
                line_width,
                line_index,
            );
            let mut rich_line = Vec::with_capacity(render_line.len());
            for span in render_line {
                let color = span_color(span, context.palette);
                rich_line.push(rich_line_span(context, renderers, span, color));
            }
            let mut decorations = Vec::new();
            let mut span_backgrounds = Vec::with_capacity(render_line.len());
            if let Some(positioned_spans) =
                visual_span_positions(renderers, render_line, context.metrics, line_x)
            {
                for positioned in positioned_spans {
                    let visible_bounds = positioned.visible_part_bounds(
                        renderers,
                        context.metrics,
                        preserve_whitespace,
                    );
                    push_span_geometry(
                        &mut span_backgrounds,
                        &mut decorations,
                        &positioned.span,
                        positioned.x,
                        positioned.width,
                        Some(visible_bounds),
                        context,
                        renderers,
                        preserve_whitespace,
                    );
                }
            } else {
                let mut cursor_x = line_x;
                for span in render_line {
                    let width =
                        span_part_width(renderers, span, context.metrics, preserve_whitespace);
                    push_span_geometry(
                        &mut span_backgrounds,
                        &mut decorations,
                        span,
                        cursor_x,
                        width,
                        None,
                        context,
                        renderers,
                        preserve_whitespace,
                    );
                    cursor_x += width as isize;
                }
            }
            let runs = RasterRun::for_line(context, render_line, &rich_line, line_x);
            let raster_baseline = context.metrics.baseline_from_line_box_top.map(|_| {
                /* WHY: code が先頭でも本文と装飾の基準線を face の選択で動かさない。 */
                context.renderer.rich_line_raster_baseline(
                    &rich_line,
                    context.metrics.line_box_height,
                    canvas.scale_factor(),
                )
            });
            for (background_x, width, span_style) in &span_backgrounds {
                draw_span_background(
                    canvas,
                    *background_x,
                    line_box_top,
                    *width,
                    *span_style,
                    context.palette,
                    context.metrics,
                    raster_baseline.unwrap_or_default(),
                );
            }

            canvas.with_text_selection_line_id(
                logical_line_id,
                &line.separator_before,
                line.separator_is_explicit,
                |canvas| {
                    if render_line.iter().all(|span| span.text.is_empty())
                        && context
                            .node
                            .props()
                            .text
                            .spans
                            .iter()
                            .any(|span| !span.text.is_empty())
                    {
                        canvas.record_empty_text_selection_line(
                            line_x.max(0) as usize,
                            line_box_top.max(0.0).floor() as usize,
                            context.metrics.line_box_height.ceil() as usize,
                        );
                    }
                    for run in runs {
                        run.draw(canvas, context.metrics, line_box_top);
                    }
                },
            );

            for decoration in decorations {
                let y = decoration_y(
                    line_box_top,
                    context.metrics.baseline_from_line_box_top,
                    raster_baseline.unwrap_or_default(),
                    decoration.legacy_offset,
                );
                decoration.draw(canvas, y);
            }
        }
    }

    pub(super) fn line_count(
        renderer: &TextRenderer,
        code_renderer: &TextRenderer,
        node: &UiNode,
        x: usize,
        area: UiTreeRenderArea,
        metrics: UiTreeTextMetrics,
    ) -> usize {
        if node.props().text.spans.is_empty() {
            return UiTreeTextWrap::plain_lines(renderer, node, x, area, metrics)
                .len()
                .max(1);
        }
        UiTreeTextWrap::span_lines(
            SpanTextRenderers::new(renderer, code_renderer),
            node,
            x,
            area,
            metrics,
        )
        .len()
        .max(1)
    }
}

fn force_span_bold(mut span: UiTextSpan) -> UiTextSpan {
    span.style.bold = true;
    span
}

fn canvas_x(x: isize) -> Option<usize> {
    usize::try_from(x).ok()
}

pub(in crate::raster_host) fn visible_line_y(
    line_index: usize,
    y: f32,
    area: UiTreeRenderArea,
    metrics: UiTreeTextMetrics,
) -> Option<f32> {
    let line_top = line_index as f32 * metrics.line_box_height;
    let line_bottom = line_top + metrics.line_box_height;
    let scroll_y = area.scroll_y.max(0.0);
    let viewport_bottom = scroll_y + area.height as f32;
    if line_bottom <= scroll_y {
        return None;
    }
    if line_top >= viewport_bottom {
        return None;
    }
    Some(y + (line_top - scroll_y).max(0.0))
}

#[cfg(test)]
#[path = "ui_tree_canvas_text_lines_tests.rs"]
mod tests;

#[cfg(test)]
mod legacy_fractional_origin_tests {
    use super::{UiTreeTextLineContext, UiTreeTextLines};
    use crate::raster_host::canvas::Canvas;
    use crate::raster_host::text::TextRenderer;
    use crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette;
    use crate::raster_host::ui_tree_canvas_text_metrics::UiTreeTextMetrics;
    use crate::raster_host::ui_tree_canvas_types::UiTreeRenderArea;
    use katana_ui_core::atom::Text;
    use katana_ui_core::facade::UiCoreFacade;
    use katana_ui_core::render_model::{UiNode, UiTextSpan, UiTextSpanStyle};
    use katana_ui_core::theme::ThemeSnapshot;

    const WIDTH: usize = 160;
    const HEIGHT: usize = 120;
    const TEST_BACKGROUND: u32 = 0x151515;
    const CURRENT_HIGHLIGHT_BACKGROUND: u32 = 0x654100;

    #[test]
    fn legacy_rich_text_keeps_fractional_origin_until_scaled_draw() {
        let facade = UiCoreFacade::new(ThemeSnapshot::dark());
        let renderer = TextRenderer::load(&facade, "body");
        let node: UiNode = Text::new("Current")
            .text_role("body")
            .text_spans(vec![UiTextSpan {
                text: "Current".into(),
                style: UiTextSpanStyle {
                    current_highlight: true,
                    color_rgba: [255, 0, 0, 255],
                    ..UiTextSpanStyle::default()
                },
                link_target: String::new(),
            }])
            .into();
        let metrics = UiTreeTextMetrics::for_node(&node);
        assert!(metrics.baseline_from_line_box_top.is_none());
        let palette = UiTreeCanvasPalette::from_theme(&ThemeSnapshot::dark());
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: WIDTH,
            height: HEIGHT,
            scroll_y: 0.0,
        };

        let draw = |y: f32| {
            let mut canvas = Canvas::new_scaled(WIDTH, HEIGHT, 2.0, TEST_BACKGROUND);
            UiTreeTextLines::draw_spans_at_logical_y(
                &mut canvas,
                UiTreeTextLineContext {
                    renderer: &renderer,
                    code_renderer: &renderer,
                    node: &node,
                    area,
                    palette,
                    metrics,
                },
                0,
                y,
            );
            canvas
        };

        let integer = draw(31.0);
        let fractional = draw(31.5);
        let first_glyph_row = |canvas: &Canvas| {
            (0..canvas.height())
                .find(|row| {
                    (0..canvas.width()).any(|column| {
                        let pixel = canvas.pixels()[row * canvas.width() + column];
                        pixel != TEST_BACKGROUND && pixel != CURRENT_HIGHLIGHT_BACKGROUND
                    })
                })
                .expect("rich text glyphs should be visible")
        };

        assert_eq!(first_glyph_row(&integer) + 1, first_glyph_row(&fractional));
    }
}
