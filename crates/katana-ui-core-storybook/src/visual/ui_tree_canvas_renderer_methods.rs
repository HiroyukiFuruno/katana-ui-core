use super::{
    Canvas, ContainerPadding, INDENT, NODE_GAP, TEXT_HEIGHT, UiNode, UiNodeKind,
    UiTreeCanvasPalette, UiTreeCanvasRenderer, UiTreeRenderArea, UiTreeSettingsContext,
    UiTreeTextMetrics, UiVisualRole, absolute_child_rect, child_container_x, child_render_area,
    dimension_px, draw_hover_surface, draw_label, gap_after_child, has_absolute_child, is_absolute,
    is_outside_vertical_viewport, remaining_width, should_draw_container_label, stack_frame_height,
};
use crate::visual::ui_tree_canvas_scroll_measure::measured_node_height;

impl UiTreeCanvasRenderer {
    pub(super) fn draw_container(
        &self,
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) {
        if node.kind() == UiNodeKind::Stack && has_absolute_child(node) {
            self.draw_overlay_stack(canvas, node, x, y, area, palette);
            return;
        }
        let hover_surface_y = *y;
        let hover_surface_height = self.hover_surface_height(node, x, area, palette);
        if should_draw_container_label(node) {
            draw_label(canvas, &self.text, node, x, y, palette);
        }
        let padding = ContainerPadding::from_node(node);
        let child_x = child_container_x(node, x).saturating_add(padding.left);
        let child_area = child_render_area(area, node, child_x, padding);
        *y = y.saturating_add(padding.top);
        let requested_height = dimension_px(&node.props().common.height);
        if requested_height > 0 {
            let clip_height = hover_surface_child_clip_height(node, requested_height);
            canvas.with_clip(x, *y, remaining_width(area, x), clip_height, |canvas| {
                for (index, child) in node.children().iter().enumerate() {
                    self.draw_container_child(canvas, child, child_x, y, child_area, palette);
                    if index + 1 < node.children().len() {
                        *y = y.saturating_add(gap_after_child(
                            node,
                            child,
                            &node.children()[index + 1],
                        ));
                    }
                }
            });
        } else {
            for (index, child) in node.children().iter().enumerate() {
                self.draw_container_child(canvas, child, child_x, y, child_area, palette);
                if index + 1 < node.children().len() {
                    *y =
                        y.saturating_add(gap_after_child(node, child, &node.children()[index + 1]));
                }
            }
        }
        *y = y.saturating_add(padding.bottom);
        draw_hover_surface(
            canvas,
            node,
            x,
            hover_surface_y,
            area,
            palette,
            hover_surface_height,
        );
    }

    fn draw_container_child(
        &self,
        canvas: &mut Canvas,
        child: &UiNode,
        child_x: usize,
        y: &mut usize,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) {
        let height =
            self.measured_scroll_node_height(child, self.text_context(palette), child_x, area);
        if is_outside_vertical_viewport(*y, height, area) {
            *y = y.saturating_add(height);
            return;
        }
        self.render_node(canvas, child, child_x, y, area, palette);
    }

    fn hover_surface_height(
        &self,
        node: &UiNode,
        x: usize,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) -> usize {
        let requested_height = dimension_px(&node.props().common.height);
        if requested_height > 0 {
            return requested_height;
        }
        if node.props().visual_role != UiVisualRole::HoverSurface {
            return TEXT_HEIGHT;
        }
        measured_node_height(node, self.text_context(palette), x, area)
    }

    pub(super) fn draw_overlay_stack(
        &self,
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) {
        let frame_top = *y;
        let frame_width = remaining_width(area, x);
        let frame_height = stack_frame_height(node).max(TEXT_HEIGHT);
        let visible_top = frame_top.max(area.y);
        let visible_bottom = frame_top
            .saturating_add(frame_height)
            .min(area.y.saturating_add(area.height));
        let visible_height = visible_bottom.saturating_sub(visible_top);
        if visible_height == 0 {
            *y = frame_top.saturating_add(frame_height);
            return;
        }
        let scroll_y = area.scroll_y.round().max(0.0) as usize;
        canvas.with_clip(x, visible_top, frame_width, visible_height, |canvas| {
            if matches!(
                node.props().visual_role,
                UiVisualRole::MediaFrame | UiVisualRole::ExportMediaFrame
            ) {
                canvas.fill_rect(x, frame_top, frame_width, frame_height, palette.background);
            }
            let stack_area = UiTreeRenderArea {
                x,
                y: frame_top,
                width: frame_width,
                height: visible_height,
                scroll_y: area.scroll_y,
            };
            for child in node.children().iter().filter(|child| !is_absolute(child)) {
                let mut child_y = frame_top;
                self.render_node(canvas, child, x, &mut child_y, stack_area, palette);
            }
            for child in node.children().iter().filter(|child| is_absolute(child)) {
                let rect = absolute_child_rect(x, frame_top, frame_width, frame_height, child);
                let Some(child_y) = rect.y.checked_sub(scroll_y) else {
                    continue;
                };
                let mut child_y = child_y;
                self.render_node(canvas, child, rect.x, &mut child_y, stack_area, palette);
            }
        });
        *y = frame_top.saturating_add(frame_height);
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
        let document_accordion = node.props().text.role == "html-accordion";
        let metrics = UiTreeTextMetrics::for_node_with_typography(node, self.typography);
        let label = if document_accordion {
            node.props().label.clone()
        } else if node.props().interaction.open {
            format!("v {}", node.props().label)
        } else {
            format!("> {}", node.props().label)
        };
        self.text.draw(
            canvas,
            &label,
            x,
            y.saturating_add(metrics.top_margin),
            metrics.font_size,
            palette.text,
        );
        *y = y.saturating_add(metrics.line_height);
        if node.props().interaction.open {
            let child_x = if document_accordion {
                x
            } else {
                x.saturating_add(INDENT)
            };
            for child in node.children() {
                self.render_node(canvas, child, child_x, y, area, palette);
            }
            if !document_accordion {
                *y = y.saturating_add(NODE_GAP);
            }
        }
    }

    pub(super) fn settings_context(
        &self,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) -> UiTreeSettingsContext<'_> {
        UiTreeSettingsContext {
            renderer: self,
            text: &self.text,
            area,
            palette,
        }
    }
}

fn hover_surface_child_clip_height(node: &UiNode, requested_height: usize) -> usize {
    if node.props().visual_role == UiVisualRole::HoverSurface {
        return requested_height.saturating_add(TEXT_HEIGHT);
    }
    requested_height
}
