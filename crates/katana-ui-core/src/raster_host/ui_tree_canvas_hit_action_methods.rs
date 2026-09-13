use super::{
    UI_LINK_OPEN_ACTION_ID, UiHostActionPlan, UiNode, UiTreeHitRect, UiTreeHostActionHit,
    UiTreeHostActionHitCollector, UiTreeNodeHit, UiTreeTextMetrics, UiTreeTextRoleRenderer,
    node_cursor, semantic_node_id,
};
use crate::raster_host::ui_tree_canvas_text::text_lines::{
    text_wrap::UiTreeTextWrap, visible_line_y,
};
use crate::raster_host::ui_tree_canvas_text_line_width::{preserves_whitespace, span_line_width};
use katana_ui_core::render_model::UiTextSpan;

impl UiTreeHostActionHitCollector<'_> {
    pub(super) fn actions_for_node(&self, node: &UiNode) -> Vec<UiHostActionPlan> {
        if self.actions.is_empty() {
            return UiHostActionPlan::collect_from_node(node);
        }
        self.actions
            .iter()
            .filter(move |action| action.target.as_str() == node.id().as_str())
            .cloned()
            .collect()
    }

    pub(super) fn tree_row_actions(
        &self,
        node: &UiNode,
        tree_node_id: &str,
    ) -> Vec<UiHostActionPlan> {
        let actions = self.actions_for_node(node);
        actions
            .iter()
            .filter(move |action| action.target.as_str() == node.id().as_str())
            .filter(|action| {
                action
                    .tree_row_action_target()
                    .is_some_and(|target| target.node_id == tree_node_id)
            })
            .cloned()
            .collect()
    }

    pub(super) fn push_node_action_hits(&mut self, node: &UiNode, rect: UiTreeHitRect) {
        self.push_node_hit(node, rect);
        self.push_action_hits(node, self.actions_for_node(node), rect);
    }

    pub(super) fn push_node_hit(&mut self, node: &UiNode, rect: UiTreeHitRect) {
        let semantic_node_id = semantic_node_id(node).or_else(|| self.semantic_node_id.clone());
        self.node_hits.push(UiTreeNodeHit {
            node_id: node.id().clone(),
            semantic_node_id,
            rect,
            cursor: node_cursor(node),
        });
    }

    pub(super) fn push_action_hits(
        &mut self,
        node: &UiNode,
        actions: impl IntoIterator<Item = UiHostActionPlan>,
        rect: UiTreeHitRect,
    ) {
        let cursor = node_cursor(node);
        for action in actions {
            self.hits.push(UiTreeHostActionHit {
                action,
                rect,
                cursor,
            });
        }
    }

    pub(super) fn push_text_link_action_hits(
        &mut self,
        node: &UiNode,
        x: usize,
        logical_y: f32,
        height: usize,
        actions: &[UiHostActionPlan],
    ) {
        if !actions
            .iter()
            .any(|action| action.action_id == UI_LINK_OPEN_ACTION_ID)
        {
            return;
        }
        let cursor = node_cursor(node);
        let metrics = UiTreeTextMetrics::for_node_with_typography(node, self.typography);
        let renderers = self.span_text_renderers(node);
        let preserve_whitespace = preserves_whitespace(node);
        let lines = UiTreeTextWrap::span_lines(renderers, node, x, self.area, metrics);
        let source_actions = source_link_actions(node, actions);
        let mut source_spans = SourceSpanCursor::new(&node.props().text.spans, preserve_whitespace);
        let mut link_hits = Vec::new();
        for (line_index, line) in lines.iter().enumerate() {
            let line_source_indices = line
                .iter()
                .map(|span| source_spans.consume(span))
                .collect::<Vec<_>>();
            let line_width = span_line_width(renderers, line, metrics, preserve_whitespace);
            let mut span_x =
                UiTreeTextRoleRenderer::line_x(node, x, x, self.area, line_width, line_index);
            let line_y = if self.scroll_clip == super::ScrollHitClip::Document {
                logical_y + line_index as f32 * metrics.line_box_height
            } else {
                let Some(line_y) = visible_line_y(line_index, logical_y, self.area, metrics) else {
                    continue;
                };
                line_y
            };
            let line_y = (line_y + metrics.top_margin as f32).floor().max(0.0) as usize;
            let document_line_offset = (metrics.top_margin as f32
                + line_index as f32 * metrics.line_box_height)
                .floor()
                .max(0.0) as usize;
            let line_height = (metrics.line_box_height.ceil() as usize)
                .min(height.saturating_sub(document_line_offset));
            if line_height == 0 {
                continue;
            }
            for (span, source_index) in line.iter().zip(line_source_indices) {
                let width = self.text_span_render_width(node, span);
                if !span.link_target.trim().is_empty()
                    && let Some(action) = source_index
                        .and_then(|index| source_actions.get(index))
                        .and_then(Option::as_ref)
                {
                    let (visible_offset, visible_width) =
                        self.text_span_visible_hit_bounds(node, span);
                    let visible_x = span_x + visible_offset as isize;
                    let rect_x = visible_x.max(0) as usize;
                    let hidden_width =
                        visible_x.unsigned_abs() * usize::from(visible_x.is_negative());
                    let clipped_width = visible_width.saturating_sub(hidden_width);
                    if clipped_width > 0 {
                        link_hits.push(UiTreeHostActionHit {
                            action: action.clone(),
                            rect: UiTreeHitRect {
                                x: rect_x,
                                y: line_y,
                                width: clipped_width,
                                height: line_height.min(height),
                            },
                            cursor,
                        });
                    }
                }
                span_x += width as isize;
            }
        }
        self.hits.extend(link_hits);
    }
}

fn source_link_actions(
    node: &UiNode,
    actions: &[UiHostActionPlan],
) -> Vec<Option<UiHostActionPlan>> {
    node.props()
        .text
        .spans
        .iter()
        .map(|span| {
            (!span.link_target.trim().is_empty())
                .then(|| {
                    actions
                        .iter()
                        .find(|action| {
                            action.action_id == UI_LINK_OPEN_ACTION_ID
                                && action.payload == span.link_target
                                && action.label == span.text
                        })
                        .cloned()
                })
                .flatten()
        })
        .collect()
}

struct SourceSpanCursor<'a> {
    spans: &'a [UiTextSpan],
    index: usize,
    offset: usize,
    preserve_whitespace: bool,
}

impl<'a> SourceSpanCursor<'a> {
    fn new(spans: &'a [UiTextSpan], preserve_whitespace: bool) -> Self {
        Self {
            spans,
            index: 0,
            offset: 0,
            preserve_whitespace,
        }
    }

    fn consume(&mut self, rendered: &UiTextSpan) -> Option<usize> {
        self.skip_normalized_whitespace(rendered.text.as_str());
        let source = self.spans.get(self.index)?;
        let source_index = self.index;
        self.offset = self.offset.saturating_add(rendered.text.len());
        if self.offset >= source.text.len() {
            self.index += 1;
            self.offset = 0;
        }
        Some(source_index)
    }

    fn skip_normalized_whitespace(&mut self, rendered: &str) {
        while let Some(source) = self.spans.get(self.index) {
            let remaining = &source.text[self.offset..];
            if remaining.is_empty() {
                self.index += 1;
                self.offset = 0;
                continue;
            }
            if remaining.starts_with(rendered) {
                return;
            }
            let Some(whitespace) = remaining
                .chars()
                .next()
                .filter(|character| character.is_whitespace())
            else {
                return;
            };
            if self.preserve_whitespace && !whitespace_prefix_has_line_break(remaining) {
                return;
            }
            self.offset += whitespace.len_utf8();
        }
    }
}

fn whitespace_prefix_has_line_break(value: &str) -> bool {
    value
        .chars()
        .take_while(|character| character.is_whitespace())
        .any(|character| matches!(character, '\n' | '\r'))
}

#[cfg(test)]
mod source_cursor_tests {
    use super::{SourceSpanCursor, UiTextSpan};

    #[test]
    fn normalization_never_discards_source_text_or_preserved_indentation() {
        for (source, preserve_whitespace, expected_offset) in [
            ("prefix suffix", false, 0),
            (" suffix", true, 0),
            (" \n suffix", true, 2),
        ] {
            let spans = [UiTextSpan::plain(source)];
            let mut cursor = SourceSpanCursor::new(&spans, preserve_whitespace);
            cursor.skip_normalized_whitespace("suffix");

            assert_eq!(0, cursor.index);
            assert_eq!(expected_offset, cursor.offset);
        }
    }
}
