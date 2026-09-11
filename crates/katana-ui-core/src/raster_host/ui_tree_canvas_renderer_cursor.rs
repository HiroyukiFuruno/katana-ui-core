use super::{
    Canvas, INDENT, NODE_GAP, UiNode, UiNodeKind, UiTreeCanvasPalette, UiTreeCanvasRenderer,
    UiTreeRenderArea, UiTreeTextMetrics, is_outside_vertical_viewport,
};
use crate::raster_host::text::RichTextStyle;
use crate::raster_host::ui_tree_canvas_text::UiTreeTextRenderer;

impl UiTreeCanvasRenderer {
    pub(super) fn draw_container_child(
        &self,
        canvas: &mut Canvas,
        child: &UiNode,
        child_x: usize,
        y: &mut usize,
        logical_y: &mut f32,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) {
        self.render_node_with_logical_cursor(canvas, child, child_x, logical_y, area, palette);
        *y = logical_canvas_boundary(*logical_y);
    }

    pub(super) fn draw_accordion(
        &self,
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) {
        let mut logical_y = *y as f32;
        self.draw_accordion_with_logical_cursor(canvas, node, x, &mut logical_y, area, palette);
        *y = logical_canvas_boundary(logical_y);
    }

    fn render_node_with_logical_cursor(
        &self,
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        logical_y: &mut f32,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) {
        match node.kind() {
            UiNodeKind::Text => UiTreeTextRenderer::draw_node_with_logical_cursor(
                canvas,
                self.text_context(palette),
                node,
                x,
                logical_y,
                area,
            ),
            UiNodeKind::Accordion => {
                self.draw_accordion_with_logical_cursor(canvas, node, x, logical_y, area, palette);
            }
            _ => self
                .draw_integer_node_with_logical_cursor(canvas, node, x, logical_y, area, palette),
        }
    }

    fn draw_integer_node_with_logical_cursor(
        &self,
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        logical_y: &mut f32,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) {
        let physical_start = logical_canvas_boundary(*logical_y);
        let height = self.measured_scroll_node_height(node, self.text_context(palette), x, area);
        if is_outside_vertical_viewport(physical_start, height, area) {
            *logical_y += height as f32;
            return;
        }
        let mut physical_y = physical_start;
        canvas.with_fractional_y_origin(*logical_y, physical_start, |canvas| {
            self.render_node(canvas, node, x, &mut physical_y, area, palette);
        });
        *logical_y += physical_y.saturating_sub(physical_start) as f32;
    }

    fn draw_accordion_with_logical_cursor(
        &self,
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        logical_y: &mut f32,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) {
        let document_accordion = node.props().text.role == "html-accordion";
        let metrics = UiTreeTextMetrics::for_node_with_typography(node, self.typography);
        let physical_y = logical_canvas_boundary(*logical_y);
        let logical_label_y = accordion_label_origin(*logical_y, metrics.top_margin);
        let label = if document_accordion {
            node.props().label.clone()
        } else if node.props().interaction.open {
            format!("v {}", node.props().label)
        } else {
            format!("> {}", node.props().label)
        };
        if let Some(baseline_from_line_box_top) = metrics.baseline_from_line_box_top {
            self.text.draw_signed_styled_in_line_box(
                canvas,
                &label,
                x as isize,
                logical_label_y,
                metrics.line_box_height,
                baseline_from_line_box_top,
                RichTextStyle::new(metrics.font_size, palette.text),
            );
        } else {
            self.text.draw(
                canvas,
                &label,
                x,
                physical_y.saturating_add(metrics.top_margin),
                metrics.font_size,
                palette.text,
            );
        }
        *logical_y += accordion_header_height(metrics);
        if node.props().interaction.open {
            let child_x = if document_accordion {
                x
            } else {
                x.saturating_add(INDENT)
            };
            for child in node.children() {
                self.render_node_with_logical_cursor(
                    canvas, child, child_x, logical_y, area, palette,
                );
            }
            if !document_accordion {
                *logical_y += NODE_GAP as f32;
            }
        }
    }
}

fn accordion_label_origin(logical_y: f32, top_margin: usize) -> f32 {
    logical_y + top_margin as f32
}

fn accordion_header_height(metrics: UiTreeTextMetrics) -> f32 {
    metrics
        .baseline_from_line_box_top
        .map_or(metrics.line_height as f32, |_| metrics.line_box_height)
}

fn logical_canvas_boundary(value: f32) -> usize {
    value.floor().max(0.0) as usize
}

#[cfg(test)]
mod tests {
    use super::{
        Canvas, UiNode, UiNodeKind, UiTreeCanvasPalette, UiTreeCanvasRenderer, UiTreeRenderArea,
    };
    use crate::raster_host::{UiTreeDocumentTypography, UiTreeTextRoleBaselineTypography};
    use crate::render_model::UiTextProps;
    use crate::theme::ThemeSnapshot;

    #[test]
    fn document_accordion_labels_keep_fractional_origins_until_scaled_paint() {
        let theme = ThemeSnapshot::dark();
        let palette = UiTreeCanvasPalette::from_theme(&theme);
        let renderer = UiTreeCanvasRenderer::with_document_typography(
            theme,
            UiTreeDocumentTypography::new()
                .with_body_baseline(UiTreeTextRoleBaselineTypography::new(16.0, 31.5, 18.5)),
        );
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 240,
            height: 120,
            scroll_y: 0.0,
        };

        for role in ["html-accordion", "html-accordion-preview"] {
            let node = UiNode::new(UiNodeKind::Accordion, "Document").text(UiTextProps {
                role: role.to_owned(),
                ..UiTextProps::default()
            });
            let mut integral_canvas = Canvas::new_scaled(240, 120, 2.0, palette.background);
            let mut integral_y = 31.0;
            renderer.draw_accordion_with_logical_cursor(
                &mut integral_canvas,
                &node,
                0,
                &mut integral_y,
                area,
                palette,
            );
            let integral_top = first_non_background_row(&integral_canvas, palette.background);

            let mut fractional_canvas = Canvas::new_scaled(240, 120, 2.0, palette.background);
            let mut fractional_y = 31.5;
            renderer.draw_accordion_with_logical_cursor(
                &mut fractional_canvas,
                &node,
                0,
                &mut fractional_y,
                area,
                palette,
            );
            let fractional_top = first_non_background_row(&fractional_canvas, palette.background);

            assert_eq!(
                integral_top + 1,
                fractional_top,
                "{role} label must retain its half-pixel logical origin at scale 2"
            );
        }
    }

    fn first_non_background_row(canvas: &Canvas, background: u32) -> usize {
        canvas
            .pixels()
            .iter()
            .position(|pixel| *pixel != background)
            .map(|index| index / canvas.width())
            .expect("accordion label is drawn")
    }
}
