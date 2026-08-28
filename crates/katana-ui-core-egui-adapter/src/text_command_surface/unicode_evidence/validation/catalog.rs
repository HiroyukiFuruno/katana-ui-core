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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text_command_surface::unicode_evidence::model::{
        KucBounds, KucCaretObservation, KucHitTestObservation, KucImeTraceEvidence,
        KucRgbaCropEvidence,
    };
    use katana_ui_core_text_raster::{
        PlatformColorEmojiError, PlatformColorEmojiFaceRecord, PlatformColorEmojiUnavailableReason,
        PlatformEmojiFontCandidate, PlatformEmojiFontLoadError, PlatformFontCatalogFingerprint,
        PlatformFontCatalogPolicy, PlatformFontProfile, PlatformFontSha256,
    };
    use std::path::PathBuf;

    fn input() -> KucUnicodeColorGlyphEvidenceInput {
        let path = PathBuf::from("/opaque/font.ttf");
        let hash = PlatformFontSha256::digest(b"font");
        let candidate = PlatformEmojiFontCandidate::new(path.clone(), "Family")
            .with_expected_raw_file_sha256(hash);
        let policy = PlatformFontCatalogPolicy::new(
            PlatformFontProfile::Unsupported,
            Vec::new(),
            Vec::new(),
            vec![candidate],
        );
        KucUnicodeColorGlyphEvidenceInput {
            profile: PlatformFontProfile::Unsupported,
            face: PlatformColorEmojiFaceRecord {
                platform_profile: PlatformFontProfile::Unsupported,
                family_identity: "Family".into(),
                source_file_path: Some(path),
                raw_file_sha256: Some(hash),
                catalog_fingerprint: policy.fingerprint(),
                availability: PlatformColorEmojiAvailability::Resolved,
            },
            catalog_policy: policy,
            final_text: String::new(),
            ime: KucImeTraceEvidence {
                preedit_scalars: Vec::new(),
                commit_scalars: Vec::new(),
                preedit_event_seen: true,
                commit_event_seen: true,
            },
            caret: KucCaretObservation {
                bounds: KucBounds::new(0, 0, 1, 1),
            },
            hit_tests: Vec::<KucHitTestObservation>::new(),
            star_crop: KucRgbaCropEvidence::new(KucBounds::new(0, 0, 1, 1), Vec::new()),
            control_crop: KucRgbaCropEvidence::new(KucBounds::new(0, 0, 1, 1), Vec::new()),
            accesskit_text_snapshot_hash: "a".into(),
            root_frame_hash: "b".into(),
            root_record_hash: "c".into(),
            root_rgba_hash: "d".into(),
        }
    }

    #[test]
    fn catalog_validation_covers_profile_fingerprint_availability_and_pin_paths() {
        let valid = input();
        assert!(validate(&valid).is_ok());

        let mut profile = input();
        profile.profile = PlatformFontProfile::Linux;
        assert!(matches!(
            validate(&profile),
            Err(KucUnicodeColorGlyphEvidenceError::ProfileMismatch { .. })
        ));

        let mut fingerprint = input();
        fingerprint.face.catalog_fingerprint = PlatformFontCatalogFingerprint::from_bytes([9; 32]);
        assert!(matches!(
            validate(&fingerprint),
            Err(KucUnicodeColorGlyphEvidenceError::CatalogFingerprintMismatch { .. })
        ));

        let mut unavailable = input();
        unavailable.face.availability = PlatformColorEmojiAvailability::Unavailable(
            PlatformColorEmojiUnavailableReason::NoCandidates,
        );
        assert!(matches!(
            validate(&unavailable),
            Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnavailable { .. })
        ));

        let mut face_error = input();
        face_error.face.availability =
            PlatformColorEmojiAvailability::Error(PlatformColorEmojiError::CandidateLoad {
                source_file_path: PathBuf::from("/opaque/font.ttf"),
                error: PlatformEmojiFontLoadError::Missing {
                    source_file_path: PathBuf::from("/opaque/font.ttf"),
                },
            });
        assert!(matches!(
            validate(&face_error),
            Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiFaceError { .. })
        ));

        let mut no_path = input();
        no_path.face.source_file_path = None;
        assert!(matches!(
            validate(&no_path),
            Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnpinned { .. })
        ));

        let mut no_hash = input();
        no_hash.face.raw_file_sha256 = None;
        assert!(matches!(
            validate(&no_hash),
            Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnpinned { .. })
        ));

        let mut wrong_hash = input();
        wrong_hash.face.raw_file_sha256 = Some(PlatformFontSha256::digest(b"other"));
        assert!(matches!(
            validate(&wrong_hash),
            Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnpinned { .. })
        ));
    }

    #[test]
    fn face_artifact_requires_pinned_source_and_hash() {
        let valid = input();
        let artifact = face_artifact(&valid, "unsupported", "fingerprint")
            .expect("pinned face should project into evidence");
        assert_eq!(artifact.family, "Family");
        assert_eq!(artifact.profile_id, "unsupported");

        let mut no_path = input();
        no_path.face.source_file_path = None;
        assert!(face_artifact(&no_path, "unsupported", "fingerprint").is_err());

        let mut no_hash = input();
        no_hash.face.raw_file_sha256 = None;
        assert!(face_artifact(&no_hash, "unsupported", "fingerprint").is_err());
    }
}
