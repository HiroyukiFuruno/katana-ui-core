use super::*;
use crate::text_surface::TextSurfacePaintTexture;

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
