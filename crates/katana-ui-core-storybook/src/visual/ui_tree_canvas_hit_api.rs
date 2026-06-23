use super::ui_tree_canvas::UiTreeCanvasRenderer;
use super::ui_tree_canvas_context_menu::UiTreeContextMenuRenderer;
use super::ui_tree_canvas_hit::UiTreeHostActionHitCollector;
use super::ui_tree_canvas_types::{
    UiTreeHostActionHit, UiTreeHostActionHitQuery, UiTreeNodeHit, UiTreeRenderArea,
};
use katana_ui_core::render_model::UiNode;

impl UiTreeCanvasRenderer {
    #[must_use]
    pub fn host_action_hit_rects(
        &self,
        root: &UiNode,
        area: UiTreeRenderArea,
    ) -> Vec<UiTreeHostActionHit> {
        UiTreeHostActionHitCollector::collect_with_renderers(
            root,
            area,
            &self.document_text,
            &self.export_text,
            &self.code_text,
            self.typography,
        )
    }

    #[must_use]
    pub fn viewport_host_action_hit_rects(
        &self,
        root: &UiNode,
        area: UiTreeRenderArea,
    ) -> Vec<UiTreeHostActionHit> {
        UiTreeHostActionHitCollector::collect_viewport_with_renderers(
            root,
            area,
            &self.document_text,
            &self.export_text,
            &self.code_text,
            self.typography,
        )
    }

    #[must_use]
    pub fn viewport_interaction_hit_rects(
        &self,
        root: &UiNode,
        area: UiTreeRenderArea,
    ) -> (Vec<UiTreeHostActionHit>, Vec<UiTreeNodeHit>) {
        UiTreeHostActionHitCollector::collect_viewport_interaction_with_renderers(
            root,
            area,
            &self.document_text,
            &self.export_text,
            &self.code_text,
            self.typography,
        )
    }

    #[must_use]
    pub fn document_host_action_hit_rects(
        &self,
        root: &UiNode,
        area: UiTreeRenderArea,
    ) -> Vec<UiTreeHostActionHit> {
        UiTreeHostActionHitCollector::collect_document_with_renderers(
            root,
            area,
            &self.document_text,
            &self.export_text,
            &self.code_text,
            self.typography,
        )
    }

    #[must_use]
    pub fn document_node_hit_rects(
        &self,
        root: &UiNode,
        area: UiTreeRenderArea,
    ) -> Vec<UiTreeNodeHit> {
        UiTreeHostActionHitCollector::collect_node_hits_with_renderers(
            root,
            area,
            &self.document_text,
            &self.export_text,
            &self.code_text,
            self.typography,
        )
    }

    #[must_use]
    pub fn viewport_node_hit_rects(
        &self,
        root: &UiNode,
        area: UiTreeRenderArea,
    ) -> Vec<UiTreeNodeHit> {
        UiTreeHostActionHitCollector::collect_viewport_node_hits_with_renderers(
            root,
            area,
            &self.document_text,
            &self.export_text,
            &self.code_text,
            self.typography,
        )
    }

    #[must_use]
    pub fn host_action_hit_at(
        &self,
        root: &UiNode,
        area: UiTreeRenderArea,
        x: f32,
        y: f32,
    ) -> Option<UiTreeHostActionHit> {
        let hits = self.host_action_hit_rects(root, area);
        UiTreeHostActionHitQuery::new(&hits)
            .cloned_hits_at(x, y)
            .next()
    }

    #[must_use]
    pub fn context_menu_item_id_at(root: &UiNode, x: f32, y: f32) -> Option<String> {
        UiTreeContextMenuRenderer::item_id_at(root, x, y)
    }

    #[must_use]
    pub fn context_menu_item_center_for_id(root: &UiNode, item_id: &str) -> Option<(f32, f32)> {
        UiTreeContextMenuRenderer::item_center_for_id(root, item_id)
    }
}
