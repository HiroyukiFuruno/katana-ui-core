use super::raster::{RasterFrame, layout_for_surface};
use crate::render_model::UiRect;
use crate::text_surface::{
    TextSurface, TextSurfaceEvent, TextSurfacePoint, TextSurfaceScrollRequestResult,
};

pub(super) fn synchronize_scroll_bounds(
    surface: &mut TextSurface,
    raster_frame: &RasterFrame,
    viewport_bounds: UiRect,
) {
    let layout = layout_for_surface(raster_frame, surface, TextSurfacePoint::new(0, 0));
    surface.synchronize_scroll_bounds(layout.content_bounds, viewport_bounds);
}

pub(super) fn synchronize_scroll_request(
    surface: &mut TextSurface,
    raster_frame: &RasterFrame,
    viewport_bounds: UiRect,
    scale_factor: f32,
) -> Option<TextSurfaceScrollRequestResult> {
    synchronize_scroll_bounds(surface, raster_frame, viewport_bounds);
    let layout = layout_for_surface(
        raster_frame,
        surface,
        TextSurfacePoint::new(
            viewport_bounds.x.saturating_sub(surface.state().scroll_x),
            viewport_bounds.y.saturating_sub(surface.state().scroll_y),
        ),
    );
    surface.resolve_controlled_scroll_request_with_scale(&layout, viewport_bounds, scale_factor)
}

pub(super) fn scroll_request_event(value: &TextSurfaceScrollRequestResult) -> TextSurfaceEvent {
    match value {
        TextSurfaceScrollRequestResult::Acknowledged(value) => {
            TextSurfaceEvent::ScrollRequestAcknowledged(value.clone())
        }
        TextSurfaceScrollRequestResult::Rejected { token, reason } => {
            TextSurfaceEvent::ScrollRequestRejected {
                token: token.clone(),
                reason: *reason,
            }
        }
    }
}
