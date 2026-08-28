mod catalog;
mod crop;
mod grapheme;
mod serialization;

use super::constants::{
    CONTROL_STAR_TEXT, IME_COMMIT_TEXT, IME_PREEDIT_TEXT, STAR_TEXT, UNICODE_EVIDENCE_SCHEMA,
    UNICODE_EVIDENCE_SCHEMA_VERSION, ZWJ_TEXT,
};
use super::model::{
    KucCaretArtifact, KucRgbaCropArtifact, KucUnicodeColorGlyphEvidence,
    KucUnicodeColorGlyphEvidenceInput, KucUnicodeColorGlyphEvidenceProfileArtifact,
};
use super::types::KucUnicodeColorGlyphEvidenceError;
pub(super) use super::validation_types::KucUnicodeColorGlyphEvidenceBuilder;

impl KucUnicodeColorGlyphEvidenceBuilder {
    pub fn build(
        input: KucUnicodeColorGlyphEvidenceInput,
    ) -> Result<KucUnicodeColorGlyphEvidence, KucUnicodeColorGlyphEvidenceError> {
        catalog::validate(&input)?;
        let ranges = grapheme::artifacts(&input.final_text);
        let star_range = grapheme::required_range(&input.final_text, STAR_TEXT, &ranges)?;
        let control_range =
            grapheme::required_range(&input.final_text, CONTROL_STAR_TEXT, &ranges)?;
        let zwj_range = grapheme::required_range(&input.final_text, ZWJ_TEXT, &ranges)?;
        validate_ime_scalars(&input)?;
        validate_ime_events(&input)?;
        validate_caret(&input)?;
        crop::validate_hit_test(&input.hit_tests, "star", star_range)?;
        crop::validate_hit_test(&input.hit_tests, "control_star", control_range)?;
        crop::validate_hit_test(&input.hit_tests, "zwj", zwj_range)?;
        let star = crop::artifact("star", &input.star_crop, true)?;
        let control_star = crop::artifact("control_star", &input.control_crop, false)?;
        let chromatic_pixel_delta = chromatic_pixel_delta(&star, &control_star)?;
        validate_hashes(&input)?;

        let profile_id = input.profile.as_str().to_string();
        let catalog_fingerprint = input.catalog_policy.fingerprint().to_hex();
        let face = catalog::face_artifact(&input, &profile_id, &catalog_fingerprint)?;
        let mut artifact = KucUnicodeColorGlyphEvidence {
            schema: UNICODE_EVIDENCE_SCHEMA.to_string(),
            schema_version: UNICODE_EVIDENCE_SCHEMA_VERSION,
            profile: KucUnicodeColorGlyphEvidenceProfileArtifact {
                profile_id,
                catalog_fingerprint,
            },
            catalog_face: face,
            graphemes: ranges,
            ime: super::model::KucImeArtifact {
                preedit_scalar_sequence: input.ime.preedit_scalars,
                commit_scalar_sequence: input.ime.commit_scalars,
                preedit_event_seen: input.ime.preedit_event_seen,
                commit_event_seen: input.ime.commit_event_seen,
            },
            caret: KucCaretArtifact {
                bounds: input.caret.bounds,
            },
            hit_tests: input.hit_tests.into_iter().map(hit_test_artifact).collect(),
            star,
            control_star,
            chromatic_pixel_delta,
            accesskit_text_snapshot_hash: input.accesskit_text_snapshot_hash,
            root_frame_hash: input.root_frame_hash,
            root_record_hash: input.root_record_hash,
            root_rgba_hash: input.root_rgba_hash,
            artifact_sha256: String::new(),
        };
        artifact.artifact_sha256 = serialization::canonical_hash(&artifact)?;
        Ok(artifact)
    }
}

fn validate_ime_scalars(
    input: &KucUnicodeColorGlyphEvidenceInput,
) -> Result<(), KucUnicodeColorGlyphEvidenceError> {
    grapheme::validate_scalar_sequence(
        &input.ime.preedit_scalars,
        IME_PREEDIT_TEXT,
        &grapheme::scalars(IME_PREEDIT_TEXT),
    )?;
    grapheme::validate_scalar_sequence(
        &input.ime.commit_scalars,
        IME_COMMIT_TEXT,
        &grapheme::scalars(IME_COMMIT_TEXT),
    )
}

fn validate_ime_events(
    input: &KucUnicodeColorGlyphEvidenceInput,
) -> Result<(), KucUnicodeColorGlyphEvidenceError> {
    if !input.ime.preedit_event_seen {
        return Err(KucUnicodeColorGlyphEvidenceError::MissingImeEvent { kind: "preedit" });
    }
    if !input.ime.commit_event_seen {
        return Err(KucUnicodeColorGlyphEvidenceError::MissingImeEvent { kind: "commit" });
    }
    Ok(())
}

fn validate_caret(
    input: &KucUnicodeColorGlyphEvidenceInput,
) -> Result<(), KucUnicodeColorGlyphEvidenceError> {
    if input.caret.bounds.width == 0 || input.caret.bounds.height == 0 {
        Err(KucUnicodeColorGlyphEvidenceError::InvalidCaret)
    } else {
        Ok(())
    }
}

fn chromatic_pixel_delta(
    star: &KucRgbaCropArtifact,
    control_star: &KucRgbaCropArtifact,
) -> Result<i64, KucUnicodeColorGlyphEvidenceError> {
    if star == control_star {
        return Err(KucUnicodeColorGlyphEvidenceError::IndistinguishableCrops);
    }
    let delta = star
        .chromatic_pixel_count
        .saturating_sub(control_star.chromatic_pixel_count);
    if delta == 0 {
        return Err(KucUnicodeColorGlyphEvidenceError::IndistinguishableCrops);
    }
    i64::try_from(delta).map_err(|_| {
        KucUnicodeColorGlyphEvidenceError::Serialization(
            "chromatic pixel delta exceeds i64".to_string(),
        )
    })
}

fn validate_hashes(
    input: &KucUnicodeColorGlyphEvidenceInput,
) -> Result<(), KucUnicodeColorGlyphEvidenceError> {
    [
        (
            "accesskit_text_snapshot_hash",
            input.accesskit_text_snapshot_hash.as_str(),
        ),
        ("root_frame_hash", input.root_frame_hash.as_str()),
        ("root_record_hash", input.root_record_hash.as_str()),
        ("root_rgba_hash", input.root_rgba_hash.as_str()),
    ]
    .into_iter()
    .find_map(|(field, value)| value.is_empty().then_some(field))
    .map_or(Ok(()), |field| {
        Err(KucUnicodeColorGlyphEvidenceError::EmptyEvidenceHash { field })
    })
}

fn hit_test_artifact(hit: super::model::KucHitTestObservation) -> super::model::KucHitTestArtifact {
    super::model::KucHitTestArtifact {
        target: hit.target,
        query_x: hit.query_x,
        query_y: hit.query_y,
        byte_start: hit.byte_start,
        byte_end: hit.byte_end,
    }
}

#[cfg(test)]
mod tests {
    use super::super::model::{
        KucBounds, KucCaretObservation, KucHitTestObservation, KucImeTraceEvidence,
        KucRgbaCropEvidence,
    };
    use super::*;
    use katana_ui_core_text_raster::{
        PlatformColorEmojiAvailability, PlatformColorEmojiFaceRecord,
        PlatformFontCatalogFingerprint, PlatformFontCatalogPolicy, PlatformFontProfile,
    };

    fn input() -> KucUnicodeColorGlyphEvidenceInput {
        KucUnicodeColorGlyphEvidenceInput {
            profile: PlatformFontProfile::Unsupported,
            catalog_policy: PlatformFontCatalogPolicy::new(
                PlatformFontProfile::Unsupported,
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
            face: PlatformColorEmojiFaceRecord {
                platform_profile: PlatformFontProfile::Unsupported,
                family_identity: String::new(),
                source_file_path: None,
                raw_file_sha256: None,
                catalog_fingerprint: PlatformFontCatalogFingerprint::from_bytes([0; 32]),
                availability: PlatformColorEmojiAvailability::Resolved,
            },
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
            hit_tests: Vec::new(),
            star_crop: KucRgbaCropEvidence::new(KucBounds::new(0, 0, 1, 1), Vec::new()),
            control_crop: KucRgbaCropEvidence::new(KucBounds::new(0, 0, 1, 1), Vec::new()),
            accesskit_text_snapshot_hash: "a".into(),
            root_frame_hash: "b".into(),
            root_record_hash: "c".into(),
            root_rgba_hash: "d".into(),
        }
    }

    #[test]
    fn ime_caret_hash_and_hit_artifact_helpers_cover_all_boundaries() {
        let mut value = input();
        assert!(validate_ime_events(&value).is_ok());
        value.ime.preedit_event_seen = false;
        assert!(validate_ime_events(&value).is_err());
        value.ime.preedit_event_seen = true;
        value.ime.commit_event_seen = false;
        assert!(validate_ime_events(&value).is_err());

        value.caret.bounds.width = 0;
        assert!(validate_caret(&value).is_err());
        value.caret.bounds.width = 1;
        value.caret.bounds.height = 0;
        assert!(validate_caret(&value).is_err());
        value.caret.bounds.height = 1;
        assert!(validate_caret(&value).is_ok());

        assert!(validate_hashes(&value).is_ok());
        value.root_record_hash.clear();
        assert!(matches!(
            validate_hashes(&value),
            Err(KucUnicodeColorGlyphEvidenceError::EmptyEvidenceHash {
                field: "root_record_hash"
            })
        ));

        let artifact = hit_test_artifact(KucHitTestObservation {
            target: "opaque".into(),
            query_x: 2,
            query_y: 3,
            byte_start: 4,
            byte_end: 5,
        });
        assert_eq!((artifact.query_x, artifact.query_y), (2, 3));
        assert_eq!((artifact.byte_start, artifact.byte_end), (4, 5));
    }

    #[test]
    fn ime_scalar_validation_rejects_preedit_and_commit_mismatches() {
        let mut value = input();
        value.ime.preedit_scalars = grapheme::scalars(IME_PREEDIT_TEXT);
        value.ime.commit_scalars = grapheme::scalars(IME_COMMIT_TEXT);
        assert!(validate_ime_scalars(&value).is_ok());
        value.ime.preedit_scalars.clear();
        assert!(validate_ime_scalars(&value).is_err());
        value.ime.preedit_scalars = grapheme::scalars(IME_PREEDIT_TEXT);
        value.ime.commit_scalars.clear();
        assert!(validate_ime_scalars(&value).is_err());
    }

    #[test]
    fn chromatic_delta_rejects_equal_zero_and_unrepresentable_values() {
        let artifact = |count| KucRgbaCropArtifact {
            bounds: KucBounds::new(0, 0, 1, 1),
            rgba_sha256: "hash".into(),
            pixel_count: 1,
            chromatic_pixel_count: count,
        };
        assert!(chromatic_pixel_delta(&artifact(1), &artifact(1)).is_err());
        assert!(chromatic_pixel_delta(&artifact(0), &artifact(1)).is_err());
        assert_eq!(
            chromatic_pixel_delta(&artifact(2), &artifact(1)).ok(),
            Some(1)
        );
        assert!(chromatic_pixel_delta(&artifact(usize::MAX), &artifact(0)).is_err());
    }
}
