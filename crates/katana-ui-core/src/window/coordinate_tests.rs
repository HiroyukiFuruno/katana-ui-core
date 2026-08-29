use super::{WindowCanvasPoint, WindowInputNormalizer, WindowSurfacePoint, WindowSurfaceSize};

const CANVAS_WIDTH: usize = 1440;
const CANVAS_HEIGHT: usize = 920;
const RETINA_SURFACE_WIDTH: usize = 2879;
const RETINA_SURFACE_HEIGHT: usize = 1728;
const SURFACE_2X_WIDTH: usize = 2880;
const SURFACE_2X_HEIGHT: usize = 1840;
const PIXEL_CENTER_OFFSET: f32 = 0.5;

#[test]
fn scaled_surface_point_maps_back_to_canvas_point() {
    let canvas = WindowSurfaceSize::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    let surface = WindowSurfaceSize::new(2160, 1380);
    let point = WindowSurfacePoint::new(465.0, 156.0);

    assert_eq!(
        Some(WindowCanvasPoint { x: 310, y: 104 }),
        WindowInputNormalizer::canvas_point_for_surface_point(point, surface, canvas)
    );
}

#[test]
fn non_integer_scale_mapping_keeps_canvas_point_center() {
    let canvas = WindowSurfaceSize::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    let surface = WindowSurfaceSize::new(RETINA_SURFACE_WIDTH, RETINA_SURFACE_HEIGHT);
    let point =
        surface_point_for_canvas_point(WindowCanvasPoint { x: 310, y: 120 }, surface, canvas);

    assert_eq!(
        Some(WindowCanvasPoint { x: 310, y: 120 }),
        WindowInputNormalizer::canvas_point_for_surface_point(point, surface, canvas),
    );
}

#[test]
fn non_integer_scale_mapping_keeps_canvas_bottom_row_center() {
    let canvas = WindowSurfaceSize::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    let surface = WindowSurfaceSize::new(RETINA_SURFACE_WIDTH, RETINA_SURFACE_HEIGHT);
    let point = surface_bottom_right_point(surface, canvas);

    assert_eq!(
        Some(WindowCanvasPoint {
            x: CANVAS_WIDTH - 1,
            y: CANVAS_HEIGHT - 1
        }),
        WindowInputNormalizer::canvas_point_for_surface_point(point, surface, canvas),
    );
}

#[test]
fn integer_scale_mapping_keeps_logical_canvas_point() {
    let canvas = WindowSurfaceSize::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    let surface = WindowSurfaceSize::new(SURFACE_2X_WIDTH, SURFACE_2X_HEIGHT);
    let canvas_point = WindowCanvasPoint { x: 310, y: 104 };
    let surface_point = surface_point_for_canvas_point(canvas_point, surface, canvas);

    assert_eq!(
        Some(canvas_point),
        WindowInputNormalizer::canvas_point_for_surface_point(surface_point, surface, canvas),
    );
}

#[test]
fn mouse_point_and_surface_must_share_the_same_coordinate_space() {
    let canvas = WindowSurfaceSize::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    let surface = WindowSurfaceSize::new(2160, 1380);
    let surface_space_point = WindowSurfacePoint::new(465.0, 156.0);
    let scaled_mouse_point = WindowSurfacePoint::new(310.0, 104.0);

    assert_eq!(
        Some(WindowCanvasPoint { x: 310, y: 104 }),
        WindowInputNormalizer::canvas_point_for_surface_point(surface_space_point, surface, canvas)
    );
    assert_ne!(
        Some(WindowCanvasPoint { x: 310, y: 104 }),
        WindowInputNormalizer::canvas_point_for_surface_point(scaled_mouse_point, surface, canvas)
    );
}

#[test]
fn letterboxed_surface_point_removes_margin_before_mapping() {
    let canvas = WindowSurfaceSize::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    let surface = WindowSurfaceSize::new(2000, 1200);
    let point = WindowSurfacePoint::new(465.3, 135.7);

    assert_eq!(
        Some(WindowCanvasPoint { x: 310, y: 104 }),
        WindowInputNormalizer::canvas_point_for_surface_point(point, surface, canvas)
    );
}

#[test]
fn point_outside_rendered_canvas_returns_none() {
    let canvas = WindowSurfaceSize::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    let surface = WindowSurfaceSize::new(2000, 1200);
    let point = WindowSurfacePoint::new(20.0, 100.0);

    assert_eq!(
        None,
        WindowInputNormalizer::canvas_point_for_surface_point(point, surface, canvas)
    );
}

#[test]
fn zero_extent_and_vertical_letterbox_paths_are_explicit() {
    let point = WindowSurfacePoint::new(25.0, 100.0);
    assert_eq!(
        None,
        WindowInputNormalizer::canvas_point_for_surface_point(
            point,
            WindowSurfaceSize::new(0, 200),
            WindowSurfaceSize::new(200, 100),
        )
    );
    assert_eq!(
        Some(WindowCanvasPoint { x: 50, y: 50 }),
        WindowInputNormalizer::canvas_point_for_surface_point(
            point,
            WindowSurfaceSize::new(100, 200),
            WindowSurfaceSize::new(200, 100),
        )
    );
}

fn surface_point_for_canvas_point(
    point: WindowCanvasPoint,
    surface: WindowSurfaceSize,
    canvas: WindowSurfaceSize,
) -> WindowSurfacePoint {
    let rect = rendered_canvas_rect(surface, canvas);
    WindowSurfacePoint::new(
        rect.x + (point.x as f32 + PIXEL_CENTER_OFFSET) * rect.width / canvas.width as f32,
        rect.y + (point.y as f32 + PIXEL_CENTER_OFFSET) * rect.height / canvas.height as f32,
    )
}

fn surface_bottom_right_point(
    surface: WindowSurfaceSize,
    canvas: WindowSurfaceSize,
) -> WindowSurfacePoint {
    let rect = rendered_canvas_rect(surface, canvas);
    WindowSurfacePoint::new(
        rect.x + rect.width - PIXEL_CENTER_OFFSET,
        rect.y + rect.height - PIXEL_CENTER_OFFSET,
    )
}

fn rendered_canvas_rect(surface: WindowSurfaceSize, canvas: WindowSurfaceSize) -> TestCanvasRect {
    let surface_aspect = surface.width as f32 / surface.height as f32;
    let canvas_aspect = canvas.width as f32 / canvas.height as f32;
    if canvas_aspect > surface_aspect {
        let height = surface.width as f32 / canvas_aspect;
        return TestCanvasRect {
            x: 0.0,
            y: (surface.height as f32 - height) / 2.0,
            width: surface.width as f32,
            height,
        };
    }
    let width = surface.height as f32 * canvas_aspect;
    TestCanvasRect {
        x: (surface.width as f32 - width) / 2.0,
        y: 0.0,
        width,
        height: surface.height as f32,
    }
}

struct TestCanvasRect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}
