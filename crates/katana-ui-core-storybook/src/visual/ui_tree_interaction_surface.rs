use super::ui_tree_canvas::UiTreeCanvasRenderer;
use super::ui_tree_canvas_types::{
    UiTreeHostActionHit, UiTreeHostActionHitQuery, UiTreeInteractionTarget, UiTreeNodeHit,
    UiTreeRenderArea,
};
use katana_ui_core::render_model::{UiCursor, UiNode, UiNodeId};
use katana_ui_core::theme::ThemeSnapshot;

#[derive(Debug, Clone)]
pub struct UiTreeInteractionSurface {
    hits: Vec<UiTreeHostActionHit>,
    node_hits: Vec<UiTreeNodeHit>,
}

impl UiTreeInteractionSurface {
    #[must_use]
    pub fn from_rendered_tree(root: &UiNode, area: UiTreeRenderArea, theme: ThemeSnapshot) -> Self {
        let renderer = UiTreeCanvasRenderer::new(theme);
        let (hits, node_hits) = renderer.viewport_interaction_hit_rects(root, area);
        Self::from_interaction_hits(hits, node_hits)
    }

    #[must_use]
    pub fn from_hits(hits: Vec<UiTreeHostActionHit>) -> Self {
        Self::from_interaction_hits(hits, Vec::new())
    }

    #[must_use]
    pub fn from_interaction_hits(
        hits: Vec<UiTreeHostActionHit>,
        node_hits: Vec<UiTreeNodeHit>,
    ) -> Self {
        Self { hits, node_hits }
    }

    #[must_use]
    pub fn hits(&self) -> &[UiTreeHostActionHit] {
        &self.hits
    }

    #[must_use]
    pub fn node_hits(&self) -> &[UiTreeNodeHit] {
        &self.node_hits
    }

    pub fn hits_at(&self, x: f32, y: f32) -> impl Iterator<Item = &UiTreeHostActionHit> {
        UiTreeHostActionHitQuery::new(&self.hits).hits_at(x, y)
    }

    pub fn cloned_hits_at(&self, x: f32, y: f32) -> impl Iterator<Item = UiTreeHostActionHit> + '_ {
        self.hits_at(x, y).cloned()
    }

    #[must_use]
    pub fn target_at(&self, x: f32, y: f32) -> Option<UiTreeInteractionTarget> {
        UiTreeInteractionTarget::from_hits_at(&self.hits, &self.node_hits, x, y)
    }

    #[must_use]
    pub fn cursor_at(&self, x: f32, y: f32) -> UiCursor {
        self.target_at(x, y)
            .and_then(|target| (target.cursor != UiCursor::Default).then_some(target.cursor))
            .unwrap_or(UiCursor::Default)
    }

    #[must_use]
    pub fn hovered_action_node_id_at(&self, x: f32, y: f32) -> Option<UiNodeId> {
        self.hits_at(x, y)
            .next()
            .map(|hit| hit.action.target.clone())
    }

    #[must_use]
    pub fn hovered_node_id_at(&self, x: f32, y: f32) -> Option<UiNodeId> {
        self.target_at(x, y).map(|target| target.hover_node_id())
    }
}

#[cfg(test)]
mod tests {
    use super::UiTreeInteractionSurface;
    use crate::test_assert::KucTestExpect;
    use crate::visual::{UiTreeHitRect, UiTreeNodeHit};
    use katana_ui_core::atom::Toggle;
    use katana_ui_core::render_model::{UiCursor, UiHostActionPlan, UiHostActionSpec, UiNodeId};
    use katana_ui_core::render_model::{UiHostActionSpec as HostAction, UiNode};
    use katana_ui_core::theme::ThemeSnapshot;

    #[test]
    fn interaction_surface_returns_cursor_from_rendered_hit() {
        let surface = UiTreeInteractionSurface::from_hits(vec![hit("target", 10, 20, 30, 40)]);

        assert_eq!(UiCursor::Pointer, surface.cursor_at(20.0, 30.0));
        assert_eq!(UiCursor::Default, surface.cursor_at(0.0, 0.0));
    }

    #[test]
    fn interaction_surface_public_accessors_share_rendered_tree_hits() {
        let root = UiNode::from(Toggle::new("Enabled").checked(true))
            .host_action(HostAction::command("toggle.enabled", "Toggle enabled"));
        let surface = UiTreeInteractionSurface::from_rendered_tree(
            &root,
            crate::visual::UiTreeRenderArea {
                x: 0,
                y: 0,
                width: 160,
                height: 48,
                scroll_y: 0.0,
            },
            ThemeSnapshot::dark(),
        );

        assert!(!surface.hits().is_empty());
        assert!(!surface.node_hits().is_empty());
        let hit = surface.hits()[0].clone();
        let (x, y) = hit.center_point();
        assert_eq!(1, surface.cloned_hits_at(x, y).count());
        assert!(surface.hovered_node_id_at(x, y).is_some());
    }

    #[test]
    fn interaction_surface_returns_action_node_from_rendered_hit() {
        let surface = UiTreeInteractionSurface::from_hits(vec![hit("target", 10, 20, 30, 40)]);

        assert_eq!(
            Some(UiNodeId::new("target")),
            surface.hovered_action_node_id_at(20.0, 30.0)
        );
    }

    #[test]
    fn interaction_surface_returns_node_target_when_no_action_hit_exists() {
        let surface = UiTreeInteractionSurface::from_interaction_hits(
            Vec::new(),
            vec![node_hit(
                "generated-text",
                Some("viewer-block"),
                10,
                20,
                30,
                40,
            )],
        );
        let target = surface
            .target_at(20.0, 30.0)
            .kuc_expect("node interaction target");

        assert!(target.action.is_none());
        assert_eq!(UiNodeId::new("generated-text"), target.node_id);
        assert_eq!(UiNodeId::new("viewer-block"), target.hover_node_id());
        assert_eq!(
            Some(UiNodeId::new("viewer-block")),
            surface.hovered_node_id_at(20.0, 30.0)
        );
    }

    #[test]
    fn interaction_surface_prefers_action_target_over_overlapping_node_hit() {
        let surface = UiTreeInteractionSurface::from_interaction_hits(
            vec![hit("action-target", 10, 20, 30, 40)],
            vec![node_hit(
                "generated-text",
                Some("viewer-block"),
                10,
                20,
                30,
                40,
            )],
        );
        let target = surface
            .target_at(20.0, 30.0)
            .kuc_expect("action interaction target");

        assert!(target.action.is_some());
        assert_eq!(UiNodeId::new("action-target"), target.node_id);
        assert_eq!(UiCursor::Pointer, target.cursor);
        assert_eq!(
            Some(UiNodeId::new("action-target")),
            surface.hovered_node_id_at(20.0, 30.0)
        );
    }

    fn hit(
        target: &str,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> crate::visual::UiTreeHostActionHit {
        crate::visual::UiTreeHostActionHit {
            action: UiHostActionPlan::new(
                UiNodeId::new(target),
                UiHostActionSpec::command("test", "Test"),
            ),
            rect: UiTreeHitRect {
                x,
                y,
                width,
                height,
            },
            cursor: UiCursor::Pointer,
        }
    }

    fn node_hit(
        node_id: &str,
        semantic_node_id: Option<&str>,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> UiTreeNodeHit {
        UiTreeNodeHit {
            node_id: UiNodeId::new(node_id),
            semantic_node_id: semantic_node_id.map(UiNodeId::new),
            rect: UiTreeHitRect {
                x,
                y,
                width,
                height,
            },
            cursor: UiCursor::Default,
        }
    }
}
