use super::ops;
use super::types::MenuButtonPlacement;
use crate::layout::popover::{AnchorRect, Placement, PlacementResolver};

const OFFSET: f32 = 4.0;
const POPOVER_WIDTH: f32 = 160.0;
const POPOVER_HEIGHT: f32 = 128.0;
const VIEWPORT_WIDTH: f32 = 1024.0;
const VIEWPORT_HEIGHT: f32 = 768.0;
const CENTER_ANCHOR_X: f32 = 400.0;
const CENTER_ANCHOR_Y: f32 = 300.0;
const TRIGGER_WIDTH: f32 = 80.0;
const TRIGGER_HEIGHT: f32 = 32.0;
const TOP_Y: f32 = 168.0;
const SIDE_Y: f32 = 252.0;
const BOTTOM_Y: f32 = 336.0;
const LEFT_SIDE_X: f32 = 236.0;
const RIGHT_SIDE_X: f32 = 484.0;
const ALIGN_CENTER_X: f32 = 360.0;

fn center_anchor() -> AnchorRect {
    AnchorRect::new(
        CENTER_ANCHOR_X,
        CENTER_ANCHOR_Y,
        TRIGGER_WIDTH,
        TRIGGER_HEIGHT,
    )
}

fn origin(
    placement: MenuButtonPlacement,
    anchor: AnchorRect,
) -> (f32, f32, crate::layout::popover::Placement) {
    let resolved = PlacementResolver::resolve_origin(
        placement.as_popover_placement(),
        anchor,
        OFFSET,
        POPOVER_WIDTH,
        POPOVER_HEIGHT,
        VIEWPORT_WIDTH,
        VIEWPORT_HEIGHT,
    );
    (resolved.x, resolved.y, resolved.placement)
}

fn assert_origin(placement: MenuButtonPlacement, expected_x: f32, expected_y: f32) {
    let (actual_x, actual_y, actual_placement) = origin(placement, center_anchor());
    assert_eq!(actual_placement, placement.as_popover_placement());
    assert_eq!(actual_x, expected_x);
    assert_eq!(actual_y, expected_y);
}

#[test]
fn four_directions_resolve_to_expected_coordinates() {
    assert_origin(MenuButtonPlacement::Top, ALIGN_CENTER_X, TOP_Y);
    assert_origin(MenuButtonPlacement::Bottom, ALIGN_CENTER_X, BOTTOM_Y);
    assert_origin(MenuButtonPlacement::Left, LEFT_SIDE_X, SIDE_Y);
    assert_origin(MenuButtonPlacement::Right, RIGHT_SIDE_X, SIDE_Y);
}

#[test]
fn edge_directions_flip_to_visible_opposite_side() {
    let cases = [
        (
            MenuButtonPlacement::Left,
            AnchorRect::new(10.0, CENTER_ANCHOR_Y, TRIGGER_WIDTH, TRIGGER_HEIGHT),
            Placement::Right,
            94.0,
            SIDE_Y,
        ),
        (
            MenuButtonPlacement::Right,
            AnchorRect::new(980.0, CENTER_ANCHOR_Y, 40.0, TRIGGER_HEIGHT),
            Placement::Left,
            816.0,
            SIDE_Y,
        ),
        (
            MenuButtonPlacement::Top,
            AnchorRect::new(CENTER_ANCHOR_X, 10.0, TRIGGER_WIDTH, TRIGGER_HEIGHT),
            Placement::Bottom,
            ALIGN_CENTER_X,
            46.0,
        ),
        (
            MenuButtonPlacement::Bottom,
            AnchorRect::new(CENTER_ANCHOR_X, 700.0, TRIGGER_WIDTH, 30.0),
            Placement::Top,
            ALIGN_CENTER_X,
            568.0,
        ),
    ];

    for (placement, anchor, expected_placement, expected_x, expected_y) in cases {
        let (actual_x, actual_y, actual_placement) = origin(placement, anchor);
        assert_eq!(actual_placement, expected_placement);
        assert_eq!(actual_x, expected_x);
        assert_eq!(actual_y, expected_y);
    }
}

#[test]
fn close_gestures_match_menu_button_contract() {
    assert_eq!(ops::close_intent_for_escape(true), ops::CloseIntent::Close);
    assert_eq!(
        ops::close_intent_for_escape(false),
        ops::CloseIntent::KeepOpen
    );
    assert_eq!(
        ops::close_intent_for_trigger_press(true),
        ops::CloseIntent::Close
    );
    assert_eq!(
        ops::close_intent_for_trigger_press(false),
        ops::CloseIntent::KeepOpen
    );
    assert_eq!(
        ops::close_intent_for_outside_pointer(true),
        ops::CloseIntent::Close
    );
    assert_eq!(
        ops::close_intent_for_outside_pointer(false),
        ops::CloseIntent::KeepOpen
    );
}
