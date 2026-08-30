use super::*;
use crate::text_surface::TextSurfacePaintTexture;
use std::cell::Cell;

fn paint_texture(width: u32, height: u32, pixels: Vec<u8>) -> TextSurfacePaintTexture {
    TextSurfacePaintTexture {
        identity: "test-texture".to_owned(),
        width,
        height,
        rgba_pixels: pixels,
    }
}

#[test]
fn fill_rejects_negative_canvas_x_indexing() {
    let mut pixels = vec![0, 0, 0, 0];
    let canvas = UiRect::new(0, 0, 1, 1);
    assert!(matches!(
        source_over(&mut pixels, canvas, i32::MIN, 0, [255, 0, 0, 255]),
        Err(ArtifactCompositeError::Overflow { context }) if context == "indexing canvas x"
    ));
}

#[test]
fn fill_skips_pixels_outside_intersection() {
    let mut pixels = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let canvas = UiRect::new(0, 0, 2, 2);
    let clip = UiRect::new(3, 3, 1, 1);
    fill(
        &mut pixels,
        canvas,
        clip,
        UiRect::new(0, 0, 1, 1),
        [255, 255, 255, 255],
    )
    .expect("no-op clip should not fail");
    assert_eq!(pixels, vec![0; 16]);
}

#[test]
fn fill_blends_pixels_within_intersection() {
    let mut pixels = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let canvas = UiRect::new(0, 0, 2, 2);
    fill(
        &mut pixels,
        canvas,
        UiRect::new(0, 0, 2, 2),
        UiRect::new(0, 0, 2, 1),
        [255, 0, 0, 255],
    )
    .expect("blend into first row");
    assert_eq!(&pixels[0..8], &[255, 0, 0, 255, 255, 0, 0, 255]);
    assert_eq!(&pixels[8..16], &[0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn texture_rejects_zero_dimensions() {
    let mut pixels = vec![0; 4];
    let error = super::texture(
        &mut pixels,
        UiRect::new(0, 0, 1, 1),
        UiRect::new(0, 0, 1, 1),
        UiRect::new(0, 0, 1, 1),
        &paint_texture(0, 1, vec![]),
    )
    .expect_err("zero-width texture rejected");
    assert!(
        matches!(error, ArtifactCompositeError::ZeroTexture { identity } if identity == "test-texture")
    );
}

#[test]
fn texture_rejects_mismatched_byte_length_before_sampling() {
    let mut pixels = vec![0; 4];
    let error = super::texture(
        &mut pixels,
        UiRect::new(0, 0, 1, 1),
        UiRect::new(0, 0, 1, 1),
        UiRect::new(0, 0, 1, 1),
        &paint_texture(1, 1, vec![255, 255, 255]),
    )
    .expect_err("mismatched rgba length rejected");
    assert!(matches!(
        error,
        ArtifactCompositeError::TextureByteLength {
            identity,
            expected,
            actual
        } if identity == "test-texture" && expected == 4 && actual == 3
    ));
}

#[test]
fn source_over_with_zero_alpha_leaves_canvas_unchanged() {
    let mut pixels = vec![0, 0, 0, 0];
    fill(
        &mut pixels,
        UiRect::new(0, 0, 1, 1),
        UiRect::new(0, 0, 1, 1),
        UiRect::new(0, 0, 1, 1),
        [255, 0, 0, 0],
    )
    .expect("transparent source should be skipped");
    assert_eq!(pixels, vec![0, 0, 0, 0]);
}

#[test]
fn rounded_fill_masks_corner_pixels() {
    let mut pixels = vec![0; 36];
    rounded_fill(
        &mut pixels,
        UiRect::new(0, 0, 3, 3),
        UiRect::new(0, 0, 3, 3),
        UiRect::new(0, 0, 3, 3),
        [255, 255, 255, 255],
        1,
    )
    .expect("rounded fill should complete");
    let opaque_pixels = pixels.chunks_exact(4).filter(|pixel| pixel[3] != 0).count();
    assert_eq!(opaque_pixels, 5);
}

#[test]
fn texture_with_zero_area_bounds_is_a_noop() {
    let mut pixels = vec![0; 4 * 3 * 3];
    let noop = texture(
        &mut pixels,
        UiRect::new(0, 0, 3, 3),
        UiRect::new(0, 0, 3, 3),
        UiRect::new(1, 1, 0, 2),
        &paint_texture(1, 2, vec![255, 255, 255, 255, 255, 255, 255, 255]),
    )
    .expect("zero-width texture bounds should short-circuit");
    assert_eq!(noop, ());
    assert!(pixels.iter().all(|value| *value == 0));
}

#[test]
fn rounded_fill_with_disjoint_clip_is_a_noop() {
    let mut pixels = vec![0; 4 * 2 * 2];

    rounded_fill(
        &mut pixels,
        UiRect::new(0, 0, 2, 2),
        UiRect::new(8, 8, 1, 1),
        UiRect::new(0, 0, 2, 2),
        [255, 255, 255, 255],
        1,
    )
    .expect("a disjoint rounded fill must be a no-op");

    assert!(pixels.iter().all(|value| *value == 0));
}

#[test]
fn source_over_rejects_negative_canvas_y_indexing() {
    let mut pixels = vec![0, 0, 0, 0];
    let error = source_over(
        &mut pixels,
        UiRect::new(0, 0, 1, 1),
        0,
        i32::MIN,
        [255, 0, 0, 255],
    )
    .expect_err("a point above the canvas must fail closed");

    assert!(matches!(
        error,
        ArtifactCompositeError::Overflow { context } if context == "indexing canvas y"
    ));
}

#[test]
fn texture_fails_closed_if_its_pixel_storage_changes_after_validation() {
    let mut pixels = vec![0; 4];
    let texture = ChangingTexture::invalidating();

    let error = super::texture(
        &mut pixels,
        UiRect::new(0, 0, 1, 1),
        UiRect::new(0, 0, 1, 1),
        UiRect::new(0, 0, 1, 1),
        &texture,
    )
    .expect_err("a texture that invalidates its storage must fail closed");

    assert!(matches!(
        error,
        ArtifactCompositeError::TexturePixelRange {
            identity,
            start: 0,
            end: 4,
            actual: 0,
        } if identity == "changing-texture"
    ));
}

#[test]
fn changing_texture_type_also_composes_when_storage_remains_stable() {
    let mut pixels = vec![0; 4];
    let texture = ChangingTexture::stable();

    super::texture(
        &mut pixels,
        UiRect::new(0, 0, 1, 1),
        UiRect::new(0, 0, 1, 1),
        UiRect::new(0, 0, 1, 1),
        &texture,
    )
    .expect("stable texture storage should compose");

    assert_eq!(pixels, vec![255, 255, 255, 255]);
}

struct ChangingTexture {
    pixel_reads: Cell<usize>,
    invalidate_after_validation: bool,
}

impl ChangingTexture {
    fn invalidating() -> Self {
        Self {
            pixel_reads: Cell::new(0),
            invalidate_after_validation: true,
        }
    }

    fn stable() -> Self {
        Self {
            pixel_reads: Cell::new(0),
            invalidate_after_validation: false,
        }
    }
}

impl TextureRef for ChangingTexture {
    fn identity(&self) -> &str {
        "changing-texture"
    }

    fn width(&self) -> u32 {
        1
    }

    fn height(&self) -> u32 {
        1
    }

    fn rgba_pixels(&self) -> &[u8] {
        let current = self.pixel_reads.get();
        self.pixel_reads.set(current.saturating_add(1));
        if current == 0 || !self.invalidate_after_validation {
            &[255, 255, 255, 255]
        } else {
            &[]
        }
    }
}
