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

pub(super) fn texture(
    pixels: &mut [u8],
    canvas: UiRect,
    clip: UiRect,
    bounds: UiRect,
    texture: &dyn TextureRef,
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

fn validate_texture(texture: &dyn TextureRef) -> Result<(), ArtifactCompositeError> {
    if texture.width() == 0 || texture.height() == 0 {
        return Err(ArtifactCompositeError::ZeroTexture {
            identity: texture.identity().to_owned(),
        });
    }
    let expected = u64::from(texture.width())
        .checked_mul(u64::from(texture.height()))
        .and_then(|pixels| pixels.checked_mul(RGBA_CHANNELS as u64))
        .and_then(|bytes| usize::try_from(bytes).ok())
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

fn nearest_texture_pixel(
    texture: &dyn TextureRef,
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
    Ok([
        pixels[0],
        pixels[GREEN_CHANNEL],
        pixels[BLUE_CHANNEL],
        pixels[ALPHA_CHANNEL],
    ])
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
mod tests {
    use super::*;
    use crate::artifact_compositor::ArtifactCompositeError;
    use katana_ui_core::render_model::UiRect;

    #[derive(Debug)]
    struct TestTexture {
        identity: &'static str,
        width: u32,
        height: u32,
        rgba_pixels: Vec<u8>,
    }

    impl TestTexture {
        fn new(identity: &'static str, width: u32, height: u32, rgba_pixels: Vec<u8>) -> Self {
            Self {
                identity,
                width,
                height,
                rgba_pixels,
            }
        }
    }

    impl TextureRef for TestTexture {
        fn identity(&self) -> &str {
            self.identity
        }

        fn width(&self) -> u32 {
            self.width
        }

        fn height(&self) -> u32 {
            self.height
        }

        fn rgba_pixels(&self) -> &[u8] {
            &self.rgba_pixels
        }
    }

    #[test]
    fn fill_is_noop_when_clip_and_bounds_do_not_overlap() {
        let mut pixels = vec![0u8; 4 * 4];
        let canvas = UiRect::new(0, 0, 2, 2);
        let clip = UiRect::new(3, 3, 1, 1);
        let bounds = UiRect::new(0, 0, 1, 1);
        fill(&mut pixels, canvas, clip, bounds, [255, 0, 0, 255])
            .expect("empty clip should succeed");
        assert_eq!(pixels, vec![0u8; 16]);
    }

    #[test]
    fn fill_blends_source_over_alpha() {
        let mut pixels = vec![0u8; 16];
        let canvas = UiRect::new(0, 0, 2, 2);
        let clip = UiRect::new(0, 0, 2, 2);
        fill(&mut pixels, canvas, clip, canvas, [255, 0, 0, 128]).expect("fill should blend");
        assert_eq!(&pixels[0..4], &[255, 0, 0, 128]);
        assert_eq!(&pixels[4..8], &[255, 0, 0, 128]);
        assert_eq!(&pixels[8..12], &[255, 0, 0, 128]);
        assert_eq!(&pixels[12..16], &[255, 0, 0, 128]);
    }

    #[test]
    fn texture_rejects_bad_dimensions_and_bad_byte_lengths() {
        let mut pixels = vec![0u8; 16];
        let canvas = UiRect::new(0, 0, 2, 2);
        let clip = UiRect::new(0, 0, 2, 2);
        let bounds = UiRect::new(0, 0, 1, 1);
        let zero = TestTexture::new("zero", 0, 1, vec![0]);
        assert!(matches!(
            texture(&mut pixels, canvas, clip, bounds, &zero),
            Err(ArtifactCompositeError::ZeroTexture { .. })
        ));

        let huge = TestTexture::new("huge", u32::MAX, u32::MAX, Vec::new());
        assert!(matches!(
            validate_texture(&huge),
            Err(ArtifactCompositeError::Overflow { .. })
        ));
    }

    #[test]
    fn texture_rejects_byte_length_and_blends_pixels() {
        let mut pixels = vec![0u8; 16];
        let canvas = UiRect::new(0, 0, 2, 1);
        let clip = UiRect::new(0, 0, 2, 1);
        let bounds = UiRect::new(0, 0, 2, 1);
        let wrong_length = TestTexture::new("bad", 2, 1, vec![0, 0, 255]);
        assert!(matches!(
            texture(&mut pixels, canvas, clip, bounds, &wrong_length),
            Err(ArtifactCompositeError::TextureByteLength { .. })
        ));

        let source = TestTexture::new("rgba", 2, 1, vec![0, 0, 255, 255, 255, 0, 0, 255]);
        texture(&mut pixels, canvas, clip, bounds, &source).expect("texture should blend");
        assert_eq!(&pixels[..4], &[0, 0, 255, 255]);
        assert_eq!(&pixels[4..8], &[255, 0, 0, 255]);
    }

    #[test]
    fn source_over_preserves_destination_when_source_is_transparent() {
        let mut pixels = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
        let canvas = UiRect::new(0, 0, 2, 1);
        source_over(&mut pixels, canvas, 0, 0, [10, 20, 30, 0]).expect("no-op should pass");
        assert_eq!(pixels, vec![1u8, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn nearest_texture_pixel_clamps_coordinates_to_edge() {
        let texture = TestTexture::new(
            "edge",
            2,
            2,
            vec![
                10, 20, 30, 40, 50, 60, 70, 80, //
                90, 100, 110, 120, 130, 140, 150, 160,
            ],
        );
        let bounds = UiRect::new(10, 10, 2, 2);
        assert_eq!(
            nearest_texture_pixel(&texture, bounds, 11, 11).expect("center should hit in-bounds"),
            [130, 140, 150, 160]
        );
        assert_eq!(
            nearest_texture_pixel(&texture, bounds, 20, 20).expect("out-of-range should clamp"),
            [130, 140, 150, 160]
        );
    }

    #[test]
    fn texture_zero_bounds_and_invalid_pixel_ranges_fail_safely() {
        let valid = TestTexture::new("one", 1, 1, vec![1, 2, 3, 4]);
        let mut pixels = vec![0; 4];
        texture(
            &mut pixels,
            UiRect::new(0, 0, 1, 1),
            UiRect::new(0, 0, 1, 1),
            UiRect::new(0, 0, 0, 1),
            &valid,
        )
        .expect("zero destination width is a no-op");

        let short = TestTexture::new("short", 2, 2, vec![0; 4]);
        assert!(matches!(
            nearest_texture_pixel(&short, UiRect::new(0, 0, 2, 2), 1, 1),
            Err(ArtifactCompositeError::TexturePixelRange { .. })
        ));
        let huge = TestTexture::new("huge", u32::MAX, u32::MAX, Vec::new());
        assert!(matches!(
            nearest_texture_pixel(
                &huge,
                UiRect::new(0, 0, u32::MAX, u32::MAX),
                i32::MAX,
                i32::MAX,
            ),
            Err(ArtifactCompositeError::Overflow { .. })
        ));
        assert!(matches!(
            source_over(&mut pixels, UiRect::new(0, 0, 1, 1), -1, 0, [1, 2, 3, 4],),
            Err(ArtifactCompositeError::Overflow { .. })
        ));
        assert!(matches!(
            source_over(&mut pixels, UiRect::new(0, 0, 1, 1), 0, -1, [1, 2, 3, 4],),
            Err(ArtifactCompositeError::Overflow { .. })
        ));
    }
}
