#![cfg(feature = "egui")]
use katana_ui_core::egui::text_command_surface::{
    CONTROL_STAR_TEXT, IME_COMMIT_TEXT, IME_PREEDIT_TEXT, KucAccessKitNodeObservation, KucBounds,
    KucCaretObservation, KucHitTestObservation, KucImeTraceEvidence, KucRgbaCropEvidence,
    KucUnicodeColorGlyphEvidenceBuilder, KucUnicodeColorGlyphEvidenceCapture,
    KucUnicodeColorGlyphEvidenceError, KucUnicodeColorGlyphEvidenceInput,
    KucUnicodeColorGlyphEvidenceOptions, STAR_TEXT, ZWJ_TEXT,
};
use katana_ui_core::render_model::UiRect;
use katana_ui_core::text_raster::{
    PlatformColorEmojiAvailability, PlatformColorEmojiError, PlatformColorEmojiFaceRecord,
    PlatformColorEmojiUnavailableReason, PlatformEmojiFontCandidate, PlatformFontCatalog,
    PlatformFontCatalogPolicy, PlatformFontProfile, PlatformFontSha256, PlatformTextGraphemeRange,
};
use std::path::PathBuf;

const FIXTURE_TEXT: &str = "日本語 ⭐️ ☆ 👩‍💻";

fn fixture_input() -> KucUnicodeColorGlyphEvidenceInput {
    let path = PathBuf::from("kuc-fixture-color-emoji.ttf");
    let font_hash = PlatformFontSha256::digest(b"kuc fixture color face");
    let policy = PlatformFontCatalogPolicy::new(
        PlatformFontProfile::Linux,
        Vec::new(),
        Vec::new(),
        vec![
            PlatformEmojiFontCandidate::new(path.clone(), "Noto Color Emoji")
                .with_expected_raw_file_sha256(font_hash),
        ],
    );
    let face = PlatformColorEmojiFaceRecord {
        platform_profile: PlatformFontProfile::Linux,
        family_identity: "Noto Color Emoji".to_string(),
        source_file_path: Some(path),
        raw_file_sha256: Some(font_hash),
        catalog_fingerprint: policy.fingerprint(),
        availability: PlatformColorEmojiAvailability::Resolved,
    };
    let ranges = PlatformTextGraphemeRange::ranges(FIXTURE_TEXT);
    let range_for = |target: &str| {
        ranges
            .iter()
            .find(|range| FIXTURE_TEXT.get(range.byte_start..range.byte_end) == Some(target))
            .map(|range| (range.byte_start, range.byte_end))
            .unwrap_or((0, 0))
    };
    KucUnicodeColorGlyphEvidenceInput {
        profile: PlatformFontProfile::Linux,
        catalog_policy: policy,
        face,
        final_text: FIXTURE_TEXT.to_string(),
        ime: KucImeTraceEvidence {
            preedit_scalars: IME_PREEDIT_TEXT.chars().map(u32::from).collect(),
            commit_scalars: IME_COMMIT_TEXT.chars().map(u32::from).collect(),
            preedit_event_seen: true,
            commit_event_seen: true,
        },
        caret: KucCaretObservation::from_ui_rect(UiRect::new(0, 0, 2, 16)),
        hit_tests: [
            ("star", STAR_TEXT),
            ("control_star", CONTROL_STAR_TEXT),
            ("zwj", ZWJ_TEXT),
        ]
        .into_iter()
        .map(|(target, value)| {
            let (byte_start, byte_end) = range_for(value);
            KucHitTestObservation {
                target: target.to_string(),
                query_x: 1,
                query_y: 1,
                byte_start,
                byte_end,
            }
        })
        .collect(),
        star_crop: KucRgbaCropEvidence::new(
            KucBounds::new(0, 0, 2, 2),
            vec![
                [255, 0, 0, 255],
                [0, 255, 0, 255],
                [0, 0, 255, 255],
                [255, 255, 0, 255],
            ],
        ),
        control_crop: KucRgbaCropEvidence::new(
            KucBounds::new(0, 0, 2, 2),
            vec![
                [80, 80, 80, 255],
                [120, 120, 120, 255],
                [160, 160, 160, 255],
                [200, 200, 200, 255],
            ],
        ),
        accesskit_text_input: Some(KucAccessKitNodeObservation {
            node_id: "fixture-text-input".to_string(),
            role: "MultilineTextInput".to_string(),
            value: FIXTURE_TEXT.to_string(),
            scalar_sequence: FIXTURE_TEXT.chars().map(u32::from).collect(),
            bounds: KucBounds::new(0, 0, 320, 120),
        }),
        accesskit_text_snapshot_hash: "accesskit-hash".to_string(),
        root_frame_hash: "root-frame-hash".to_string(),
        root_record_hash: "root-record-hash".to_string(),
        root_rgba_hash: "root-rgba-hash".to_string(),
    }
}

#[test]
fn fixture_builder_emits_stable_canonical_artifact_and_hash() -> Result<(), String> {
    let first = KucUnicodeColorGlyphEvidenceBuilder::build(fixture_input())
        .map_err(|error| error.to_string())?;
    let second = KucUnicodeColorGlyphEvidenceBuilder::build(fixture_input())
        .map_err(|error| error.to_string())?;
    assert_eq!(
        first.canonical_json().map_err(|error| error.to_string())?,
        second.canonical_json().map_err(|error| error.to_string())?
    );
    assert_eq!(first.artifact_sha256(), second.artifact_sha256());
    let canonical_json = first.canonical_json().map_err(|error| error.to_string())?;
    assert!(canonical_json.starts_with("{\"schema\":"));
    assert!(canonical_json.contains("\"artifact_sha256\":"));
    assert!(first.chromatic_pixel_delta > 0);
    Ok(())
}

#[test]
fn fixture_builder_rejects_profile_and_catalog_fingerprint_mismatch() {
    let mut profile_mismatch = fixture_input();
    profile_mismatch.profile = PlatformFontProfile::Windows;
    assert!(matches!(
        KucUnicodeColorGlyphEvidenceBuilder::build(profile_mismatch),
        Err(KucUnicodeColorGlyphEvidenceError::ProfileMismatch { .. })
    ));

    let mut fingerprint_mismatch = fixture_input();
    fingerprint_mismatch.face.catalog_fingerprint =
        katana_ui_core::text_raster::PlatformFontCatalogFingerprint::from_bytes([0xAB; 32]);
    assert!(matches!(
        KucUnicodeColorGlyphEvidenceBuilder::build(fingerprint_mismatch),
        Err(KucUnicodeColorGlyphEvidenceError::CatalogFingerprintMismatch { .. })
    ));
}

#[test]
fn fixture_builder_rejects_unavailable_color_emoji_face() {
    let mut unavailable_face = fixture_input();
    unavailable_face.face.availability = PlatformColorEmojiAvailability::Unavailable(
        PlatformColorEmojiUnavailableReason::MissingCandidates {
            source_file_paths: vec![PathBuf::from("missing-color-emoji.ttf")],
        },
    );
    assert!(matches!(
        KucUnicodeColorGlyphEvidenceBuilder::build(unavailable_face),
        Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnavailable { .. })
    ));
}

#[test]
fn fixture_builder_rejects_font_error_as_fail_closed_path() {
    let mut face_error = fixture_input();
    face_error.face.availability =
        PlatformColorEmojiAvailability::Error(PlatformColorEmojiError::HashMismatch {
            source_file_path: PathBuf::from("mismatch.ttf"),
            expected: PlatformFontSha256::digest(b"expected"),
            actual: PlatformFontSha256::digest(b"actual"),
        });
    assert!(matches!(
        KucUnicodeColorGlyphEvidenceBuilder::build(face_error),
        Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiFaceError { .. })
    ));
}

#[test]
fn fixture_builder_accepts_pinned_catalog_face_and_crop_signature() -> Result<(), String> {
    let artifact = KucUnicodeColorGlyphEvidenceBuilder::build(fixture_input())
        .map_err(|error| error.to_string())?;
    assert_eq!(
        artifact.profile.profile_id,
        PlatformFontProfile::Linux.as_str()
    );
    assert_eq!(
        artifact.catalog_face.catalog_fingerprint,
        artifact.profile.catalog_fingerprint
    );
    assert!(artifact.chromatic_pixel_delta > 0);
    assert_ne!(artifact.star.rgba_sha256, artifact.control_star.rgba_sha256);
    Ok(())
}

#[test]
fn fixture_builder_rejects_unpinned_vs16_and_invalid_crops() {
    let mut unpinned = fixture_input();
    unpinned.catalog_policy.emoji_candidates[0].expected_raw_file_sha256 = None;
    unpinned.face.catalog_fingerprint = unpinned.catalog_policy.fingerprint();
    assert!(matches!(
        KucUnicodeColorGlyphEvidenceBuilder::build(unpinned),
        Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnpinned { .. })
    ));

    let mut no_vs16 = fixture_input();
    no_vs16.final_text = no_vs16.final_text.replace(STAR_TEXT, "⭐");
    assert!(matches!(
        KucUnicodeColorGlyphEvidenceBuilder::build(no_vs16),
        Err(KucUnicodeColorGlyphEvidenceError::RequiredGraphemeMissing { .. })
    ));

    let mut monochrome = fixture_input();
    monochrome.star_crop =
        KucRgbaCropEvidence::new(KucBounds::new(0, 0, 2, 2), vec![[100, 100, 100, 255]; 4]);
    assert!(matches!(
        KucUnicodeColorGlyphEvidenceBuilder::build(monochrome),
        Err(KucUnicodeColorGlyphEvidenceError::InvalidCrop { .. })
    ));

    let mut indistinguishable = fixture_input();
    indistinguishable.control_crop = indistinguishable.star_crop.clone();
    assert!(matches!(
        KucUnicodeColorGlyphEvidenceBuilder::build(indistinguishable),
        Err(KucUnicodeColorGlyphEvidenceError::IndistinguishableCrops)
    ));
}

#[test]
fn fixture_builder_rejects_missing_catalog_pin_material() {
    let mut missing_path = fixture_input();
    missing_path.face.source_file_path = None;
    assert!(matches!(
        KucUnicodeColorGlyphEvidenceBuilder::build(missing_path),
        Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnpinned { .. })
    ));

    let mut missing_hash = fixture_input();
    missing_hash.face.raw_file_sha256 = None;
    assert!(matches!(
        KucUnicodeColorGlyphEvidenceBuilder::build(missing_hash),
        Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnpinned { .. })
    ));
}

#[test]
fn fixture_builder_rejects_changed_ime_scalar_sequences() {
    let mut changed_preedit = fixture_input();
    changed_preedit.ime.preedit_scalars = vec![0x41];
    assert!(matches!(
        KucUnicodeColorGlyphEvidenceBuilder::build(changed_preedit),
        Err(KucUnicodeColorGlyphEvidenceError::ExpectedScalarSequenceChanged { target, .. })
            if target == IME_PREEDIT_TEXT
    ));

    let mut changed_commit = fixture_input();
    changed_commit.ime.commit_scalars = vec![0x42];
    assert!(matches!(
        KucUnicodeColorGlyphEvidenceBuilder::build(changed_commit),
        Err(KucUnicodeColorGlyphEvidenceError::ExpectedScalarSequenceChanged { target, .. })
            if target == IME_COMMIT_TEXT
    ));
}

#[test]
fn fixture_builder_rejects_any_missing_evidence_hash() {
    let mut no_accesskit_snapshot = fixture_input();
    no_accesskit_snapshot.accesskit_text_snapshot_hash.clear();
    assert!(matches!(
        KucUnicodeColorGlyphEvidenceBuilder::build(no_accesskit_snapshot),
        Err(KucUnicodeColorGlyphEvidenceError::EmptyEvidenceHash { field })
            if field == "accesskit_text_snapshot_hash"
    ));

    let mut no_frame_hash = fixture_input();
    no_frame_hash.root_frame_hash.clear();
    assert!(matches!(
        KucUnicodeColorGlyphEvidenceBuilder::build(no_frame_hash),
        Err(KucUnicodeColorGlyphEvidenceError::EmptyEvidenceHash { field }) if field == "root_frame_hash"
    ));

    let mut no_record_hash = fixture_input();
    no_record_hash.root_record_hash.clear();
    assert!(matches!(
        KucUnicodeColorGlyphEvidenceBuilder::build(no_record_hash),
        Err(KucUnicodeColorGlyphEvidenceError::EmptyEvidenceHash { field }) if field == "root_record_hash"
    ));

    let mut no_rgba_hash = fixture_input();
    no_rgba_hash.root_rgba_hash.clear();
    assert!(matches!(
        KucUnicodeColorGlyphEvidenceBuilder::build(no_rgba_hash),
        Err(KucUnicodeColorGlyphEvidenceError::EmptyEvidenceHash { field }) if field == "root_rgba_hash"
    ));
}

#[test]
fn current_platform_trace_is_typed_unavailable_or_runs_actual_pinned_root() {
    let mut options = KucUnicodeColorGlyphEvidenceOptions::default();
    let policy = options.config.catalog_policy();
    let Some(candidate) = policy.emoji_candidates.first() else {
        assert!(matches!(
            KucUnicodeColorGlyphEvidenceCapture::capture(options),
            Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnavailable { .. })
        ));
        return;
    };
    let Ok(bytes) = std::fs::read(&candidate.source_file_path) else {
        assert!(matches!(
            KucUnicodeColorGlyphEvidenceCapture::capture(options),
            Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnavailable { .. })
        ));
        return;
    };
    options.config = options
        .config
        .with_emoji_candidate_sha256([PlatformFontSha256::digest(&bytes)]);
    match KucUnicodeColorGlyphEvidenceCapture::capture(options) {
        Ok(evidence) => {
            assert_eq!(
                evidence.profile.profile_id,
                PlatformFontProfile::current().as_str()
            );
            assert!(evidence.chromatic_pixel_delta > 0);
        }
        Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnavailable { .. }) => {
            panic!("a present current-platform color face was reported unavailable")
        }
        Err(error) => panic!("current-platform evidence failed: {error}"),
    }
}

#[test]
fn current_platform_capture_runs_actual_pinned_root_when_a_compatible_candidate_exists() {
    let mut options = KucUnicodeColorGlyphEvidenceOptions::default();
    let policy = options.config.catalog_policy();

    let mut configured = None;
    for candidate in policy.emoji_candidates {
        let Ok(bytes) = std::fs::read(&candidate.source_file_path) else {
            continue;
        };
        let hash = PlatformFontSha256::digest(&bytes);
        let mut config = options.config.clone();
        config.emoji_candidates = vec![candidate.source_file_path.clone()];
        config.emoji_candidate_sha256 = vec![hash];

        let candidate_catalog = PlatformFontCatalog::new(config.catalog_policy());
        if candidate_catalog.emoji_face().is_available() {
            configured = Some(config);
            break;
        }
    }

    if let Some(config) = configured {
        options.config = config;
        let evidence = KucUnicodeColorGlyphEvidenceCapture::capture(options)
            .expect("found a resolved local emoji candidate");
        assert_eq!(
            evidence.profile.profile_id,
            PlatformFontProfile::current().as_str()
        );
        assert!(evidence.chromatic_pixel_delta > 0);
        assert!(!evidence.accesskit_text_input.value.is_empty());
    }
}

#[test]
fn current_profile_capture_is_fail_closed_when_face_hash_is_wrong() {
    let mut options = KucUnicodeColorGlyphEvidenceOptions::default();
    let catalog_policy = options.config.catalog_policy();
    if catalog_policy.emoji_candidates.is_empty() {
        assert!(matches!(
            KucUnicodeColorGlyphEvidenceCapture::capture(options),
            Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnavailable { .. })
        ));
        return;
    }
    options.config = options
        .config
        .with_emoji_candidate_sha256([PlatformFontSha256::digest(b"wrong emoji hash")]);
    let error = KucUnicodeColorGlyphEvidenceCapture::capture(options)
        .expect_err("pinned hash mismatch must fail");
    assert!(matches!(
        error,
        KucUnicodeColorGlyphEvidenceError::ColorEmojiUnpinned { .. }
            | KucUnicodeColorGlyphEvidenceError::ColorEmojiUnavailable { .. }
            | KucUnicodeColorGlyphEvidenceError::ColorEmojiFaceError { .. }
    ));
}

#[test]
fn unicode_error_display_uses_debug_fallback_without_panic() {
    let error = KucUnicodeColorGlyphEvidenceError::RootTrace("coverage".to_string());
    let rendered = format!("{error}");
    assert!(rendered.contains("RootTrace"));
    let another = KucUnicodeColorGlyphEvidenceError::InvalidAccessKitNode {
        reason: "node bounds are missing",
    };
    let rendered = format!("{another}");
    assert!(rendered.contains("InvalidAccessKitNode"));
}
