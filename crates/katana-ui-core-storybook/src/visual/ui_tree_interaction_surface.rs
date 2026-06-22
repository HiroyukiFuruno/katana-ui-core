use super::ui_tree_canvas::UiTreeCanvasRenderer;
use super::ui_tree_canvas_types::{
    UiTreeHostActionHit, UiTreeHostActionHitQuery, UiTreeRenderArea,
};
use katana_ui_core::render_model::{UiCursor, UiNode, UiNodeId};
use katana_ui_core::theme::ThemeSnapshot;

#[derive(Debug, Clone)]
pub struct UiTreeInteractionSurface {
    hits: Vec<UiTreeHostActionHit>,
}

impl UiTreeInteractionSurface {
    #[must_use]
    pub fn from_rendered_tree(root: &UiNode, area: UiTreeRenderArea, theme: ThemeSnapshot) -> Self {
        let renderer = UiTreeCanvasRenderer::new(theme);
        Self::from_hits(renderer.host_action_hit_rects(root, area))
    }

    #[must_use]
    pub fn from_hits(hits: Vec<UiTreeHostActionHit>) -> Self {
        Self { hits }
    }

    #[must_use]
    pub fn hits(&self) -> &[UiTreeHostActionHit] {
        &self.hits
    }

    pub fn hits_at(&self, x: f32, y: f32) -> impl Iterator<Item = &UiTreeHostActionHit> {
        UiTreeHostActionHitQuery::new(&self.hits).hits_at(x, y)
    }

    pub fn cloned_hits_at(&self, x: f32, y: f32) -> impl Iterator<Item = UiTreeHostActionHit> + '_ {
        self.hits_at(x, y).cloned()
    }

    #[must_use]
    pub fn cursor_at(&self, x: f32, y: f32) -> UiCursor {
        self.hits_at(x, y)
            .find_map(|hit| (hit.cursor != UiCursor::Default).then_some(hit.cursor))
            .unwrap_or(UiCursor::Default)
    }

    #[must_use]
    pub fn hovered_action_node_id_at(&self, x: f32, y: f32) -> Option<UiNodeId> {
        self.hits_at(x, y)
            .next()
            .map(|hit| hit.action.target.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::UiTreeInteractionSurface;
    use crate::visual::UiTreeHitRect;
    use katana_ui_core::render_model::{UiCursor, UiHostActionPlan, UiHostActionSpec, UiNodeId};

    #[test]
    fn interaction_surface_returns_cursor_from_rendered_hit() {
        let surface = UiTreeInteractionSurface::from_hits(vec![hit("target", 10, 20, 30, 40)]);

        assert_eq!(UiCursor::Pointer, surface.cursor_at(20.0, 30.0));
        assert_eq!(UiCursor::Default, surface.cursor_at(0.0, 0.0));
    }

    #[test]
    fn interaction_surface_returns_action_node_from_rendered_hit() {
        let surface = UiTreeInteractionSurface::from_hits(vec![hit("target", 10, 20, 30, 40)]);

        assert_eq!(
            Some(UiNodeId::new("target")),
            surface.hovered_action_node_id_at(20.0, 30.0)
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
}
