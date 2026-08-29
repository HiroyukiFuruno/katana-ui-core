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
