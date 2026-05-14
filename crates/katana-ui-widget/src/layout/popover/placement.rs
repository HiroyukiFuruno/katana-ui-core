use super::placement_math::PlacementMath;
use super::{AnchorRect, Placement};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlacementOrigin {
    pub x: f32,
    pub y: f32,
    pub placement: Placement,
}

pub struct PlacementResolver;

impl PlacementResolver {
    pub fn resolve_origin(
        placement: Placement,
        anchor: AnchorRect,
        offset: f32,
        popover_width: f32,
        popover_height: f32,
        viewport_width: f32,
        viewport_height: f32,
    ) -> PlacementOrigin {
        let resolved = Self::resolve_placement(
            placement,
            anchor,
            offset,
            popover_width,
            popover_height,
            viewport_width,
            viewport_height,
        );
        let (x, y) =
            PlacementMath::origin_for(resolved, anchor, offset, popover_width, popover_height);
        PlacementOrigin {
            x,
            y,
            placement: resolved,
        }
    }

    pub fn resolve_placement(
        placement: Placement,
        anchor: AnchorRect,
        offset: f32,
        popover_width: f32,
        popover_height: f32,
        viewport_width: f32,
        viewport_height: f32,
    ) -> Placement {
        Self::flip_if_needed(
            placement,
            anchor,
            offset,
            popover_width,
            popover_height,
            viewport_width,
            viewport_height,
        )
    }

    fn flip_if_needed(
        placement: Placement,
        anchor: AnchorRect,
        offset: f32,
        popover_width: f32,
        popover_height: f32,
        viewport_width: f32,
        viewport_height: f32,
    ) -> Placement {
        if PlacementMath::fits(
            placement,
            anchor,
            offset,
            popover_width,
            popover_height,
            viewport_width,
            viewport_height,
        ) {
            return placement;
        }

        let opposite = PlacementMath::opposite_placement(placement);
        if PlacementMath::fits(
            opposite,
            anchor,
            offset,
            popover_width,
            popover_height,
            viewport_width,
            viewport_height,
        ) {
            opposite
        } else {
            placement
        }
    }
}
