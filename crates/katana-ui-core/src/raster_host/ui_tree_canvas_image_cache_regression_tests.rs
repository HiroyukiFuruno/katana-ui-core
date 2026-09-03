use super::canvas::Canvas;
use super::ui_tree_canvas_image_cache::try_blit_cached_image;
use super::ui_tree_canvas_image_cache_key::CachedImageKey;
use super::ui_tree_canvas_image_raster_cache::rasterize_cached_surface;
use super::{RgbaBlitRequest, RgbaSourceRect, UiTreeRenderArea};
use crate::test_assert::KucTestExpect;
use katana_ui_core::render_model::UiImageSurfaceProps;

const RED_STEP: u32 = 8;
const GREEN_STEP: u32 = 16;
const BLUE_VALUE: u8 = 128;
const RETINA_CONTENT_SCALE_PERCENT: u32 = 200;
const TRANSLUCENT_EDGE_OUTER_ALPHA: u8 = 48;
const TRANSLUCENT_EDGE_INNER_ALPHA: u8 = 128;
const TRANSLUCENT_EDGE_COLOR: u8 = 220;

#[test]
fn cached_image_key_accepts_retina_canvas_scale() {
    let image = image_surface(8, 4);

    let key = CachedImageKey::new(&image, 2.0, 16, 8);

    assert!(key.is_some());
}

#[test]
fn cached_image_key_rejects_fractional_reference_scale() {
    let image = image_surface(8, 4);

    let key = CachedImageKey::new(&image, 1.85, 16, 8);

    assert!(key.is_none());
}

#[test]
fn cached_image_surface_uses_physical_retina_extent() {
    let image = image_surface(8, 4);

    let cached = rasterize_cached_surface(&image, 16, 8, 2.0);

    assert_eq!(32, cached.width);
    assert_eq!(16, cached.height);
}

#[test]
fn cached_retina_blit_matches_direct_retina_blit() {
    let image = image_surface(32, 16);
    let request = RgbaBlitRequest {
        rgba: &image.rgba,
        width: image.width,
        height: image.height,
        source: RgbaSourceRect::full(image.width, image.height),
        area: UiTreeRenderArea {
            x: 2,
            y: 3,
            width: 16,
            height: 8,
            scroll_y: 0.0,
        },
    };
    let mut direct = Canvas::new_scaled(40, 24, 2.0, 0);
    direct.blit_rgba(request);

    let mut cached = Canvas::new_scaled(40, 24, 2.0, 0);
    assert!(try_blit_cached_image(&mut cached, &image, request, 16, 8));

    assert_eq!(direct.pixels(), cached.pixels());
}

#[test]
fn cached_retina_blit_matches_direct_for_translucent_svg_edges() {
    let image = translucent_edge_surface(32, 16);
    let request = RgbaBlitRequest {
        rgba: &image.rgba,
        width: image.width,
        height: image.height,
        source: RgbaSourceRect::full(image.width, image.height),
        area: UiTreeRenderArea {
            x: 2,
            y: 3,
            width: 16,
            height: 8,
            scroll_y: 0.0,
        },
    };
    let background = 0x1e1e1e;
    let mut direct = Canvas::new_scaled(40, 24, 2.0, background);
    direct.blit_rgba(request);

    let mut cached = Canvas::new_scaled(40, 24, 2.0, background);
    assert!(try_blit_cached_image(&mut cached, &image, request, 16, 8));

    assert_eq!(direct.pixels(), cached.pixels());
}

#[test]
fn cached_retina_blit_falls_back_to_direct_when_bottom_clipped_by_viewport() {
    let image = image_surface(2048, 329);
    let full_target_height = 202usize;
    let visible_height = 112usize;
    let visible_source_height =
        visible_height as f32 * image.height as f32 / full_target_height as f32;
    let request = RgbaBlitRequest {
        rgba: &image.rgba,
        width: image.width,
        height: image.height,
        source: RgbaSourceRect {
            x: 0.0,
            y: 0.0,
            width: image.width as f32,
            height: visible_source_height,
        },
        area: UiTreeRenderArea {
            x: 250,
            y: 2_288,
            width: 1_256,
            height: 112,
            scroll_y: 0.0,
        },
    };
    let mut direct = Canvas::new_scaled(1_280, 2_400, 2.0, 0);
    direct.blit_rgba(request);

    let mut cached = Canvas::new_scaled(1_280, 2_400, 2.0, 0);
    assert!(!try_blit_cached_image(
        &mut cached,
        &image,
        request,
        1_256,
        full_target_height
    ));
    cached.blit_rgba(request);

    assert_eq!(
        first_pixel_diff(direct.pixels(), cached.pixels()),
        None,
        "cached bottom-clipped blit must match direct blit"
    );
}

#[test]
fn clipped_cached_translucent_surface_uses_span_blending_inside_the_clip() {
    let mut image = translucent_edge_surface(8, 4);
    image.fingerprint = "clipped-translucent-cache".to_string();
    let request = RgbaBlitRequest {
        rgba: &image.rgba,
        width: image.width,
        height: image.height,
        source: RgbaSourceRect::full(image.width, image.height),
        area: UiTreeRenderArea {
            x: 1,
            y: 1,
            width: 4,
            height: 2,
            scroll_y: 0.0,
        },
    };
    let mut warm = Canvas::new_scaled(6, 4, 2.0, 0x1e1e1e);
    assert!(try_blit_cached_image(&mut warm, &image, request, 4, 2));

    let mut clipped = Canvas::new_scaled(6, 4, 2.0, 0x1e1e1e);
    clipped.with_clip(2, 1, 2, 2, &mut |canvas| {
        assert!(try_blit_cached_image(canvas, &image, request, 4, 2));
    });

    assert_ne!(0x1e1e1e, clipped.pixels()[2 * clipped.width() + 4]);
}

fn image_surface(width: u32, height: u32) -> UiImageSurfaceProps {
    let mut rgba = Vec::new();
    for y in 0..height {
        for x in 0..width {
            rgba.push((x * RED_STEP) as u8);
            rgba.push((y * GREEN_STEP) as u8);
            rgba.push(BLUE_VALUE);
            rgba.push(u8::MAX);
        }
    }
    UiImageSurfaceProps::new("test-image", width, height, rgba)
        .kuc_expect("test image surface should be valid")
        .content_scale(RETINA_CONTENT_SCALE_PERCENT)
}

fn translucent_edge_surface(width: u32, height: u32) -> UiImageSurfaceProps {
    let mut rgba = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let edge_distance = x
                .min(y)
                .min(width.saturating_sub(1).saturating_sub(x))
                .min(height.saturating_sub(1).saturating_sub(y));
            let alpha = match edge_distance {
                0 => TRANSLUCENT_EDGE_OUTER_ALPHA,
                1 => TRANSLUCENT_EDGE_INNER_ALPHA,
                _ => u8::MAX,
            };
            rgba.push(TRANSLUCENT_EDGE_COLOR);
            rgba.push(TRANSLUCENT_EDGE_COLOR);
            rgba.push(TRANSLUCENT_EDGE_COLOR);
            rgba.push(alpha);
        }
    }
    UiImageSurfaceProps::new("translucent-edge", width, height, rgba)
        .kuc_expect("test image surface should be valid")
        .content_scale(RETINA_CONTENT_SCALE_PERCENT)
}

fn first_pixel_diff(left: &[u32], right: &[u32]) -> Option<(usize, u32, u32)> {
    left.iter()
        .zip(right.iter())
        .enumerate()
        .find_map(|(index, (left, right))| (left != right).then_some((index, *left, *right)))
        .or_else(|| {
            (left.len() != right.len()).then_some((
                left.len().min(right.len()),
                left.len() as u32,
                right.len() as u32,
            ))
        })
}
