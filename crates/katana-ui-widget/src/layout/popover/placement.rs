use super::placement_math::PlacementMath;
use super::{AnchorRect, Placement};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlacementOrigin {
    pub x: f32,
    pub y: f32,
    pub placement: Placement,
}

pub struct PlacementResolver;

const AUTO_ORDER: [Placement; 8] = [
    Placement::BottomStart,
    Placement::Bottom,
    Placement::BottomEnd,
    Placement::TopStart,
    Placement::Top,
    Placement::TopEnd,
    Placement::End,
    Placement::Start,
];
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
        match placement {
            Placement::Auto => Self::auto_placement(
                anchor,
                offset,
                popover_width,
                popover_height,
                viewport_width,
                viewport_height,
            ),
            Placement::Free(_) => placement,
            _ => Self::flip_if_needed(
                placement,
                anchor,
                offset,
                popover_width,
                popover_height,
                viewport_width,
                viewport_height,
            ),
        }
    }

    fn auto_placement(
        anchor: AnchorRect,
        offset: f32,
        popover_width: f32,
        popover_height: f32,
        viewport_width: f32,
        viewport_height: f32,
    ) -> Placement {
        AUTO_ORDER
            .into_iter()
            .find(|placement| {
                PlacementMath::fits(
                    *placement,
                    anchor,
                    offset,
                    popover_width,
                    popover_height,
                    viewport_width,
                    viewport_height,
                )
            })
            .unwrap_or(Placement::BottomStart)
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
