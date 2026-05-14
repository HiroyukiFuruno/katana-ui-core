use super::{AnchorRect, Placement};

pub(super) struct PlacementMath;

const CENTER_RATE: f32 = 0.5;

impl PlacementMath {
    pub(super) fn origin_for(
        placement: Placement,
        anchor: AnchorRect,
        offset: f32,
        popover_width: f32,
        popover_height: f32,
    ) -> (f32, f32) {
        match placement {
            Placement::Bottom => (
                Self::center_x(anchor, popover_width),
                anchor.y + anchor.height + offset,
            ),
            Placement::Top => (
                Self::center_x(anchor, popover_width),
                anchor.y - popover_height - offset,
            ),
            Placement::Right => (
                anchor.x + anchor.width + offset,
                Self::center_y(anchor, popover_height),
            ),
            Placement::Left => (
                anchor.x - popover_width - offset,
                Self::center_y(anchor, popover_height),
            ),
        }
    }

    pub(super) fn fits(
        placement: Placement,
        anchor: AnchorRect,
        offset: f32,
        popover_width: f32,
        popover_height: f32,
        viewport_width: f32,
        viewport_height: f32,
    ) -> bool {
        let (x, y) = Self::origin_for(placement, anchor, offset, popover_width, popover_height);
        x >= 0.0
            && y >= 0.0
            && x + popover_width <= viewport_width
            && y + popover_height <= viewport_height
    }

    pub(super) fn opposite_placement(placement: Placement) -> Placement {
        match placement {
            Placement::Top => Placement::Bottom,
            Placement::Bottom => Placement::Top,
            Placement::Left => Placement::Right,
            Placement::Right => Placement::Left,
        }
    }

    fn center_x(anchor: AnchorRect, popover_width: f32) -> f32 {
        anchor.x + ((anchor.width - popover_width) * CENTER_RATE)
    }

    fn center_y(anchor: AnchorRect, popover_height: f32) -> f32 {
        anchor.y + ((anchor.height - popover_height) * CENTER_RATE)
    }
}
