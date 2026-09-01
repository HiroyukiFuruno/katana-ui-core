use super::*;

#[test]
fn texture_rejects_zero_dimensions() {
    let mut pixels = vec![0; 4];
    let Some(error) = require_err(
        super::super::texture(
            &mut pixels,
            UiRect::new(0, 0, 1, 1),
            UiRect::new(0, 0, 1, 1),
            UiRect::new(0, 0, 1, 1),
            &paint_texture(0, 1, vec![]),
        ),
        "zero-width texture rejected",
    ) else {
        return;
    };
    assert!(
        matches!(error, ArtifactCompositeError::ZeroTexture { identity } if identity == "test-texture")
    );
}

#[test]
fn texture_rejects_mismatched_byte_length_before_sampling() {
    let mut pixels = vec![0; 4];
    let Some(error) = require_err(
        super::super::texture(
            &mut pixels,
            UiRect::new(0, 0, 1, 1),
            UiRect::new(0, 0, 1, 1),
            UiRect::new(0, 0, 1, 1),
            &paint_texture(1, 1, vec![255, 255, 255]),
        ),
        "mismatched rgba length rejected",
    ) else {
        return;
    };
    assert!(
        matches!(error, ArtifactCompositeError::TextureByteLength { identity, expected, actual } if identity == "test-texture" && expected == 4 && actual == 3)
    );
}

#[test]
fn source_over_rejects_negative_canvas_y_indexing() {
    let mut pixels = vec![0, 0, 0, 0];
    let Some(error) = require_err(
        source_over(
            &mut pixels,
            UiRect::new(0, 0, 1, 1),
            0,
            i32::MIN,
            [255, 0, 0, 255],
        ),
        "a point above the canvas must fail closed",
    ) else {
        return;
    };
    assert!(
        matches!(error, ArtifactCompositeError::Overflow { context } if context == "indexing canvas y")
    );
}

#[test]
fn texture_fails_closed_if_its_pixel_storage_changes_after_validation() {
    let mut pixels = vec![0; 4];
    let texture = ChangingTexture::invalidating();
    let Some(error) = require_err(
        super::super::texture(
            &mut pixels,
            UiRect::new(0, 0, 1, 1),
            UiRect::new(0, 0, 1, 1),
            UiRect::new(0, 0, 1, 1),
            &texture,
        ),
        "a texture that invalidates its storage must fail closed",
    ) else {
        return;
    };
    assert!(
        matches!(error, ArtifactCompositeError::TexturePixelRange { identity, start: 0, end: 4, actual: 0 } if identity == "changing-texture")
    );
}
