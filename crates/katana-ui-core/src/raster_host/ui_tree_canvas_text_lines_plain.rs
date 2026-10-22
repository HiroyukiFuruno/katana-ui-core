use super::super::super::canvas::Canvas;
use super::super::super::text::RichTextStyle;
use super::super::super::ui_tree_canvas_text_role::UiTreeTextRoleRenderer;
use super::{UiTreeTextLineContext, UiTreeTextWrap, visible_line_y};

pub(super) fn draw_plain_at_logical_y(
    canvas: &mut Canvas,
    context: UiTreeTextLineContext<'_>,
    origin_x: usize,
    x: usize,
    y: f32,
) {
    for (index, line) in UiTreeTextWrap::plain_lines_with_metadata(
        context.renderer,
        context.node,
        x,
        context.area,
        context.metrics,
    )
    .iter()
    .enumerate()
    {
        let Some(line_box_top) = visible_line_y(index, y, context.area, context.metrics) else {
            continue;
        };
        let color = UiTreeTextRoleRenderer::line_color(context.node, context.palette, index);
        let bold = UiTreeTextRoleRenderer::line_bold(context.node, index);
        let style = RichTextStyle::new(context.metrics.font_size, color)
            .bold(bold)
            .raster_vertical_scale(context.metrics.raster_vertical_scale);
        let width = if bold {
            context.renderer.measure_width_rich(&line.content, style)
        } else {
            context
                .renderer
                .measure_width(&line.content, context.metrics.font_size)
        }
        .max(1);
        let line_x =
            UiTreeTextRoleRenderer::line_x(context.node, origin_x, x, context.area, width, index);
        canvas.with_text_selection_line_separator(
            &line.separator_before,
            line.separator_is_explicit,
            |canvas| {
                if line.content.is_empty() && !context.node.props().label.is_empty() {
                    canvas.record_empty_text_selection_line(
                        line_x.max(0) as usize,
                        line_box_top.max(0.0).floor() as usize,
                        context.metrics.line_box_height.ceil() as usize,
                    );
                }
                if let Some(baseline) = context.metrics.baseline_from_line_box_top {
                    context.renderer.draw_signed_styled_in_line_box(
                        canvas,
                        &line.content,
                        line_x,
                        line_box_top,
                        context.metrics.line_box_height,
                        baseline,
                        style,
                    );
                } else {
                    let integer_line_top = line_box_top.max(0.0).floor() as usize;
                    canvas.with_fractional_y_origin(line_box_top, integer_line_top, |canvas| {
                        context.renderer.draw_signed_styled(
                            canvas,
                            &line.content,
                            line_x,
                            integer_line_top,
                            style,
                        );
                    });
                }
            },
        );
    }
}
