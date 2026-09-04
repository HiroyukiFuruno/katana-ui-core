use super::canvas::Canvas;
use super::ui_tree_canvas_types::{
    UiTreeHostActionHit, UiTreeInteractionTarget, UiTreeNodeHit, UiTreeRenderArea,
};
use super::ui_tree_storybook_host::UiTreeStorybookHost;
use katana_ui_core::render_model::{UiCursor, UiNode, UiNodeId};
use katana_ui_core::text_raster::{PlatformTextFaceSelection, PlatformTextRasterConfig};
use katana_ui_core::theme::ThemeSnapshot;

pub struct UiTreeSurfaceHost {
    host: UiTreeStorybookHost,
}

impl UiTreeSurfaceHost {
    #[must_use]
    pub fn new(theme: ThemeSnapshot) -> Self {
        Self::with_text_raster_config(
            theme,
            PlatformTextRasterConfig::default(),
            PlatformTextFaceSelection::System,
        )
    }

    #[must_use]
    pub fn with_text_raster_config(
        theme: ThemeSnapshot,
        text_raster_config: PlatformTextRasterConfig,
        face_selection: PlatformTextFaceSelection,
    ) -> Self {
        Self {
            host: UiTreeStorybookHost::with_text_raster_config(
                theme,
                text_raster_config,
                face_selection,
            ),
        }
    }

    pub fn render(&self, canvas: &mut Canvas, root: &UiNode, area: UiTreeRenderArea) {
        self.host.render(canvas, root, area);
    }

    #[must_use]
    pub fn host_action_hits(
        &self,
        root: &UiNode,
        area: UiTreeRenderArea,
    ) -> Vec<UiTreeHostActionHit> {
        self.host.host_action_hits(root, area)
    }

    #[must_use]
    pub fn viewport_host_action_hits(
        &self,
        root: &UiNode,
        area: UiTreeRenderArea,
    ) -> Vec<UiTreeHostActionHit> {
        self.host.viewport_host_action_hits(root, area)
    }

    #[must_use]
    pub fn viewport_interaction_hits(
        &self,
        root: &UiNode,
        area: UiTreeRenderArea,
    ) -> (Vec<UiTreeHostActionHit>, Vec<UiTreeNodeHit>) {
        self.host.viewport_interaction_hits(root, area)
    }

    #[must_use]
    pub fn interaction_target_at(
        &self,
        root: &UiNode,
        area: UiTreeRenderArea,
        x: f32,
        y: f32,
    ) -> Option<UiTreeInteractionTarget> {
        self.host.interaction_target_at(root, area, x, y)
    }

    #[must_use]
    pub fn interaction_target_for_hits_at(
        hits: &[UiTreeHostActionHit],
        node_hits: &[UiTreeNodeHit],
        x: f32,
        y: f32,
    ) -> Option<UiTreeInteractionTarget> {
        UiTreeStorybookHost::interaction_target_for_hits_at(hits, node_hits, x, y)
    }

    #[must_use]
    pub fn host_action_hits_at(
        &self,
        root: &UiNode,
        area: UiTreeRenderArea,
        x: f32,
        y: f32,
    ) -> Vec<UiTreeHostActionHit> {
        self.host.host_action_hits_at(root, area, x, y)
    }

    #[must_use]
    pub fn document_host_action_hits(
        &self,
        root: &UiNode,
        area: UiTreeRenderArea,
    ) -> Vec<UiTreeHostActionHit> {
        self.host.document_host_action_hits(root, area)
    }

    #[must_use]
    pub fn document_node_hits(&self, root: &UiNode, area: UiTreeRenderArea) -> Vec<UiTreeNodeHit> {
        self.host.document_node_hits(root, area)
    }

    #[must_use]
    pub fn viewport_node_hits(&self, root: &UiNode, area: UiTreeRenderArea) -> Vec<UiTreeNodeHit> {
        self.host.viewport_node_hits(root, area)
    }

    #[must_use]
    pub fn hovered_node_id_at(hits: &[UiTreeNodeHit], x: f32, y: f32) -> Option<UiNodeId> {
        hits.iter()
            .filter(|hit| hit.contains_point(x, y))
            .min_by_key(|hit| hit.rect.area())
            .map(|hit| {
                hit.semantic_node_id
                    .clone()
                    .unwrap_or_else(|| hit.node_id.clone())
            })
    }

    #[must_use]
    pub fn hits_at(hits: &[UiTreeHostActionHit], x: f32, y: f32) -> Vec<UiTreeHostActionHit> {
        UiTreeStorybookHost::filter_host_action_hits_at(hits, x, y)
    }

    #[must_use]
    pub fn cursor_at(hits: &[UiTreeHostActionHit], x: f32, y: f32) -> UiCursor {
        UiTreeStorybookHost::cursor_for_host_action_hits_at(hits, x, y)
    }

    #[must_use]
    pub fn hovered_action_node_id_at(
        hits: &[UiTreeHostActionHit],
        x: f32,
        y: f32,
    ) -> Option<UiNodeId> {
        UiTreeStorybookHost::hovered_action_node_id_for_host_action_hits_at(hits, x, y)
    }

    #[must_use]
    pub fn context_menu_item_id_at(root: &UiNode, x: f32, y: f32) -> Option<String> {
        UiTreeStorybookHost::context_menu_item_id_at(root, x, y)
    }

    #[must_use]
    pub fn context_menu_host_action_at(
        root: &UiNode,
        x: f32,
        y: f32,
    ) -> Option<katana_ui_core::render_model::UiHostActionPlan> {
        UiTreeStorybookHost::context_menu_host_action_at(root, x, y)
    }

    #[must_use]
    pub fn context_menu_item_center_for_id(root: &UiNode, item_id: &str) -> Option<(f32, f32)> {
        UiTreeStorybookHost::context_menu_item_center_for_id(root, item_id)
    }
}
