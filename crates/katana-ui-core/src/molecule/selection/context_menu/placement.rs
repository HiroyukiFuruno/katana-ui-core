use super::types::{ContextMenuAnchor, ContextMenuPlacement, ContextMenuRect};
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
        let anchor_rect = anchor_rect(anchor);
        for placement in priority {
            let candidate = candidate_for(anchor_rect, menu_size, *placement);
            if fits(candidate, menu_size, viewport) {
                return candidate;
            }
        }
        clamp(
            candidate_for(anchor_rect, menu_size, ContextMenuPlacement::BelowStart),
            menu_size,
            viewport,
        )
    }
}

fn anchor_rect(anchor: &ContextMenuAnchor) -> ContextMenuRect {
    match anchor {
        ContextMenuAnchor::Pointer { x, y } => ContextMenuRect::new(*x, *y, 0, 0),
        ContextMenuAnchor::VirtualRect(rect) => *rect,
        ContextMenuAnchor::NodeId(_) => ContextMenuRect::new(0, 0, 0, 0),
    }
}

fn candidate_for(
    anchor: ContextMenuRect,
    menu_size: ContextMenuSize,
    placement: ContextMenuPlacement,
) -> ContextMenuPlacementResult {
    let anchor_right = anchor.x + anchor.width as i32;
    let anchor_bottom = anchor.y + anchor.height as i32;
    let menu_width = menu_size.width as i32;
    let menu_height = menu_size.height as i32;
    let (x, y) = match placement {
        ContextMenuPlacement::BelowStart => (anchor.x, anchor_bottom),
        ContextMenuPlacement::BelowEnd => (anchor_right - menu_width, anchor_bottom),
        ContextMenuPlacement::AboveStart => (anchor.x, anchor.y - menu_height),
        ContextMenuPlacement::AboveEnd => (anchor_right - menu_width, anchor.y - menu_height),
        ContextMenuPlacement::RightStart => (anchor_right, anchor.y),
        ContextMenuPlacement::LeftStart => (anchor.x - menu_width, anchor.y),
    };
    ContextMenuPlacementResult { placement, x, y }
}

fn fits(
    result: ContextMenuPlacementResult,
    menu_size: ContextMenuSize,
    viewport: ContextMenuViewport,
) -> bool {
    let margin = viewport.margin as i32;
    let right = result.x + menu_size.width as i32;
    let bottom = result.y + menu_size.height as i32;
    result.x >= margin
        && result.y >= margin
        && right <= viewport.width as i32 - margin
        && bottom <= viewport.height as i32 - margin
}

fn clamp(
    result: ContextMenuPlacementResult,
    menu_size: ContextMenuSize,
    viewport: ContextMenuViewport,
) -> ContextMenuPlacementResult {
    let margin = viewport.margin as i32;
    let max_x = viewport.width as i32 - margin - menu_size.width as i32;
    let max_y = viewport.height as i32 - margin - menu_size.height as i32;
    ContextMenuPlacementResult {
        placement: result.placement,
        x: result.x.clamp(margin, max_x.max(margin)),
        y: result.y.clamp(margin, max_y.max(margin)),
    }
}
