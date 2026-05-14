use super::PlacementResolver;
use super::types::{AnchorRect, Placement, PopoverProps};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PopoverOrigin {
    pub x: f32,
    pub y: f32,
}

pub(super) fn compute_origin(
    anchor: AnchorRect,
    placement: Placement,
    offset: f32,
    popover_width: f32,
    popover_height: f32,
    viewport_width: f32,
    viewport_height: f32,
) -> PopoverOrigin {
    let origin = PlacementResolver::resolve_origin(
        placement,
        anchor,
        offset,
        popover_width,
        popover_height,
        viewport_width,
        viewport_height,
    );
    PopoverOrigin {
        x: origin.x,
        y: origin.y,
    }
}

pub(super) fn resolve_placement(
    placement: Placement,
    anchor: AnchorRect,
    offset: f32,
    popover_width: f32,
    popover_height: f32,
    viewport_width: f32,
    viewport_height: f32,
) -> Placement {
    PlacementResolver::resolve_placement(
        placement,
        anchor,
        offset,
        popover_width,
        popover_height,
        viewport_width,
        viewport_height,
    )
}

pub(super) fn should_dismiss_on_outside_click(props: &PopoverProps) -> bool {
    props.open && props.dismiss_on_outside_click
}

pub(super) fn should_dismiss_on_esc(props: &PopoverProps) -> bool {
    props.open && props.dismiss_on_esc
}
