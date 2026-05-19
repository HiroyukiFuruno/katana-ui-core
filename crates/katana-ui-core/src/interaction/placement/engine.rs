use super::{Placement, PlacementRequest, PlacementResult, Point, Rect};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlacementEngine;

impl PlacementEngine {
    #[must_use]
    pub fn resolve(request: &PlacementRequest) -> PlacementResult {
        resolve_placement(request)
    }
}

fn resolve_placement(request: &PlacementRequest) -> PlacementResult {
    let anchor = request.anchor.rect();
    let placement = choose_placement(request, anchor);
    let raw_position = position_for(anchor, placement, request);
    let position = clamp_position(raw_position, request);

    PlacementResult {
        placement_used: placement,
        position,
        arrow_offset: arrow_offset(anchor, placement, position, request),
        clamped: raw_position != position,
    }
}

fn choose_placement(request: &PlacementRequest, anchor: Rect) -> Placement {
    placement_priority(request)
        .into_iter()
        .find(|placement| fits(position_for(anchor, *placement, request), request))
        .unwrap_or(request.preferred)
}

fn placement_priority(request: &PlacementRequest) -> Vec<Placement> {
    let mut candidates = if request.priority.is_empty() {
        vec![request.preferred, request.preferred.flipped()]
    } else {
        request.priority.clone()
    };
    if !candidates.contains(&request.preferred) {
        candidates.insert(0, request.preferred);
    }
    candidates
}

fn position_for(anchor: Rect, placement: Placement, request: &PlacementRequest) -> Point {
    let width = request.panel_size.width as i32;
    let height = request.panel_size.height as i32;
    let offset = request.offset;
    match placement {
        Placement::Top => Point::new(anchor.center_x() - width / 2, anchor.y - height - offset),
        Placement::TopStart => Point::new(anchor.x, anchor.y - height - offset),
        Placement::TopEnd => Point::new(anchor.right() - width, anchor.y - height - offset),
        Placement::Bottom => Point::new(anchor.center_x() - width / 2, anchor.bottom() + offset),
        Placement::BottomStart => Point::new(anchor.x, anchor.bottom() + offset),
        Placement::BottomEnd => Point::new(anchor.right() - width, anchor.bottom() + offset),
        Placement::Right => Point::new(anchor.right() + offset, anchor.center_y() - height / 2),
        Placement::RightStart => Point::new(anchor.right() + offset, anchor.y),
        Placement::RightEnd => Point::new(anchor.right() + offset, anchor.bottom() - height),
        Placement::Left => Point::new(anchor.x - width - offset, anchor.center_y() - height / 2),
        Placement::LeftStart => Point::new(anchor.x - width - offset, anchor.y),
        Placement::LeftEnd => Point::new(anchor.x - width - offset, anchor.bottom() - height),
    }
}

fn fits(position: Point, request: &PlacementRequest) -> bool {
    let margin = request.clamp_margin;
    position.x >= request.viewport.x + margin
        && position.y >= request.viewport.y + margin
        && position.x + request.panel_size.width as i32 <= request.viewport.right() - margin
        && position.y + request.panel_size.height as i32 <= request.viewport.bottom() - margin
}

fn clamp_position(position: Point, request: &PlacementRequest) -> Point {
    let margin = request.clamp_margin;
    Point::new(
        clamp_axis(
            position.x,
            request.viewport.x + margin,
            request.viewport.right() - request.panel_size.width as i32 - margin,
        ),
        clamp_axis(
            position.y,
            request.viewport.y + margin,
            request.viewport.bottom() - request.panel_size.height as i32 - margin,
        ),
    )
}

fn arrow_offset(
    anchor: Rect,
    placement: Placement,
    position: Point,
    request: &PlacementRequest,
) -> Option<i32> {
    request.arrow_size?;
    let raw = if placement.is_vertical() {
        anchor.center_x() - position.x
    } else {
        anchor.center_y() - position.y
    };
    let axis = if placement.is_vertical() {
        request.panel_size.width as i32
    } else {
        request.panel_size.height as i32
    };
    Some(clamp_axis(
        raw,
        request.clamp_margin,
        axis - request.clamp_margin,
    ))
}

fn clamp_axis(value: i32, minimum: i32, maximum: i32) -> i32 {
    if minimum > maximum {
        return minimum;
    }
    value.clamp(minimum, maximum)
}
