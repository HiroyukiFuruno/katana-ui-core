use super::canvas::Canvas;
use super::document_typography::UiTreeDocumentTypography;
use super::ui_tree_canvas::UiTreeCanvasRenderer;
use super::ui_tree_canvas_context_menu::UiTreeContextMenuRenderer;
use super::ui_tree_canvas_types::{
    UiTreeHostActionHit, UiTreeHostActionHitQuery, UiTreeInteractionTarget, UiTreeNodeHit,
    UiTreeRenderArea,
};
use katana_ui_core::render_model::{UiCursor, UiNode, UiNodeId};
use katana_ui_core::text_raster::{PlatformTextFaceSelection, PlatformTextRasterConfig};
use katana_ui_core::theme::ThemeSnapshot;

pub struct UiTreeStorybookHost {
    renderer: UiTreeCanvasRenderer,
}

impl UiTreeStorybookHost {
    #[must_use]
    pub fn new(theme: ThemeSnapshot) -> Self {
        Self::with_document_typography(theme, UiTreeDocumentTypography::default())
    }

    /// Creates a host with optional document-role typography overrides.
    #[must_use]
    pub fn with_document_typography(
        theme: ThemeSnapshot,
        document_typography: UiTreeDocumentTypography,
    ) -> Self {
        Self::with_text_raster_config_and_document_typography(
            theme,
            PlatformTextRasterConfig::default(),
            PlatformTextFaceSelection::System,
            document_typography,
        )
    }

    #[must_use]
    pub fn with_text_raster_config(
        theme: ThemeSnapshot,
        text_raster_config: PlatformTextRasterConfig,
        face_selection: PlatformTextFaceSelection,
    ) -> Self {
        Self::with_text_raster_config_and_document_typography(
            theme,
            text_raster_config,
            face_selection,
            UiTreeDocumentTypography::default(),
        )
    }

    /// Creates a host with explicit text-raster and document-role typography settings.
    #[must_use]
    pub fn with_text_raster_config_and_document_typography(
        theme: ThemeSnapshot,
        text_raster_config: PlatformTextRasterConfig,
        face_selection: PlatformTextFaceSelection,
        document_typography: UiTreeDocumentTypography,
    ) -> Self {
        Self {
            renderer: UiTreeCanvasRenderer::with_text_raster_config_and_document_typography(
                theme,
                text_raster_config,
                face_selection,
                document_typography,
            ),
        }
    }

    pub fn render(&self, canvas: &mut Canvas, root: &UiNode, area: UiTreeRenderArea) {
        self.renderer.render(canvas, root, area);
    }

    #[must_use]
    pub fn host_action_hits(
        &self,
        root: &UiNode,
        area: UiTreeRenderArea,
    ) -> Vec<UiTreeHostActionHit> {
        self.renderer.host_action_hit_rects(root, area)
    }

    #[must_use]
    pub fn viewport_host_action_hits(
        &self,
        root: &UiNode,
        area: UiTreeRenderArea,
    ) -> Vec<UiTreeHostActionHit> {
        self.renderer.viewport_host_action_hit_rects(root, area)
    }

    #[must_use]
    pub fn viewport_interaction_hits(
        &self,
        root: &UiNode,
        area: UiTreeRenderArea,
    ) -> (Vec<UiTreeHostActionHit>, Vec<UiTreeNodeHit>) {
        self.renderer.viewport_interaction_hit_rects(root, area)
    }

    #[must_use]
    pub fn interaction_target_at(
        &self,
        root: &UiNode,
        area: UiTreeRenderArea,
        x: f32,
        y: f32,
    ) -> Option<UiTreeInteractionTarget> {
        let (hits, node_hits) = self.viewport_interaction_hits(root, area);
        Self::interaction_target_for_hits_at(&hits, &node_hits, x, y)
    }

    #[must_use]
    pub fn interaction_target_for_hits_at(
        hits: &[UiTreeHostActionHit],
        node_hits: &[UiTreeNodeHit],
        x: f32,
        y: f32,
    ) -> Option<UiTreeInteractionTarget> {
        UiTreeInteractionTarget::from_hits_at(hits, node_hits, x, y)
    }

    #[must_use]
    pub fn document_host_action_hits(
        &self,
        root: &UiNode,
        area: UiTreeRenderArea,
    ) -> Vec<UiTreeHostActionHit> {
        self.renderer.document_host_action_hit_rects(root, area)
    }

    #[must_use]
    pub fn document_node_hits(&self, root: &UiNode, area: UiTreeRenderArea) -> Vec<UiTreeNodeHit> {
        self.renderer.document_node_hit_rects(root, area)
    }

    #[must_use]
    pub fn viewport_node_hits(&self, root: &UiNode, area: UiTreeRenderArea) -> Vec<UiTreeNodeHit> {
        self.renderer.viewport_node_hit_rects(root, area)
    }

    #[must_use]
    pub fn host_action_hit_at(
        &self,
        root: &UiNode,
        area: UiTreeRenderArea,
        x: f32,
        y: f32,
    ) -> Option<UiTreeHostActionHit> {
        self.host_action_hits_at(root, area, x, y)
            .into_iter()
            .next()
    }

    #[must_use]
    pub fn host_action_hits_at(
        &self,
        root: &UiNode,
        area: UiTreeRenderArea,
        x: f32,
        y: f32,
    ) -> Vec<UiTreeHostActionHit> {
        let hits = self.host_action_hits(root, area);
        Self::filter_host_action_hits_at(&hits, x, y)
    }

    #[must_use]
    pub fn filter_host_action_hits_at(
        hits: &[UiTreeHostActionHit],
        x: f32,
        y: f32,
    ) -> Vec<UiTreeHostActionHit> {
        UiTreeHostActionHitQuery::new(hits)
            .cloned_hits_at(x, y)
            .collect()
    }

    #[must_use]
    pub fn cursor_for_host_action_hits_at(
        hits: &[UiTreeHostActionHit],
        x: f32,
        y: f32,
    ) -> UiCursor {
        Self::filter_host_action_hits_at(hits, x, y)
            .into_iter()
            .find_map(|hit| (hit.cursor != UiCursor::Default).then_some(hit.cursor))
            .unwrap_or(UiCursor::Default)
    }

    #[must_use]
    pub fn hovered_action_node_id_for_host_action_hits_at(
        hits: &[UiTreeHostActionHit],
        x: f32,
        y: f32,
    ) -> Option<UiNodeId> {
        Self::filter_host_action_hits_at(hits, x, y)
            .into_iter()
            .next()
            .map(|hit| hit.action.target)
    }

    #[must_use]
    pub fn cursor_at(&self, root: &UiNode, area: UiTreeRenderArea, x: f32, y: f32) -> UiCursor {
        let hits = self.host_action_hits(root, area);
        Self::cursor_for_host_action_hits_at(&hits, x, y)
    }

    #[must_use]
    pub fn hovered_action_node_id_at(
        &self,
        root: &UiNode,
        area: UiTreeRenderArea,
        x: f32,
        y: f32,
    ) -> Option<UiNodeId> {
        let hits = self.host_action_hits(root, area);
        Self::hovered_action_node_id_for_host_action_hits_at(&hits, x, y)
    }

    #[must_use]
    pub fn context_menu_item_id_at(root: &UiNode, x: f32, y: f32) -> Option<String> {
        UiTreeContextMenuRenderer::item_id_at(root, x, y)
    }

    #[must_use]
    pub fn context_menu_host_action_at(
        root: &UiNode,
        x: f32,
        y: f32,
    ) -> Option<katana_ui_core::render_model::UiHostActionPlan> {
        UiTreeContextMenuRenderer::host_action_at(root, x, y)
    }

    #[must_use]
    pub fn context_menu_item_center_for_id(root: &UiNode, item_id: &str) -> Option<(f32, f32)> {
        UiTreeContextMenuRenderer::item_center_for_id(root, item_id)
    }
}
