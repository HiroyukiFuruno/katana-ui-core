use super::ui_tree_canvas_types::{RgbaBlitRequest, RgbaSourceRect, UiTreeRenderArea};
use katana_ui_core::render_model::UiImageSurfaceProps;

#[allow(clippy::too_many_arguments)]
pub(super) fn transformed_image_blit_request<'a>(
    image: &'a UiImageSurfaceProps,
    base_x: usize,
    base_y: i64,
    target_width: usize,
    target_height: usize,
    container_x: usize,
    container_y: usize,
    container_width: usize,
    container_height: usize,
) -> Option<RgbaBlitRequest<'a>> {
    let zoom = image.transform.zoom_factor().max(f32::MIN_POSITIVE);
    let visual_width = ((target_width as f32 * zoom).round() as usize).max(1);
    let visual_height = ((target_height as f32 * zoom).round() as usize).max(1);
    let left = base_x as i64 + i64::from(image.transform.pan_x);
    let top = base_y + i64::from(image.transform.pan_y);
    let visible = clipped_target_rect(
        left,
        top,
        visual_width,
        visual_height,
        container_x,
        container_y,
        container_width,
        container_height,
    )?;
    let source = clipped_source_rect(
        image.width,
        image.height,
        left,
        top,
        visual_width,
        visual_height,
        visible,
    );
    Some(RgbaBlitRequest {
        rgba: &image.rgba,
        width: image.width,
        height: image.height,
        source,
        area: UiTreeRenderArea {
            x: visible.x,
            y: visible.y,
            width: visible.width,
            height: visible.height,
            scroll_y: 0.0,
        },
    })
}

#[allow(clippy::too_many_arguments)]
fn clipped_target_rect(
    left: i64,
    top: i64,
    width: usize,
    height: usize,
    container_x: usize,
    container_y: usize,
    container_width: usize,
    container_height: usize,
) -> Option<UiTreeTargetRect> {
    let right = left.saturating_add(width as i64);
    let bottom = top.saturating_add(height as i64);
    let clip_left = left.max(container_x as i64);
    let clip_top = top.max(container_y as i64);
    let clip_right = right.min(container_x.saturating_add(container_width) as i64);
    let clip_bottom = bottom.min(container_y.saturating_add(container_height) as i64);
    if clip_left >= clip_right || clip_top >= clip_bottom {
        return None;
    }
    Some(UiTreeTargetRect {
        x: clip_left as usize,
        y: clip_top as usize,
        width: (clip_right - clip_left) as usize,
        height: (clip_bottom - clip_top) as usize,
    })
}

fn clipped_source_rect(
    source_width: u32,
    source_height: u32,
    target_left: i64,
    target_top: i64,
    target_width: usize,
    target_height: usize,
    visible: UiTreeTargetRect,
) -> RgbaSourceRect {
    let clipped_x = (visible.x as i64 - target_left).max(0) as f32;
    let clipped_y = (visible.y as i64 - target_top).max(0) as f32;
    RgbaSourceRect {
        x: clipped_x * source_width as f32 / target_width.max(1) as f32,
        y: clipped_y * source_height as f32 / target_height.max(1) as f32,
        width: visible.width as f32 * source_width as f32 / target_width.max(1) as f32,
        height: visible.height as f32 * source_height as f32 / target_height.max(1) as f32,
    }
}

#[derive(Clone, Copy)]
struct UiTreeTargetRect {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}
