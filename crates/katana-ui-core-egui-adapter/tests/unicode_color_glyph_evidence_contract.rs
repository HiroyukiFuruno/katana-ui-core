use katana_ui_core::render_model::UiRect;
use katana_ui_core_egui_adapter::text_command_surface::{
    CONTROL_STAR_TEXT, IME_COMMIT_TEXT, IME_PREEDIT_TEXT, KucAccessKitNodeObservation, KucBounds,
    KucCaretObservation, KucHitTestObservation, KucImeTraceEvidence, KucRgbaCropEvidence,
    KucUnicodeColorGlyphEvidenceBuilder, KucUnicodeColorGlyphEvidenceCapture,
    KucUnicodeColorGlyphEvidenceError, KucUnicodeColorGlyphEvidenceInput,
    KucUnicodeColorGlyphEvidenceOptions, STAR_TEXT, ZWJ_TEXT,
};
use katana_ui_core_text_raster::{
    PlatformColorEmojiAvailability, PlatformColorEmojiFaceRecord, PlatformEmojiFontCandidate,
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
            bounds: KucBounds::new(0, 0, 640, 240),
        }),
        accesskit_text_snapshot_hash: "accesskit-hash".to_string(),
        root_frame_hash: "root-frame-hash".to_string(),
        root_record_hash: "root-record-hash".to_string(),
        root_rgba_hash: "root-rgba-hash".to_string(),
    }
}

#[test]
fn fixture_builder_rejects_missing_or_invalid_accesskit_node() {
    let mut missing = fixture_input();
    missing.accesskit_text_input = None;
    assert!(matches!(
        KucUnicodeColorGlyphEvidenceBuilder::build(missing),
        Err(KucUnicodeColorGlyphEvidenceError::MissingAccessKitNode)
    ));

    let mut missing_value = fixture_input();
    missing_value
        .accesskit_text_input
        .as_mut()
        .expect("fixture node")
        .value
        .clear();
    assert!(matches!(
        KucUnicodeColorGlyphEvidenceBuilder::build(missing_value),
        Err(KucUnicodeColorGlyphEvidenceError::InvalidAccessKitNode { .. })
    ));

    let mut invalid_bounds = fixture_input();
    invalid_bounds
        .accesskit_text_input
        .as_mut()
        .expect("fixture node")
        .bounds = KucBounds::new(0, 0, 0, 240);
    assert!(matches!(
        KucUnicodeColorGlyphEvidenceBuilder::build(invalid_bounds),
        Err(KucUnicodeColorGlyphEvidenceError::InvalidAccessKitNode { .. })
    ));
}

#[test]
fn fixture_builder_rejects_accesskit_role_and_scalar_mismatch() {
    let mut role_mismatch = fixture_input();
    role_mismatch
        .accesskit_text_input
        .as_mut()
        .expect("fixture node")
        .role = "TextInput".to_string();
    assert!(matches!(
        KucUnicodeColorGlyphEvidenceBuilder::build(role_mismatch),
        Err(KucUnicodeColorGlyphEvidenceError::AccessKitRoleMismatch { .. })
    ));

    let mut scalar_mismatch = fixture_input();
    scalar_mismatch
        .accesskit_text_input
        .as_mut()
        .expect("fixture node")
        .scalar_sequence[0] = 0;
    assert!(matches!(
        KucUnicodeColorGlyphEvidenceBuilder::build(scalar_mismatch),
        Err(KucUnicodeColorGlyphEvidenceError::AccessKitScalarSequenceMismatch { .. })
    ));
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
        katana_ui_core_text_raster::PlatformFontCatalogFingerprint::from_bytes([0xAB; 32]);
    assert!(matches!(
        KucUnicodeColorGlyphEvidenceBuilder::build(fingerprint_mismatch),
        Err(KucUnicodeColorGlyphEvidenceError::CatalogFingerprintMismatch { .. })
    ));
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
fn release_profile_requires_actual_pinned_color_emoji_root_evidence() {
    let mut options = KucUnicodeColorGlyphEvidenceOptions::default();
    let policy = options.config.catalog_policy();
    let candidate = match policy.emoji_candidates.first() {
        Some(candidate) => candidate,
        None => panic!("release profile has no configured color emoji candidate"),
    };
    let bytes = match std::fs::read(&candidate.source_file_path) {
        Ok(bytes) => bytes,
        Err(error) => panic!(
            "release profile color emoji candidate is unavailable at {}: {error}",
            candidate.source_file_path.display()
        ),
    };
    options.config = options
        .config
        .with_emoji_candidate_sha256([PlatformFontSha256::digest(&bytes)]);
    let evidence = KucUnicodeColorGlyphEvidenceCapture::capture(options)
        .unwrap_or_else(|error| panic!("release profile Unicode root evidence failed: {error}"));

    assert_eq!(
        evidence.profile.profile_id,
        PlatformFontProfile::current().as_str()
    );
    assert!(evidence.chromatic_pixel_delta > 0);
    assert_eq!("MultilineTextInput", evidence.accesskit_text_input.role);
    assert!(evidence.accesskit_text_input.value.contains("⭐️"));
    assert!(evidence.accesskit_text_input.value.contains('☆'));
    assert!(
        evidence
            .accesskit_text_input
            .scalar_sequence
            .windows(2)
            .any(|scalars| scalars == [0x2B50, 0xFE0F])
    );
}
