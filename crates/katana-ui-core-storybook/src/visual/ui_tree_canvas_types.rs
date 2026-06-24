use katana_ui_core::render_model::{UiCursor, UiHostActionPlan, UiNodeId};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiTreeRenderArea {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub scroll_y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RgbaBlitRequest<'a> {
    pub rgba: &'a [u8],
    pub width: u32,
    pub height: u32,
    pub source: RgbaSourceRect,
    pub area: UiTreeRenderArea,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RgbaSourceRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl RgbaSourceRect {
    #[must_use]
    pub fn full(width: u32, height: u32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: width as f32,
            height: height as f32,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanvasBlitRequest {
    pub dest_x: usize,
    pub dest_y: usize,
    pub width: usize,
    pub height: usize,
    pub source_y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiTreeHitRect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl UiTreeHitRect {
    #[must_use]
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        let left = self.x as f32;
        let top = self.y as f32;
        let right = left + self.width as f32;
        let bottom = top + self.height as f32;
        x >= left && x < right && y >= top && y < bottom
    }

    #[must_use]
    pub fn center_point(&self) -> (f32, f32) {
        (
            self.x as f32 + self.width as f32 / 2.0,
            self.y as f32 + self.height as f32 / 2.0,
        )
    }

    #[must_use]
    pub fn area(&self) -> usize {
        self.width.saturating_mul(self.height)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UiTreeHostActionHit {
    pub action: UiHostActionPlan,
    pub rect: UiTreeHitRect,
    pub cursor: UiCursor,
}

impl UiTreeHostActionHit {
    #[must_use]
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        self.rect.contains_point(x, y)
    }

    #[must_use]
    pub fn center_point(&self) -> (f32, f32) {
        self.rect.center_point()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UiTreeNodeHit {
    pub node_id: UiNodeId,
    pub semantic_node_id: Option<UiNodeId>,
    pub rect: UiTreeHitRect,
    pub cursor: UiCursor,
}

impl UiTreeNodeHit {
    #[must_use]
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        self.rect.contains_point(x, y)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UiTreeInteractionTarget {
    pub node_id: UiNodeId,
    pub semantic_node_id: Option<UiNodeId>,
    pub action: Option<UiHostActionPlan>,
    pub rect: UiTreeHitRect,
    pub cursor: UiCursor,
}

impl UiTreeInteractionTarget {
    #[must_use]
    pub fn from_hits_at(
        hits: &[UiTreeHostActionHit],
        node_hits: &[UiTreeNodeHit],
        x: f32,
        y: f32,
    ) -> Option<Self> {
        UiTreeHostActionHitQuery::new(hits)
            .cloned_hits_at(x, y)
            .next()
            .map(Self::from_action_hit)
            .or_else(|| {
                node_hits
                    .iter()
                    .filter(|hit| hit.contains_point(x, y))
                    .min_by_key(|hit| hit.rect.area())
                    .cloned()
                    .map(Self::from_node_hit)
            })
    }

    #[must_use]
    pub fn from_action_hit(hit: UiTreeHostActionHit) -> Self {
        Self {
            node_id: hit.action.target.clone(),
            semantic_node_id: None,
            action: Some(hit.action),
            rect: hit.rect,
            cursor: hit.cursor,
        }
    }

    #[must_use]
    pub fn from_node_hit(hit: UiTreeNodeHit) -> Self {
        Self {
            node_id: hit.node_id,
            semantic_node_id: hit.semantic_node_id,
            action: None,
            rect: hit.rect,
            cursor: hit.cursor,
        }
    }

    #[must_use]
    pub fn hover_node_id(&self) -> UiNodeId {
        self.semantic_node_id
            .clone()
            .unwrap_or_else(|| self.node_id.clone())
    }

    #[must_use]
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        self.rect.contains_point(x, y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UiTreeHostActionHitQuery<'a> {
    hits: &'a [UiTreeHostActionHit],
}

impl<'a> UiTreeHostActionHitQuery<'a> {
    #[must_use]
    pub fn new(hits: &'a [UiTreeHostActionHit]) -> Self {
        Self { hits }
    }

    pub fn hits_at(self, x: f32, y: f32) -> impl Iterator<Item = &'a UiTreeHostActionHit> {
        let mut hits = self
            .hits
            .iter()
            .filter(move |hit| hit.contains_point(x, y))
            .collect::<Vec<_>>();
        hits.sort_by_key(|hit| (hit_priority(hit), hit.rect.area()));
        hits.into_iter()
    }

    pub fn cloned_hits_at(self, x: f32, y: f32) -> impl Iterator<Item = UiTreeHostActionHit> + 'a {
        self.hits_at(x, y).cloned()
    }
}

fn hit_priority(hit: &UiTreeHostActionHit) -> usize {
    let Some(target) = hit.action.settings_field_control_target() else {
        return 1;
    };
    let row_target = format!("settings-field:{}", target.field_id);
    if hit.action.target.as_str() == row_target {
        return 0;
    }
    1
}

#[cfg(test)]
mod tests {
    use super::{UiTreeHitRect, UiTreeHostActionHit, UiTreeHostActionHitQuery};
    use katana_ui_core::render_model::{UiCursor, UiHostActionPlan, UiHostActionSpec, UiNodeId};

    #[test]
    fn hit_rect_contains_point_uses_rendered_bounds_contract() {
        let rect = UiTreeHitRect {
            x: 10,
            y: 20,
            width: 30,
            height: 40,
        };

        assert!(rect.contains_point(10.0, 20.0));
        assert!(rect.contains_point(39.9, 59.9));
        assert!(!rect.contains_point(9.9, 20.0));
        assert!(!rect.contains_point(40.0, 60.0));
    }

    #[test]
    fn hit_rect_center_point_uses_rendered_bounds_contract() {
        let rect = UiTreeHitRect {
            x: 10,
            y: 20,
            width: 30,
            height: 40,
        };

        assert_eq!((25.0, 40.0), rect.center_point());
    }

    #[test]
    fn host_action_query_filters_hits_by_rendered_bounds_contract() {
        let hits = vec![
            host_action_hit("outside", 0, 0, 10, 10),
            host_action_hit("inside", 20, 20, 10, 10),
        ];

        let ids = UiTreeHostActionHitQuery::new(&hits)
            .hits_at(25.0, 25.0)
            .map(|hit| hit.action.target.as_str())
            .collect::<Vec<_>>();

        assert_eq!(vec!["inside"], ids);
    }

    #[test]
    fn host_action_query_prioritizes_specific_nested_hit_rect() {
        let hits = vec![
            host_action_hit("row", 0, 0, 200, 32),
            host_action_hit("checkbox", 8, 8, 16, 16),
        ];

        let ids = UiTreeHostActionHitQuery::new(&hits)
            .hits_at(12.0, 12.0)
            .map(|hit| hit.action.target.as_str())
            .collect::<Vec<_>>();

        assert_eq!(vec!["checkbox", "row"], ids);
    }

    fn host_action_hit(
        target: &str,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> UiTreeHostActionHit {
        UiTreeHostActionHit {
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
