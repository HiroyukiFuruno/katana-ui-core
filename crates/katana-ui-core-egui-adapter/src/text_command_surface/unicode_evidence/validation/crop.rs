use super::super::constants::ALPHA_CHANNEL_INDEX;
use super::super::model::{KucHitTestObservation, KucRgbaCropArtifact, KucRgbaCropEvidence};
use super::super::types::KucUnicodeColorGlyphEvidenceError;

pub(super) fn validate_hit_test(
    hits: &[KucHitTestObservation],
    target: &str,
    expected: (usize, usize),
) -> Result<(), KucUnicodeColorGlyphEvidenceError> {
    let Some(hit) = hits.iter().find(|hit| hit.target == target) else {
        return Err(KucUnicodeColorGlyphEvidenceError::MissingHitTest {
            target: target.to_string(),
        });
    };
    if (hit.byte_start, hit.byte_end) != expected {
        return Err(KucUnicodeColorGlyphEvidenceError::InvalidHitTest {
            target: target.to_string(),
        });
    }
    Ok(())
}

pub(super) fn artifact(
    target: &str,
    crop: &KucRgbaCropEvidence,
    require_chromatic: bool,
) -> Result<KucRgbaCropArtifact, KucUnicodeColorGlyphEvidenceError> {
    let Some(expected_pixels) = crop
        .bounds
        .width
        .checked_mul(crop.bounds.height)
        .and_then(|pixels| usize::try_from(pixels).ok())
    else {
        return Err(KucUnicodeColorGlyphEvidenceError::InvalidCrop {
            target: target.to_string(),
            reason: "missing",
        });
    };
    if crop.bounds.width == 0 || crop.bounds.height == 0 || crop.pixels.is_empty() {
        return Err(KucUnicodeColorGlyphEvidenceError::InvalidCrop {
            target: target.to_string(),
            reason: "empty",
        });
    }
    if crop.pixels.len() != expected_pixels {
        return Err(KucUnicodeColorGlyphEvidenceError::InvalidCrop {
            target: target.to_string(),
            reason: "missing",
        });
    }
    let visible = crop
        .pixels
        .iter()
        .filter(|pixel| pixel[ALPHA_CHANNEL_INDEX] != 0)
        .count();
    if visible == 0 {
        return Err(KucUnicodeColorGlyphEvidenceError::InvalidCrop {
            target: target.to_string(),
            reason: "empty",
        });
    }
    let chromatic_pixel_count = crop
        .pixels
        .iter()
        .filter(|pixel| {
            pixel[ALPHA_CHANNEL_INDEX] != 0 && (pixel[0] != pixel[1] || pixel[1] != pixel[2])
        })
        .count();
    if require_chromatic && chromatic_pixel_count == 0 {
        return Err(KucUnicodeColorGlyphEvidenceError::InvalidCrop {
            target: target.to_string(),
            reason: "monochrome",
        });
    }
    Ok(KucRgbaCropArtifact {
        bounds: crop.bounds,
        rgba_sha256: super::serialization::hash_pixels(&crop.pixels),
        pixel_count: crop.pixels.len(),
        chromatic_pixel_count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text_command_surface::unicode_evidence::model::KucBounds;

    fn crop(bounds: KucBounds, pixels: Vec<[u8; 4]>) -> KucRgbaCropEvidence {
        KucRgbaCropEvidence::new(bounds, pixels)
    }

    #[test]
    fn hit_validation_covers_missing_invalid_and_valid_observations() {
        assert!(validate_hit_test(&[], "star", (1, 2)).is_err());
        let hit = KucHitTestObservation {
            target: "star".into(),
            query_x: 1,
            query_y: 2,
            byte_start: 0,
            byte_end: 1,
        };
        assert!(validate_hit_test(std::slice::from_ref(&hit), "star", (1, 2)).is_err());
        assert!(validate_hit_test(&[hit], "star", (0, 1)).is_ok());
    }

    #[test]
    fn crop_artifact_validation_covers_all_rejection_reasons_and_success() {
        assert!(
            artifact(
                "star",
                &crop(KucBounds::new(0, 0, u32::MAX, 2), vec![]),
                false
            )
            .is_err()
        );
        assert!(artifact("star", &crop(KucBounds::new(0, 0, 0, 1), vec![]), false).is_err());
        assert!(
            artifact(
                "star",
                &crop(KucBounds::new(0, 0, 2, 1), vec![[1, 1, 1, 255]]),
                false
            )
            .is_err()
        );
        assert!(
            artifact(
                "star",
                &crop(KucBounds::new(0, 0, 1, 1), vec![[0, 0, 0, 0]]),
                false
            )
            .is_err()
        );
        assert!(
            artifact(
                "star",
                &crop(KucBounds::new(0, 0, 1, 1), vec![[8, 8, 8, 255]]),
                true
            )
            .is_err()
        );

        let artifact = artifact(
            "star",
            &crop(
                KucBounds::new(0, 0, 2, 1),
                vec![[8, 8, 8, 255], [1, 2, 3, 255]],
            ),
            true,
        )
        .expect("chromatic crop should be accepted");
        assert_eq!(artifact.pixel_count, 2);
        assert_eq!(artifact.chromatic_pixel_count, 1);
        assert!(!artifact.rgba_sha256.is_empty());
    }
}
