use super::*;
use crate::text_command_surface::unicode_evidence::KucBounds;

const RGBA_CHANNEL_COUNT: usize = 4;

fn crop(bounds: KucBounds, pixels: Vec<[u8; RGBA_CHANNEL_COUNT]>) -> KucRgbaCropEvidence {
    KucRgbaCropEvidence::new(bounds, pixels)
}

#[test]
fn validate_hit_test_rejects_missing_and_wrong_ranges() {
    let hit = KucHitTestObservation {
        target: "star".to_string(),
        query_x: 1,
        query_y: 1,
        byte_start: 2,
        byte_end: 4,
    };
    assert!(matches!(
        validate_hit_test(&[], "star", (2, 4)),
        Err(KucUnicodeColorGlyphEvidenceError::MissingHitTest { target }) if target == "star"
    ));
    assert!(matches!(
        validate_hit_test(&[hit], "star", (0, 1)),
        Err(KucUnicodeColorGlyphEvidenceError::InvalidHitTest { target }) if target == "star"
    ));
}

#[test]
fn validate_hit_test_accepts_matching_observation_for_expected_target() {
    let hit = KucHitTestObservation {
        target: "zwj".to_string(),
        query_x: 4,
        query_y: 6,
        byte_start: 8,
        byte_end: 12,
    };
    assert!(matches!(validate_hit_test(&[hit], "zwj", (8, 12)), Ok(())));
}

#[test]
fn artifact_rejects_empty_malformed_invisible_and_monochrome_crops() {
    let empty = KucBounds::new(0, 0, 1, 1);
    for evidence in [
        crop(empty, Vec::new()),
        crop(empty, vec![[1, 2, 3, 4], [1, 2, 3, 4]]),
        crop(empty, vec![[1, 2, 3, 0]]),
        crop(empty, vec![[1, 1, 1, 4]]),
    ] {
        assert!(matches!(
            artifact("star", &evidence, true),
            Err(KucUnicodeColorGlyphEvidenceError::InvalidCrop { .. })
        ));
    }
    let result = artifact("star", &crop(empty, vec![[1, 2, 3, 4]]), true)
        .expect("visible chromatic crop is accepted");
    assert_eq!(result.pixel_count, 1);
    assert_eq!(result.chromatic_pixel_count, 1);
}

#[test]
fn artifact_rejects_size_mismatch_and_accepts_chromatic_success_path() {
    let mut malformed_size = KucBounds::new(0, 0, 2, 2);
    assert!(matches!(
        artifact("star", &crop(malformed_size, vec![[1, 2, 3, 4]]), true),
        Err(KucUnicodeColorGlyphEvidenceError::InvalidCrop { .. })
    ));

    malformed_size.width = 0;
    assert!(matches!(
        artifact("star", &crop(malformed_size, vec![[1, 2, 3, 4]]), true),
        Err(KucUnicodeColorGlyphEvidenceError::InvalidCrop { .. })
    ));

    let evidence = KucBounds::new(0, 0, 1, 1);
    let result = artifact("star", &crop(evidence, vec![[10, 20, 30, 255]]), true)
        .expect("chromatic pixel should satisfy star crop validation");
    assert_eq!(result.pixel_count, 1);
    assert_eq!(result.chromatic_pixel_count, 1);
}

#[test]
fn artifact_rejects_pixel_count_overflow_before_reading_pixels() {
    let crop = KucRgbaCropEvidence::new(KucBounds::new(0, 0, u32::MAX, 2), Vec::new());
    assert!(matches!(
        artifact("overflow", &crop, false),
        Err(KucUnicodeColorGlyphEvidenceError::InvalidCrop { target, reason })
            if target == "overflow" && reason == "missing"
    ));
}

#[test]
fn artifact_accepts_monochrome_crop_when_chromatic_check_disabled() {
    let evidence = KucRgbaCropEvidence::new(
        KucBounds::new(0, 0, 2, 1),
        vec![[10, 10, 10, 255], [20, 20, 20, 255]],
    );
    let artifact = artifact("control_star", &evidence, false)
        .expect("monochrome crop is allowed when chromatic check is disabled");
    assert_eq!(artifact.pixel_count, 2);
    assert_eq!(artifact.chromatic_pixel_count, 0);
}
