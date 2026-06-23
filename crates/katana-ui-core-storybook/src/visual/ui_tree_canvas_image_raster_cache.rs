use super::canvas_scale::physical_size;
use super::ui_tree_canvas_image_cache_entry::CachedImageSurface;
use super::ui_tree_canvas_image_cache_key::CachedImageKey;
use super::ui_tree_canvas_rgba::{packed_rgb, rgba_alpha, rgba_sample};
use super::ui_tree_canvas_types::{RgbaBlitRequest, RgbaSourceRect};
use katana_ui_core::render_model::UiImageSurfaceProps;

pub(super) fn rasterize_cached_surface(
    image: &UiImageSurfaceProps,
    target_width: usize,
    target_height: usize,
    canvas_scale: f32,
) -> CachedImageSurface {
    let physical_width = physical_size(target_width, canvas_scale).max(1);
    let physical_height = physical_size(target_height, canvas_scale).max(1);
    let pixel_count = physical_width * physical_height;
    let mut pixels = vec![0; pixel_count];
    let mut alpha = vec![0; pixel_count];
    let mut opaque_rows = vec![true; physical_height];
    let mut opaque_spans = vec![Vec::new(); physical_height];
    let mut translucent_spans = vec![Vec::new(); physical_height];
    let request = RgbaBlitRequest {
        rgba: &image.rgba,
        width: image.width,
        height: image.height,
        source: RgbaSourceRect::full(image.width, image.height),
        area: super::ui_tree_canvas_types::UiTreeRenderArea {
            x: 0,
            y: 0,
            width: target_width,
            height: target_height,
            scroll_y: 0.0,
        },
    };
    for y in 0..physical_height {
        let source_y = sample_position(y, image.height as f32, physical_height);
        let mut opaque_span_start = None;
        let mut translucent_span_start = None;
        for x in 0..physical_width {
            let source_x = sample_position(x, image.width as f32, physical_width);
            let sample = rgba_sample(&request, source_x, source_y);
            let offset = y * physical_width + x;
            pixels[offset] = packed_rgb(sample);
            let sample_alpha = rgba_alpha(sample);
            alpha[offset] = sample_alpha;
            opaque_rows[y] &= sample_alpha == u8::MAX;
            match sample_alpha {
                0 => {
                    if let Some(start) = opaque_span_start.take() {
                        opaque_spans[y].push((start, x));
                    }
                    if let Some(start) = translucent_span_start.take() {
                        translucent_spans[y].push((start, x));
                    }
                }
                u8::MAX => {
                    if let Some(start) = translucent_span_start.take() {
                        translucent_spans[y].push((start, x));
                    }
                    if opaque_span_start.is_none() {
                        opaque_span_start = Some(x);
                    }
                }
                _ => {
                    if let Some(start) = opaque_span_start.take() {
                        opaque_spans[y].push((start, x));
                    }
                    if translucent_span_start.is_none() {
                        translucent_span_start = Some(x);
                    }
                }
            }
        }
        if let Some(start) = opaque_span_start {
            opaque_spans[y].push((start, physical_width));
        }
        if let Some(start) = translucent_span_start {
            translucent_spans[y].push((start, physical_width));
        }
    }
    CachedImageSurface {
        key: CachedImageKey::placeholder(),
        width: physical_width,
        height: physical_height,
        pixels,
        alpha,
        opaque_rows,
        opaque_spans,
        translucent_spans,
    }
}

fn sample_position(target_index: usize, source_extent: f32, target_extent: usize) -> f32 {
    const PIXEL_CENTER_OFFSET: f32 = 0.5;
    ((target_index as f32 + PIXEL_CENTER_OFFSET) * source_extent / target_extent.max(1) as f32
        - PIXEL_CENTER_OFFSET)
        .max(0.0)
}
