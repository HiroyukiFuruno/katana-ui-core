use katana_ui_core::interaction::{
    UiGestureSurface, UiSurfaceGestureCapabilities, UiSurfaceGestureCommand,
    UiSurfaceGestureController, UiSurfaceGestureInput, UiSurfaceHostEvent, UiSurfacePoint,
};
use katana_ui_core::render_model::{UiRect, UiStateId};

const SURFACE: &str = "storybook-generic-surface";

fn point(x: i32, y: i32) -> UiSurfacePoint {
    UiSurfacePoint::new(x, y)
}

#[test]
fn normal_surface_drag_and_pinch_use_kuc_hit_coordinates_without_scroll_capture() {
    let capabilities = UiSurfaceGestureCapabilities::default()
        .pointer_pan(true)
        .zoom(true);
    let mut controller = UiSurfaceGestureController::new([UiGestureSurface::new(
        SURFACE,
        UiRect::new(120, 80, 640, 360),
    )
    .capabilities(capabilities)]);

    assert!(
        controller
            .apply(UiSurfaceGestureInput::PointerDown {
                pointer_id: 1,
                position: point(200, 160),
            })
            .captured
    );
    assert_eq!(
        controller
            .apply(UiSurfaceGestureInput::PointerMove {
                pointer_id: 1,
                position: point(224, 172),
            })
            .command,
        Some(UiSurfaceGestureCommand::PanBy {
            delta_x: 24.0,
            delta_y: 12.0,
        })
    );
    assert_eq!(
        controller
            .apply(UiSurfaceGestureInput::Zoom {
                multiplier: 1.25,
                center: point(240, 180),
            })
            .command,
        Some(UiSurfaceGestureCommand::ZoomBy {
            multiplier: 1.25,
            center: point(240, 180),
        })
    );
    assert!(
        !controller
            .apply(UiSurfaceGestureInput::SmoothScroll {
                position: point(240, 180),
                delta_x: 0.0,
                delta_y: 18.0,
            })
            .captured,
        "normal document scrolling remains available unless explicitly captured"
    );
}

#[test]
fn fullscreen_surface_captures_smooth_pan_zoom_and_emits_typed_host_state() {
    let capabilities = UiSurfaceGestureCapabilities::default()
        .pointer_pan(true)
        .smooth_scroll_pan(true)
        .zoom(true)
        .fullscreen(true);
    let mut controller = UiSurfaceGestureController::new([UiGestureSurface::new(
        SURFACE,
        UiRect::new(0, 0, 1280, 720),
    )
    .capabilities(capabilities)
    .fullscreen(true)]);

    assert_eq!(
        controller
            .apply(UiSurfaceGestureInput::SmoothScroll {
                position: point(640, 360),
                delta_x: -8.0,
                delta_y: 12.0,
            })
            .command,
        Some(UiSurfaceGestureCommand::PanBy {
            delta_x: -8.0,
            delta_y: 12.0,
        })
    );
    assert!(
        controller
            .apply(UiSurfaceGestureInput::Zoom {
                multiplier: 0.8,
                center: point(640, 360),
            })
            .captured
    );
    assert_eq!(
        controller.set_fullscreen(&UiStateId::new(SURFACE), false),
        Some(UiSurfaceHostEvent::FullscreenChanged {
            target: UiStateId::new(SURFACE),
            fullscreen: false,
        })
    );
}
