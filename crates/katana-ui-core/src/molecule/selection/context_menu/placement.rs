use super::types::{ContextMenuAnchor, ContextMenuPlacement, ContextMenuRect};
use crate::interaction::placement::{
    AnchorKind, Placement, PlacementConsumer, PlacementEngine, PlacementRequest, Point, Rect, Size,
};
use crate::render_model::UiNodeId;
use serde::{Deserialize, Serialize};

const DEFAULT_VIEWPORT_MARGIN: u32 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextMenuSize {
    pub width: u32,
    pub height: u32,
}

impl ContextMenuSize {
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextMenuViewport {
    pub width: u32,
    pub height: u32,
    pub margin: u32,
}

impl ContextMenuViewport {
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            margin: DEFAULT_VIEWPORT_MARGIN,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextMenuPlacementResult {
    pub placement: ContextMenuPlacement,
    pub x: i32,
    pub y: i32,
    pub render_height: u32,
    pub scrollable: bool,
}

pub struct ContextMenuPlacementResolver;

impl ContextMenuPlacementResolver {
    #[must_use]
    pub fn resolve(
        anchor: &ContextMenuAnchor,
        menu_size: ContextMenuSize,
        viewport: ContextMenuViewport,
        priority: &[ContextMenuPlacement],
    ) -> ContextMenuPlacementResult {
        let fit_size = fit_size(menu_size, viewport);
        let effective_priority = if priority.is_empty() {
            vec![
                ContextMenuPlacement::BelowStart,
                ContextMenuPlacement::AboveStart,
            ]
        } else {
            priority.to_vec()
        };
        let preferred = effective_priority[0];
        let request = PlacementRequest::new(
            anchor_kind(anchor),
            to_common_placement(preferred),
            Size::new(fit_size.width, fit_size.height),
            Rect::new(0, 0, viewport.width, viewport.height),
        )
        .priority(effective_priority.iter().copied().map(to_common_placement))
        .clamp_margin(viewport.margin as i32);
        let result = PlacementEngine::resolve_for(PlacementConsumer::ContextMenu, &request);
        ContextMenuPlacementResult {
            placement: effective_priority
                .iter()
                .copied()
                .find(|placement| to_common_placement(*placement) == result.placement_used)
                .unwrap_or(preferred),
            x: result.position.x,
            y: result.position.y,
            render_height: fit_size.height,
            scrollable: fit_size.height < menu_size.height,
        }
    }
}

fn anchor_kind(anchor: &ContextMenuAnchor) -> AnchorKind {
    match anchor {
        ContextMenuAnchor::Pointer { x, y } => AnchorKind::pointer(Point::new(*x, *y)),
        ContextMenuAnchor::VirtualRect(rect) => AnchorKind::virtual_rect(rect_from_context(*rect)),
        ContextMenuAnchor::NodeId(id) => {
            AnchorKind::node_rect(UiNodeId::new(id.clone()), Rect::new(0, 0, 0, 0))
        }
    }
}

fn rect_from_context(rect: ContextMenuRect) -> Rect {
    Rect::new(rect.x, rect.y, rect.width, rect.height)
}

fn to_common_placement(placement: ContextMenuPlacement) -> Placement {
    match placement {
        ContextMenuPlacement::BelowStart => Placement::BottomStart,
        ContextMenuPlacement::BelowEnd => Placement::BottomEnd,
        ContextMenuPlacement::AboveStart => Placement::TopStart,
        ContextMenuPlacement::AboveEnd => Placement::TopEnd,
        ContextMenuPlacement::RightStart => Placement::RightStart,
        ContextMenuPlacement::LeftStart => Placement::LeftStart,
    }
}

fn fit_size(menu_size: ContextMenuSize, viewport: ContextMenuViewport) -> ContextMenuSize {
    let max_height = viewport.height.saturating_sub(viewport.margin * 2);
    ContextMenuSize {
        width: menu_size.width,
        height: menu_size.height.min(max_height),
    }
}
