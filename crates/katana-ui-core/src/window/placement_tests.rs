use super::{
    DisplayBounds, ModalWindowPlacement, ModalWindowPlacementError, WindowId, WindowPoint,
    WindowRect, WindowSize,
};

const DISPLAY_WIDTH: f32 = 1440.0;
const DISPLAY_HEIGHT: f32 = 900.0;
const DISPLAY_SCALE: f32 = 2.0;
const PARENT_X: f32 = 100.0;
const PARENT_Y: f32 = 120.0;
const PARENT_OUTSIDE_X: f32 = 2000.0;
const PARENT_WIDTH: f32 = 800.0;
const PARENT_HEIGHT: f32 = 600.0;
const MODAL_WIDTH: f32 = 360.0;
const MODAL_HEIGHT: f32 = 240.0;
const EXPECTED_MODAL_X: f32 = 320.0;
const EXPECTED_MODAL_Y: f32 = 300.0;

#[test]
fn modal_same_display_plan_is_frontmost_and_inside_display() {
    let placement = ModalWindowPlacement::same_display(
        WindowId::new("parent"),
        WindowId::new("modal"),
        WindowRect::new(
            WindowPoint::new(PARENT_X, PARENT_Y),
            WindowSize::new(PARENT_WIDTH, PARENT_HEIGHT),
        ),
        WindowSize::new(MODAL_WIDTH, MODAL_HEIGHT),
        main_display(),
    );

    let resolved = placement.resolve();
    assert!(resolved.is_ok());
    let Ok(plan) = resolved else {
        return;
    };

    assert!(plan.same_display);
    assert!(plan.frontmost);
    assert_eq!(
        WindowPoint::new(EXPECTED_MODAL_X, EXPECTED_MODAL_Y),
        plan.position
    );
}

#[test]
fn modal_same_display_plan_fails_when_parent_is_not_on_display() {
    let placement = ModalWindowPlacement::same_display(
        WindowId::new("parent"),
        WindowId::new("modal"),
        WindowRect::new(
            WindowPoint::new(PARENT_OUTSIDE_X, PARENT_Y),
            WindowSize::new(PARENT_WIDTH, PARENT_HEIGHT),
        ),
        WindowSize::new(MODAL_WIDTH, MODAL_HEIGHT),
        main_display(),
    );

    let resolved = placement.resolve();
    assert!(resolved.is_err());
    let Err(error) = resolved else {
        return;
    };

    assert_eq!(ModalWindowPlacementError::ParentOutsideDisplay, error);
}

#[test]
fn modal_same_display_rejects_oversized_and_non_finite_modal_geometry() {
    let parent = WindowRect::new(
        WindowPoint::new(PARENT_X, PARENT_Y),
        WindowSize::new(PARENT_WIDTH, PARENT_HEIGHT),
    );
    let oversized = ModalWindowPlacement::same_display(
        WindowId::new("parent"),
        WindowId::new("oversized"),
        parent,
        WindowSize::new(DISPLAY_WIDTH + 1.0, MODAL_HEIGHT),
        main_display(),
    );
    assert_eq!(
        Err(ModalWindowPlacementError::ModalLargerThanDisplay),
        oversized.resolve()
    );

    let non_finite = ModalWindowPlacement::same_display(
        WindowId::new("parent"),
        WindowId::new("non-finite"),
        parent,
        WindowSize::new(f32::NAN, MODAL_HEIGHT),
        main_display(),
    );
    assert_eq!(
        Err(ModalWindowPlacementError::ModalOutsideDisplay),
        non_finite.resolve()
    );
}

fn main_display() -> DisplayBounds {
    DisplayBounds::new(
        "main",
        WindowRect::new(
            WindowPoint::new(0.0, 0.0),
            WindowSize::new(DISPLAY_WIDTH, DISPLAY_HEIGHT),
        ),
        DISPLAY_SCALE,
    )
}
