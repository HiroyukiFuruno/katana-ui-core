use super::text_surface::TextSurfacePaintTexture;
use crate::render_model::{RGBA_CHANNEL_COUNT, UiRect};

pub(super) fn texture_tiles(
    bounds: UiRect,
    texture: &TextSurfacePaintTexture,
    max_texture_side: usize,
) -> impl Iterator<Item = (UiRect, TextSurfacePaintTexture)> + '_ {
    rgba_texture_tiles(
        bounds,
        &texture.identity,
        texture.width,
        texture.height,
        &texture.rgba_pixels,
        max_texture_side,
    )
}

pub(super) fn rgba_texture_tiles<'a>(
    bounds: UiRect,
    identity: &'a str,
    texture_width: u32,
    texture_height: u32,
    rgba_pixels: &'a [u8],
    max_texture_side: usize,
) -> impl Iterator<Item = (UiRect, TextSurfacePaintTexture)> + 'a {
    let width = usize::try_from(texture_width).unwrap_or_default();
    let height = usize::try_from(texture_height).unwrap_or_default();
    let tile_width = width.min(max_texture_side);
    let tile_height = height.min(max_texture_side);
    let mut x = 0;
    let mut y = 0;
    std::iter::from_fn(move || {
        if width == 0 || height == 0 || max_texture_side == 0 || y >= height {
            return None;
        }
        let current_width = tile_width.min(width - x);
        let current_height = tile_height.min(height - y);
        let mut pixels = Vec::with_capacity(
            current_width
                .saturating_mul(current_height)
                .saturating_mul(RGBA_CHANNEL_COUNT),
        );
        for row in y..y + current_height {
            let start = row
                .saturating_mul(width)
                .saturating_add(x)
                .saturating_mul(RGBA_CHANNEL_COUNT);
            let end = start.saturating_add(current_width.saturating_mul(RGBA_CHANNEL_COUNT));
            pixels.extend_from_slice(&rgba_pixels[start..end]);
        }
        let tile = TextSurfacePaintTexture {
            identity: format!("{}:{x}:{y}:{current_width}x{current_height}", identity),
            width: u32::try_from(current_width).unwrap_or_default(),
            height: u32::try_from(current_height).unwrap_or_default(),
            rgba_pixels: pixels,
        };
        let tile_bounds = UiRect::new(
            bounds
                .x
                .saturating_add((x.saturating_mul(bounds.width as usize) / width) as i32),
            bounds
                .y
                .saturating_add((y.saturating_mul(bounds.height as usize) / height) as i32),
            u32::try_from(
                ((x + current_width).saturating_mul(bounds.width as usize) / width)
                    .saturating_sub(x.saturating_mul(bounds.width as usize) / width),
            )
            .unwrap_or_default(),
            u32::try_from(
                ((y + current_height).saturating_mul(bounds.height as usize) / height)
                    .saturating_sub(y.saturating_mul(bounds.height as usize) / height),
            )
            .unwrap_or_default(),
        );
        x = x.saturating_add(current_width);
        if x >= width {
            x = 0;
            y = y.saturating_add(current_height);
        }
        Some((tile_bounds, tile))
    })
}
