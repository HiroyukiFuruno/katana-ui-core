use super::*;

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
    let mut pixels = vec![0; 16];
    let canvas = UiRect::new(0, 0, 2, 2);
    let clip = UiRect::new(3, 3, 1, 1);
    assert!(
        require_ok(
            fill(
                &mut pixels,
                canvas,
                clip,
                UiRect::new(0, 0, 1, 1),
                [255, 255, 255, 255]
            ),
            "no-op clip should not fail"
        )
        .is_some()
    );
    assert_eq!(pixels, vec![0; 16]);
}

#[test]
fn fill_blends_pixels_within_intersection() {
    let mut pixels = vec![0; 16];
    let canvas = UiRect::new(0, 0, 2, 2);
    assert!(
        require_ok(
            fill(
                &mut pixels,
                canvas,
                UiRect::new(0, 0, 2, 2),
                UiRect::new(0, 0, 2, 1),
                [255, 0, 0, 255]
            ),
            "blend into first row"
        )
        .is_some()
    );
    assert_eq!(&pixels[0..8], &[255, 0, 0, 255, 255, 0, 0, 255]);
    assert_eq!(&pixels[8..16], &[0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn source_over_with_zero_alpha_leaves_canvas_unchanged() {
    let mut pixels = vec![0, 0, 0, 0];
    assert!(
        require_ok(
            fill(
                &mut pixels,
                UiRect::new(0, 0, 1, 1),
                UiRect::new(0, 0, 1, 1),
                UiRect::new(0, 0, 1, 1),
                [255, 0, 0, 0]
            ),
            "transparent source should be skipped"
        )
        .is_some()
    );
    assert_eq!(pixels, vec![0, 0, 0, 0]);
}

#[test]
fn rounded_fill_masks_corner_pixels() {
    let mut pixels = vec![0; 36];
    assert!(
        require_ok(
            rounded_fill(
                &mut pixels,
                UiRect::new(0, 0, 3, 3),
                UiRect::new(0, 0, 3, 3),
                UiRect::new(0, 0, 3, 3),
                [255, 255, 255, 255],
                1
            ),
            "rounded fill should complete"
        )
        .is_some()
    );
    let opaque_pixels = pixels.chunks_exact(4).filter(|pixel| pixel[3] != 0).count();
    assert_eq!(opaque_pixels, 5);
}

#[test]
fn texture_with_zero_area_bounds_is_a_noop() {
    let mut pixels = vec![0; 4 * 3 * 3];
    assert!(
        require_ok(
            texture(
                &mut pixels,
                UiRect::new(0, 0, 3, 3),
                UiRect::new(0, 0, 3, 3),
                UiRect::new(1, 1, 0, 2),
                &paint_texture(1, 2, vec![255; 8])
            ),
            "zero-width texture bounds should short-circuit"
        )
        .is_some()
    );
    assert!(pixels.iter().all(|value| *value == 0));
}

#[test]
fn rounded_fill_with_disjoint_clip_is_a_noop() {
    let mut pixels = vec![0; 4 * 2 * 2];
    assert!(
        require_ok(
            rounded_fill(
                &mut pixels,
                UiRect::new(0, 0, 2, 2),
                UiRect::new(8, 8, 1, 1),
                UiRect::new(0, 0, 2, 2),
                [255, 255, 255, 255],
                1
            ),
            "a disjoint rounded fill must be a no-op"
        )
        .is_some()
    );
    assert!(pixels.iter().all(|value| *value == 0));
}

#[test]
fn changing_texture_type_also_composes_when_storage_remains_stable() {
    let mut pixels = vec![0; 4];
    let texture = ChangingTexture::stable();
    assert!(
        require_ok(
            super::super::texture(
                &mut pixels,
                UiRect::new(0, 0, 1, 1),
                UiRect::new(0, 0, 1, 1),
                UiRect::new(0, 0, 1, 1),
                &texture
            ),
            "stable texture storage should compose"
        )
        .is_some()
    );
    assert_eq!(pixels, vec![255, 255, 255, 255]);
}
