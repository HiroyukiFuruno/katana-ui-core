use super::*;
use std::cell::Cell;

fn pan_surface() -> UiGestureSurface {
    UiGestureSurface::new("surface", UiRect::new(0, 0, 20, 20))
        .capabilities(UiSurfaceGestureCapabilities::default().pointer_pan(true))
}

#[test]
fn override_returns_unhandled_input_without_calling_host_and_keeps_default_command() {
    let mut controller = UiSurfaceGestureController::new([pan_surface()]);
    let host_called = Cell::new(false);
    let unhandled = controller.apply_with_override(
        UiSurfaceGestureInput::PointerUp {
            pointer_id: 1,
            position: UiSurfacePoint::new(30, 30),
        },
        |_| {
            host_called.set(true);
            UiSurfaceGestureOverride::UseDefault
        },
    );
    assert!(!host_called.get());
    assert!(!unhandled.captured);
    assert!(unhandled.event.is_none());

    let pressed = controller.apply_with_override(
        UiSurfaceGestureInput::PointerDown {
            pointer_id: 2,
            position: UiSurfacePoint::new(5, 5),
        },
        |_| UiSurfaceGestureOverride::UseDefault,
    );
    assert!(pressed.captured);
    assert!(pressed.event.is_some());
    assert!(pressed.command.is_none());
}
