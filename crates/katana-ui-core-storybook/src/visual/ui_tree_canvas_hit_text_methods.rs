use super::{
    TEXT_HEIGHT, TextRenderer, ThemeSnapshot, UiNode, UiTextSpan, UiTreeCanvasPalette,
    UiTreeHostActionHitCollector, UiTreeTextContext, UiTreeTextMetrics, UiTreeTextRenderer,
    whitespace_width,
};
use crate::visual::ui_tree_canvas_text_line_width::{
    SpanTextRenderers, preserves_whitespace, span_line_width, span_part_width,
    span_visible_part_bounds,
};

impl UiTreeHostActionHitCollector<'_> {
    pub(super) fn text_hit_height(&self, node: &UiNode, x: usize) -> usize {
        UiTreeTextRenderer::measure_node_height(self.text_context(), node, x, self.area)
            .max(TEXT_HEIGHT)
    }

    pub(super) fn text_span_hit_width(&self, node: &UiNode, text: &str) -> usize {
        let metrics = UiTreeTextMetrics::for_node_with_typography(node, self.typography);
        let preserve_whitespace = node.props().text.role == "code";
        let mut width = 0usize;
        let mut segment = String::new();
        for character in text.chars() {
            if character.is_whitespace() && character != '\n' {
                width = width.saturating_add(self.measured_text_width(node, &segment, metrics));
                segment.clear();
                width =
                    width.saturating_add(whitespace_width(metrics.font_size, preserve_whitespace));
                continue;
            }
            segment.push(character);
        }
        width
            .saturating_add(self.measured_text_width(node, &segment, metrics))
            .max(1)
    }

    pub(super) fn text_span_render_width(&self, node: &UiNode, span: &UiTextSpan) -> usize {
        span_part_width(
            self.span_text_renderers(node),
            span,
            UiTreeTextMetrics::for_node_with_typography(node, self.typography),
            preserves_whitespace(node),
        )
    }

    pub(super) fn text_span_visible_hit_bounds(
        &self,
        node: &UiNode,
        span: &UiTextSpan,
    ) -> (usize, usize) {
        span_visible_part_bounds(
            self.span_text_renderers(node),
            span,
            UiTreeTextMetrics::for_node_with_typography(node, self.typography),
            preserves_whitespace(node),
        )
    }

    pub(super) fn text_hit_width(&self, node: &UiNode) -> usize {
        if node.props().text.spans.is_empty() {
            return self.text_span_hit_width(node, node.props().label.as_str());
        }
        span_line_width(
            self.span_text_renderers(node),
            node.props().text.spans.as_slice(),
            UiTreeTextMetrics::for_node_with_typography(node, self.typography),
            preserves_whitespace(node),
        )
    }

    pub(super) fn measured_text_width(
        &self,
        node: &UiNode,
        text: &str,
        metrics: UiTreeTextMetrics,
    ) -> usize {
        if text.is_empty() {
            return 0;
        }
        self.text_renderer(node)
            .measure_width(text, metrics.font_size)
            .max(1)
    }

    pub(super) fn text_renderer(&self, node: &UiNode) -> &TextRenderer {
        if node.props().font_role == "code" || node.props().font_role == "document-code" {
            return self.code_text;
        }
        if node.props().font_role == "document-export-body" {
            return self.export_text;
        }
        self.text
    }

    fn span_text_renderers(&self, node: &UiNode) -> SpanTextRenderers<'_> {
        SpanTextRenderers::new(self.text_renderer(node), self.code_text)
    }

    pub(super) fn text_context(&self) -> UiTreeTextContext<'_> {
        UiTreeTextContext {
            text: self.text,
            export_text: self.export_text,
            code_text: self.code_text,
            palette: UiTreeCanvasPalette::from_theme(&ThemeSnapshot::dark()),
            typography: self.typography,
        }
    }
}
