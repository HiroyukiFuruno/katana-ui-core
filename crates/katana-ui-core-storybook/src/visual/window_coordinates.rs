use katana_ui_core::window::WindowInputNormalizer;
pub(super) use katana_ui_core::window::{
    WindowCanvasPoint as CanvasPoint, WindowSurfacePoint as WindowPoint,
    WindowSurfaceSize as SurfaceSize,
};

pub(super) fn window_point_to_canvas_point(
    point: WindowPoint,
    window: SurfaceSize,
    canvas: SurfaceSize,
) -> Option<CanvasPoint> {
    WindowInputNormalizer::canvas_point_for_surface_point(point, window, canvas)
}
