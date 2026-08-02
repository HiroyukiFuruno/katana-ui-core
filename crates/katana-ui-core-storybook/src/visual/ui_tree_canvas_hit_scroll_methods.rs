use super::{
    MeasuredNodeHeightCache, ScrollContainerPadding, ScrollHitClip, ThemeSnapshot,
    UiHostActionPlan, UiNode, UiTreeCanvasPalette, UiTreeHostActionHit,
    UiTreeHostActionHitCollector, UiTreeRenderArea, UiTreeTextContext,
    can_render_children_incrementally, child_container_x, clip_scroll_hit, remaining_width,
    scroll_child_render_area, scroll_container_gap, scroll_source_y,
};

impl UiTreeHostActionHitCollector<'_> {
    pub(super) fn scroll_area(&mut self, node: &UiNode, x: usize) {
        let viewport_top = self.y;
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
        self.y = viewport_top.saturating_add(viewport_height);
    }

    pub(super) fn collect_scroll_area_hits(
        &mut self,
        node: &UiNode,
        viewport_x: usize,
        viewport_y: usize,
        viewport_width: usize,
        viewport_height: usize,
        source_y: usize,
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

    fn collect_visible_children(&mut self, node: &UiNode, x: usize, source_y: usize) {
        for child in node.children() {
            self.collect_visible_node(child, x, source_y);
        }
    }

    pub(in crate::visual) fn collect_visible_node(
        &mut self,
        node: &UiNode,
        x: usize,
        source_y: usize,
    ) {
        if can_render_children_incrementally(node) {
            self.collect_visible_incremental_container(node, x, source_y);
            return;
        }
        let node_top = self.y;
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
        let node_bottom = node_top.saturating_add(node_height);
        self.y = node_bottom;
        if node_bottom <= source_y || node_top >= source_y.saturating_add(self.area.height) {
            return;
        }
        let previous_y = self.y;
        self.y = node_top;
        self.node(node, x);
        self.y = previous_y;
    }

    fn collect_visible_incremental_container(&mut self, node: &UiNode, x: usize, source_y: usize) {
        let padding = ScrollContainerPadding::from_node(node);
        self.y = self.y.saturating_add(padding.top);
        let child_x = child_container_x(node, x).saturating_add(padding.left);
        let previous_area = self.area;
        self.area = scroll_child_render_area(self.area, node, child_x, padding);
        let gap = scroll_container_gap(node);
        for (index, child) in node.children().iter().enumerate() {
            if index > 0 {
                self.y = self.y.saturating_add(gap);
            }
            if self.y >= source_y.saturating_add(previous_area.height) {
                break;
            }
            self.collect_visible_node(child, child_x, source_y);
        }
        self.area = previous_area;
        self.y = self.y.saturating_add(padding.bottom);
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
