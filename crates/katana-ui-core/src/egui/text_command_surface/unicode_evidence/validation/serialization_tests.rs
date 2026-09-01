use super::super::super::model::{
    KucAccessKitNodeArtifact, KucBounds, KucCaretArtifact, KucCaretObservation,
    KucColorEmojiFaceArtifact, KucImeArtifact, KucImeTraceEvidence, KucRgbaCropArtifact,
    KucRgbaCropEvidence, KucUnicodeColorGlyphEvidence, KucUnicodeColorGlyphEvidenceProfileArtifact,
};
use super::super::catalog;
use super::*;
use crate::render_model::UiRect;
use crate::text_raster::{
    PlatformColorEmojiAvailability, PlatformColorEmojiError, PlatformColorEmojiFaceRecord,
    PlatformColorEmojiUnavailableReason, PlatformEmojiFontCandidate, PlatformFontCatalogPolicy,
    PlatformFontProfile, PlatformFontSha256,
};
use std::path::PathBuf;
use std::{collections::BTreeMap, error::Error};

fn catalog_input() -> super::super::super::model::KucUnicodeColorGlyphEvidenceInput {
    let path = PathBuf::from("kuc-fixture-color-emoji.ttf");
    let hash = PlatformFontSha256::digest(b"fixture");
    let policy = PlatformFontCatalogPolicy::new(
        PlatformFontProfile::Linux,
        Vec::new(),
        Vec::new(),
        vec![
            PlatformEmojiFontCandidate::new(path.clone(), "Noto Color Emoji")
                .with_expected_raw_file_sha256(hash),
        ],
    );
    let face = PlatformColorEmojiFaceRecord {
        platform_profile: PlatformFontProfile::Linux,
        family_identity: "Noto Color Emoji".to_string(),
        source_file_path: Some(path.clone()),
        raw_file_sha256: Some(hash),
        catalog_fingerprint: policy.fingerprint(),
        availability: PlatformColorEmojiAvailability::Resolved,
    };
    super::super::super::model::KucUnicodeColorGlyphEvidenceInput {
        profile: PlatformFontProfile::Linux,
        catalog_policy: policy,
        face,
        final_text: "fixture".to_string(),
        ime: KucImeTraceEvidence {
            preedit_scalars: vec![],
            commit_scalars: vec![],
            preedit_event_seen: true,
            commit_event_seen: true,
        },
        caret: KucCaretObservation::from_ui_rect(UiRect::new(0, 0, 2, 2)),
        hit_tests: Vec::new(),
        star_crop: KucRgbaCropEvidence::new(KucBounds::new(0, 0, 1, 1), vec![[255, 0, 0, 255]]),
        control_crop: KucRgbaCropEvidence::new(KucBounds::new(0, 0, 1, 1), vec![[16, 16, 16, 255]]),
        accesskit_text_input: None,
        accesskit_text_snapshot_hash: "accesskit-hash".to_string(),
        root_frame_hash: "frame-hash".to_string(),
        root_record_hash: "record-hash".to_string(),
        root_rgba_hash: "rgba-hash".to_string(),
    }
}

#[test]
fn hash_pixels_is_stable_and_sensitive_to_rgba_content() {
    let first = hash_pixels(&[[1, 2, 3, 4]]);
    assert_eq!(first, hash_pixels(&[[1, 2, 3, 4]]));
    assert_ne!(first, hash_pixels(&[[1, 2, 3, 5]]));
    assert_eq!(first.len(), 64);
}

#[test]
fn catalog_validate_passes_for_resolved_pinned_face() -> Result<(), String> {
    let input = catalog_input();
    catalog::validate(&input).map_err(|error| format!("catalog validation must pass: {error}"))?;
    let profile_id = input.profile.as_str();
    let fingerprint = input.catalog_policy.fingerprint().to_hex();
    let face = catalog::face_artifact(&input, profile_id, &fingerprint)
        .map_err(|error| format!("catalog face artifact should pass: {error}"))?;
    assert_eq!(face.profile_id, profile_id.to_string());
    assert_eq!(face.catalog_fingerprint, fingerprint);
    Ok(())
}

#[test]
fn catalog_validate_rejects_face_status_and_fingerprint_failures() {
    let mut input = catalog_input();
    input.face.availability = PlatformColorEmojiAvailability::Unavailable(
        PlatformColorEmojiUnavailableReason::MissingCandidates {
            source_file_paths: vec![PathBuf::from("/tmp/kuc-unavailable.ttf")],
        },
    );
    assert!(matches!(
        catalog::validate(&input),
        Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnavailable { .. })
    ));

    let mut input = catalog_input();
    input.face.availability =
        PlatformColorEmojiAvailability::Error(PlatformColorEmojiError::HashMismatch {
            source_file_path: PathBuf::from("/tmp/kuc-hash-mismatch.ttf"),
            expected: PlatformFontSha256::digest(b"expected"),
            actual: PlatformFontSha256::digest(b"actual"),
        });
    assert!(matches!(
        catalog::validate(&input),
        Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiFaceError { .. })
    ));

    let mut input = catalog_input();
    input.catalog_policy =
        PlatformFontCatalogPolicy::new(PlatformFontProfile::Linux, Vec::new(), Vec::new(), vec![]);
    input.face.catalog_fingerprint = input.catalog_policy.fingerprint();
    assert!(matches!(
        catalog::validate(&input),
        Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnpinned { .. })
    ));
}

#[test]
fn catalog_face_artifact_fails_closed_when_face_is_unusable() {
    let mut input = catalog_input();
    input.face.source_file_path = None;
    let profile_id = input.profile.as_str().to_string();
    assert!(matches!(
        catalog::face_artifact(
            &input,
            &profile_id,
            &input.catalog_policy.fingerprint().to_hex()
        ),
        Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnpinned { .. })
    ));
}

#[test]
fn catalog_face_artifact_fails_closed_without_raw_hash() {
    let mut input = catalog_input();
    input.face.raw_file_sha256 = None;
    assert!(matches!(
        catalog::validate(&input),
        Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnpinned { .. })
    ));
    assert!(matches!(
        catalog::face_artifact(
            &input,
            input.profile.as_str(),
            &input.catalog_policy.fingerprint().to_hex()
        ),
        Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnpinned { .. })
    ));
}

#[test]
fn canonical_hash_is_deterministic_for_stable_artifact() {
    let artifact = KucUnicodeColorGlyphEvidence {
        schema: "kana-unicode-evidence".to_string(),
        schema_version: 1,
        profile: KucUnicodeColorGlyphEvidenceProfileArtifact {
            profile_id: "x".to_string(),
            catalog_fingerprint: "f1".to_string(),
        },
        catalog_face: KucColorEmojiFaceArtifact {
            profile_id: "x".to_string(),
            family: "Noto Color Emoji".to_string(),
            source_file_path: "/tmp/example.ttf".to_string(),
            raw_file_sha256: PlatformFontSha256::digest(b"fixture").to_hex(),
            catalog_fingerprint: "f1".to_string(),
        },
        graphemes: Vec::new(),
        ime: KucImeArtifact {
            preedit_scalar_sequence: vec![0xD83D, 0xDE00],
            commit_scalar_sequence: vec![0xD83D, 0xDE01],
            preedit_event_seen: true,
            commit_event_seen: true,
        },
        caret: KucCaretArtifact {
            bounds: KucBounds::new(0, 0, 1, 1),
        },
        hit_tests: Vec::new(),
        star: KucRgbaCropArtifact {
            bounds: KucBounds::new(0, 0, 1, 1),
            rgba_sha256: "s1".to_string(),
            pixel_count: 1,
            chromatic_pixel_count: 1,
        },
        control_star: KucRgbaCropArtifact {
            bounds: KucBounds::new(0, 1, 1, 1),
            rgba_sha256: "s2".to_string(),
            pixel_count: 1,
            chromatic_pixel_count: 0,
        },
        accesskit_text_input: KucAccessKitNodeArtifact {
            node_id: "node".to_string(),
            role: "MultilineTextInput".to_string(),
            value: "value".to_string(),
            scalar_sequence: vec![1],
            bounds: KucBounds::new(0, 0, 10, 10),
        },
        chromatic_pixel_delta: 1,
        accesskit_text_snapshot_hash: "snap".to_string(),
        root_frame_hash: "frame".to_string(),
        root_record_hash: "record".to_string(),
        root_rgba_hash: "rgba".to_string(),
        artifact_sha256: String::new(),
    };
    let first = canonical_hash(&artifact).expect("canonical hash should be computed");
    let second = canonical_hash(&artifact).expect("canonical hash should be stable");
    assert_eq!(first, second);
    assert_eq!(first.len(), 64);
}

#[test]
fn canonical_hash_uses_the_real_json_serialization_route() {
    let artifact = KucUnicodeColorGlyphEvidence {
        schema: "schema".to_string(),
        schema_version: 1,
        profile: KucUnicodeColorGlyphEvidenceProfileArtifact {
            profile_id: "profile".to_string(),
            catalog_fingerprint: "catalog".to_string(),
        },
        catalog_face: KucColorEmojiFaceArtifact {
            profile_id: "profile".to_string(),
            family: "family".to_string(),
            source_file_path: "font.ttf".to_string(),
            raw_file_sha256: "raw".to_string(),
            catalog_fingerprint: "catalog".to_string(),
        },
        graphemes: Vec::new(),
        ime: KucImeArtifact {
            preedit_scalar_sequence: Vec::new(),
            commit_scalar_sequence: Vec::new(),
            preedit_event_seen: true,
            commit_event_seen: true,
        },
        caret: KucCaretArtifact {
            bounds: KucBounds::new(0, 0, 1, 1),
        },
        hit_tests: Vec::new(),
        star: KucRgbaCropArtifact {
            bounds: KucBounds::new(0, 0, 1, 1),
            rgba_sha256: "star".to_string(),
            pixel_count: 1,
            chromatic_pixel_count: 1,
        },
        control_star: KucRgbaCropArtifact {
            bounds: KucBounds::new(0, 0, 1, 1),
            rgba_sha256: "control".to_string(),
            pixel_count: 1,
            chromatic_pixel_count: 0,
        },
        accesskit_text_input: KucAccessKitNodeArtifact {
            node_id: "node".to_string(),
            role: "MultilineTextInput".to_string(),
            value: "value".to_string(),
            scalar_sequence: vec![1],
            bounds: KucBounds::new(0, 0, 1, 1),
        },
        chromatic_pixel_delta: 1,
        accesskit_text_snapshot_hash: "snapshot".to_string(),
        root_frame_hash: "frame".to_string(),
        root_record_hash: "record".to_string(),
        root_rgba_hash: "rgba".to_string(),
        artifact_sha256: String::new(),
    };
    let serialized = serde_json::to_vec(&artifact).expect("artifact is serializable");
    let expected = hex::encode(sha2::Sha256::digest(serialized));
    assert_eq!(
        canonical_hash(&artifact).expect("hash route succeeds"),
        expected
    );
}

#[test]
fn serialization_error_conversion_preserves_a_real_json_serializer_failure() {
    let invalid_json_map = BTreeMap::from([(vec![1_u8, 2], "value")]);
    let source =
        serde_json::to_vec(&invalid_json_map).expect_err("JSON object keys cannot be byte arrays");
    let expected_source = source.source().map(ToString::to_string);
    let error = serialization_error(source);
    assert!(matches!(
        error,
        KucUnicodeColorGlyphEvidenceError::Serialization(message)
            if !message.is_empty() && expected_source.is_none_or(|source| message.contains(&source))
    ));
}
