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
