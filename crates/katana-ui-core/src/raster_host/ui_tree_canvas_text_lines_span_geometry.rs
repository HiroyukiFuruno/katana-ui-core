use super::{
    UiTreeTextLineContext, canvas_x,
    line_decoration::{TextDecorationLine, underline_part_bounds},
    span_style::{should_strikethrough, should_underline, span_color},
    underline_y_offset,
};
use crate::raster_host::ui_tree_canvas_text_line_width::{
    SpanTextRenderers, span_visible_part_bounds,
};
use katana_ui_core::render_model::{UiTextSpan, UiTextSpanStyle};

const STRIKETHROUGH_LINE_THICKNESS: usize = 1;
const UNDERLINE_LINE_THICKNESS: usize = 1;

pub(super) fn push_span_geometry(
    span_backgrounds: &mut Vec<(usize, usize, UiTextSpanStyle)>,
    decorations: &mut Vec<TextDecorationLine>,
    span: &UiTextSpan,
    span_x: isize,
    width: usize,
    context: UiTreeTextLineContext<'_>,
    renderers: SpanTextRenderers<'_>,
    preserve_whitespace: bool,
) {
    if let Some(background_x) = canvas_x(span_x) {
        span_backgrounds.push((background_x, width, span.style));
    }
    let color = span_color(span, context.palette);
    if should_underline(span) {
        let (decoration_x, decoration_width) = underline_part_bounds(
            renderers,
            span,
            context.metrics,
            preserve_whitespace,
            context.node,
            width,
        );
        decorations.push(TextDecorationLine {
            x: span_x.saturating_add(decoration_x as isize),
            legacy_offset: underline_y_offset(context.metrics, context.node),
            width: decoration_width,
            color,
            thickness: underline_line_thickness(),
        });
    }
    if should_strikethrough(span) {
        let (decoration_x, decoration_width) =
            span_visible_part_bounds(renderers, span, context.metrics, preserve_whitespace);
        decorations.push(TextDecorationLine {
            x: span_x.saturating_add(decoration_x as isize),
            legacy_offset: context.metrics.strikethrough_offset,
            width: decoration_width,
            color,
            thickness: STRIKETHROUGH_LINE_THICKNESS,
        });
    }
}

const fn underline_line_thickness() -> usize {
    UNDERLINE_LINE_THICKNESS
}
