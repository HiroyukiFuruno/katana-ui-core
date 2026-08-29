use super::super::model::{KucColorEmojiFaceArtifact, KucUnicodeColorGlyphEvidenceInput};
use super::super::types::KucUnicodeColorGlyphEvidenceError;
use katana_ui_core_text_raster::PlatformColorEmojiAvailability;

pub(super) fn validate(
    input: &KucUnicodeColorGlyphEvidenceInput,
) -> Result<(), KucUnicodeColorGlyphEvidenceError> {
    let expected_profile = input.catalog_policy.platform_profile.as_str().to_string();
    let actual_profile = input.profile.as_str().to_string();
    if input.catalog_policy.platform_profile != input.profile
        || input.face.platform_profile != input.profile
    {
        return Err(KucUnicodeColorGlyphEvidenceError::ProfileMismatch {
            expected: expected_profile,
            actual: actual_profile,
        });
    }
    let expected_fingerprint = input.catalog_policy.fingerprint();
    let actual_fingerprint = input.face.catalog_fingerprint;
    if expected_fingerprint != actual_fingerprint {
        return Err(
            KucUnicodeColorGlyphEvidenceError::CatalogFingerprintMismatch {
                expected: expected_fingerprint.to_hex(),
                actual: actual_fingerprint.to_hex(),
            },
        );
    }
    match &input.face.availability {
        PlatformColorEmojiAvailability::Resolved => {}
        PlatformColorEmojiAvailability::Unavailable(_) => {
            return Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnavailable {
                face: Box::new(input.face.clone()),
            });
        }
        PlatformColorEmojiAvailability::Error(_) => {
            return Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiFaceError {
                face: Box::new(input.face.clone()),
            });
        }
    }
    let Some(source_path) = input.face.source_file_path.as_ref() else {
        return Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnpinned {
            profile_id: actual_profile,
        });
    };
    let Some(raw_hash) = input.face.raw_file_sha256 else {
        return Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnpinned {
            profile_id: actual_profile,
        });
    };
    let pinned = input
        .catalog_policy
        .emoji_candidates
        .iter()
        .any(|candidate| {
            candidate.source_file_path == *source_path
                && candidate.expected_raw_file_sha256 == Some(raw_hash)
        });
    if pinned {
        Ok(())
    } else {
        Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnpinned {
            profile_id: actual_profile,
        })
    }
}

pub(super) fn face_artifact(
    input: &KucUnicodeColorGlyphEvidenceInput,
    profile_id: &str,
    catalog_fingerprint: &str,
) -> Result<KucColorEmojiFaceArtifact, KucUnicodeColorGlyphEvidenceError> {
    let Some(source_file_path) = input.face.source_file_path.as_ref() else {
        return Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnpinned {
            profile_id: profile_id.to_string(),
        });
    };
    let Some(raw_file_sha256) = input.face.raw_file_sha256 else {
        return Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnpinned {
            profile_id: profile_id.to_string(),
        });
    };
    Ok(KucColorEmojiFaceArtifact {
        profile_id: profile_id.to_string(),
        family: input.face.family_identity.clone(),
        source_file_path: source_file_path.to_string_lossy().into_owned(),
        raw_file_sha256: raw_file_sha256.to_hex(),
        catalog_fingerprint: catalog_fingerprint.to_string(),
    })
}
