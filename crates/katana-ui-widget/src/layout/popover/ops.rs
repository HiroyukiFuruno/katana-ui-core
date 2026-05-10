use super::types::{AnchorRect, Placement};

/// Resolved position for the popover origin (top-left corner).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PopoverOrigin {
    pub x: f32,
    pub y: f32,
}

/// Computes the popover origin given anchor rect, placement, offset, and popover size.
/// Automatically flips placement when the popover would exceed viewport bounds.
pub(super) fn compute_origin(
    anchor: AnchorRect,
    placement: Placement,
    offset: f32,
    popover_width: f32,
    popover_height: f32,
    viewport_width: f32,
    viewport_height: f32,
) -> PopoverOrigin {
    let effective = flip_if_needed(
        placement,
        anchor,
        offset,
        popover_width,
        popover_height,
        viewport_width,
        viewport_height,
    );
    origin_for(effective, anchor, offset)
}

fn origin_for(placement: Placement, anchor: AnchorRect, offset: f32) -> PopoverOrigin {
    match placement {
        Placement::Bottom => PopoverOrigin {
            x: anchor.x,
            y: anchor.y + anchor.height + offset,
        },
        Placement::Top => PopoverOrigin {
            x: anchor.x,
            y: anchor.y - offset,
        },
        Placement::End => PopoverOrigin {
            x: anchor.x + anchor.width + offset,
            y: anchor.y,
        },
        Placement::Start => PopoverOrigin {
            x: anchor.x - offset,
            y: anchor.y,
        },
    }
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
    match placement {
        Placement::Bottom => {
            let bottom_edge = anchor.y + anchor.height + offset + popover_height;
            if bottom_edge > viewport_height && anchor.y - offset - popover_height >= 0.0 {
                Placement::Top
            } else {
                Placement::Bottom
            }
        }
        Placement::Top => {
            let top_edge = anchor.y - offset - popover_height;
            if top_edge < 0.0
                && anchor.y + anchor.height + offset + popover_height <= viewport_height
            {
                Placement::Bottom
            } else {
                Placement::Top
            }
        }
        Placement::End => {
            let right_edge = anchor.x + anchor.width + offset + popover_width;
            if right_edge > viewport_width && anchor.x - offset - popover_width >= 0.0 {
                Placement::Start
            } else {
                Placement::End
            }
        }
        Placement::Start => {
            let left_edge = anchor.x - offset - popover_width;
            if left_edge < 0.0 && anchor.x + anchor.width + offset + popover_width <= viewport_width
            {
                Placement::End
            } else {
                Placement::Start
            }
        }
    }
}

/// Returns whether an outside click should close the popover.
pub(super) fn should_dismiss_on_outside_click(dismiss: bool) -> bool {
    dismiss
}

/// Returns whether an Esc key press should close the popover.
pub(super) fn should_dismiss_on_esc(dismiss: bool) -> bool {
    dismiss
}

#[cfg(test)]
mod tests {
    use super::*;

    fn anchor() -> AnchorRect {
        AnchorRect {
            x: 100.0,
            y: 100.0,
            width: 80.0,
            height: 32.0,
        }
    }

    #[test]
    fn bottom_placement_positions_below_anchor() {
        let o = compute_origin(anchor(), Placement::Bottom, 4.0, 120.0, 80.0, 800.0, 600.0);
        assert!((o.y - (100.0 + 32.0 + 4.0)).abs() < f32::EPSILON);
        assert!((o.x - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn top_placement_positions_above_anchor() {
        let o = compute_origin(anchor(), Placement::Top, 4.0, 120.0, 80.0, 800.0, 600.0);
        assert!((o.y - (100.0 - 4.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn end_placement_positions_right_of_anchor() {
        let o = compute_origin(anchor(), Placement::End, 4.0, 120.0, 80.0, 800.0, 600.0);
        assert!((o.x - (100.0 + 80.0 + 4.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn start_placement_positions_left_of_anchor() {
        let a = AnchorRect {
            x: 300.0,
            y: 100.0,
            width: 80.0,
            height: 32.0,
        };
        let o = compute_origin(a, Placement::Start, 4.0, 120.0, 80.0, 800.0, 600.0);
        assert!((o.x - (300.0 - 4.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn flips_bottom_to_top_near_bottom_edge() {
        let a = AnchorRect {
            x: 100.0,
            y: 550.0,
            width: 80.0,
            height: 32.0,
        };
        let o = compute_origin(a, Placement::Bottom, 4.0, 120.0, 80.0, 800.0, 600.0);
        assert!(
            o.y < a.y,
            "should flip to top: origin.y={} anchor.y={}",
            o.y,
            a.y
        );
    }

    #[test]
    fn flips_end_to_start_near_right_edge() {
        let a = AnchorRect {
            x: 700.0,
            y: 100.0,
            width: 80.0,
            height: 32.0,
        };
        let o = compute_origin(a, Placement::End, 4.0, 120.0, 80.0, 800.0, 600.0);
        assert!(
            o.x < a.x,
            "should flip to start: origin.x={} anchor.x={}",
            o.x,
            a.x
        );
    }
}
