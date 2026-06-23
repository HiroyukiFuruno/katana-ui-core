use super::canvas::Canvas;
use super::ui_tree_canvas_image_cache_entry::CachedImageSurface;
use super::ui_tree_canvas_image_cache_key::CachedImageKey;
use super::ui_tree_canvas_image_raster_cache::rasterize_cached_surface;
use super::ui_tree_canvas_types::{RgbaBlitRequest, RgbaSourceRect};
use katana_ui_core::render_model::UiImageSurfaceProps;
use std::cell::RefCell;

const MAX_CACHE_ENTRIES: usize = 24;
thread_local! {
    static IMAGE_CACHE: RefCell<Vec<CachedImageSurface>> = const { RefCell::new(Vec::new()) };
}

pub(super) fn try_blit_cached_image(
    canvas: &mut Canvas,
    image: &UiImageSurfaceProps,
    request: RgbaBlitRequest<'_>,
    target_width: usize,
    target_height: usize,
) -> bool {
    let Some(key) = CachedImageKey::new(image, canvas.scale_factor(), target_width, target_height)
    else {
        return false;
    };
    if !can_use_cached_request(&request, image, target_width) {
        return false;
    }
    if blit_cached_surface(canvas, &key, request, image.height, target_height) {
        return true;
    }
    let cached =
        rasterize_cached_surface(image, target_width, target_height, canvas.scale_factor());
    blit_cached_canvas(canvas, &cached, request, image.height, target_height);
    remember_cached_canvas(key, cached);
    true
}

fn blit_cached_canvas(
    canvas: &mut Canvas,
    cached: &CachedImageSurface,
    request: RgbaBlitRequest<'_>,
    source_height: u32,
    _target_height: usize,
) {
    let left = canvas.to_physical_x(request.area.x);
    let right = canvas.to_physical_x(request.area.x.saturating_add(request.area.width));
    let top = canvas.to_physical_y(request.area.y);
    let bottom = canvas.to_physical_y(request.area.y.saturating_add(request.area.height));
    let target_width = right.saturating_sub(left).min(cached.width);
    let target_height = bottom.saturating_sub(top).min(cached.height);
    if target_width == 0 || target_height == 0 {
        return;
    }
    let source_y = cached_source_y(request.source, source_height, cached.height);
    if canvas.clip.is_none() {
        blit_cached_canvas_unclipped(
            canvas,
            cached,
            left,
            top,
            target_width,
            target_height,
            source_y,
        );
        return;
    }
    let (draw_left, draw_top, draw_width, draw_height) =
        cached_draw_rect(canvas, left, top, target_width, target_height);
    if draw_width == 0 || draw_height == 0 {
        return;
    }
    let source_x = draw_left.saturating_sub(left);
    let source_y = source_y.saturating_add(draw_top.saturating_sub(top));
    for y in 0..draw_height {
        let cache_y = source_y.saturating_add(y);
        if cache_y >= cached.height {
            break;
        }
        let cache_row = cache_y.saturating_mul(cached.width);
        let row_start_x = source_x;
        let row_end_x = source_x.saturating_add(draw_width).min(cached.width);
        let cache_start = cache_row.saturating_add(row_start_x);
        let cache_end = cache_row.saturating_add(row_end_x).min(cached.pixels.len());
        if cache_end > cache_start && cached.row_opaque(cache_y, cache_start, cache_end) {
            let dest_y = draw_top.saturating_add(y);
            if dest_y >= canvas.height {
                break;
            }
            let dest_start = dest_y
                .saturating_mul(canvas.width)
                .saturating_add(draw_left);
            let dest_end = dest_start
                .saturating_add(cache_end.saturating_sub(cache_start))
                .min(canvas.pixels.len());
            let copy_width = dest_end.saturating_sub(dest_start);
            canvas.pixels[dest_start..dest_end]
                .copy_from_slice(&cached.pixels[cache_start..cache_start + copy_width]);
            continue;
        }
        blit_cached_spans(
            canvas,
            cached,
            cache_y,
            row_start_x,
            row_end_x,
            draw_left,
            draw_top.saturating_add(y),
        );
    }
}

fn cached_draw_rect(
    canvas: &Canvas,
    left: usize,
    top: usize,
    target_width: usize,
    target_height: usize,
) -> (usize, usize, usize, usize) {
    let mut draw_left = left;
    let mut draw_top = top;
    let mut draw_right = left.saturating_add(target_width).min(canvas.width);
    let mut draw_bottom = top.saturating_add(target_height).min(canvas.height);
    if let Some(clip) = canvas.clip {
        draw_left = draw_left.max(clip.x);
        draw_top = draw_top.max(clip.y);
        draw_right = draw_right.min(clip.right());
        draw_bottom = draw_bottom.min(clip.bottom());
    }
    (
        draw_left,
        draw_top,
        draw_right.saturating_sub(draw_left),
        draw_bottom.saturating_sub(draw_top),
    )
}

fn blit_cached_canvas_unclipped(
    canvas: &mut Canvas,
    cached: &CachedImageSurface,
    left: usize,
    top: usize,
    target_width: usize,
    target_height: usize,
    source_y: usize,
) {
    for y in 0..target_height {
        let cache_y = source_y.saturating_add(y);
        if cache_y >= cached.height {
            break;
        }
        let cache_row = cache_y.saturating_mul(cached.width);
        if cached.alpha[cache_row..cache_row + target_width]
            .iter()
            .all(|alpha| *alpha == u8::MAX)
        {
            let dest_y = top.saturating_add(y);
            if dest_y >= canvas.height {
                break;
            }
            let dest_start = dest_y.saturating_mul(canvas.width).saturating_add(left);
            let dest_end = dest_start
                .saturating_add(target_width)
                .min(canvas.pixels.len());
            let copy_width = dest_end.saturating_sub(dest_start);
            canvas.pixels[dest_start..dest_end]
                .copy_from_slice(&cached.pixels[cache_row..cache_row + copy_width]);
            continue;
        }
        let dest_y = top.saturating_add(y);
        if dest_y >= canvas.height {
            break;
        }
        blit_cached_spans(canvas, cached, cache_y, 0, target_width, left, dest_y);
    }
}

fn blit_cached_spans(
    canvas: &mut Canvas,
    cached: &CachedImageSurface,
    cache_y: usize,
    row_start_x: usize,
    row_end_x: usize,
    dest_left: usize,
    dest_y: usize,
) {
    if dest_y >= canvas.height || row_start_x >= row_end_x {
        return;
    }
    let cache_row = cache_y.saturating_mul(cached.width);
    if let Some(spans) = cached.opaque_spans.get(cache_y) {
        for &(span_start_x, span_end_x) in spans {
            let span_start_x = span_start_x.max(row_start_x);
            let span_end_x = span_end_x.min(row_end_x);
            if span_start_x >= span_end_x {
                continue;
            }
            let cache_start = cache_row.saturating_add(span_start_x);
            let cache_end = cache_row
                .saturating_add(span_end_x)
                .min(cached.pixels.len());
            if cache_start >= cache_end {
                continue;
            }
            let dest_x = dest_left.saturating_add(span_start_x.saturating_sub(row_start_x));
            let dest_start = dest_y.saturating_mul(canvas.width).saturating_add(dest_x);
            let dest_end = dest_start
                .saturating_add(cache_end.saturating_sub(cache_start))
                .min(canvas.pixels.len());
            let copy_width = dest_end.saturating_sub(dest_start);
            canvas.pixels[dest_start..dest_end]
                .copy_from_slice(&cached.pixels[cache_start..cache_start + copy_width]);
        }
    }
    let Some(spans) = cached.translucent_spans.get(cache_y) else {
        return;
    };
    for &(span_start_x, span_end_x) in spans {
        let span_start_x = span_start_x.max(row_start_x);
        let span_end_x = span_end_x.min(row_end_x);
        if span_start_x >= span_end_x {
            continue;
        }
        let cache_start = cache_row.saturating_add(span_start_x);
        let cache_end = cache_row
            .saturating_add(span_end_x)
            .min(cached.pixels.len());
        if cache_start >= cache_end {
            continue;
        }
        let dest_x = dest_left.saturating_add(span_start_x.saturating_sub(row_start_x));
        let dest_start = dest_y.saturating_mul(canvas.width).saturating_add(dest_x);
        let dest_end = dest_start
            .saturating_add(cache_end.saturating_sub(cache_start))
            .min(canvas.pixels.len());
        let copy_width = dest_end.saturating_sub(dest_start);
        for index in 0..copy_width {
            let source_index = cache_start.saturating_add(index);
            let dest_index = dest_start.saturating_add(index);
            canvas.pixels[dest_index] = super::canvas_color::blend_color(
                canvas.pixels[dest_index],
                cached.pixels[source_index],
                cached.alpha[source_index],
            );
        }
    }
}

fn blit_cached_surface(
    canvas: &mut Canvas,
    key: &CachedImageKey,
    request: RgbaBlitRequest<'_>,
    source_height: u32,
    target_height: usize,
) -> bool {
    IMAGE_CACHE.with(|cache| {
        let cache = cache.borrow();
        let Some(cached) = cache.iter().find(|entry| &entry.key == key) else {
            return false;
        };
        blit_cached_canvas(canvas, cached, request, source_height, target_height);
        true
    })
}

fn remember_cached_canvas(key: CachedImageKey, surface: CachedImageSurface) {
    IMAGE_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        cache.retain(|entry| entry.key != key);
        cache.push(CachedImageSurface { key, ..surface });
        while cache.len() > MAX_CACHE_ENTRIES {
            cache.remove(0);
        }
    });
}

fn cached_source_y(source: RgbaSourceRect, source_height: u32, target_height: usize) -> usize {
    if source.y <= 0.0 || source.height <= 0.0 {
        return 0;
    }
    (source.y * target_height as f32 / source_height.max(1) as f32)
        .round()
        .max(0.0) as usize
}

fn can_use_cached_request(
    request: &RgbaBlitRequest<'_>,
    image: &UiImageSurfaceProps,
    target_width: usize,
) -> bool {
    request.source.x.abs() < f32::EPSILON
        && (request.source.width - image.width as f32).abs() < 1.0
        && request.source.y.abs() < f32::EPSILON
        && (request.source.height - image.height as f32).abs() < 1.0
        && request.area.width <= target_width
}
