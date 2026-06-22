use katana_ui_core::render_model::UiImageSurfaceProps;

const CONTENT_SCALE_PERCENT: u64 = 100;
const DISPLAY_SIZE_MILLI: f32 = 1000.0;

pub(super) fn image_target_size(
    width: f32,
    height: f32,
    max_width: usize,
    requested_height: usize,
) -> (usize, usize) {
    let width = width.max(1.0);
    let height = height.max(1.0);
    let base_scale = (max_width.max(1) as f32 / width).min(1.0);
    let width_limited = scaled_dimension(width, base_scale);
    let height_limited = scaled_dimension(height, base_scale);
    if requested_height == 0 || height_limited <= requested_height {
        return (width_limited, height_limited);
    }
    let height_scale = requested_height.max(1) as f32 / height;
    let target_width = scaled_dimension(width, height_scale).min(max_width);
    let target_height = scaled_dimension(height, height_scale).min(requested_height);
    (target_width, target_height)
}

pub(super) fn logical_image_extent(physical_extent: u32, content_scale: u32) -> usize {
    let scale = content_scale.max(1);
    ((u64::from(physical_extent) * CONTENT_SCALE_PERCENT).div_ceil(u64::from(scale)) as usize)
        .max(1)
}

pub(super) fn logical_image_width_exact(image: &UiImageSurfaceProps) -> f32 {
    if image.display_width_milli > 0 {
        return image.display_width_milli as f32 / DISPLAY_SIZE_MILLI;
    }
    if image.display_width > 0 {
        return image.display_width as f32;
    }
    logical_image_extent(image.width, image.content_scale) as f32
}

pub(super) fn logical_image_height(image: &UiImageSurfaceProps) -> usize {
    logical_image_height_exact(image).ceil() as usize
}

pub(super) fn logical_image_height_exact(image: &UiImageSurfaceProps) -> f32 {
    if image.display_height_milli > 0 {
        return image.display_height_milli as f32 / DISPLAY_SIZE_MILLI;
    }
    if image.display_height > 0 {
        return image.display_height as f32;
    }
    logical_image_extent(image.height, image.content_scale) as f32
}

fn scaled_dimension(value: f32, scale: f32) -> usize {
    (value * scale).round().max(1.0) as usize
}
