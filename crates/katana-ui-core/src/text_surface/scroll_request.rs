use super::layout_model::TextSurfaceLayout;
use super::scroll_request_types::{
    TextSurfaceLogicalPixels, TextSurfaceScrollAlignment, TextSurfaceScrollRequest,
    TextSurfaceScrollRequestRejection, TextSurfaceScrollTarget,
};
use super::state::TextSurfaceScrollBounds;
use crate::render_model::UiRect;

pub(super) fn target_bounds(
    layout: &TextSurfaceLayout,
    target: &TextSurfaceScrollTarget,
) -> Result<Option<UiRect>, TextSurfaceScrollRequestRejection> {
    match target {
        TextSurfaceScrollTarget::LogicalRow { logical_row } => layout
            .line_bounds(*logical_row)
            .map(Some)
            .ok_or(TextSurfaceScrollRequestRejection::LogicalRowNotFound),
        TextSurfaceScrollTarget::ByteOffset { byte_offset } => layout
            .bounds_for_byte_offset(*byte_offset)
            .map(Some)
            .ok_or(TextSurfaceScrollRequestRejection::InvalidUtf8Boundary),
        TextSurfaceScrollTarget::ByteRange {
            byte_start,
            byte_end,
        } => layout
            .bounds_for_byte_range(*byte_start, *byte_end)
            .map(Some)
            .ok_or_else(|| {
                if byte_start > byte_end {
                    TextSurfaceScrollRequestRejection::InvalidByteRange
                } else {
                    TextSurfaceScrollRequestRejection::InvalidUtf8Boundary
                }
            }),
        TextSurfaceScrollTarget::RelativePixels { delta_x, delta_y } => {
            if delta_x.is_finite() && delta_y.is_finite() {
                Ok(None)
            } else {
                Err(TextSurfaceScrollRequestRejection::NonFiniteRelativePixels)
            }
        }
    }
}

pub(super) fn aligned_scroll_offset(
    current_x: i32,
    current_y: i32,
    target: Option<UiRect>,
    viewport: UiRect,
    request: &TextSurfaceScrollRequest,
    bounds: TextSurfaceScrollBounds,
    scale_factor: f32,
) -> (i32, i32) {
    let (mut next_x, mut next_y) = (current_x, current_y);
    match (&request.target, target) {
        (TextSurfaceScrollTarget::RelativePixels { delta_x, delta_y }, _) => {
            next_x = next_x.saturating_add(normalize_logical_pixels(*delta_x, scale_factor));
            next_y = next_y.saturating_add(normalize_logical_pixels(*delta_y, scale_factor));
        }
        (_, Some(target)) => {
            let target_top = target.y.saturating_add(current_y);
            let target_bottom = target_top.saturating_add(target.height as i32);
            let viewport_top = viewport.y;
            let viewport_bottom = viewport.y.saturating_add(viewport.height as i32);
            next_y = match request.alignment {
                TextSurfaceScrollAlignment::Start => target_top.saturating_sub(viewport_top),
                TextSurfaceScrollAlignment::Center => {
                    target_top.saturating_sub(viewport_top).saturating_sub(
                        (viewport.height as i32).saturating_sub(target.height as i32) / 2,
                    )
                }
                TextSurfaceScrollAlignment::End => target_bottom.saturating_sub(viewport_bottom),
                TextSurfaceScrollAlignment::Nearest => {
                    if target.y < viewport_top {
                        target_top.saturating_sub(viewport_top)
                    } else if target.y.saturating_add(target.height as i32) > viewport_bottom {
                        target_bottom.saturating_sub(viewport_bottom)
                    } else {
                        current_y
                    }
                }
            };
        }
        (_, None) => {}
    }
    (next_x.clamp(0, bounds.max_x), next_y.clamp(0, bounds.max_y))
}

fn normalize_logical_pixels(value: TextSurfaceLogicalPixels, scale_factor: f32) -> i32 {
    let scale = if scale_factor.is_finite() && scale_factor > 0.0 {
        scale_factor
    } else {
        1.0
    };
    let device_pixels = (value.value() * scale).round();
    let logical_pixels = (device_pixels / scale).round();
    logical_pixels.clamp(i32::MIN as f32, i32::MAX as f32) as i32
}
