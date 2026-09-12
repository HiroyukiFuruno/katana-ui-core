use super::{
    MeasuredNodeHeightCache, ScrollContainerPadding, ScrollHitClip, ThemeSnapshot,
    UiHostActionPlan, UiNode, UiTreeCanvasPalette, UiTreeHostActionHit,
    UiTreeHostActionHitCollector, UiTreeRenderArea, UiTreeTextContext,
    can_render_children_incrementally, child_container_x, clip_scroll_hit, remaining_width,
    scroll_child_render_area, scroll_container_gap, scroll_source_y,
};
use crate::raster_host::ui_tree_canvas_hit_metrics::dimension_px;

impl UiTreeHostActionHitCollector<'_> {
    pub(super) fn scroll_area(&mut self, node: &UiNode, x: usize) {
        let viewport_top = self.y;
        let viewport_logical_top = self.text_logical_y.max(viewport_top as f32);
        let viewport_width = (node.props().scroll_area.viewport_width as usize)
            .min(remaining_width(self.area, x))
            .max(1);
        let viewport_height = (node.props().scroll_area.viewport_height as usize)
            .min(
                self.area
                    .height
                    .saturating_sub(viewport_top.saturating_sub(self.area.y)),
            )
            .max(1);
        let previous_area = self.area;
        let source_y = scroll_source_y(node, previous_area);
        match self.scroll_clip {
            ScrollHitClip::Viewport => self.collect_scroll_area_hits(
                node,
                x,
                viewport_top,
                viewport_width,
                viewport_height,
                source_y,
            ),
            ScrollHitClip::Document => {
                self.collect_scroll_area_document_hits(node, x, viewport_top, viewport_width)
            }
        }
        self.text_logical_y = viewport_logical_top + viewport_height as f32;
        self.y = self.text_logical_y.floor().max(0.0) as usize;
    }

    pub(super) fn collect_scroll_area_hits(
        &mut self,
        node: &UiNode,
        viewport_x: usize,
        viewport_y: usize,
        viewport_width: usize,
        viewport_height: usize,
        source_y: f32,
    ) {
        let content_area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: viewport_width,
            height: viewport_height,
            scroll_y: 0.0,
        };
        let mut content_collector = UiTreeHostActionHitCollector {
            area: content_area,
            actions: self.actions.clone(),
            hits: Vec::new(),
            node_hits: Vec::new(),
            y: 0,
            text_logical_y: 0.0,
            text: self.text,
            export_text: self.export_text,
            code_text: self.code_text,
            typography: self.typography,
            scroll_clip: self.scroll_clip,
            semantic_node_id: self.semantic_node_id.clone(),
            height_cache: MeasuredNodeHeightCache::default(),
        };
        content_collector.collect_visible_children(node, 0, source_y);
        self.hits
            .extend(content_collector.hits.into_iter().filter_map(|hit| {
                clip_scroll_hit(
                    hit,
                    viewport_x,
                    viewport_y,
                    viewport_width,
                    viewport_height,
                    source_y,
                )
            }));
        self.node_hits.extend(
            content_collector
                .node_hits
                .into_iter()
                .filter_map(|mut hit| {
                    let action_hit = UiTreeHostActionHit {
                        action: UiHostActionPlan::new(
                            hit.node_id.clone(),
                            katana_ui_core::render_model::UiHostActionSpec::command(
                                "node-hit", "Node hit",
                            ),
                        ),
                        rect: hit.rect,
                        cursor: hit.cursor,
                    };
                    clip_scroll_hit(
                        action_hit,
                        viewport_x,
                        viewport_y,
                        viewport_width,
                        viewport_height,
                        source_y,
                    )
                    .map(|clipped| {
                        hit.rect = clipped.rect;
                        hit
                    })
                }),
        );
    }

    fn collect_visible_children(&mut self, node: &UiNode, x: usize, source_y: f32) {
        for child in node.children() {
            self.collect_visible_node(child, x, source_y);
        }
    }

    pub(in crate::raster_host) fn collect_visible_node(
        &mut self,
        node: &UiNode,
        x: usize,
        source_y: f32,
    ) {
        if can_render_children_incrementally(node) {
            self.collect_visible_incremental_container(node, x, source_y);
            return;
        }
        let node_top = self.y;
        let node_logical_top = self.text_logical_y.max(node_top as f32);
        let text_context = UiTreeTextContext {
            text: self.text,
            export_text: self.export_text,
            code_text: self.code_text,
            palette: UiTreeCanvasPalette::from_theme(&ThemeSnapshot::dark()),
            typography: self.typography,
        };
        let node_height = self
            .height_cache
            .height(node, text_context, x, self.area)
            .max(1);
        let node_logical_height = if node.kind() == katana_ui_core::render_model::UiNodeKind::Text {
            self.logical_text_hit_height(node, x)
        } else {
            node_height as f32
        };
        let node_logical_bottom = node_logical_top + node_logical_height;
        let node_bottom = node_logical_bottom.floor().max(0.0) as usize;
        self.y = node_bottom;
        self.text_logical_y = node_logical_bottom;
        if node_bottom as f32 <= source_y || node_top as f32 >= source_y + self.area.height as f32 {
            return;
        }
        self.y = node_top;
        self.text_logical_y = node_logical_top;
        self.node(node, x);
    }

    fn collect_visible_incremental_container(&mut self, node: &UiNode, x: usize, source_y: f32) {
        let container_logical_top = self.text_logical_y;
        let requested_height = dimension_px(&node.props().common.height);
        let requested_logical_bottom = container_logical_top + requested_height as f32;
        let padding = ScrollContainerPadding::from_node(node);
        self.advance_y(padding.top);
        let child_x = child_container_x(node, x).saturating_add(padding.left);
        let previous_area = self.area;
        self.area = scroll_child_render_area(self.area, node, child_x, padding);
        let gap = scroll_container_gap(node);
        for (index, child) in node.children().iter().enumerate() {
            if index > 0 {
                self.advance_y(gap);
            }
            if self.y as f32 >= source_y + previous_area.height as f32 {
                break;
            }
            if requested_height > 0 && self.text_logical_y >= requested_logical_bottom {
                break;
            }
            self.collect_visible_node(child, child_x, source_y);
        }
        self.area = previous_area;
        if requested_height > 0 {
            self.text_logical_y = requested_logical_bottom;
            self.y = requested_logical_bottom.floor().max(0.0) as usize;
        } else {
            self.advance_y(padding.bottom);
        }
    }

    pub(super) fn collect_scroll_area_document_hits(
        &mut self,
        node: &UiNode,
        viewport_x: usize,
        viewport_y: usize,
        viewport_width: usize,
    ) {
        let content_height = node
            .props()
            .scroll_area
            .content_height
            .max(node.props().scroll_area.viewport_height)
            .max(1) as usize;
        let content_area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: viewport_width,
            height: content_height,
            scroll_y: 0.0,
        };
        let mut content_collector = UiTreeHostActionHitCollector {
            area: content_area,
            actions: self.actions.clone(),
            hits: Vec::new(),
            node_hits: Vec::new(),
            y: 0,
            text_logical_y: 0.0,
            text: self.text,
            export_text: self.export_text,
            code_text: self.code_text,
            typography: self.typography,
            scroll_clip: self.scroll_clip,
            semantic_node_id: self.semantic_node_id.clone(),
            height_cache: MeasuredNodeHeightCache::default(),
        };
        for child in node.children() {
            content_collector.node(child, 0);
        }
        self.hits
            .extend(content_collector.hits.into_iter().map(|mut hit| {
                hit.rect.x = viewport_x.saturating_add(hit.rect.x);
                hit.rect.y = viewport_y.saturating_add(hit.rect.y);
                hit
            }));
        self.node_hits
            .extend(content_collector.node_hits.into_iter().map(|mut hit| {
                hit.rect.x = viewport_x.saturating_add(hit.rect.x);
                hit.rect.y = viewport_y.saturating_add(hit.rect.y);
                hit
            }));
    }
}
