use super::ArtifactCompositeError;
use super::artifact_compositor_geometry::{intersect, rect_edges};
use katana_ui_core::render_model::UiRect;

pub(super) const RGBA_CHANNELS: usize = 4;
const GREEN_CHANNEL: usize = 1;
const BLUE_CHANNEL: usize = 2;
const ALPHA_CHANNEL: usize = 3;
const RGB_CHANNELS: usize = 3;
const OPAQUE_ALPHA: u32 = 255;

pub(super) trait TextureRef {
    fn identity(&self) -> &str;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn rgba_pixels(&self) -> &[u8];
}

pub(super) fn fill(
    pixels: &mut [u8],
    canvas: UiRect,
    clip: UiRect,
    bounds: UiRect,
    color: [u8; RGBA_CHANNELS],
) -> Result<(), ArtifactCompositeError> {
    let Some(target) = intersect(clip, bounds)? else {
        return Ok(());
    };
    let (right, bottom) = rect_edges(target)?;
    for y in target.y..bottom {
        for x in target.x..right {
            source_over(pixels, canvas, x, y, color)?;
        }
    }
    Ok(())
}

pub(super) fn rounded_fill(
    pixels: &mut [u8],
    canvas: UiRect,
    clip: UiRect,
    bounds: UiRect,
    color: [u8; RGBA_CHANNELS],
    radius: u32,
) -> Result<(), ArtifactCompositeError> {
    let Some(target) = intersect(clip, bounds)? else {
        return Ok(());
    };
    let (right, bottom) = rect_edges(target)?;
    let radius = radius.min(bounds.width / 2).min(bounds.height / 2) as i32;
    for y in target.y..bottom {
        for x in target.x..right {
            let dx = (x - bounds.x).min(bounds.x + bounds.width as i32 - 1 - x);
            let dy = (y - bounds.y).min(bounds.y + bounds.height as i32 - 1 - y);
            let in_corner = dx < radius && dy < radius;
            if in_corner {
                let cx = if x < bounds.x + radius {
                    bounds.x + radius
                } else {
                    bounds.x + bounds.width as i32 - radius - 1
                };
                let cy = if y < bounds.y + radius {
                    bounds.y + radius
                } else {
                    bounds.y + bounds.height as i32 - radius - 1
                };
                if (x - cx).pow(2) + (y - cy).pow(2) > radius.pow(2) {
                    continue;
                }
            }
            source_over(pixels, canvas, x, y, color)?;
        }
    }
    Ok(())
}

pub(super) fn texture<T: TextureRef>(
    pixels: &mut [u8],
    canvas: UiRect,
    clip: UiRect,
    bounds: UiRect,
    texture: &T,
) -> Result<(), ArtifactCompositeError> {
    validate_texture(texture)?;
    if bounds.width == 0 || bounds.height == 0 {
        return Ok(());
    }
    let Some(target) = intersect(clip, bounds)? else {
        return Ok(());
    };
    let (right, bottom) = rect_edges(target)?;
    for y in target.y..bottom {
        for x in target.x..right {
            let source = nearest_texture_pixel(texture, bounds, x, y)?;
            source_over(pixels, canvas, x, y, source)?;
        }
    }
    Ok(())
}

fn validate_texture<T: TextureRef>(texture: &T) -> Result<(), ArtifactCompositeError> {
    if texture.width() == 0 || texture.height() == 0 {
        return Err(ArtifactCompositeError::ZeroTexture {
            identity: texture.identity().to_owned(),
        });
    }
    let expected = (texture.width() as usize)
        .checked_mul(texture.height() as usize)
        .and_then(|pixels| pixels.checked_mul(RGBA_CHANNELS))
        .ok_or(ArtifactCompositeError::Overflow {
            context: "sizing texture RGBA bytes",
        })?;
    if texture.rgba_pixels().len() != expected {
        return Err(ArtifactCompositeError::TextureByteLength {
            identity: texture.identity().to_owned(),
            expected,
            actual: texture.rgba_pixels().len(),
        });
    }
    Ok(())
}

fn nearest_texture_pixel<T: TextureRef>(
    texture: &T,
    bounds: UiRect,
    x: i32,
    y: i32,
) -> Result<[u8; RGBA_CHANNELS], ArtifactCompositeError> {
    let source_x = ((i64::from(x) - i64::from(bounds.x)) * i64::from(texture.width())
        / i64::from(bounds.width))
    .clamp(0, i64::from(texture.width() - 1)) as usize;
    let source_y = ((i64::from(y) - i64::from(bounds.y)) * i64::from(texture.height())
        / i64::from(bounds.height))
    .clamp(0, i64::from(texture.height() - 1)) as usize;
    let index = source_y
        .checked_mul(texture.width() as usize)
        .and_then(|value| value.checked_add(source_x))
        .and_then(|value| value.checked_mul(RGBA_CHANNELS))
        .ok_or(ArtifactCompositeError::Overflow {
            context: "indexing texture",
        })?;
    let end = index
        .checked_add(RGBA_CHANNELS)
        .ok_or(ArtifactCompositeError::Overflow {
            context: "sizing texture pixel range",
        })?;
    let pixels = texture.rgba_pixels().get(index..end).ok_or_else(|| {
        ArtifactCompositeError::TexturePixelRange {
            identity: texture.identity().to_owned(),
            start: index,
            end,
            actual: texture.rgba_pixels().len(),
        }
    })?;
    Ok([pixels[0], pixels[1], pixels[2], pixels[3]])
}

fn source_over(
    pixels: &mut [u8],
    canvas: UiRect,
    x: i32,
    y: i32,
    source: [u8; RGBA_CHANNELS],
) -> Result<(), ArtifactCompositeError> {
    let local_x = usize::try_from(i64::from(x) - i64::from(canvas.x)).map_err(|_| {
        ArtifactCompositeError::Overflow {
            context: "indexing canvas x",
        }
    })?;
    let local_y = usize::try_from(i64::from(y) - i64::from(canvas.y)).map_err(|_| {
        ArtifactCompositeError::Overflow {
            context: "indexing canvas y",
        }
    })?;
    let index = local_y
        .checked_mul(canvas.width as usize)
        .and_then(|value| value.checked_add(local_x))
        .and_then(|value| value.checked_mul(RGBA_CHANNELS))
        .ok_or(ArtifactCompositeError::Overflow {
            context: "indexing canvas",
        })?;
    let destination = [
        pixels[index],
        pixels[index + GREEN_CHANNEL],
        pixels[index + BLUE_CHANNEL],
        pixels[index + ALPHA_CHANNEL],
    ];
    let source_alpha = u32::from(source[ALPHA_CHANNEL]);
    let destination_alpha = u32::from(destination[ALPHA_CHANNEL]);
    let inverse_source_alpha = OPAQUE_ALPHA - source_alpha;
    let output_alpha = source_alpha + destination_alpha * inverse_source_alpha / OPAQUE_ALPHA;
    if output_alpha == 0 {
        return Ok(());
    }
    for channel in 0..RGB_CHANNELS {
        let numerator = u32::from(source[channel]) * source_alpha * OPAQUE_ALPHA
            + u32::from(destination[channel]) * destination_alpha * inverse_source_alpha;
        let normalized =
            numerator
                .checked_div(output_alpha)
                .ok_or(ArtifactCompositeError::Overflow {
                    context: "normalizing source-over alpha",
                })?;
        pixels[index + channel] = (normalized / OPAQUE_ALPHA) as u8;
    }
    pixels[index + ALPHA_CHANNEL] = output_alpha as u8;
    Ok(())
}

#[cfg(test)]
#[path = "artifact_compositor_blend_tests.rs"]
mod tests;
