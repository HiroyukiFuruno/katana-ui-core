use super::{
    Canvas, ContainerPadding, INDENT, NODE_GAP, TEXT_HEIGHT, UiNode, UiNodeKind,
    UiTreeCanvasPalette, UiTreeCanvasRenderer, UiTreeRenderArea, UiTreeTextMetrics, UiVisualRole,
    child_container_x, child_render_area, dimension_px, draw_hover_background, draw_hover_surface,
    gap_after_child, has_absolute_child, is_outside_vertical_viewport, remaining_width,
    should_draw_container_label,
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

    pub(super) fn render_node_with_logical_cursor(
        &self,
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        logical_y: &mut f32,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) {
        let physical_y = logical_canvas_boundary(*logical_y);
        canvas.with_fractional_y_origin(*logical_y, physical_y, |canvas| {
            draw_hover_background(canvas, node, x, physical_y, area, palette);
        });
        match node.kind() {
            UiNodeKind::Text => {
                self.draw_text_with_logical_cursor(canvas, node, x, logical_y, area, palette)
            }
            UiNodeKind::Accordion => {
                self.draw_sized_accordion_with_logical_cursor(
                    canvas, node, x, logical_y, area, palette,
                );
            }
            UiNodeKind::Row => {
                self.draw_row_with_logical_cursor(canvas, node, x, logical_y, area, palette);
            }
            UiNodeKind::Stack if has_absolute_child(node) => {
                self.draw_overlay_stack_with_logical_cursor(
                    canvas, node, x, logical_y, area, palette,
                );
            }
            UiNodeKind::AlignCenter
            | UiNodeKind::AlignNode
            | UiNodeKind::Card
            | UiNodeKind::Column
            | UiNodeKind::List
            | UiNodeKind::Stack => {
                self.draw_container_with_logical_cursor(canvas, node, x, logical_y, area, palette);
            }
            _ => self
                .draw_integer_node_with_logical_cursor(canvas, node, x, logical_y, area, palette),
        }
    }

    fn draw_overlay_stack_with_logical_cursor(
        &self,
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        logical_y: &mut f32,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) {
        let logical_start = *logical_y;
        let physical_start = logical_canvas_boundary(logical_start);
        let mut physical_y = physical_start;
        canvas.with_fractional_y_origin(logical_start, physical_start, |canvas| {
            self.draw_overlay_stack(canvas, node, x, &mut physical_y, area, palette);
        });
        *logical_y += physical_y.saturating_sub(physical_start) as f32;
    }

    fn draw_text_with_logical_cursor(
        &self,
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        logical_y: &mut f32,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) {
        let requested_height = dimension_px(&node.props().common.height);
        let logical_start = *logical_y;
        let mut draw = |canvas: &mut Canvas| {
            UiTreeTextRenderer::draw_node_with_logical_cursor(
                canvas,
                self.text_context(palette),
                node,
                x,
                logical_y,
                area,
            );
        };
        if requested_height > 0 {
            canvas.with_clip_at_logical_y(
                x,
                logical_start,
                remaining_width(area, x),
                requested_height as f32,
                &mut draw,
            );
        } else {
            draw(canvas);
        }
    }

    fn draw_container_with_logical_cursor(
        &self,
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        logical_y: &mut f32,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) {
        let container_origin = *logical_y;
        let hover_surface_y = logical_canvas_boundary(container_origin);
        let requested_height = dimension_px(&node.props().common.height);
        let hover_surface_height = if requested_height > 0 {
            requested_height
        } else if node.props().visual_role != UiVisualRole::HoverSurface {
            TEXT_HEIGHT
        } else {
            self.measured_scroll_node_height(node, self.text_context(palette), x, area)
        };
        if should_draw_container_label(node) {
            let mut label_y = hover_surface_y;
            canvas.with_fractional_y_origin(container_origin, hover_surface_y, |canvas| {
                super::draw_label(canvas, &self.text, node, x, &mut label_y, palette);
            });
            *logical_y += label_y.saturating_sub(hover_surface_y) as f32;
        }
        let padding = ContainerPadding::from_node(node);
        let child_x = child_container_x(node, x).saturating_add(padding.left);
        let child_area = child_render_area(area, node, child_x, padding);
        *logical_y += padding.top as f32;
        let child_clip_y = *logical_y;
        let mut draw_children = |canvas: &mut Canvas| {
            for (index, child) in node.children().iter().enumerate() {
                self.render_node_with_logical_cursor(
                    canvas, child, child_x, logical_y, child_area, palette,
                );
                if index + 1 < node.children().len() {
                    *logical_y += gap_after_child(node, child, &node.children()[index + 1]) as f32;
                }
            }
        };
        if requested_height > 0 {
            let clip_height = super::hover_surface_child_clip_height(node, requested_height);
            canvas.with_clip_at_logical_y(
                x,
                child_clip_y,
                remaining_width(area, x),
                clip_height as f32,
                &mut draw_children,
            );
            *logical_y = container_origin + requested_height as f32;
        } else {
            draw_children(canvas);
            *logical_y += padding.bottom as f32;
        }
        canvas.with_fractional_y_origin(container_origin, hover_surface_y, |canvas| {
            draw_hover_surface(
                canvas,
                node,
                x,
                hover_surface_y,
                area,
                palette,
                hover_surface_height,
            );
        });
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

    pub(super) fn draw_accordion_with_logical_cursor(
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
            let integer_label_y = physical_y.saturating_add(metrics.top_margin);
            canvas.with_fractional_y_origin(logical_label_y, integer_label_y, |canvas| {
                self.text.draw(
                    canvas,
                    &label,
                    x,
                    integer_label_y,
                    metrics.font_size,
                    palette.text,
                );
            });
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
    use crate::render_model::{
        UiCommonProps, UiDimension, UiEdgeInsets, UiTextProps, UiVisualRole,
    };
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

    #[test]
    fn ordinary_accordion_labels_keep_fractional_origins_until_scaled_paint() {
        let theme = ThemeSnapshot::dark();
        let palette = UiTreeCanvasPalette::from_theme(&theme);
        let renderer = UiTreeCanvasRenderer::new(theme);
        let node = UiNode::new(UiNodeKind::Accordion, "Storybook");
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 240,
            height: 120,
            scroll_y: 0.0,
        };

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

        assert_eq!(integral_top + 1, fractional_top);
    }

    #[test]
    fn container_chrome_keeps_fractional_origin_until_scaled_paint() {
        let theme = ThemeSnapshot::dark();
        let palette = UiTreeCanvasPalette::from_theme(&theme);
        let renderer = UiTreeCanvasRenderer::new(theme);
        let node = UiNode::new(UiNodeKind::Card, "Card")
            .height(UiDimension::px(10))
            .visual_role(UiVisualRole::HoverSurface);
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 180,
            height: 80,
            scroll_y: 0.0,
        };
        let mut integral_canvas = Canvas::new_scaled(180, 80, 2.0, palette.background);
        let mut integral_y = 31.0;
        renderer.draw_container_with_logical_cursor(
            &mut integral_canvas,
            &node,
            0,
            &mut integral_y,
            area,
            palette,
        );
        let mut fractional_canvas = Canvas::new_scaled(180, 80, 2.0, palette.background);
        let mut fractional_y = 31.5;
        renderer.draw_container_with_logical_cursor(
            &mut fractional_canvas,
            &node,
            0,
            &mut fractional_y,
            area,
            palette,
        );

        let x = 100;
        assert_ne!(
            palette.background,
            integral_canvas.pixels()[62 * integral_canvas.width() + x],
            "the integral container hover surface starts on physical row 62"
        );
        assert_eq!(
            palette.background,
            fractional_canvas.pixels()[62 * fractional_canvas.width() + x],
            "a 31.5px container origin must not paint chrome on physical row 62"
        );
        assert_ne!(
            palette.background,
            fractional_canvas.pixels()[63 * fractional_canvas.width() + x],
            "a 31.5px container origin must paint chrome on physical row 63"
        );
    }

    #[test]
    fn fixed_height_container_clip_keeps_fractional_start_and_end_boundaries() {
        let theme = ThemeSnapshot::dark();
        let palette = UiTreeCanvasPalette::from_theme(&theme);
        let renderer = UiTreeCanvasRenderer::new(theme);
        let node = UiNode::new(UiNodeKind::Card, "")
            .height(UiDimension::px(10))
            .child(UiNode::new(UiNodeKind::Button, "button").height(UiDimension::px(20)));
        let mut canvas = Canvas::new_scaled(180, 80, 2.0, palette.background);
        let mut logical_y = 31.5;
        renderer.draw_container_with_logical_cursor(
            &mut canvas,
            &node,
            0,
            &mut logical_y,
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: 180,
                height: 80,
                scroll_y: 0.0,
            },
            palette,
        );

        let width = canvas.width();
        let child_x = 32;
        assert_ne!(
            palette.selection,
            canvas.pixels()[62 * width + child_x],
            "the child must remain clipped before the physical fractional boundary"
        );
        assert_eq!(
            palette.selection,
            canvas.pixels()[63 * width + child_x],
            "the child must start on the physical fractional boundary"
        );
        assert_eq!(
            palette.selection,
            canvas.pixels()[82 * width + child_x],
            "the clip must retain the final physical row of a 10px region from 31.5px"
        );
        assert_ne!(
            palette.selection,
            canvas.pixels()[83 * width + child_x],
            "the child must be clipped after the 10px fractional region"
        );
    }

    #[test]
    fn nested_containers_preserve_fractional_child_extents() {
        let theme = ThemeSnapshot::dark();
        let palette = UiTreeCanvasPalette::from_theme(&theme);
        let renderer = UiTreeCanvasRenderer::with_document_typography(
            theme,
            UiTreeDocumentTypography::new()
                .with_body_baseline(UiTreeTextRoleBaselineTypography::new(16.0, 31.5, 18.5)),
        );
        let column = UiNode::new(UiNodeKind::Column, "")
            .child(UiNode::new(UiNodeKind::Column, "").child(
                UiNode::new(UiNodeKind::Text, "first").text(UiTextProps {
                    role: "body".to_owned(),
                    ..UiTextProps::default()
                }),
            ))
            .child(UiNode::new(UiNodeKind::Column, "").child(
                UiNode::new(UiNodeKind::Text, "second").text(UiTextProps {
                    role: "body".to_owned(),
                    ..UiTextProps::default()
                }),
            ));
        let mut canvas = Canvas::new(240, 120, palette.background);
        let mut y = 0;
        renderer.draw_container(
            &mut canvas,
            &column,
            0,
            &mut y,
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: 240,
                height: 120,
                scroll_y: 0.0,
            },
            palette,
        );

        assert_eq!(63, y);
    }

    #[test]
    fn logical_container_cursor_renders_fixed_and_hover_surface_children() {
        let theme = ThemeSnapshot::dark();
        let palette = UiTreeCanvasPalette::from_theme(&theme);
        let renderer = UiTreeCanvasRenderer::new(theme);
        let root = UiNode::new(UiNodeKind::Column, "")
            .child(
                UiNode::new(UiNodeKind::Card, "Card")
                    .common(
                        UiCommonProps::default()
                            .height(UiDimension::px(40))
                            .padding(UiEdgeInsets::axis(UiDimension::px(3), UiDimension::px(2))),
                    )
                    .child(UiNode::new(UiNodeKind::Text, "clipped first"))
                    .child(UiNode::new(UiNodeKind::Text, "clipped second")),
            )
            .child(
                UiNode::new(UiNodeKind::Stack, "")
                    .visual_role(UiVisualRole::HoverSurface)
                    .child(UiNode::new(UiNodeKind::Text, "hover first"))
                    .child(UiNode::new(UiNodeKind::Text, "hover second")),
            );
        let mut canvas = Canvas::new(180, 120, palette.background);
        let mut y = 0;

        renderer.draw_container(
            &mut canvas,
            &root,
            0,
            &mut y,
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: 180,
                height: 120,
                scroll_y: 0.0,
            },
            palette,
        );

        assert_eq!(
            104, y,
            "logical container children must advance their parent cursor"
        );
        assert!(
            canvas.pixels()[64 * canvas.width()] != palette.background,
            "an auto-height hover surface must paint after its children"
        );
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
